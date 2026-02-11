//! Embedded font support for PPTX output
//!
//! Allows embedding font files into the presentation so they render correctly
//! on systems that don't have the fonts installed. Generates proper
//! `<p:embeddedFontLst>` XML in presentation.xml.

/// Font style variant
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Regular,
    Bold,
    Italic,
    BoldItalic,
}

impl FontStyle {
    pub fn xml_element(&self) -> &'static str {
        match self {
            FontStyle::Regular => "regular",
            FontStyle::Bold => "bold",
            FontStyle::Italic => "italic",
            FontStyle::BoldItalic => "boldItalic",
        }
    }

    pub fn is_bold(&self) -> bool {
        matches!(self, FontStyle::Bold | FontStyle::BoldItalic)
    }

    pub fn is_italic(&self) -> bool {
        matches!(self, FontStyle::Italic | FontStyle::BoldItalic)
    }
}

/// Character set for the font
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum FontCharset {
    #[default]
    Ansi,
    Symbol,
    ShiftJis,
    Hangul,
    Gb2312,
    ChineseBig5,
    Greek,
    Turkish,
    Hebrew,
    Arabic,
    Baltic,
    Russian,
    Thai,
    EastEurope,
}

impl FontCharset {
    pub fn code(&self) -> u8 {
        match self {
            FontCharset::Ansi => 0x00,
            FontCharset::Symbol => 0x02,
            FontCharset::ShiftJis => 0x80,
            FontCharset::Hangul => 0x81,
            FontCharset::Gb2312 => 0x86,
            FontCharset::ChineseBig5 => 0x88,
            FontCharset::Greek => 0xA1,
            FontCharset::Turkish => 0xA2,
            FontCharset::Hebrew => 0xB1,
            FontCharset::Arabic => 0xB2,
            FontCharset::Baltic => 0xBA,
            FontCharset::Russian => 0xCC,
            FontCharset::Thai => 0xDE,
            FontCharset::EastEurope => 0xEE,
        }
    }
}

/// A single embedded font entry
#[derive(Clone, Debug)]
pub struct EmbeddedFont {
    pub typeface: String,
    pub style: FontStyle,
    pub charset: FontCharset,
    pub panose: Option<String>,
    pub pitch_family: u8,
    pub data: Vec<u8>,
    pub relationship_id: String,
}

impl EmbeddedFont {
    /// Create a new embedded font entry
    pub fn new(typeface: &str, style: FontStyle, data: Vec<u8>, rel_id: &str) -> Self {
        Self {
            typeface: typeface.to_string(),
            style,
            charset: FontCharset::default(),
            panose: None,
            pitch_family: 0x22, // Variable pitch, Roman family
            data,
            relationship_id: rel_id.to_string(),
        }
    }

    pub fn charset(mut self, charset: FontCharset) -> Self {
        self.charset = charset;
        self
    }

    pub fn panose(mut self, panose: &str) -> Self {
        self.panose = Some(panose.to_string());
        self
    }

    pub fn pitch_family(mut self, pf: u8) -> Self {
        self.pitch_family = pf;
        self
    }

    /// Font data size in bytes
    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    /// Content type for the embedded font part
    pub fn content_type() -> &'static str {
        "application/x-fontdata"
    }

    /// Part name in the ZIP archive
    pub fn part_name(&self) -> String {
        format!(
            "ppt/fonts/{}-{}.fntdata",
            self.typeface.replace(' ', ""),
            self.style.xml_element()
        )
    }
}

/// Manages all embedded fonts for a presentation
#[derive(Clone, Debug, Default)]
pub struct EmbeddedFontList {
    fonts: Vec<EmbeddedFont>,
}

