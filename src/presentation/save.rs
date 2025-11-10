//! Save presentation functionality

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::opc::packuri::PackURI;
use std::io::{Seek, Write};

/// Save the presentation to a writer
pub fn save<W: Write + Seek>(
    part: &mut PresentationPart,
    package: &mut crate::opc::package::Package,
    writer: W,
) -> Result<()> {
    use crate::opc::constants::{CONTENT_TYPE, RELATIONSHIP_TYPE};
    use crate::opc::serialized::PackageWriter;
    use crate::opc::relationships::Relationships;
    
    // Create package relationships
    let mut pkg_rels = Relationships::new();
    pkg_rels.add(
        "rId1".to_string(),
        RELATIONSHIP_TYPE::OFFICE_DOCUMENT.to_string(),
        "ppt/presentation.xml".to_string(),
        false,
    );
    
    // Add core properties relationship (rId3)
    pkg_rels.add(
        "rId3".to_string(),
        RELATIONSHIP_TYPE::CORE_PROPERTIES.to_string(),
        "docProps/core.xml".to_string(),
        false,
    );
    
    // Add app properties relationship (rId4)
    pkg_rels.add(
        "rId4".to_string(),
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties".to_string(),
        "docProps/app.xml".to_string(),
        false,
    );
    
    // Get the blob and URI directly instead of using trait objects
    use crate::opc::part::Part;
    let uri = Part::uri(part).clone();
    let content_type = Part::content_type(part).to_string();
    let relationships = Part::relationships(part).clone();
    let blob = Part::blob(part)?;
    
    // Create a simple part wrapper that owns its data
    struct OwnedPart {
        content_type: String,
        uri: crate::opc::packuri::PackURI,
        blob: Vec<u8>,
        relationships: Relationships,
    }
    
    impl crate::opc::part::Part for OwnedPart {
        fn content_type(&self) -> &str {
            &self.content_type
        }
        fn uri(&self) -> &crate::opc::packuri::PackURI {
            &self.uri
        }
        fn relationships(&self) -> &Relationships {
            &self.relationships
        }
        fn relationships_mut(&mut self) -> &mut Relationships {
            &mut self.relationships
        }
        fn blob(&self) -> Result<Vec<u8>> {
            Ok(self.blob.clone())
        }
        fn to_xml(&self) -> Result<String> {
            String::from_utf8(self.blob.clone())
                .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))
        }
        fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
            Err(crate::error::PptError::NotImplemented("OwnedPart::from_xml".to_string()))
        }
    }
    
    // Collect all parts: presentation part, core properties, slides, and their related parts
    let mut parts_map: std::collections::HashMap<PackURI, OwnedPart> = std::collections::HashMap::new();
    
    // Clear existing relationships and add relationships in python-pptx order
    // Order: rId1=slideMaster, rId2=printerSettings, rId3-6=properties, rId7+=slides
    *part.relationships_mut() = Relationships::new();
    
    // rId1: Slide Master (always first)
    part.relationships_mut().add(
        "rId1".to_string(),
        RELATIONSHIP_TYPE::SLIDE_MASTER.to_string(),
        "slideMasters/slideMaster1.xml".to_string(),
        false,
    );
    
    // rId2: Printer Settings (python-pptx always includes this)
    part.relationships_mut().add(
        "rId2".to_string(),
        RELATIONSHIP_TYPE::PRINTER_SETTINGS.to_string(),
        "printerSettings/printerSettings1.bin".to_string(),
        false,
    );
    
    // rId3-6: Core properties (python-pptx order)
    part.relationships_mut().add(
        "rId3".to_string(),
        RELATIONSHIP_TYPE::PRES_PROPS.to_string(),
        "presProps.xml".to_string(),
        false,
    );
    part.relationships_mut().add(
        "rId4".to_string(),
        RELATIONSHIP_TYPE::VIEW_PROPS.to_string(),
        "viewProps.xml".to_string(),
        false,
    );
    part.relationships_mut().add(
        "rId5".to_string(),
        RELATIONSHIP_TYPE::THEME.to_string(),
        "theme/theme1.xml".to_string(),
        false,
    );
    part.relationships_mut().add(
        "rId6".to_string(),
        RELATIONSHIP_TYPE::TABLE_STYLES.to_string(),
        "tableStyles.xml".to_string(),
        false,
    );
    
    // rId7+: Slides (python-pptx puts slides AFTER properties)
    let slide_count = part.slide_id_manager().all().len();
    for i in 0..slide_count {
        let r_id = format!("rId{}", i + 7); // rId7, rId8, rId9, ...
        part.relationships_mut().add(
            r_id,
            RELATIONSHIP_TYPE::SLIDE.to_string(),
            format!("slides/slide{}.xml", i + 1),
            false,
        );
    }
    
    // Generate presentation.xml with slide IDs
    let presentation_xml = part.generate_presentation_xml();
    
    // Add presentation part with generated XML and relationships
    parts_map.insert(uri.clone(), OwnedPart {
        content_type: content_type.clone(),
        uri: uri.clone(),
        blob: presentation_xml?.as_bytes().to_vec(),
        relationships: part.relationships().clone(),
    });
    
    // Add core properties part if it exists
    if let Ok(core_props) = part.core_properties() {
        use crate::opc::part::Part;
        let core_blob = Part::blob(&core_props)?;
        let core_uri = Part::uri(&core_props).clone();
        let core_content_type = Part::content_type(&core_props);
        parts_map.insert(core_uri.clone(), OwnedPart {
            content_type: core_content_type.to_string(),
            uri: core_uri,
            blob: core_blob,
            relationships: Relationships::new(),
        });
    }
    
    // Add printer settings (printerSettings/printerSettings1.bin)
    // Minimal binary file to match python-pptx structure
    let printer_settings_bin: Vec<u8> = vec![
        0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let printer_settings_uri = PackURI::new("/ppt/printerSettings/printerSettings1.bin")?;
    parts_map.insert(printer_settings_uri.clone(), OwnedPart {
        content_type: CONTENT_TYPE::PML_PRINTER_SETTINGS.to_string(),
        uri: printer_settings_uri,
        blob: printer_settings_bin,
        relationships: Relationships::new(),
    });
    
    // Add presentation properties (presProps.xml)
    let pres_props_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentationPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:extLst>
    <p:ext uri="{E76CE94A-603C-4142-B9EB-6D1370010A27}"><p14:discardImageEditData xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" val="0"/></p:ext>
    <p:ext uri="{D31A062A-798A-4329-ABDD-BBA856620510}"><p14:defaultImageDpi xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" val="0"/></p:ext>
  </p:extLst>
</p:presentationPr>"#;
    let pres_props_uri = PackURI::new("/ppt/presProps.xml")?;
    parts_map.insert(pres_props_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.presProps+xml".to_string(),
        uri: pres_props_uri,
        blob: pres_props_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Presentation relationships are automatically generated by PackageWriter
    // from the relationships field in the presentation part (line 105)
    // No need to manually add ppt/_rels/presentation.xml.rels
    
    // Add view properties (viewProps.xml)
    let view_props_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:viewPr xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" lastView="sldThumbnail">
  <p:normalViewPr>
    <p:restoredLeft sz="15616"/>
    <p:restoredTop sz="94595"/>
  </p:normalViewPr>
  <p:slideViewPr/>
  <p:outlineViewPr/>
  <p:notesViewPr/>
  <p:handoutViewPr/>
  <p:forceOffscreen val="0"/>
</p:viewPr>"#;
    let view_props_uri = PackURI::new("/ppt/viewProps.xml")?;
    parts_map.insert(view_props_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml".to_string(),
        uri: view_props_uri,
        blob: view_props_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Add app properties (docProps/app.xml)
    let app_props_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <TotalTime>0</TotalTime>
  <Words>0</Words>
  <Application>ppt-rs</Application>
  <PresentationFormat>On-screen Show (4:3)</PresentationFormat>
  <Paragraphs>0</Paragraphs>
  <Slides>0</Slides>
  <Notes>0</Notes>
  <HiddenSlides>0</HiddenSlides>
  <MMClips>0</MMClips>
</Properties>"#;
    let app_props_uri = PackURI::new("/docProps/app.xml")?;
    parts_map.insert(app_props_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.extended-properties+xml".to_string(),
        uri: app_props_uri,
        blob: app_props_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Add theme (ppt/theme/theme1.xml)
    let theme_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="Office">
      <a:dk1><a:srgbClr val="000000"/></a:dk1>
      <a:lt1><a:srgbClr val="FFFFFF"/></a:lt1>
      <a:dk2><a:srgbClr val="1F497D"/></a:dk2>
      <a:lt2><a:srgbClr val="EBEBEB"/></a:lt2>
      <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
      <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
      <a:accent3><a:srgbClr val="A5A5A5"/></a:accent3>
      <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
      <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
      <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
      <a:hyperlink><a:srgbClr val="0563C1"/></a:hyperlink>
      <a:folHyperlink><a:srgbClr val="954F72"/></a:folHyperlink>
    </a:clrScheme>
    <a:fontScheme name="Office">
      <a:majorFont>
        <a:latin typeface="Calibri Light" pitchFamily="2" charset="0"/>
        <a:ea typeface="" pitchFamily="2" charset="0"/>
        <a:cs typeface="" pitchFamily="2" charset="0"/>
      </a:majorFont>
      <a:minorFont>
        <a:latin typeface="Calibri" pitchFamily="2" charset="0"/>
        <a:ea typeface="" pitchFamily="2" charset="0"/>
        <a:cs typeface="" pitchFamily="2" charset="0"/>
      </a:minorFont>
    </a:fontScheme>
    <a:fmtScheme name="Office">
      <a:fillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0"><a:schemeClr val="phClr"><a:lumMod val="110000"/><a:satMod val="105000"/><a:tint val="67000"/></a:schemeClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="phClr"><a:lumMod val="105000"/><a:satMod val="103000"/><a:tint val="73000"/></a:schemeClr></a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0"><a:schemeClr val="phClr"><a:satMod val="103000"/><a:lumMod val="102000"/><a:tint val="94000"/></a:schemeClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="phClr"><a:satMod val="110000"/><a:lumMod val="100000"/><a:shade val="100000"/></a:schemeClr></a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
      </a:fillStyleLst>
      <a:lnStyleLst>
        <a:ln w="9525"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:prstCap val="flat"/><a:round/></a:ln>
        <a:ln w="25400"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:prstCap val="flat"/><a:round/></a:ln>
        <a:ln w="38100"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/><a:prstCap val="flat"/><a:round/></a:ln>
      </a:lnStyleLst>
      <a:effectStyleLst>
        <a:effectLst/>
        <a:effectLst/>
        <a:effectLst>
          <a:outerShdw blurRad="101600" dist="38100" dir="2700000" algn="tl" rotWithShape="0"><a:srgbClr val="000000"><a:alpha val="38000"/></a:srgbClr></a:outerShdw>
        </a:effectLst>
      </a:effectStyleLst>
      <a:bgFillStyleLst>
        <a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="40000"/><a:satMod val="350000"/></a:schemeClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="phClr"><a:tint val="45000"/><a:shade val="99000"/><a:satMod val="350000"/></a:schemeClr></a:gs>
          </a:gsLst>
          <a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path>
        </a:gradFill>
        <a:gradFill rotWithShape="1">
          <a:gsLst>
            <a:gs pos="0"><a:schemeClr val="phClr"><a:satMod val="300000"/><a:tint val="80000"/></a:schemeClr></a:gs>
            <a:gs pos="100000"><a:schemeClr val="phClr"><a:satMod val="300000"/><a:shade val="80000"/></a:schemeClr></a:gs>
          </a:gsLst>
          <a:lin ang="5400000" scaled="0"/>
        </a:gradFill>
      </a:bgFillStyleLst>
    </a:fmtScheme>
  </a:themeElements>
</a:theme>"#;
    let theme_uri = PackURI::new("/ppt/theme/theme1.xml")?;
    parts_map.insert(theme_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.theme+xml".to_string(),
        uri: theme_uri,
        blob: theme_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Add slide master (ppt/slideMasters/slideMaster1.xml)
    let mut slide_master = crate::slide::SlideMaster::new();
    let predefined_layouts = crate::slide::PredefinedLayouts::new();
    for layout_id in predefined_layouts.layout_ids() {
        slide_master.add_layout_id(layout_id);
    }
    let master_xml = slide_master.to_xml();
    let master_uri = PackURI::new("/ppt/slideMasters/slideMaster1.xml")?;
    parts_map.insert(master_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml".to_string(),
        uri: master_uri,
        blob: master_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Add slide master relationships (ppt/slideMasters/_rels/slideMaster1.xml.rels)
    let master_rels_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>"#;
    let master_rels_uri = PackURI::new("/ppt/slideMasters/_rels/slideMaster1.xml.rels")?;
    parts_map.insert(master_rels_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-package.relationships+xml".to_string(),
        uri: master_rels_uri,
        blob: master_rels_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Add all 11 slide layouts
    for layout in predefined_layouts.all() {
        let layout_index = layout.index();
        let layout_xml = layout.to_xml();
        let layout_uri = PackURI::new(&format!("/ppt/slideLayouts/slideLayout{}.xml", layout_index))?;
        parts_map.insert(layout_uri.clone(), OwnedPart {
            content_type: "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml".to_string(),
            uri: layout_uri,
            blob: layout_xml.as_bytes().to_vec(),
            relationships: Relationships::new(),
        });
        
        // Add layout relationships
        let layout_rels_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#;
        let layout_rels_uri = PackURI::new(&format!("/ppt/slideLayouts/_rels/slideLayout{}.xml.rels", layout_index))?;
        parts_map.insert(layout_rels_uri.clone(), OwnedPart {
            content_type: "application/vnd.openxmlformats-package.relationships+xml".to_string(),
            uri: layout_rels_uri,
            blob: layout_rels_xml.as_bytes().to_vec(),
            relationships: Relationships::new(),
        });
    }
    
    // Add table styles (ppt/tableStyles.xml)
    let table_styles_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:tblStyleLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" def="{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}"/>"#;
    let table_styles_uri = PackURI::new("/ppt/tableStyles.xml")?;
    parts_map.insert(table_styles_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml".to_string(),
        uri: table_styles_uri,
        blob: table_styles_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
    // Collect all parts from internal package
    // This includes slides, images, and other related parts
    // Slides will be generated manually below - no need to access slides_collection
    
    // Generate slide XML files manually (slides from package have incorrect URIs)
    {
        let slide_count = part.slide_id_manager().all().len();
        for i in 0..slide_count {
            let slide_index = i + 1;
            let slide_uri = PackURI::new(&format!("/ppt/slides/slide{}.xml", slide_index))?;
            
            // Try to get slide from package first (which has placeholders), otherwise use default
            let slide_xml = if let Some(slide_part) = package.get_part(&slide_uri) {
                // Use the slide from package (which has placeholders from add_slide)
                use crate::opc::part::Part;
                if let Ok(blob) = Part::blob(slide_part) {
                    if let Ok(xml_str) = String::from_utf8(blob) {
                        // Compact the XML (remove newlines and extra spaces)
                        xml_str.lines().map(|l| l.trim()).collect::<Vec<_>>().join("")
                    } else {
                        // Fallback to default
                        r#"<?xml version='1.0' encoding='UTF-8' standalone='yes'?><p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>"#.to_string()
                    }
                } else {
                    // Fallback to default
                    r#"<?xml version='1.0' encoding='UTF-8' standalone='yes'?><p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>"#.to_string()
                }
            } else {
                // Fallback to default
                r#"<?xml version='1.0' encoding='UTF-8' standalone='yes'?><p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>"#.to_string()
            };
            
            parts_map.insert(slide_uri.clone(), OwnedPart {
                content_type: "application/vnd.openxmlformats-officedocument.presentationml.slide+xml".to_string(),
                uri: slide_uri.clone(),
                blob: slide_xml.as_bytes().to_vec(),
                relationships: Relationships::new(),
            });
            
            // Generate slide relationship file
            let slide_rels_xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>"#;
            let slide_rels_uri = PackURI::new(&format!("/ppt/slides/_rels/slide{}.xml.rels", slide_index))?;
            parts_map.insert(slide_rels_uri.clone(), OwnedPart {
                content_type: "application/vnd.openxmlformats-package.relationships+xml".to_string(),
                uri: slide_rels_uri,
                blob: slide_rels_xml.as_bytes().to_vec(),
                relationships: Relationships::new(),
            });
        }
    }
    
    // Now collect all parts from package (including slides and images)
    // The slides_collection is dropped, so we can use package again
    for part in package.iter_parts() {
        use crate::opc::part::Part;
        let part_uri = Part::uri(part.as_ref()).clone();
        
        // Skip [Content_Types].xml - PackageWriter generates it automatically
        // Skip printerSettings, thumbnail, and slides - they cause issues
        // Slides will be generated manually below
        if part_uri.as_str() == "/[Content_Types].xml" 
            || part_uri.as_str().contains("printerSettings")
            || part_uri.as_str().contains("thumbnail")
            || part_uri.as_str().contains("/slides/") {
            continue;
        }
        
        let part_blob = Part::blob(part.as_ref())?;
        let part_content_type = Part::content_type(part.as_ref());
        let part_rels = Part::relationships(part.as_ref()).clone();
        
        // Add part to collection if not already there
        if !parts_map.contains_key(&part_uri) {
            parts_map.insert(part_uri.clone(), OwnedPart {
                content_type: part_content_type.to_string(),
                uri: part_uri.clone(),
                blob: part_blob,
                relationships: part_rels.clone(),
            });
        }
        
        // Collect image parts from slide relationships
        for (_img_r_id, img_rel) in part_rels.iter() {
            use crate::opc::constants::RELATIONSHIP_TYPE;
            if img_rel.rel_type == RELATIONSHIP_TYPE::IMAGE && !img_rel.is_external {
                // Resolve image URI
                let img_uri = if img_rel.target.starts_with('/') {
                    PackURI::new(&img_rel.target)?
                } else {
                    // Relative path - resolve from part's base URI
                    let base_uri_str = part_uri.base_uri();
                    let resolved = if base_uri_str == "/" {
                        format!("/{}", img_rel.target)
                    } else {
                        format!("{}/{}", base_uri_str, img_rel.target)
                    };
                    PackURI::new(&resolved)?
                };
                
                // Try to get image part from package
                if let Some(img_part) = package.get_part(&img_uri) {
                    let img_blob = Part::blob(img_part)?;
                    let img_content_type = Part::content_type(img_part);
                    
                    if !parts_map.contains_key(&img_uri) {
                        parts_map.insert(img_uri.clone(), OwnedPart {
                            content_type: img_content_type.to_string(),
                            uri: img_uri,
                            blob: img_blob,
                            relationships: Relationships::new(),
                        });
                    }
                }
            }
        }
    }
    
    // Build content types manager
    let mut content_types_manager = crate::opc::ContentTypesManager::new();
    
    // Add slides to content types
    let slide_count = part.slide_id_manager().all().len();
    for i in 1..=slide_count {
        content_types_manager.add_slide(i);
    }
    
    // Add images to content types
    for (uri, _part) in &parts_map {
        let uri_str = uri.membername();
        if uri_str.starts_with("ppt/media/") {
            let content_type = if uri_str.ends_with(".png") {
                "image/png"
            } else if uri_str.ends_with(".jpg") || uri_str.ends_with(".jpeg") {
                "image/jpeg"
            } else if uri_str.ends_with(".gif") {
                "image/gif"
            } else {
                "application/octet-stream"
            };
            content_types_manager.add_image(&format!("/{}", uri_str), content_type);
        }
    }
    
    // Add thumbnail image (docProps/thumbnail.jpeg)
    use crate::util::generate_thumbnail_jpeg;
    let thumbnail_jpeg = generate_thumbnail_jpeg()?;
    let thumbnail_uri = PackURI::new("/docProps/thumbnail.jpeg")?;
    parts_map.insert(thumbnail_uri.clone(), OwnedPart {
        content_type: "image/jpeg".to_string(),
        uri: thumbnail_uri,
        blob: thumbnail_jpeg,
        relationships: Relationships::new(),
    });
    
    // Convert parts_map to Vec
    let parts: Vec<Box<dyn crate::opc::part::Part>> = parts_map
        .into_values()
        .map(|p| Box::new(p) as Box<dyn crate::opc::part::Part>)
        .collect();
    
    // Note: [Content_Types].xml is automatically generated by PackageWriter
    // Don't add it as a part here to avoid duplicates
    
    // Write the package
    PackageWriter::write(writer, &pkg_rels, &parts)
}
