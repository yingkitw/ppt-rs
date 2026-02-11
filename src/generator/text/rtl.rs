//! RTL (right-to-left) text support
//!
//! Provides text direction control for RTL languages like Arabic, Hebrew, Urdu, etc.

/// Text direction for paragraphs and runs
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    LTR,
    RTL,
}

impl TextDirection {
    /// Get the OOXML attribute value
    pub fn to_xml_attr(&self) -> &'static str {
        match self {
            TextDirection::LTR => "0",
            TextDirection::RTL => "1",
        }
    }

    /// Whether this is RTL
    pub fn is_rtl(&self) -> bool {
        matches!(self, TextDirection::RTL)
    }
}

/// RTL language presets with appropriate default fonts
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum RtlLanguage {
    Arabic,
    Hebrew,
    Urdu,
    Persian,
    Pashto,
    Sindhi,
    Kurdish,
    Yiddish,
}

impl RtlLanguage {
    /// Get the BCP 47 language tag
    pub fn lang_tag(&self) -> &'static str {
        match self {
            RtlLanguage::Arabic => "ar-SA",
            RtlLanguage::Hebrew => "he-IL",
            RtlLanguage::Urdu => "ur-PK",
            RtlLanguage::Persian => "fa-IR",
            RtlLanguage::Pashto => "ps-AF",
            RtlLanguage::Sindhi => "sd-PK",
            RtlLanguage::Kurdish => "ku-IQ",
            RtlLanguage::Yiddish => "yi-001",
        }
    }

    /// Get the recommended default font for this language
    pub fn default_font(&self) -> &'static str {
        match self {
            RtlLanguage::Arabic | RtlLanguage::Urdu | RtlLanguage::Persian
            | RtlLanguage::Pashto | RtlLanguage::Sindhi | RtlLanguage::Kurdish => "Arial",
            RtlLanguage::Hebrew | RtlLanguage::Yiddish => "Arial",
        }
    }

    /// Get the text direction (always RTL for these languages)
    pub fn direction(&self) -> TextDirection {
        TextDirection::RTL
    }
}

/// RTL text properties for XML generation
#[derive(Clone, Debug, Default)]
pub struct RtlTextProps {
    pub direction: TextDirection,
    pub language: Option<String>,
    pub font_complex_script: Option<String>,
}

impl RtlTextProps {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set RTL direction
    pub fn rtl(mut self) -> Self {
        self.direction = TextDirection::RTL;
        self
    }

    /// Set LTR direction
    pub fn ltr(mut self) -> Self {
        self.direction = TextDirection::LTR;
        self
    }

    /// Set language tag (e.g., "ar-SA", "he-IL")
    pub fn language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }

    /// Set from an RTL language preset
    pub fn from_language(mut self, lang: RtlLanguage) -> Self {
        self.direction = lang.direction();
        self.language = Some(lang.lang_tag().to_string());
        self.font_complex_script = Some(lang.default_font().to_string());
        self
    }

    /// Set complex script font (used for RTL rendering)
    pub fn complex_script_font(mut self, font: &str) -> Self {
        self.font_complex_script = Some(font.to_string());
        self
    }

    /// Generate paragraph property XML attributes for RTL
    pub fn to_ppr_xml_attr(&self) -> String {
        if self.direction.is_rtl() {
            r#" rtl="1""#.to_string()
        } else {
            String::new()
        }
    }

    /// Generate run property XML attributes for RTL
    pub fn to_rpr_xml_attrs(&self) -> String {
        let mut attrs = String::new();
        if let Some(ref lang) = self.language {
            attrs.push_str(&format!(r#" lang="{lang}""#));
        }
        attrs
    }

    /// Generate complex script font XML element
    pub fn to_cs_font_xml(&self) -> String {
        if let Some(ref font) = self.font_complex_script {
            format!(r#"<a:cs typeface="{}"/>"#, super::escape_xml(font))
        } else {
            String::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_direction_default() {
        let dir = TextDirection::default();
        assert_eq!(dir, TextDirection::LTR);
        assert!(!dir.is_rtl());
    }

    #[test]
    fn test_text_direction_rtl() {
        let dir = TextDirection::RTL;
        assert!(dir.is_rtl());
        assert_eq!(dir.to_xml_attr(), "1");
    }

    #[test]
    fn test_text_direction_ltr() {
        let dir = TextDirection::LTR;
        assert!(!dir.is_rtl());
        assert_eq!(dir.to_xml_attr(), "0");
    }

    #[test]
    fn test_rtl_language_arabic() {
        let lang = RtlLanguage::Arabic;
        assert_eq!(lang.lang_tag(), "ar-SA");
        assert_eq!(lang.default_font(), "Arial");
        assert_eq!(lang.direction(), TextDirection::RTL);
    }

    #[test]
    fn test_rtl_language_hebrew() {
        let lang = RtlLanguage::Hebrew;
        assert_eq!(lang.lang_tag(), "he-IL");
        assert_eq!(lang.default_font(), "Arial");
    }

    #[test]
    fn test_rtl_language_all_variants() {
        let languages = [
            RtlLanguage::Arabic, RtlLanguage::Hebrew, RtlLanguage::Urdu,
            RtlLanguage::Persian, RtlLanguage::Pashto, RtlLanguage::Sindhi,
            RtlLanguage::Kurdish, RtlLanguage::Yiddish,
        ];
        for lang in &languages {
            assert_eq!(lang.direction(), TextDirection::RTL);
            assert!(!lang.lang_tag().is_empty());
            assert!(!lang.default_font().is_empty());
        }
    }

    #[test]
    fn test_rtl_text_props_default() {
        let props = RtlTextProps::new();
        assert_eq!(props.direction, TextDirection::LTR);
        assert!(props.language.is_none());
        assert!(props.font_complex_script.is_none());
    }

    #[test]
    fn test_rtl_text_props_builder() {
        let props = RtlTextProps::new()
            .rtl()
            .language("ar-SA")
            .complex_script_font("Arial");
        assert!(props.direction.is_rtl());
        assert_eq!(props.language.as_deref(), Some("ar-SA"));
        assert_eq!(props.font_complex_script.as_deref(), Some("Arial"));
    }

    #[test]
    fn test_rtl_text_props_from_language() {
        let props = RtlTextProps::new().from_language(RtlLanguage::Arabic);
        assert!(props.direction.is_rtl());
        assert_eq!(props.language.as_deref(), Some("ar-SA"));
        assert_eq!(props.font_complex_script.as_deref(), Some("Arial"));
    }

    #[test]
    fn test_rtl_ppr_xml_attr() {
        let rtl_props = RtlTextProps::new().rtl();
        assert_eq!(rtl_props.to_ppr_xml_attr(), r#" rtl="1""#);

        let ltr_props = RtlTextProps::new().ltr();
        assert_eq!(ltr_props.to_ppr_xml_attr(), "");
    }

    #[test]
    fn test_rtl_rpr_xml_attrs() {
        let props = RtlTextProps::new().language("he-IL");
        assert!(props.to_rpr_xml_attrs().contains(r#"lang="he-IL""#));
    }

    #[test]
    fn test_rtl_cs_font_xml() {
        let props = RtlTextProps::new().complex_script_font("Arial");
        let xml = props.to_cs_font_xml();
        assert!(xml.contains(r#"<a:cs typeface="Arial"/>"#));

        let empty = RtlTextProps::new();
        assert_eq!(empty.to_cs_font_xml(), "");
    }

    #[test]
    fn test_rtl_props_ltr_override() {
        let props = RtlTextProps::new().rtl().ltr();
        assert!(!props.direction.is_rtl());
    }
}
