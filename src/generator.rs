//! PPTX file generator - creates proper ZIP-based PPTX files

use std::io::{Write, Cursor};
use zip::ZipWriter;
use zip::write::FileOptions;

/// Create a minimal but valid PPTX file
pub fn create_pptx(title: &str, slides: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);
    let options = FileOptions::default();

    // 1. Create [Content_Types].xml - must include all slides
    let content_types = create_content_types_xml(slides);
    zip.start_file("[Content_Types].xml", options)?;
    zip.write_all(content_types.as_bytes())?;

    // 2. Create _rels/.rels
    let rels = create_rels_xml();
    zip.start_file("_rels/.rels", options)?;
    zip.write_all(rels.as_bytes())?;

    // 3. Create ppt/_rels/presentation.xml.rels
    let pres_rels = create_presentation_rels_xml(slides);
    zip.start_file("ppt/_rels/presentation.xml.rels", options)?;
    zip.write_all(pres_rels.as_bytes())?;

    // 4. Create ppt/presentation.xml
    let presentation = create_presentation_xml(title, slides);
    zip.start_file("ppt/presentation.xml", options)?;
    zip.write_all(presentation.as_bytes())?;

    // 5. Create ppt/slides/slide1.xml through slideN.xml
    for i in 1..=slides {
        let slide_xml = create_slide_xml(i, title);
        zip.start_file(&format!("ppt/slides/slide{}.xml", i), options)?;
        zip.write_all(slide_xml.as_bytes())?;
    }

    // 6. Create ppt/slides/_rels/slide1.xml.rels through slideN.xml.rels
    for i in 1..=slides {
        let slide_rels = create_slide_rels_xml();
        zip.start_file(&format!("ppt/slides/_rels/slide{}.xml.rels", i), options)?;
        zip.write_all(slide_rels.as_bytes())?;
    }

    // 7. Create ppt/slideLayouts/slideLayout1.xml
    let slide_layout = create_slide_layout_xml();
    zip.start_file("ppt/slideLayouts/slideLayout1.xml", options)?;
    zip.write_all(slide_layout.as_bytes())?;

    // 8. Create ppt/slideLayouts/_rels/slideLayout1.xml.rels
    let layout_rels = create_layout_rels_xml();
    zip.start_file("ppt/slideLayouts/_rels/slideLayout1.xml.rels", options)?;
    zip.write_all(layout_rels.as_bytes())?;

    // 9. Create ppt/slideMasters/slideMaster1.xml
    let slide_master = create_slide_master_xml();
    zip.start_file("ppt/slideMasters/slideMaster1.xml", options)?;
    zip.write_all(slide_master.as_bytes())?;

    // 10. Create ppt/slideMasters/_rels/slideMaster1.xml.rels
    let master_rels = create_master_rels_xml();
    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", options)?;
    zip.write_all(master_rels.as_bytes())?;

    // 11. Create ppt/theme/theme1.xml
    let theme = create_theme_xml();
    zip.start_file("ppt/theme/theme1.xml", options)?;
    zip.write_all(theme.as_bytes())?;

    // 12. Create docProps/core.xml
    let core_props = create_core_props_xml(title);
    zip.start_file("docProps/core.xml", options)?;
    zip.write_all(core_props.as_bytes())?;

    // 13. Create docProps/app.xml
    let app_props = create_app_props_xml(slides);
    zip.start_file("docProps/app.xml", options)?;
    zip.write_all(app_props.as_bytes())?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}

fn create_content_types_xml(slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>"#.to_string();

    // Add each slide
    for i in 1..=slides {
        xml.push_str(&format!(
            "\n<Override PartName=\"/ppt/slides/slide{}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>",
            i
        ));
    }

    xml.push_str(r#"
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#);
    xml
}

fn create_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
<Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#.to_string()
}

fn create_presentation_rels_xml(slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>"#.to_string();

    for i in 1..=slides {
        xml.push_str(&format!(
            "\n    <Relationship Id=\"rId{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{}.xml\"/>",
            i + 2,
            i
        ));
    }

    xml.push_str("\n</Relationships>");
    xml
}

