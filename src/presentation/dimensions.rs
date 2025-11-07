//! Slide dimensions handling

use crate::error::{PptError, Result};
use crate::parts::presentation::PresentationPart;

/// Get slide width in EMU (English Metric Units)
pub fn slide_width(part: &PresentationPart) -> Option<u32> {
    use crate::opc::part::Part;
    // Parse from XML blob
    if let Ok(blob) = Part::blob(part) {
        if let Ok(xml) = String::from_utf8(blob) {
            // Look for sldSz cx="..." pattern
            if let Some(start) = xml.find("sldSz cx=\"") {
                let start = start + 10;
                if let Some(end) = xml[start..].find('"') {
                    if let Ok(width) = xml[start..start+end].parse::<u32>() {
                        return Some(width);
                    }
                }
            }
        }
    }
    Some(9144000) // Default 10 inches
}

/// Set slide width in EMU
pub fn set_slide_width(part: &mut PresentationPart, width: u32) -> Result<()> {
    use crate::opc::part::Part;
    // Parse XML, update width, and store back
    let mut xml = Part::to_xml(part)?;
    // Replace cx value in sldSz
    let pattern = r#"sldSz cx="[0-9]+""#;
    let replacement = format!(r#"sldSz cx="{}""#, width);
    xml = regex::Regex::new(pattern)
        .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?
        .replace_all(&xml, replacement.as_str())
        .to_string();
    
    // If sldSz doesn't exist, add it
    if !xml.contains("sldSz") {
        let sld_sz = format!(r#"<p:sldSz cx="{}" cy="6858000"/>"#, width);
        xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
    }
    
    // Store updated XML
    let uri = Part::uri(part).clone();
    *part = PresentationPart::with_xml(uri, xml)?;
    Ok(())
}

/// Get slide height in EMU
pub fn slide_height(part: &PresentationPart) -> Option<u32> {
    use crate::opc::part::Part;
    // Parse from XML blob
    if let Ok(blob) = Part::blob(part) {
        if let Ok(xml) = String::from_utf8(blob) {
            // Look for sldSz cy="..." pattern
            if let Some(start) = xml.find("sldSz cy=\"") {
                let start = start + 10;
                if let Some(end) = xml[start..].find('"') {
                    if let Ok(height) = xml[start..start+end].parse::<u32>() {
                        return Some(height);
                    }
                }
            }
        }
    }
    Some(6858000) // Default 7.5 inches
}

/// Set slide height in EMU
pub fn set_slide_height(part: &mut PresentationPart, height: u32) -> Result<()> {
    use crate::opc::part::Part;
    // Parse XML, update height, and store back
    let mut xml = Part::to_xml(part)?;
    // Replace cy value in sldSz
    let pattern = r#"sldSz cx="[0-9]+" cy="[0-9]+""#;
    let width = slide_width(part).unwrap_or(9144000);
    let replacement = format!(r#"sldSz cx="{}" cy="{}""#, width, height);
    xml = regex::Regex::new(pattern)
        .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?
        .replace_all(&xml, replacement.as_str())
        .to_string();
    
    // If sldSz doesn't exist, add it
    if !xml.contains("sldSz") {
        let sld_sz = format!(r#"<p:sldSz cx="9144000" cy="{}"/>"#, height);
        xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
    }
    
    // Store updated XML
    let uri = Part::uri(part).clone();
    *part = PresentationPart::with_xml(uri, xml)?;
    Ok(())
}

