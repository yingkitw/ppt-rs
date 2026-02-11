//! Image XML generation for PPTX presentations
//!
//! Generates proper PPTX XML for image embedding and display

use crate::generator::images::{Image, ImageEffect};
use crate::core::escape_xml;

/// Generate image XML for a slide
pub fn generate_image_xml(image: &Image, shape_id: usize, rel_id: usize) -> String {
    let rel_id_str = format!("rId{}", rel_id);
    
    // Generate blipFill content (including crop)
    let blip_fill = if let Some(crop) = &image.crop {
        let l = (crop.left * 100_000.0) as u32;
        let t = (crop.top * 100_000.0) as u32;
        let r = (crop.right * 100_000.0) as u32;
        let b = (crop.bottom * 100_000.0) as u32;
        
        format!(
            r#"<p:blipFill>
<a:blip r:embed="{}"/>
<a:srcRect l="{}" t="{}" r="{}" b="{}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>"#,
            rel_id_str, l, t, r, b
        )
    } else {
        format!(
            r#"<p:blipFill>
<a:blip r:embed="{}"/>
<a:stretch>
<a:fillRect/>
</a:stretch>
</p:blipFill>"#,
            rel_id_str
        )
    };

    // Generate effects XML
    let mut effects_xml = String::new();
    if !image.effects.is_empty() {
        effects_xml.push_str("<a:effectLst>");
        for effect in &image.effects {
            match effect {
                ImageEffect::Shadow => {
                    effects_xml.push_str(r#"<a:outerShdw blurRad="40000" dist="20000" dir="5400000" rotWithShape="0"><a:srgbClr val="000000"><a:alpha val="40000"/></a:srgbClr></a:outerShdw>"#);
                }
                ImageEffect::Reflection => {
                    effects_xml.push_str(r#"<a:ref blurRad="6350" stA="50000" endA="300" endPos="35000" dist="0" dir="5400000" sy="-100000" algn="bl" rotWithShape="0"/>"#);
                }
            }
        }
        effects_xml.push_str("</a:effectLst>");
    }

    format!(
        r#"<p:pic>
<p:nvPicPr>
<p:cNvPr id="{}" name="{}"/>
<p:cNvPicPr>
<a:picLocks noChangeAspect="1"/>
</p:cNvPicPr>
<p:nvPr/>
</p:nvPicPr>
{}
<p:spPr>
<a:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="rect">
<a:avLst/>
</a:prstGeom>
{}
</p:spPr>
</p:pic>"#,
        shape_id,
        escape_xml(&image.filename),
        blip_fill,
        image.x,
        image.y,
        image.width,
        image.height,
        effects_xml
    )
}

/// Generate image relationship XML
pub fn generate_image_relationship(rel_id: usize, image_path: &str) -> String {
    format!(
        r#"<Relationship Id="rId{}" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="{}"/>"#,
        rel_id,
        escape_xml(image_path)
    )
}

/// Generate image content type entry
pub fn generate_image_content_type(extension: &str) -> String {
    let mime_type = match extension.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "tiff" => "image/tiff",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    };

    format!(
        r#"<Default Extension="{}" ContentType="{}"/>"#,
        extension.to_lowercase(),
        mime_type
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::images::Image;

    #[test]
    fn test_generate_simple_image_xml() {
        let img = Image::new("photo.png", 1920000, 1080000, "PNG");
        let xml = generate_image_xml(&img, 1, 1);

        assert!(xml.contains("p:pic"));
        assert!(xml.contains("photo.png"));
        assert!(xml.contains("rId1"));
    }

    #[test]
    fn test_generate_image_with_position() {
        let img = Image::new("photo.png", 1920000, 1080000, "PNG")
            .position(500000, 1000000);
        let xml = generate_image_xml(&img, 1, 1);

        assert!(xml.contains("x=\"500000\""));
        assert!(xml.contains("y=\"1000000\""));
    }

    #[test]
    fn test_generate_image_with_dimensions() {
        let img = Image::new("photo.png", 1920000, 1080000, "PNG");
        let xml = generate_image_xml(&img, 1, 1);

        assert!(xml.contains("cx=\"1920000\""));
        assert!(xml.contains("cy=\"1080000\""));
    }

    #[test]
    fn test_generate_image_relationship() {
        let rel = generate_image_relationship(1, "../media/image1.png");
        assert!(rel.contains("rId1"));
        assert!(rel.contains("../media/image1.png"));
    }

    #[test]
    fn test_generate_image_content_type_png() {
        let ct = generate_image_content_type("png");
        assert!(ct.contains("image/png"));
    }

    #[test]
    fn test_generate_image_content_type_jpg() {
        let ct = generate_image_content_type("jpg");
        assert!(ct.contains("image/jpeg"));
    }

    #[test]
    fn test_generate_image_content_type_gif() {
        let ct = generate_image_content_type("gif");
        assert!(ct.contains("image/gif"));
    }

    #[test]
    fn test_escape_xml_in_filename() {
        let img = Image::new("photo & <test>.png", 100, 100, "PNG");
        let xml = generate_image_xml(&img, 1, 1);

        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
    }

    #[test]
    fn test_generate_image_xml_structure() {
        let img = Image::new("photo.png", 1920000, 1080000, "PNG");
        let xml = generate_image_xml(&img, 1, 1);

        assert!(xml.contains("p:nvPicPr"));
        assert!(xml.contains("p:cNvPicPr"));
        assert!(xml.contains("a:picLocks"));
        assert!(xml.contains("p:blipFill"));
        assert!(xml.contains("a:blip"));
        assert!(xml.contains("a:stretch"));
        assert!(xml.contains("p:spPr"));
        assert!(xml.contains("a:xfrm"));
        assert!(xml.contains("a:prstGeom"));
    }
}
