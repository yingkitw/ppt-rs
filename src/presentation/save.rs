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
    use crate::opc::constants::RELATIONSHIP_TYPE;
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
    
    // Add thumbnail relationship (rId2)
    pkg_rels.add(
        "rId2".to_string(),
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail".to_string(),
        "docProps/thumbnail.jpeg".to_string(),
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
    
    // First, collect all slides from the package to populate slide IDs
    {
        use crate::slide::Slides;
        let mut slides_collection = Slides::new(part);
        let slide_count = slides_collection.len();
        
        // Add slide IDs to the manager
        for i in 0..slide_count {
            let rel_id = format!("rId{}", 7 + i); // rId7 onwards for slides
            part.slide_id_manager_mut().add_slide(rel_id);
        }
    }
    
    // Generate presentation.xml with slide IDs
    let presentation_xml = part.generate_presentation_xml();
    
    // Add presentation part with generated XML
    parts_map.insert(uri.clone(), OwnedPart {
        content_type: content_type.clone(),
        uri: uri.clone(),
        blob: presentation_xml.as_bytes().to_vec(),
        relationships: relationships.clone(),
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
    
    // Add presentation relationships (ppt/_rels/presentation.xml.rels)
    let mut rel_manager = crate::presentation::PresentationRelationshipManager::new();
    
    // Add slide relationships dynamically
    let slide_count = part.slide_id_manager().all().len();
    for i in 1..=slide_count {
        rel_manager.add_slide_rel(i);
    }
    
    let pres_rels_xml = rel_manager.to_xml();
    
    let pres_rels_uri = PackURI::new("/ppt/_rels/presentation.xml.rels")?;
    parts_map.insert(pres_rels_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-package.relationships+xml".to_string(),
        uri: pres_rels_uri,
        blob: pres_rels_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    });
    
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
    
    // Add minimal thumbnail JPEG (1x1 transparent pixel)
    let thumbnail_jpeg = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08,
        0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12,
        0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20, 0x24, 0x2E, 0x27, 0x20,
        0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27,
        0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x0B, 0x08, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00, 0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01,
        0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xFF, 0xC4, 0x00, 0xB5, 0x10, 0x00, 0x02, 0x01, 0x03,
        0x03, 0x02, 0x04, 0x03, 0x05, 0x05, 0x04, 0x04, 0x00, 0x00, 0x01, 0x7D, 0x01, 0x02, 0x03, 0x00,
        0x04, 0x11, 0x05, 0x12, 0x21, 0x31, 0x41, 0x06, 0x13, 0x51, 0x61, 0x07, 0x22, 0x71, 0x14, 0x32,
        0x81, 0x91, 0xA1, 0x08, 0x23, 0x42, 0xB1, 0xC1, 0x15, 0x52, 0xD1, 0xF0, 0x24, 0x33, 0x62, 0x72,
        0x82, 0x09, 0x0A, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x34, 0x35,
        0x36, 0x37, 0x38, 0x39, 0x3A, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x53, 0x54, 0x55,
        0x56, 0x57, 0x58, 0x59, 0x5A, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x69, 0x6A, 0x73, 0x74, 0x75,
        0x76, 0x77, 0x78, 0x79, 0x7A, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x92, 0x93, 0x94,
        0x95, 0x96, 0x97, 0x98, 0x99, 0x9A, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8, 0xA9, 0xAA, 0xB2,
        0xB3, 0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7, 0xC8, 0xC9,
        0xCA, 0xD2, 0xD3, 0xD4, 0xD5, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xE1, 0xE2, 0xE3, 0xE4, 0xE5, 0xE6,
        0xE7, 0xE8, 0xE9, 0xEA, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7, 0xF8, 0xF9, 0xFA, 0xFF, 0xDA,
        0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xFB, 0xD0, 0xFF, 0xD9
    ];
    let thumbnail_uri = PackURI::new("/docProps/thumbnail.jpeg")?;
    parts_map.insert(thumbnail_uri.clone(), OwnedPart {
        content_type: "image/jpeg".to_string(),
        uri: thumbnail_uri,
        blob: thumbnail_jpeg,
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
    
    // Add minimal printer settings binary file
    let printer_settings = vec![0u8; 0]; // Empty binary file
    let printer_settings_uri = PackURI::new("/ppt/printerSettings/printerSettings1.bin")?;
    parts_map.insert(printer_settings_uri.clone(), OwnedPart {
        content_type: "application/vnd.openxmlformats-officedocument.presentationml.printerSettings".to_string(),
        uri: printer_settings_uri,
        blob: printer_settings,
        relationships: Relationships::new(),
    });
    
    // Collect all parts from internal package
    // This includes slides, images, and other related parts
    // First, ensure all slides are loaded into the package by accessing them
    {
        use crate::slide::Slides;
        let mut slides_collection = Slides::new(part);
        let slide_count = slides_collection.len();
        // Access slides to ensure they're added to package
        for i in 0..slide_count {
            let _ = slides_collection.get(i, package); // This will add slide parts to package if not already there
        }
        // slides_collection is dropped here, releasing the borrow
    }
    
    // Generate slide XML files and relationship files for each slide
    {
        let slide_count = part.slide_id_manager().all().len();
        for i in 0..slide_count {
            let slide_index = i + 1;
            let slide_uri_str = format!("/ppt/slides/slide{}.xml", slide_index);
            let slide_uri = PackURI::new(&slide_uri_str)?;
            
            // Try to get slide content from package if it exists
            let slide_xml = if let Some(slide_part) = package.get_part(&slide_uri) {
                use crate::opc::part::Part;
                // Use existing slide XML if available
                match Part::to_xml(slide_part) {
                    Ok(xml) => xml,
                    Err(_) => {
                        // Fall back to blank slide
                        generate_blank_slide_xml()
                    }
                }
            } else {
                // Generate minimal slide XML with blank layout
                generate_blank_slide_xml()
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
    
    // Helper function to generate blank slide XML matching python-pptx format
    fn generate_blank_slide_xml() -> String {
        // Compact format matching python-pptx exactly
        r#"<?xml version='1.0' encoding='UTF-8' standalone='yes'?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sld>"#.to_string()
    }
    
    // Now collect all parts from package (including slides and images)
    // The slides_collection is dropped, so we can use package again
    for part in package.iter_parts() {
        use crate::opc::part::Part;
        let part_blob = Part::blob(part.as_ref())?;
        let part_uri = Part::uri(part.as_ref()).clone();
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
    
    // Convert parts_map to Vec
    let mut parts: Vec<Box<dyn crate::opc::part::Part>> = parts_map
        .into_values()
        .map(|p| Box::new(p) as Box<dyn crate::opc::part::Part>)
        .collect();
    
    // Add content types as a part
    let content_types_xml = content_types_manager.to_xml();
    let content_types_uri = PackURI::new("/[Content_Types].xml")?;
    parts.push(Box::new(OwnedPart {
        content_type: "application/xml".to_string(),
        uri: content_types_uri,
        blob: content_types_xml.as_bytes().to_vec(),
        relationships: Relationships::new(),
    }));
    
    // Write the package
    PackageWriter::write(writer, &pkg_rels, &parts)
}
