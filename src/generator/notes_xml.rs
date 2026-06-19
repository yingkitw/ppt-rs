//! Notes XML generation for speaker notes

use crate::core::escape_xml;

/// Generate a proper GUID for field IDs
/// Format: {XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}
fn generate_field_guid(slide_num: usize) -> String {
    // Generate a deterministic but valid-looking GUID based on slide number
    format!("{{B0E4A5D7-2C3F-4A8B-9E1D-{:012X}}}", slide_num)
}

/// Generate notes slide XML for speaker notes
pub fn create_notes_xml(slide_num: usize, notes_text: &str) -> String {
    let escaped_notes = escape_xml(notes_text);
    let field_guid = generate_field_guid(slide_num);
    
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
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
<p:cNvPr id="2" name="Slide Image Placeholder 1"/>
<p:cNvSpPr>
<a:spLocks noGrp="1" noRot="1" noChangeAspect="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="sldImg"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Notes Placeholder 2"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="body" idx="1"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" dirty="0"/>
<a:t>{escaped_notes}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Slide Number Placeholder 3"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="sldNum" sz="quarter" idx="10"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:fld id="{field_guid}" type="slidenum">
<a:rPr lang="en-US"/>
<a:t>{slide_num}</a:t>
</a:fld>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:notes>"#)
}

/// Generate notes slide relationship XML
pub fn create_notes_rels_xml(slide_num: usize) -> String {
    format!(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide" Target="../slides/slide{slide_num}.xml"/>
<Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster" Target="../notesMasters/notesMaster1.xml"/>
</Relationships>"#)
}

/// Generate notes master XML
pub fn create_notes_master_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notesMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg><p:bgRef idx="1001"><a:schemeClr val="bg1"/></p:bgRef></p:bg>
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
<p:cNvPr id="2" name="Header Placeholder 1"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="hdr" sz="quarter"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="2971800" cy="458788"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
</p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="91440" tIns="45720" rIns="91440" bIns="45720" rtlCol="0"/>
<a:lstStyle>
<a:lvl1pPr algn="l">
<a:defRPr sz="1200"/>
</a:lvl1pPr>
</a:lstStyle>
<a:p>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Slide Image Placeholder 2"/>
<p:cNvSpPr>
<a:spLocks noGrp="1" noRot="1" noChangeAspect="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="sldImg" idx="2"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="685800" y="1143000"/>
<a:ext cx="5486400" cy="3086100"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
<a:noFill/>
<a:ln w="12700">
<a:solidFill>
<a:prstClr val="black"/>
</a:solidFill>
</a:ln>
</p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="91440" tIns="45720" rIns="91440" bIns="45720" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Notes Placeholder 3"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="body" sz="quarter" idx="3"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="685800" y="4400550"/>
<a:ext cx="5486400" cy="3600450"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
</p:spPr>
<p:txBody>
<a:bodyPr vert="horz" lIns="91440" tIns="45720" rIns="91440" bIns="45720" rtlCol="0"/>
<a:lstStyle/>
<a:p>
<a:pPr lvl="0"/>
<a:r>
<a:rPr lang="en-US"/>
<a:t>Click to edit Master text styles</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/>
<p:notesStyle>
<a:lvl1pPr marL="0" algn="l" defTabSz="914400" rtl="0" eaLnBrk="1" latinLnBrk="0" hangingPunct="1">
<a:defRPr sz="1200" kern="1200">
<a:solidFill>
<a:schemeClr val="tx1"/>
</a:solidFill>
<a:latin typeface="+mn-lt"/>
<a:ea typeface="+mn-ea"/>
<a:cs typeface="+mn-cs"/>
</a:defRPr>
</a:lvl1pPr>
</p:notesStyle>
</p:notesMaster>"#.to_string()
}

/// Generate notes master relationship XML
pub fn create_notes_master_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="../theme/theme2.xml"/>
</Relationships>"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_notes_xml() {
        let xml = create_notes_xml(1, "These are my speaker notes");
        assert!(xml.contains("p:notes"));
        assert!(xml.contains("These are my speaker notes"));
        assert!(xml.contains("Notes Placeholder"));
    }

    #[test]
    fn test_create_notes_xml_escapes_special_chars() {
        let xml = create_notes_xml(1, "Notes with <special> & \"chars\"");
        assert!(xml.contains("&lt;special&gt;"));
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&quot;chars&quot;"));
    }

    #[test]
    fn test_create_notes_rels_xml() {
        let xml = create_notes_rels_xml(3);
        assert!(xml.contains("slide3.xml"));
        assert!(xml.contains("notesMaster1.xml"));
    }

    #[test]
    fn test_create_notes_master_xml() {
        let xml = create_notes_master_xml();
        assert!(xml.contains("p:notesMaster"));
        assert!(xml.contains("<p:bg>"));
        assert!(xml.contains("Notes Placeholder"));
    }

    #[test]
    fn test_create_notes_master_rels_xml() {
        let xml = create_notes_master_rels_xml();
        assert!(xml.contains("theme2.xml"));
    }
}
