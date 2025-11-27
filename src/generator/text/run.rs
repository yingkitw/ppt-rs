//! Text run - a span of text with consistent formatting

use super::format::TextFormat;
use super::escape_xml;

/// A run of text with consistent formatting
#[derive(Clone, Debug)]
pub struct Run {
    pub text: String,
    pub format: TextFormat,
}

impl Run {
    /// Create a new text run
    pub fn new(text: &str) -> Self {
        Run {
            text: text.to_string(),
            format: TextFormat::default(),
        }
    }

    /// Apply formatting
    pub fn with_format(mut self, format: TextFormat) -> Self {
        self.format = format;
        self
    }

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.format.bold = true;
        self
    }

    /// Set italic
    pub fn italic(mut self) -> Self {
        self.format.italic = true;
        self
    }

    /// Set underline
    pub fn underline(mut self) -> Self {
        self.format.underline = true;
        self
    }

    /// Set color
    pub fn color(mut self, hex: &str) -> Self {
        self.format.color = Some(hex.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set font size
    pub fn size(mut self, points: u32) -> Self {
        self.format.font_size = Some(points);
        self
    }

    /// Set font family
    pub fn font(mut self, family: &str) -> Self {
        self.format.font_family = Some(family.to_string());
        self
    }

    /// Generate XML for this run
    pub fn to_xml(&self) -> String {
        let size = self.format.font_size.unwrap_or(18) * 100;
        let bold = if self.format.bold { "1" } else { "0" };
        let italic = if self.format.italic { "1" } else { "0" };
        let underline = if self.format.underline { " u=\"sng\"" } else { "" };
        
        let color_xml = self.format.color.as_ref()
            .map(|c| format!(r#"<a:solidFill><a:srgbClr val="{}"/></a:solidFill>"#, c))
            .unwrap_or_default();
        
        let font_xml = self.format.font_family.as_ref()
            .map(|f| format!(r#"<a:latin typeface="{}"/>"#, escape_xml(f)))
            .unwrap_or_default();

        format!(
            r#"<a:r><a:rPr lang="en-US" sz="{}" b="{}" i="{}"{} dirty="0">{}{}</a:rPr><a:t>{}</a:t></a:r>"#,
            size, bold, italic, underline, color_xml, font_xml, escape_xml(&self.text)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_to_xml() {
        let run = Run::new("Hello").bold().color("FF0000").size(24);
        let xml = run.to_xml();
        
        assert!(xml.contains("Hello"));
        assert!(xml.contains("b=\"1\""));
        assert!(xml.contains("FF0000"));
        assert!(xml.contains("sz=\"2400\""));
    }

    #[test]
    fn test_font_family() {
        let run = Run::new("Arial text").font("Arial");
        let xml = run.to_xml();
        
        assert!(xml.contains("typeface=\"Arial\""));
    }
}
