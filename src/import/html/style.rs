//! CSS parsing and inline style accumulation for HTML import.

use crate::generator::slide_content::BulletTextFormat;

pub(crate) fn decode_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'&' {
            if let Some(end) = s[i..].find(';') {
                let entity = &s[i + 1..i + end];
                let ch = match entity {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" | "#39" | "#x27" => Some('\''),
                    "nbsp" => Some('\u{00a0}'),
                    "#x2018" => Some('\u{2018}'),
                    "#x2019" => Some('\u{2019}'),
                    "#x201c" => Some('\u{201c}'),
                    "#x201d" => Some('\u{201d}'),
                    "#x2014" => Some('\u{2014}'),
                    "#x2013" => Some('\u{2013}'),
                    _ => {
                        if let Some(hex) = entity.strip_prefix("#x") {
                            u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
                        } else if let Some(num) = entity.strip_prefix('#') {
                            num.parse::<u32>().ok().and_then(char::from_u32)
                        } else {
                            None
                        }
                    }
                };
                if let Some(c) = ch {
                    out.push(c);
                    i = i + end + 1;
                    continue;
                }
            }
        }
        // Preserve full Unicode characters (not just ASCII bytes)
        let c = s[i..].chars().next().unwrap();
        out.push(c);
        i += c.len_utf8();
    }
    out
}

// ---------------------------------------------------------------------------
// Inline CSS style parsing
// ---------------------------------------------------------------------------

/// Convert a named CSS color to its 6-digit hex representation
pub(crate) fn css_named_color(name: &str) -> Option<&'static str> {
    match name {
        "red" => Some("FF0000"),
        "blue" => Some("0000FF"),
        "green" => Some("008000"),
        "yellow" => Some("FFFF00"),
        "white" => Some("FFFFFF"),
        "black" => Some("000000"),
        "gray" | "grey" => Some("808080"),
        "silver" => Some("C0C0C0"),
        "maroon" => Some("800000"),
        "purple" => Some("800080"),
        "fuchsia" => Some("FF00FF"),
        "lime" => Some("00FF00"),
        "olive" => Some("808000"),
        "navy" => Some("000080"),
        "teal" => Some("008080"),
        "aqua" => Some("00FFFF"),
        "orange" => Some("FFA500"),
        "pink" => Some("FFC0CB"),
        "coral" => Some("FF7F50"),
        "tomato" => Some("FF6347"),
        "darkred" => Some("8B0000"),
        "darkblue" => Some("00008B"),
        "darkgreen" => Some("006400"),
        "darkgray" | "darkgrey" => Some("A9A9A9"),
        "lightgray" | "lightgrey" => Some("D3D3D3"),
        "darkorange" => Some("FF8C00"),
        "brown" => Some("A52A2A"),
        "crimson" => Some("DC143C"),
        "gold" => Some("FFD700"),
        "goldenrod" => Some("DAA520"),
        "indigo" => Some("4B0082"),
        "salmon" => Some("FA8072"),
        "chocolate" => Some("D2691E"),
        "steelblue" => Some("4682B4"),
        "violet" => Some("EE82EE"),
        "orchid" => Some("DA70D6"),
        "plum" => Some("DDA0DD"),
        "wheat" => Some("F5DEB3"),
        "deeppink" => Some("FF1493"),
        "hotpink" => Some("FF69B4"),
        "royalblue" => Some("4169E1"),
        "skyblue" => Some("87CEEB"),
        "seagreen" => Some("2E8B57"),
        "forestgreen" => Some("228B22"),
        _ => None,
    }
}

/// Parse a CSS color value to a 6-digit hex string (without #)
pub(crate) fn parse_css_color(value: &str) -> Option<String> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        let hex = match hex.len() {
            3 => hex.chars().map(|c| format!("{c}{c}")).collect::<String>(),
            6 => hex.to_string(),
            8 => hex[..6].to_string(), // ignore alpha
            _ => return None,
        };
        Some(hex.to_uppercase())
    } else if let Some(named) = css_named_color(value) {
        Some(named.to_string())
    } else if let Some(rgb) = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("rgb("))
    {
        if let Some(end) = rgb.rfind(')') {
            let parts: Vec<&str> = rgb[..end].split(',').collect();
            if parts.len() >= 3 {
                let r = parts[0].trim().parse::<u8>().ok()?;
                let g = parts[1].trim().parse::<u8>().ok()?;
                let b = parts[2].trim().parse::<u8>().ok()?;
                return Some(format!("{:02X}{:02X}{:02X}", r, g, b));
            }
        }
        None
    } else {
        None
    }
}

