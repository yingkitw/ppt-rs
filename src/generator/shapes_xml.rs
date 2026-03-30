//! Shape XML generation for PPTX
//!
//! Generates XML for shapes embedded in slides.

use super::shapes::{Shape, ShapeFill, ShapeLine, GradientFill};
use crate::generator::hyperlinks::generate_shape_hyperlink_xml;
use crate::core::escape_xml;

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

/// Font metrics for different font families
struct FontMetrics {
    /// Average character width as ratio of font size (in points)
    char_width_ratio: f64,
    /// Line height as ratio of font size
    line_height_ratio: f64,
    /// Is monospace font
    is_monospace: bool,
}

impl FontMetrics {
    /// Get metrics for a font family
    fn for_font(font_family: &str) -> Self {
        match font_family.to_lowercase().as_str() {
            "consolas" | "courier" | "courier new" | "monaco" | "menlo" => {
                FontMetrics {
                    char_width_ratio: 0.6,  // Monospace: wider chars
                    line_height_ratio: 1.2,
                    is_monospace: true,
                }
            }
            "arial" | "helvetica" | "calibri" | "segoe ui" => {
                FontMetrics {
                    char_width_ratio: 0.5,  // Proportional: average width
                    line_height_ratio: 1.15,
                    is_monospace: false,
                }
            }
            "times" | "times new roman" | "georgia" => {
                FontMetrics {
                    char_width_ratio: 0.45, // Serif: narrower
                    line_height_ratio: 1.2,
                    is_monospace: false,
                }
            }
            _ => {
                // Default to Calibri-like metrics
                FontMetrics {
                    char_width_ratio: 0.5,
                    line_height_ratio: 1.15,
                    is_monospace: false,
                }
            }
        }
    }
    
    /// Default metrics (Calibri)
    fn default() -> Self {
        FontMetrics {
            char_width_ratio: 0.5,
            line_height_ratio: 1.15,
            is_monospace: false,
        }
    }
}

/// Calculate optimal font size based on shape dimensions and text content
/// 
/// Enhanced algorithm that considers:
/// - Font family metrics (monospace vs proportional)
/// - Word wrapping and line breaks
/// - Configurable padding and margins
/// - Character width variations
/// - Line spacing
fn calculate_font_size(text: &str, width_emu: u32, height_emu: u32) -> u32 {
    calculate_font_size_with_font(text, width_emu, height_emu, None)
}

/// Calculate optimal font size with specific font family
fn calculate_font_size_with_font(
    text: &str,
    width_emu: u32,
    height_emu: u32,
    font_family: Option<&str>,
) -> u32 {
    // Convert EMU to points (1 inch = 914400 EMU, 1 inch = 72 points)
    let width_pt = (width_emu as f64 / 914400.0) * 72.0;
    let height_pt = (height_emu as f64 / 914400.0) * 72.0;
    
    // Get font metrics
    let metrics = font_family
        .map(FontMetrics::for_font)
        .unwrap_or_else(FontMetrics::default);
    
    // Configurable padding (EMU to points)
    // Default: 0.1 inch (7.2 pt) on each side
    let padding_horizontal = 14.4; // 0.2 inch total (left + right)
    let padding_vertical = 14.4;   // 0.2 inch total (top + bottom)
    
    let usable_width = (width_pt - padding_horizontal).max(10.0);
    let usable_height = (height_pt - padding_vertical).max(10.0);
    
    // Analyze text structure
    let lines: Vec<&str> = text.lines().collect();
    let num_lines = lines.len().max(1);
    
    // Calculate effective line count with word wrapping
    let (effective_lines, max_chars_per_line) = if metrics.is_monospace {
        // Monospace: simple character counting
        let max_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1);
        (num_lines as f64, max_len as f64)
    } else {
        // Proportional: estimate with word wrapping
        estimate_wrapped_lines(text, &lines)
    };
    
    // Calculate font size from width constraint
    // char_width = font_size * char_width_ratio
    // total_width = char_width * chars_per_line
    // font_size = usable_width / (chars_per_line * char_width_ratio)
    let font_from_width = if max_chars_per_line > 0.0 {
        usable_width / (max_chars_per_line * metrics.char_width_ratio)
    } else {
        72.0 // Default to 72pt if no text
    };
    
    // Calculate font size from height constraint
    // line_height = font_size * line_height_ratio
    // total_height = line_height * num_lines
    // font_size = usable_height / (num_lines * line_height_ratio)
    let font_from_height = if effective_lines > 0.0 {
        usable_height / (effective_lines * metrics.line_height_ratio)
    } else {
        72.0
    };
    
    // Use the smaller to ensure text fits in both dimensions
    let optimal_size_pt = font_from_width.min(font_from_height);
    
    // Convert points to hundredths of a point (PowerPoint format)
    let optimal_size_hundredths = (optimal_size_pt * 100.0) as u32;
    
    // Clamp to reasonable range: 600 (6pt) to 7200 (72pt)
    // More generous range for better flexibility
    optimal_size_hundredths.clamp(600, 7200)
}

