//! Image visual effects and blipFill XML generation
//!
//! Extracted from `images_xml.rs` so effect presets are testable and reusable.

use crate::generator::images::{Crop, ImageEffect};

/// Generate `<a:effectLst>` content for the given effects (empty if none).
pub fn generate_effect_list_xml(effects: &[ImageEffect]) -> String {
    if effects.is_empty() {
        return String::new();
    }

    let mut xml = String::from("<a:effectLst>");
    for effect in effects {
        xml.push_str(&generate_effect_xml(effect));
    }
    xml.push_str("</a:effectLst>");
    xml
}

/// Generate OOXML for a single image effect preset.
pub fn generate_effect_xml(effect: &ImageEffect) -> String {
    match effect {
        ImageEffect::Shadow => {
            r#"<a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0"><a:srgbClr val="000000"><a:alpha val="40000"/></a:srgbClr></a:outerShdw>"#.to_string()
        }
        ImageEffect::Reflection => {
            r#"<a:reflection blurRad="6350" stA="50000" endA="300" endPos="35000" dist="0" dir="5400000" sy="-100000" algn="bl" rotWithShape="0"/>"#.to_string()
        }
        ImageEffect::Glow => {
            r#"<a:glow rad="50800"><a:srgbClr val="FFD700"><a:alpha val="60000"/></a:srgbClr></a:glow>"#.to_string()
        }
        ImageEffect::SoftEdges => r#"<a:softEdge rad="50800"/>"#.to_string(),
        ImageEffect::InnerShadow => {
            r#"<a:innerShdw blurRad="40000" dist="20000" dir="2700000"><a:srgbClr val="000000"><a:alpha val="50000"/></a:srgbClr></a:innerShdw>"#.to_string()
        }
        ImageEffect::Blur => r#"<a:blur rad="38100" grow="1"/>"#.to_string(),
    }
}

/// Generate `<p:blipFill>` with optional crop via `<a:srcRect>`.
pub fn generate_blip_fill_xml(rel_id: &str, crop: Option<&Crop>) -> String {
    match crop {
        Some(crop) => {
            let l = (crop.left * 100_000.0) as u32;
            let t = (crop.top * 100_000.0) as u32;
            let r = (crop.right * 100_000.0) as u32;
            let b = (crop.bottom * 100_000.0) as u32;
            format!(
                r#"<p:blipFill>
<a:blip r:embed="{rel_id}"/>
<a:srcRect l="{l}" t="{t}" r="{r}" b="{b}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>"#
            )
        }
        None => format!(
            r#"<p:blipFill>
<a:blip r:embed="{rel_id}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>"#
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_xml_shadow() {
        let xml = generate_effect_xml(&ImageEffect::Shadow);
        assert!(xml.contains("outerShdw"));
    }

    #[test]
    fn test_effect_xml_glow() {
        let xml = generate_effect_xml(&ImageEffect::Glow);
        assert!(xml.contains("FFD700"));
    }

    #[test]
    fn test_effect_list_wraps_effects() {
        let xml = generate_effect_list_xml(&[ImageEffect::Shadow, ImageEffect::Blur]);
        assert!(xml.starts_with("<a:effectLst>"));
        assert!(xml.ends_with("</a:effectLst>"));
        assert!(xml.contains("outerShdw"));
        assert!(xml.contains("blur"));
    }

    #[test]
    fn test_blip_fill_without_crop() {
        let xml = generate_blip_fill_xml("rId1", None);
        assert!(xml.contains(r#"r:embed="rId1""#));
        assert!(!xml.contains("srcRect"));
    }

    #[test]
    fn test_blip_fill_with_crop() {
        let crop = Crop {
            left: 0.1,
            top: 0.2,
            right: 0.3,
            bottom: 0.4,
        };
        let xml = generate_blip_fill_xml("rId2", Some(&crop));
        assert!(xml.contains(r#"l="10000""#));
        assert!(xml.contains(r#"t="20000""#));
    }
}