fn create_presentation_xml(_title: &str, slides: usize) -> String {
    let mut xml = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1">
<p:sldMasterIdLst>
<p:sldMasterId id="2147483648" r:id="rId1"/>
</p:sldMasterIdLst>
<p:sldIdLst>"#.to_string();

    for i in 1..=slides {
        xml.push_str(&format!(
            "\n<p:sldId id=\"{}\" r:id=\"rId{}\"/>",
            256 + i,
            i + 2
        ));
    }

    xml.push_str(r#"
</p:sldIdLst>
<p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>
<p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#);
    xml
}

fn create_slide_xml(slide_num: usize, title: &str) -> String {
    let slide_title = if slide_num == 1 {
        title.to_string()
    } else {
        format!("Slide {}", slide_num)
    };
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title 1"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="ctrTitle"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" smtClean="0"/>
<a:t>{}</a:t>
</a:r>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#,
        slide_title
    )
}

fn create_slide_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>"#.to_string()
}

fn create_slide_layout_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="blank" preserve="1">
<p:cSld name="Blank">
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sldLayout>"#.to_string()
}

fn create_layout_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="../slideMasters/slideMaster1.xml"/>
</Relationships>"#.to_string()
}

fn create_slide_master_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
<p:sldLayoutIdLst>
<p:sldLayoutId id="2147483649" r:id="rId1"/>
</p:sldLayoutIdLst>
</p:sldMaster>"#.to_string()
}

fn create_master_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme1.xml"/>
</Relationships>"#.to_string()
}

fn create_theme_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="Office Theme">
<a:themeElements>
<a:clrScheme name="Office">
<a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
<a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
<a:dk2><a:srgbClr val="1F497D"/></a:dk2>
<a:lt2><a:srgbClr val="EEECE1"/></a:lt2>
<a:accent1><a:srgbClr val="4F81BD"/></a:accent1>
<a:accent2><a:srgbClr val="C0504D"/></a:accent2>
<a:accent3><a:srgbClr val="9BBB59"/></a:accent3>
<a:accent4><a:srgbClr val="8064A2"/></a:accent4>
<a:accent5><a:srgbClr val="4BACC6"/></a:accent5>
<a:accent6><a:srgbClr val="F79646"/></a:accent6>
<a:hlink><a:srgbClr val="0000FF"/></a:hlink>
<a:folHlink><a:srgbClr val="800080"/></a:folHlink>
</a:clrScheme>
<a:fontScheme name="Office">
<a:majorFont>
<a:latin typeface="Calibri"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:majorFont>
<a:minorFont>
<a:latin typeface="Calibri"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:minorFont>
</a:fontScheme>
<a:fmtScheme name="Office">
<a:fillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="50000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="35000"><a:schemeClr val="phClr"><a:tint val="37000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:tint val="15000"/><a:satMod val="350000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="1"/></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:shade val="51000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="80000"><a:schemeClr val="phClr"><a:shade val="93000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="94000"/><a:satMod val="135000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="0"/></a:gradFill>
</a:fillStyleLst>
<a:lnStyleLst>
<a:ln w="9525" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"><a:shade val="95000"/><a:satMod val="105000"/></a:schemeClr></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="25400" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="38100" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
</a:lnStyleLst>
<a:effectStyleLst>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
</a:effectStyleLst>
<a:bgFillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="40000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="40000"><a:schemeClr val="phClr"><a:tint val="45000"/><a:shade val="99000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="20000"/><a:satMod val="255000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="-80000" r="50000" b="180000"/></a:path></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="80000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="30000"/><a:satMod val="200000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path></a:gradFill>
</a:bgFillStyleLst>
</a:fmtScheme>
</a:themeElements>
<a:objectDefaults/>
<a:extraClrSchemeLst/>
</a:theme>"#.to_string()
}

fn create_core_props_xml(title: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<dc:title>{}</dc:title>
<dc:creator>PPTX Generator</dc:creator>
<cp:lastModifiedBy>PPTX Generator</cp:lastModifiedBy>
<cp:revision>1</cp:revision>
</cp:coreProperties>"#,
        title
    )
}

fn create_app_props_xml(slides: usize) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
<TotalTime>0</TotalTime>
<Words>0</Words>
<Application>PPTX Generator</Application>
<PresentationFormat>On-screen Show (4:3)</PresentationFormat>
<Paragraphs>0</Paragraphs>
<Slides>{}</Slides>
<Notes>0</Notes>
<HiddenSlides>0</HiddenSlides>
<MMClips>0</MMClips>
<ScaleCrop>false</ScaleCrop>
<LinksUpToDate>false</LinksUpToDate>
<SharedDoc>false</SharedDoc>
<HyperlinksChanged>false</HyperlinksChanged>
<AppVersion>16.0000</AppVersion>
</Properties>"#,
        slides
    )
}