/// Parse a CSS font-size value to points
pub(crate) fn parse_font_size(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(px) = value.strip_suffix("px") {
        let px = px.trim().parse::<f64>().ok()?;
        Some((px / 1.333).round() as u32)
    } else if let Some(pt) = value.strip_suffix("pt") {
        let pt = pt.trim().parse::<f64>().ok()?;
        Some(pt.round() as u32)
    } else {
        value.parse::<u32>().ok()
    }
}

/// Check if a CSS font-weight value represents bold
pub(crate) fn is_font_weight_bold(value: &str) -> bool {
    matches!(
        value.trim().to_lowercase().as_str(),
        "bold" | "bolder" | "700" | "800" | "900"
    )
}

/// Check if a CSS font-style represents italic
pub(crate) fn is_font_style_italic(value: &str) -> bool {
    matches!(value.trim().to_lowercase().as_str(), "italic" | "oblique")
}

/// Parsed inline CSS style declarations
#[derive(Clone, Debug, Default)]
pub(crate) struct InlineStyle {
    pub(crate) color: Option<String>,
    pub(crate) background_color: Option<String>,
    pub(crate) font_size: Option<u32>,
    pub(crate) font_weight: Option<String>,
    pub(crate) font_style: Option<String>,
    pub(crate) text_decoration: Option<String>,
    pub(crate) font_family: Option<String>,
}