impl EmbeddedFontList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an embedded font
    pub fn add(&mut self, font: EmbeddedFont) {
        self.fonts.push(font);
    }

    /// Get all fonts
    pub fn fonts(&self) -> &[EmbeddedFont] {
        &self.fonts
    }

    /// Number of embedded fonts
    pub fn len(&self) -> usize {
        self.fonts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fonts.is_empty()
    }

    /// Total size of all font data
    pub fn total_size(&self) -> usize {
        self.fonts.iter().map(|f| f.data_size()).sum()
    }

    /// Find fonts by typeface name
    pub fn find_by_typeface(&self, typeface: &str) -> Vec<&EmbeddedFont> {
        self.fonts.iter().filter(|f| f.typeface == typeface).collect()
    }

    /// Generate `<p:embeddedFontLst>` XML for presentation.xml
    pub fn to_xml(&self) -> String {
        if self.fonts.is_empty() {
            return String::new();
        }

        let mut xml = String::from("<p:embeddedFontLst>");

        // Group by typeface
        let mut seen_typefaces: Vec<String> = Vec::new();
        for font in &self.fonts {
            if !seen_typefaces.contains(&font.typeface) {
                seen_typefaces.push(font.typeface.clone());
            }
        }

        for typeface in &seen_typefaces {
            let variants: Vec<&EmbeddedFont> = self.fonts
                .iter()
                .filter(|f| &f.typeface == typeface)
                .collect();

            xml.push_str("<p:embeddedFont>");

            // Font descriptor (from first variant)
            if let Some(first) = variants.first() {
                let panose_attr = first.panose.as_ref()
                    .map(|p| format!(r#" panose="{}""#, p))
                    .unwrap_or_default();
                xml.push_str(&format!(
                    r#"<p:font typeface="{}" charset="{}" pitchFamily="{}"{}/>"#,
                    xml_escape(typeface),
                    first.charset.code(),
                    first.pitch_family,
                    panose_attr,
                ));
            }

            // Font data references per style
            for font in &variants {
                xml.push_str(&format!(
                    r#"<p:{} r:id="{}"/>"#,
                    font.style.xml_element(),
                    font.relationship_id,
                ));
            }

            xml.push_str("</p:embeddedFont>");
        }

        xml.push_str("</p:embeddedFontLst>");
        xml
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_style_xml() {
        assert_eq!(FontStyle::Regular.xml_element(), "regular");
        assert_eq!(FontStyle::Bold.xml_element(), "bold");
        assert_eq!(FontStyle::Italic.xml_element(), "italic");
        assert_eq!(FontStyle::BoldItalic.xml_element(), "boldItalic");
    }

    #[test]
    fn test_font_style_flags() {
        assert!(!FontStyle::Regular.is_bold());
        assert!(!FontStyle::Regular.is_italic());
        assert!(FontStyle::Bold.is_bold());
        assert!(!FontStyle::Bold.is_italic());
        assert!(!FontStyle::Italic.is_bold());
        assert!(FontStyle::Italic.is_italic());
        assert!(FontStyle::BoldItalic.is_bold());
        assert!(FontStyle::BoldItalic.is_italic());
    }

    #[test]
    fn test_font_charset_default() {
        assert_eq!(FontCharset::default(), FontCharset::Ansi);
        assert_eq!(FontCharset::Ansi.code(), 0x00);
    }

    #[test]
    fn test_font_charset_codes() {
        assert_eq!(FontCharset::Symbol.code(), 0x02);
        assert_eq!(FontCharset::ShiftJis.code(), 0x80);
        assert_eq!(FontCharset::Arabic.code(), 0xB2);
        assert_eq!(FontCharset::Russian.code(), 0xCC);
    }

    #[test]
    fn test_embedded_font_new() {
        let font = EmbeddedFont::new("Arial", FontStyle::Regular, vec![1, 2, 3], "rId10");
        assert_eq!(font.typeface, "Arial");
        assert_eq!(font.style, FontStyle::Regular);
        assert_eq!(font.data_size(), 3);
        assert_eq!(font.relationship_id, "rId10");
    }

    #[test]
    fn test_embedded_font_builder() {
        let font = EmbeddedFont::new("Calibri", FontStyle::Bold, vec![0; 100], "rId1")
            .charset(FontCharset::Russian)
            .panose("020F0502020204030204")
            .pitch_family(0x34);
        assert_eq!(font.charset, FontCharset::Russian);
        assert_eq!(font.panose.as_deref(), Some("020F0502020204030204"));
        assert_eq!(font.pitch_family, 0x34);
    }

    #[test]
    fn test_embedded_font_part_name() {
        let font = EmbeddedFont::new("Times New Roman", FontStyle::BoldItalic, vec![], "rId1");
        assert_eq!(font.part_name(), "ppt/fonts/TimesNewRoman-boldItalic.fntdata");
    }

    #[test]
    fn test_embedded_font_content_type() {
        assert_eq!(EmbeddedFont::content_type(), "application/x-fontdata");
    }

    #[test]
    fn test_embedded_font_list_new() {
        let list = EmbeddedFontList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.total_size(), 0);
    }

    #[test]
    fn test_embedded_font_list_add() {
        let mut list = EmbeddedFontList::new();
        list.add(EmbeddedFont::new("Arial", FontStyle::Regular, vec![0; 50], "rId1"));
        list.add(EmbeddedFont::new("Arial", FontStyle::Bold, vec![0; 60], "rId2"));
        assert_eq!(list.len(), 2);
        assert_eq!(list.total_size(), 110);
    }

    #[test]
    fn test_embedded_font_list_find() {
        let mut list = EmbeddedFontList::new();
        list.add(EmbeddedFont::new("Arial", FontStyle::Regular, vec![], "rId1"));
        list.add(EmbeddedFont::new("Calibri", FontStyle::Regular, vec![], "rId2"));
        list.add(EmbeddedFont::new("Arial", FontStyle::Bold, vec![], "rId3"));
        assert_eq!(list.find_by_typeface("Arial").len(), 2);
        assert_eq!(list.find_by_typeface("Calibri").len(), 1);
        assert_eq!(list.find_by_typeface("Missing").len(), 0);
    }

    #[test]
    fn test_embedded_font_list_xml_empty() {
        let list = EmbeddedFontList::new();
        assert_eq!(list.to_xml(), "");
    }

    #[test]
    fn test_embedded_font_list_xml() {
        let mut list = EmbeddedFontList::new();
        list.add(EmbeddedFont::new("Arial", FontStyle::Regular, vec![0; 10], "rId10"));
        list.add(EmbeddedFont::new("Arial", FontStyle::Bold, vec![0; 10], "rId11"));
        let xml = list.to_xml();
        assert!(xml.contains("<p:embeddedFontLst>"));
        assert!(xml.contains("</p:embeddedFontLst>"));
        assert!(xml.contains(r#"typeface="Arial""#));
        assert!(xml.contains(r#"r:id="rId10""#));
        assert!(xml.contains(r#"r:id="rId11""#));
        assert!(xml.contains("<p:regular"));
        assert!(xml.contains("<p:bold"));
        // Should be grouped under one <p:embeddedFont>
        assert_eq!(xml.matches("<p:embeddedFont>").count(), 1);
    }

    #[test]
    fn test_embedded_font_list_xml_multiple_typefaces() {
        let mut list = EmbeddedFontList::new();
        list.add(EmbeddedFont::new("Arial", FontStyle::Regular, vec![], "rId1"));
        list.add(EmbeddedFont::new("Calibri", FontStyle::Regular, vec![], "rId2"));
        let xml = list.to_xml();
        assert_eq!(xml.matches("<p:embeddedFont>").count(), 2);
        assert!(xml.contains("Arial"));
        assert!(xml.contains("Calibri"));
    }

    #[test]
    fn test_embedded_font_list_xml_with_panose() {
        let mut list = EmbeddedFontList::new();
        list.add(
            EmbeddedFont::new("Calibri", FontStyle::Regular, vec![], "rId1")
                .panose("020F0502020204030204"),
        );
        let xml = list.to_xml();
        assert!(xml.contains(r#"panose="020F0502020204030204""#));
    }
}