/// Estimate number of lines after word wrapping for proportional fonts
fn estimate_wrapped_lines(_text: &str, lines: &[&str]) -> (f64, f64) {
    let mut total_lines = 0.0;
    let mut max_chars: f64 = 0.0;
    
    for line in lines {
        if line.is_empty() {
            total_lines += 1.0;
            continue;
        }
        
        // Estimate average word length
        let words: Vec<&str> = line.split_whitespace().collect();
        let word_count = words.len();
        
        if word_count == 0 {
            total_lines += 1.0;
            continue;
        }
        
        // Average characters per word (including space)
        let _avg_word_len = (line.chars().count() as f64) / (word_count as f64);
        
        // Estimate: longer lines might wrap
        let char_count = line.chars().count() as f64;
        max_chars = max_chars.max(char_count);
        
        // If line is very long, assume it might wrap
        // This is a heuristic - actual wrapping depends on font size
        if char_count > 50.0 {
            // Estimate 1.5x lines for very long text
            total_lines += 1.5;
        } else if char_count > 30.0 {
            total_lines += 1.2;
        } else {
            total_lines += 1.0;
        }
    }
    
    (total_lines, max_chars)
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
                
                // Calculate proper text insets (margins) in EMU
                // Standard insets: 0.1 inch (91440 EMU) on left/right, 0.05 inch (45720 EMU) on top/bottom
                let left_inset = 91440;   // 0.1 inch
                let top_inset = 45720;    // 0.05 inch
                let right_inset = 91440;  // 0.1 inch
                let bottom_inset = 45720; // 0.05 inch
                
                // Use PowerPoint's auto-fit feature for additional safety
                format!(
                    r#"<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="{}" lIns="{}" tIns="{}" rIns="{}" bIns="{}">
<a:normAutofit fontScale="100000" lnSpcReduction="0"/>
</a:bodyPr>
<a:lstStyle/>
<a:p>
<a:pPr algn="{}" marL="0" marR="0" indent="0"/>
<a:r>
<a:rPr lang="en-US" sz="{}" dirty="0" smtClean="0"><a:solidFill><a:srgbClr val="{}"/></a:solidFill></a:rPr>
<a:t>{}</a:t>
</a:r>
</a:p>
</p:txBody>"#,
                    anchor,
                    left_inset,
                    top_inset,
                    right_inset,
                    bottom_inset,
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
        assert!(font_size >= 600, "Font should not be smaller than 6pt");
    }

    #[test]
    fn test_font_size_autofit_large_shape() {
        // Large shape with short text should get larger font
        let font_size = calculate_font_size("Hi", 3_000_000, 2_000_000);
        assert!(font_size >= 1800, "Font should be at least 18pt for large shape with short text");
        assert!(font_size <= 7200, "Font should not exceed 72pt");
    }

    #[test]
    fn test_font_size_autofit_multiline() {
        // Multi-line text should account for height
        let font_size = calculate_font_size("Line 1\nLine 2\nLine 3\nLine 4", 2_000_000, 500_000);
        assert!(font_size < 1800, "Font should be smaller for multi-line text in short shape");
    }

    #[test]
    fn test_font_size_with_monospace_font() {
        // Monospace fonts have different width ratios
        let font_size = calculate_font_size_with_font("Code text here", 1_000_000, 500_000, Some("Consolas"));
        assert!(font_size >= 600 && font_size <= 7200, "Font size should be in valid range");
    }

    #[test]
    fn test_font_size_with_proportional_font() {
        // Proportional fonts (Arial, Calibri) have different metrics
        let font_size_arial = calculate_font_size_with_font("Sample text", 1_000_000, 500_000, Some("Arial"));
        let font_size_times = calculate_font_size_with_font("Sample text", 1_000_000, 500_000, Some("Times New Roman"));
        
        // Times New Roman is narrower, should allow slightly larger font
        assert!(font_size_times >= font_size_arial, "Serif fonts should allow larger sizes");
    }

    #[test]
    fn test_font_size_word_wrapping_estimation() {
        // Very long single line should estimate wrapping
        let long_text = "This is a very long line of text that will definitely need to wrap when displayed in a shape";
        let font_size = calculate_font_size(long_text, 1_000_000, 1_000_000);
        
        // Should be smaller due to wrapping estimation
        assert!(font_size < 2000, "Long text should get smaller font due to wrapping");
    }

    #[test]
    fn test_font_metrics_for_different_families() {
        // Test that different font families have different metrics
        let consolas = FontMetrics::for_font("Consolas");
        let arial = FontMetrics::for_font("Arial");
        let times = FontMetrics::for_font("Times New Roman");
        
        assert!(consolas.is_monospace);
        assert!(!arial.is_monospace);
        assert!(!times.is_monospace);
        
        // Monospace should have wider char ratio
        assert!(consolas.char_width_ratio > arial.char_width_ratio);
        // Serif should have narrower char ratio
        assert!(times.char_width_ratio < arial.char_width_ratio);
    }

    #[test]
    fn test_estimate_wrapped_lines() {
        let text = "Short line\nThis is a medium length line\nThis is a very long line that will probably wrap when rendered in a shape with limited width";
        let lines: Vec<&str> = text.lines().collect();
        let (total_lines, max_chars) = estimate_wrapped_lines(text, &lines);
        
        // Should estimate more than 3 lines due to wrapping
        assert!(total_lines > 3.0, "Should estimate wrapped lines");
        assert!(max_chars > 50.0, "Should track longest line");
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
