//! Shape XML generation for PPTX
//!
//! Generates XML for shapes embedded in slides.

use super::shapes::{Shape, ShapeFill, ShapeLine, GradientFill};
use crate::generator::hyperlinks::generate_shape_hyperlink_xml;

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Generate XML for a shape
pub fn generate_shape_xml(shape: &Shape, shape_id: u32) -> String {
    // Generate fill XML - gradient takes precedence over solid fill
    let fill_xml = if let Some(gradient) = &shape.gradient {
        generate_gradient_xml(gradient)
    } else {
        generate_fill_xml(&shape.fill)
    };
    let line_xml = generate_line_xml(&shape.line);
    let fill_color = shape.fill.as_ref().map(|f| f.color.as_str());
    let text_xml = generate_text_xml_with_autofit(&shape.text, shape.width, shape.height, fill_color);
    
    let rot_attr = if let Some(rot) = shape.rotation {
        format!(r#" rot="{}""#, rot * 60000)
    } else {
        String::new()
    };

    let cnvpr_xml = if let Some(h) = &shape.hyperlink {
        if let Some(rid) = &h.r_id {
             format!(r#"<p:cNvPr id="{}" name="Shape {}">{}</p:cNvPr>"#, shape_id, shape_id, generate_shape_hyperlink_xml(h, rid))
        } else {
             format!(r#"<p:cNvPr id="{}" name="Shape {}"/>"#, shape_id, shape_id)
        }
    } else {
        format!(r#"<p:cNvPr id="{}" name="Shape {}"/>"#, shape_id, shape_id)
    };

    format!(
        r#"<p:sp>
<p:nvSpPr>
{}
<p:cNvSpPr/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm{}>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="{}">
<a:avLst/>
</a:prstGeom>
{}{}
</p:spPr>
{}
</p:sp>"#,
        cnvpr_xml,
        rot_attr,
        shape.x,
        shape.y,
        shape.width,
        shape.height,
        shape.shape_type.preset_name(),
        fill_xml,
        line_xml,
        text_xml,
    )
}

/// Generate fill XML for solid color
fn generate_fill_xml(fill: &Option<ShapeFill>) -> String {
    match fill {
        Some(f) => {
            let alpha = f.transparency
                .map(|t| format!(r#"<a:alpha val="{}"/>"#, t))
                .unwrap_or_default();
            
            format!(
                r#"<a:solidFill>
<a:srgbClr val="{}">{}</a:srgbClr>
</a:solidFill>"#,
                f.color, alpha
            )
        }
        None => String::new(),
    }
}

/// Generate gradient fill XML
fn generate_gradient_xml(gradient: &GradientFill) -> String {
    let mut stops_xml = String::new();
    
    for stop in &gradient.stops {
        let alpha = stop.transparency
            .map(|t| format!(r#"<a:alpha val="{}"/>"#, t))
            .unwrap_or_default();
        
        stops_xml.push_str(&format!(
            r#"<a:gs pos="{}">
<a:srgbClr val="{}">{}</a:srgbClr>
</a:gs>"#,
            stop.position, stop.color, alpha
        ));
    }
    
    format!(
        r#"<a:gradFill>
<a:gsLst>
{}
</a:gsLst>
<a:lin ang="{}" scaled="1"/>
</a:gradFill>"#,
        stops_xml,
        gradient.direction.to_angle()
    )
}

/// Generate line XML
fn generate_line_xml(line: &Option<ShapeLine>) -> String {
    match line {
        Some(l) => {
            format!(
                r#"<a:ln w="{}">
<a:solidFill>
<a:srgbClr val="{}"/>
</a:solidFill>
</a:ln>"#,
                l.width, l.color
            )
        }
        None => String::new(),
    }
}

/// Calculate optimal font size based on shape dimensions and text content
fn calculate_font_size(text: &str, width_emu: u32, height_emu: u32) -> u32 {
    // Convert EMU to approximate character width
    // 1 inch = 914400 EMU, average char width at 18pt ≈ 0.1 inch
    let width_inches = width_emu as f64 / 914400.0;
    let height_inches = height_emu as f64 / 914400.0;
    
    // Account for padding (roughly 10% on each side)
    let usable_width = width_inches * 0.8;
    let usable_height = height_inches * 0.8;
    
    // Get text metrics
    let lines: Vec<&str> = text.lines().collect();
    let num_lines = lines.len().max(1);
    let max_line_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1);
    
    // Calculate font size based on width constraint
    // At 18pt (1800 hundredths), average char is ~0.1 inch
    // So chars_per_inch ≈ 10 at 18pt, scales inversely with font size
    let chars_that_fit_width = usable_width * 10.0; // at 18pt
    let width_scale = chars_that_fit_width / max_line_len as f64;
    let font_from_width = (1800.0 * width_scale).min(4400.0); // max 44pt
    
    // Calculate font size based on height constraint
    // At 18pt, line height ≈ 0.25 inch
    let lines_that_fit = usable_height / 0.25;
    let height_scale = lines_that_fit / num_lines as f64;
    let font_from_height = (1800.0 * height_scale).min(4400.0);
    
    // Use the smaller of the two to ensure text fits
    let optimal_size = font_from_width.min(font_from_height);
    
    // Clamp to reasonable range: 800 (8pt) to 4400 (44pt)
    (optimal_size.max(800.0).min(4400.0)) as u32
}

/// Calculate if a color is dark (needs white text) or light (needs black text)
fn is_dark_color(hex_color: &str) -> bool {
    let hex = hex_color.trim_start_matches('#');
    if hex.len() < 6 {
        return false; // Default to light
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    
    // Calculate relative luminance using sRGB formula
    let luminance = 0.299 * (r as f64) + 0.587 * (g as f64) + 0.114 * (b as f64);
    luminance < 128.0
}

/// Get contrasting text color for a given background
fn get_text_color(fill_color: Option<&str>) -> &'static str {
    match fill_color {
        Some(color) if is_dark_color(color) => "FFFFFF", // White text on dark background
        _ => "000000", // Black text on light/no background
    }
}

/// Generate text body XML for shape with auto-fit font sizing
fn generate_text_xml_with_autofit(text: &Option<String>, width: u32, height: u32, fill_color: Option<&str>) -> String {
    match text {
        Some(t) => {
            // Check if this is code (starts with [ and contains language tag)
            let is_code = t.starts_with('[') && t.contains("]\n");
            
            if is_code {
                // Code block: use monospace font, left align, smaller size
                let mut paragraphs = String::new();
                for line in t.lines() {
                    let escaped = escape_xml(line);
                    paragraphs.push_str(&format!(
                        r#"<a:p>
<a:pPr algn="l"/>
<a:r>
<a:rPr lang="en-US" sz="1200" dirty="0"><a:latin typeface="Consolas"/><a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill></a:rPr>
<a:t>{}</a:t>
</a:r>
</a:p>"#,
                        escaped
                    ));
                }
                format!(
                    r#"<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="t" lIns="91440" tIns="45720" rIns="91440" bIns="45720"/>
<a:lstStyle/>
{}</p:txBody>"#,
                    paragraphs
                )
            } else {
                // Calculate optimal font size based on shape dimensions
                let font_size = calculate_font_size(t, width, height);
                
                // Get contrasting text color based on fill
                let text_color = get_text_color(fill_color);
                
                // Use left alignment for multi-line text, center for single line
                let is_multiline = t.contains('\n');
                let alignment = if is_multiline { "l" } else { "ctr" };
                let anchor = if is_multiline { "t" } else { "ctr" };
                
                // Use PowerPoint's auto-fit feature for additional safety
                format!(
                    r#"<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="{}">
<a:normAutofit/>
</a:bodyPr>
<a:lstStyle/>
<a:p>
<a:pPr algn="{}"/>
<a:r>
<a:rPr lang="en-US" sz="{}" dirty="0"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:rPr>
<a:t>{}</a:t>
</a:r>
</a:p>
</p:txBody>"#,
                    anchor,
                    alignment,
                    font_size,
                    text_color,
                    escape_xml(t)
                )
            }
        }
        None => {
            // Empty text body required for shapes
            r#"<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p/>
</p:txBody>"#.to_string()
        }
    }
}

/// Generate XML for multiple shapes
pub fn generate_shapes_xml(shapes: &[Shape], start_id: u32) -> String {
    shapes.iter()
        .enumerate()
        .map(|(i, shape)| generate_shape_xml(shape, start_id + i as u32))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate connector shape XML (for arrows connecting shapes)
pub fn generate_connector_xml(
    start_x: u32, start_y: u32,
    end_x: u32, end_y: u32,
    shape_id: u32,
    color: &str,
    width: u32,
) -> String {
    format!(
        r#"<p:cxnSp>
<p:nvCxnSpPr>
<p:cNvPr id="{}" name="Connector {}"/>
<p:cNvCxnSpPr/>
<p:nvPr/>
</p:nvCxnSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</a:xfrm>
<a:prstGeom prst="straightConnector1">
<a:avLst/>
</a:prstGeom>
<a:ln w="{}">
<a:solidFill>
<a:srgbClr val="{}"/>
</a:solidFill>
<a:tailEnd type="triangle"/>
</a:ln>
</p:spPr>
</p:cxnSp>"#,
        shape_id,
        shape_id,
        start_x.min(end_x),
        start_y.min(end_y),
        (end_x as i64 - start_x as i64).unsigned_abs() as u32,
        (end_y as i64 - start_y as i64).unsigned_abs() as u32,
        width,
        color.trim_start_matches('#').to_uppercase(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::shapes::ShapeType;

    #[test]
    fn test_generate_shape_xml() {
        let shape = Shape::new(ShapeType::Rectangle, 100000, 200000, 500000, 300000)
            .with_fill(ShapeFill::new("FF0000"));
        
        let xml = generate_shape_xml(&shape, 10);
        
        assert!(xml.contains("p:sp"));
        assert!(xml.contains("id=\"10\""));
        assert!(xml.contains("rect"));
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_generate_shape_with_text() {
        let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1000000, 500000)
            .with_text("Hello World");
        
        let xml = generate_shape_xml(&shape, 1);
        
        assert!(xml.contains("Hello World"));
        assert!(xml.contains("p:txBody"));
    }

    #[test]
    fn test_generate_shape_with_line() {
        let shape = Shape::new(ShapeType::Circle, 0, 0, 500000, 500000)
            .with_line(ShapeLine::new("000000", 25400));
        
        let xml = generate_shape_xml(&shape, 1);
        
        assert!(xml.contains("a:ln"));
        assert!(xml.contains("25400"));
    }

    #[test]
    fn test_generate_multiple_shapes() {
        let shapes = vec![
            Shape::new(ShapeType::Rectangle, 0, 0, 100000, 100000),
            Shape::new(ShapeType::Circle, 200000, 0, 100000, 100000),
        ];
        
        let xml = generate_shapes_xml(&shapes, 10);
        
        assert!(xml.contains("id=\"10\""));
        assert!(xml.contains("id=\"11\""));
        assert!(xml.contains("rect"));
        assert!(xml.contains("ellipse"));
    }

    #[test]
    fn test_generate_connector() {
        let xml = generate_connector_xml(0, 0, 1000000, 500000, 1, "0000FF", 12700);
        
        assert!(xml.contains("p:cxnSp"));
        assert!(xml.contains("straightConnector1"));
        assert!(xml.contains("triangle")); // arrow head
    }

    #[test]
    fn test_escape_xml_in_text() {
        let shape = Shape::new(ShapeType::Rectangle, 0, 0, 100000, 100000)
            .with_text("A < B & C > D");
        
        let xml = generate_shape_xml(&shape, 1);
        
        assert!(xml.contains("A &lt; B &amp; C &gt; D"));
    }

    #[test]
    fn test_font_size_autofit_small_shape() {
        // Small shape with long text should get smaller font
        let font_size = calculate_font_size("This is a very long text that needs to fit", 500_000, 300_000);
        assert!(font_size < 1800, "Font should be smaller than 18pt for small shape with long text");
        assert!(font_size >= 800, "Font should not be smaller than 8pt");
    }

    #[test]
    fn test_font_size_autofit_large_shape() {
        // Large shape with short text should get larger font
        let font_size = calculate_font_size("Hi", 3_000_000, 2_000_000);
        assert!(font_size >= 1800, "Font should be at least 18pt for large shape with short text");
    }

    #[test]
    fn test_font_size_autofit_multiline() {
        // Multi-line text should account for height
        let font_size = calculate_font_size("Line 1\nLine 2\nLine 3\nLine 4", 2_000_000, 500_000);
        assert!(font_size < 1800, "Font should be smaller for multi-line text in short shape");
    }

    #[test]
    fn test_autofit_xml_contains_norm_autofit() {
        let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1_000_000, 500_000)
            .with_text("Test text");
        
        let xml = generate_shape_xml(&shape, 1);
        
        assert!(xml.contains("normAutofit"), "Should contain PowerPoint auto-fit element");
    }

    #[test]
    fn test_dark_color_detection() {
        // Dark colors should return true
        assert!(is_dark_color("000000")); // Black
        assert!(is_dark_color("1565C0")); // Dark blue
        assert!(is_dark_color("002B36")); // Solarized base03
        
        // Light colors should return false
        assert!(!is_dark_color("FFFFFF")); // White
        assert!(!is_dark_color("E3F2FD")); // Light blue
        assert!(!is_dark_color("F3E5F5")); // Light purple
    }

    #[test]
    fn test_text_color_contrast() {
        // Dark fill should get white text
        let shape = Shape::new(ShapeType::Rectangle, 0, 0, 1_000_000, 500_000)
            .with_fill(ShapeFill::new("1565C0"))
            .with_text("Test");
        let xml = generate_shape_xml(&shape, 1);
        assert!(xml.contains("FFFFFF"), "Dark fill should have white text");
        
        // Light fill should get black text
        let shape2 = Shape::new(ShapeType::Rectangle, 0, 0, 1_000_000, 500_000)
            .with_fill(ShapeFill::new("E3F2FD"))
            .with_text("Test");
        let xml2 = generate_shape_xml(&shape2, 1);
        assert!(xml2.contains("000000"), "Light fill should have black text");
    }
}