impl InlineStyle {
    pub(crate) fn parse(style_str: &str) -> Self {
        let mut style = InlineStyle::default();
        for decl in style_str.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }
            if let Some(eq) = decl.find(':') {
                let prop = decl[..eq].trim().to_lowercase();
                let value = decl[eq + 1..].trim();
                match prop.as_str() {
                    "color" => style.color = parse_css_color(value),
                    "background-color" => style.background_color = parse_css_color(value),
                    "font-size" => style.font_size = parse_font_size(value),
                    "font-weight" => style.font_weight = Some(value.to_string()),
                    "font-style" => style.font_style = Some(value.to_string()),
                    "text-decoration" => style.text_decoration = Some(value.to_string()),
                    "font-family" => {
                        style.font_family =
                            Some(value.trim_matches('"').trim_matches('\'').to_string());
                    }
                    _ => {}
                }
            }
        }
        style
    }

    /// Merge another style on top, with the other's non-None values taking precedence
    pub(crate) fn merge(&self, other: &InlineStyle) -> InlineStyle {
        InlineStyle {
            color: other.color.clone().or_else(|| self.color.clone()),
            background_color: other
                .background_color
                .clone()
                .or_else(|| self.background_color.clone()),
            font_size: other.font_size.or(self.font_size),
            font_weight: other
                .font_weight
                .clone()
                .or_else(|| self.font_weight.clone()),
            font_style: other.font_style.clone().or_else(|| self.font_style.clone()),
            text_decoration: other
                .text_decoration
                .clone()
                .or_else(|| self.text_decoration.clone()),
            font_family: other
                .font_family
                .clone()
                .or_else(|| self.font_family.clone()),
        }
    }

    /// Returns true if no style properties are set
    fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.background_color.is_none()
            && self.font_size.is_none()
            && self.font_weight.is_none()
            && self.font_style.is_none()
            && self.text_decoration.is_none()
            && self.font_family.is_none()
    }

    /// Convert to a BulletTextFormat for PPTX output. Returns None if no relevant properties set.
    pub(crate) fn to_bullet_format(&self) -> Option<BulletTextFormat> {
        if self.is_empty() {
            return None;
        }
        let mut fmt = BulletTextFormat::new();
        if let Some(ref c) = self.color {
            fmt = fmt.color(c);
        }
        if let Some(ref bg) = self.background_color {
            fmt = fmt.highlight(bg);
        }
        if let Some(sz) = self.font_size {
            fmt = fmt.font_size(sz);
        }
        if let Some(ref fw) = self.font_weight {
            if is_font_weight_bold(fw) {
                fmt = fmt.bold();
            }
        }
        if let Some(ref fs) = self.font_style {
            if is_font_style_italic(fs) {
                fmt = fmt.italic();
            }
        }
        if let Some(ref td) = self.text_decoration {
            if td.contains("underline") {
                fmt = fmt.underline();
            }
            if td.contains("line-through") {
                fmt = fmt.strikethrough();
            }
        }
        if let Some(ref ff) = self.font_family {
            fmt = fmt.font_family(ff);
        }
        Some(fmt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_css_color_hex() {
        assert_eq!(parse_css_color("#ff0000"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#FF0000"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#f00"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("#abc"), Some("AABBCC".to_string()));
    }

    #[test]
    fn test_parse_css_color_named() {
        assert_eq!(parse_css_color("red"), Some("FF0000".to_string()));
        assert_eq!(parse_css_color("blue"), Some("0000FF".to_string()));
        assert_eq!(parse_css_color("green"), Some("008000".to_string()));
        assert_eq!(parse_css_color("white"), Some("FFFFFF".to_string()));
        assert_eq!(parse_css_color("black"), Some("000000".to_string()));
    }

    #[test]
    fn test_parse_css_color_rgb() {
        assert_eq!(parse_css_color("rgb(255,0,0)"), Some("FF0000".to_string()));
        assert_eq!(
            parse_css_color("rgb(0, 128, 0)"),
            Some("008000".to_string())
        );
        assert_eq!(
            parse_css_color("rgba(0, 0, 255, 0.5)"),
            Some("0000FF".to_string())
        );
    }

    #[test]
    fn test_parse_css_color_invalid() {
        assert_eq!(parse_css_color("notacolor"), None);
        assert_eq!(parse_css_color("transparent"), None);
        assert_eq!(parse_css_color("#ggggg"), None);
    }

    #[test]
    fn test_parse_font_size() {
        assert_eq!(parse_font_size("20px"), Some(15));
        assert_eq!(parse_font_size("16px"), Some(12));
        assert_eq!(parse_font_size("18pt"), Some(18));
        assert_eq!(parse_font_size("12pt"), Some(12));
        assert_eq!(parse_font_size("44"), Some(44));
    }

    #[test]
    fn test_is_font_weight_bold() {
        assert!(is_font_weight_bold("bold"));
        assert!(is_font_weight_bold("700"));
        assert!(!is_font_weight_bold("normal"));
    }

    #[test]
    fn test_is_font_style_italic() {
        assert!(is_font_style_italic("italic"));
        assert!(!is_font_style_italic("normal"));
    }

    #[test]
    fn test_inline_style_parse_single() {
        let fmt = InlineStyle::parse("color: red")
            .to_bullet_format()
            .expect("color should produce format");
        assert_eq!(fmt.color, Some("FF0000".to_string()));
    }

    #[test]
    fn test_inline_style_parse_multiple() {
        let fmt = InlineStyle::parse("color: #0000FF; font-size: 20px; font-weight: bold")
            .to_bullet_format()
            .expect("format");
        assert_eq!(fmt.color, Some("0000FF".to_string()));
        assert_eq!(fmt.font_size, Some(15));
        assert!(fmt.bold);
    }

    #[test]
    fn test_inline_style_parse_background() {
        let fmt = InlineStyle::parse("background-color: yellow")
            .to_bullet_format()
            .expect("format");
        assert_eq!(fmt.highlight, Some("FFFF00".to_string()));
    }

    #[test]
    fn test_inline_style_parse_text_decoration() {
        let underline = InlineStyle::parse("text-decoration: underline")
            .to_bullet_format()
            .expect("format");
        assert!(underline.underline);
        let strike = InlineStyle::parse("text-decoration: line-through")
            .to_bullet_format()
            .expect("format");
        assert!(strike.strikethrough);
    }

    #[test]
    fn test_inline_style_parse_font_family() {
        let fmt = InlineStyle::parse("font-family: Arial")
            .to_bullet_format()
            .expect("format");
        assert_eq!(fmt.font_family.as_deref(), Some("Arial"));
    }

    #[test]
    fn test_inline_style_merge_child_overrides() {
        let parent = InlineStyle::parse("color: #FF0000; font-size: 20px");
        let child = InlineStyle::parse("color: #0000FF");
        let fmt = parent.merge(&child).to_bullet_format().expect("format");
        assert_eq!(fmt.color, Some("0000FF".to_string()));
        assert_eq!(fmt.font_size, Some(15));
    }

    #[test]
    fn test_inline_style_merge_empty_child() {
        let parent = InlineStyle::parse("color: red");
        let merged = parent.merge(&InlineStyle::default());
        let fmt = merged.to_bullet_format().expect("format");
        assert_eq!(fmt.color, Some("FF0000".to_string()));
    }

    #[test]
    fn test_inline_style_merge_no_parent() {
        let merged = InlineStyle::default().merge(&InlineStyle::parse("color: red; font-size: 18pt"));
        let fmt = merged.to_bullet_format().expect("format");
        assert_eq!(fmt.color, Some("FF0000".to_string()));
        assert_eq!(fmt.font_size, Some(18));
    }
}
