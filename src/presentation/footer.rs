//! Footer and Header Support
//!
//! This module provides support for document-wide footers and headers in presentations.
//! Footers and headers can include:
//! - Custom text
//! - Slide numbers
//! - Date/time
//! - Applied to all slides or specific slides


/// Footer and header configuration
#[derive(Debug, Clone)]
pub struct FooterHeader {
    /// Footer text
    footer_text: Option<String>,
    /// Header text
    header_text: Option<String>,
    /// Show footer on all slides
    show_footer: bool,
    /// Show header on all slides
    show_header: bool,
    /// Show slide number on all slides
    show_slide_number: bool,
    /// Show date/time on all slides
    show_date: bool,
    /// Date/time text
    date_text: Option<String>,
    /// Apply to title slide
    apply_to_title: bool,
    /// Apply to notes pages
    apply_to_notes: bool,
}

impl Default for FooterHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl FooterHeader {
    /// Create a new footer/header configuration
    pub fn new() -> Self {
        Self {
            footer_text: None,
            header_text: None,
            show_footer: false,
            show_header: false,
            show_slide_number: false,
            show_date: false,
            date_text: None,
            apply_to_title: false,
            apply_to_notes: false,
        }
    }

    /// Set footer text
    pub fn set_footer(mut self, text: impl Into<String>) -> Self {
        self.footer_text = Some(text.into());
        self.show_footer = true;
        self
    }

    /// Get footer text
    pub fn footer(&self) -> Option<&str> {
        self.footer_text.as_deref()
    }

    /// Set header text
    pub fn set_header(mut self, text: impl Into<String>) -> Self {
        self.header_text = Some(text.into());
        self.show_header = true;
        self
    }

    /// Get header text
    pub fn header(&self) -> Option<&str> {
        self.header_text.as_deref()
    }

    /// Enable footer display
    pub fn enable_footer(mut self) -> Self {
        self.show_footer = true;
        self
    }

    /// Disable footer display
    pub fn disable_footer(mut self) -> Self {
        self.show_footer = false;
        self
    }

    /// Check if footer is enabled
    pub fn is_footer_enabled(&self) -> bool {
        self.show_footer
    }

    /// Enable header display
    pub fn enable_header(mut self) -> Self {
        self.show_header = true;
        self
    }

    /// Disable header display
    pub fn disable_header(mut self) -> Self {
        self.show_header = false;
        self
    }

    /// Check if header is enabled
    pub fn is_header_enabled(&self) -> bool {
        self.show_header
    }

    /// Enable slide number display
    pub fn enable_slide_number(mut self) -> Self {
        self.show_slide_number = true;
        self
    }

    /// Disable slide number display
    pub fn disable_slide_number(mut self) -> Self {
        self.show_slide_number = false;
        self
    }

    /// Check if slide number is enabled
    pub fn is_slide_number_enabled(&self) -> bool {
        self.show_slide_number
    }

    /// Enable date/time display
    pub fn enable_date(mut self) -> Self {
        self.show_date = true;
        self
    }

    /// Disable date/time display
    pub fn disable_date(mut self) -> Self {
        self.show_date = false;
        self
    }

    /// Check if date/time is enabled
    pub fn is_date_enabled(&self) -> bool {
        self.show_date
    }

    /// Set date/time text
    pub fn set_date_text(mut self, text: impl Into<String>) -> Self {
        self.date_text = Some(text.into());
        self.show_date = true;
        self
    }

    /// Get date/time text
    pub fn date_text(&self) -> Option<&str> {
        self.date_text.as_deref()
    }

    /// Apply to title slide
    pub fn apply_to_title(mut self, apply: bool) -> Self {
        self.apply_to_title = apply;
        self
    }

    /// Check if applied to title slide
    pub fn is_applied_to_title(&self) -> bool {
        self.apply_to_title
    }

    /// Apply to notes pages
    pub fn apply_to_notes(mut self, apply: bool) -> Self {
        self.apply_to_notes = apply;
        self
    }

    /// Check if applied to notes pages
    pub fn is_applied_to_notes(&self) -> bool {
        self.apply_to_notes
    }

    /// Generate XML for footer/header in presentation.xml
    pub fn to_presentation_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<p:notesMasterIdLst>");

        if self.show_footer || self.show_header || self.show_slide_number || self.show_date {
            xml.push_str("<p:hf");

            if self.show_footer {
                xml.push_str(" footerText=\"");
                if let Some(text) = &self.footer_text {
                    xml.push_str(&escape_xml(text));
                }
                xml.push('"');
            }

            if self.show_header {
                xml.push_str(" headerText=\"");
                if let Some(text) = &self.header_text {
                    xml.push_str(&escape_xml(text));
                }
                xml.push('"');
            }

            if self.show_slide_number {
                xml.push_str(" sldNumVisible=\"1\"");
            }

            if self.show_date {
                xml.push_str(" dtVisible=\"1\"");
                if let Some(text) = &self.date_text {
                    xml.push_str(" dt=\"");
                    xml.push_str(&escape_xml(text));
                    xml.push('"');
                }
            }

            if !self.apply_to_title {
                xml.push_str(" noGrpMask=\"1\"");
            }

            xml.push_str("/>");
        }

        xml.push_str("</p:notesMasterIdLst>");
        xml
    }

    /// Generate XML for slide footer/header
    pub fn to_slide_xml(&self, slide_number: usize) -> String {
        let mut xml = String::new();
        xml.push_str("<p:cSld>");

        if self.show_footer || self.show_header || self.show_slide_number || self.show_date {
            xml.push_str("<p:hf");

            if self.show_footer {
                xml.push_str(" footerText=\"");
                if let Some(text) = &self.footer_text {
                    xml.push_str(&escape_xml(text));
                }
                xml.push('"');
            }

            if self.show_header {
                xml.push_str(" headerText=\"");
                if let Some(text) = &self.header_text {
                    xml.push_str(&escape_xml(text));
                }
                xml.push('"');
            }

            if self.show_slide_number {
                xml.push_str(&format!(" sldNum=\"{}\"", slide_number));
            }

            if self.show_date {
                xml.push_str(" dt=\"");
                if let Some(text) = &self.date_text {
                    xml.push_str(&escape_xml(text));
                } else {
                    xml.push_str(&format!("{}", chrono::Local::now().format("%Y-%m-%d")));
                }
                xml.push('"');
            }

            xml.push_str("/>");
        }

        xml.push_str("</p:cSld>");
        xml
    }
}

/// Escape XML special characters
fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_header_new() {
        let fh = FooterHeader::new();
        assert!(!fh.is_footer_enabled());
        assert!(!fh.is_header_enabled());
        assert!(!fh.is_slide_number_enabled());
        assert!(!fh.is_date_enabled());
    }

    #[test]
    fn test_set_footer() {
        let fh = FooterHeader::new().set_footer("My Footer");
        assert!(fh.is_footer_enabled());
        assert_eq!(fh.footer(), Some("My Footer"));
    }

    #[test]
    fn test_set_header() {
        let fh = FooterHeader::new().set_header("My Header");
        assert!(fh.is_header_enabled());
        assert_eq!(fh.header(), Some("My Header"));
    }

    #[test]
    fn test_enable_slide_number() {
        let fh = FooterHeader::new().enable_slide_number();
        assert!(fh.is_slide_number_enabled());
    }

    #[test]
    fn test_enable_date() {
        let fh = FooterHeader::new().enable_date();
        assert!(fh.is_date_enabled());
    }

    #[test]
    fn test_set_date_text() {
        let fh = FooterHeader::new().set_date_text("November 10, 2025");
        assert!(fh.is_date_enabled());
        assert_eq!(fh.date_text(), Some("November 10, 2025"));
    }

    #[test]
    fn test_apply_to_title() {
        let fh = FooterHeader::new().apply_to_title(true);
        assert!(fh.is_applied_to_title());
    }

    #[test]
    fn test_apply_to_notes() {
        let fh = FooterHeader::new().apply_to_notes(true);
        assert!(fh.is_applied_to_notes());
    }

    #[test]
    fn test_disable_footer() {
        let fh = FooterHeader::new()
            .set_footer("Footer")
            .disable_footer();
        assert!(!fh.is_footer_enabled());
    }

    #[test]
    fn test_footer_header_combined() {
        let fh = FooterHeader::new()
            .set_footer("Company Name")
            .set_header("Confidential")
            .enable_slide_number()
            .set_date_text("November 2025");

        assert!(fh.is_footer_enabled());
        assert!(fh.is_header_enabled());
        assert!(fh.is_slide_number_enabled());
        assert!(fh.is_date_enabled());
    }

    #[test]
    fn test_escape_xml() {
        let text = "Test & <special> \"chars\"";
        let escaped = escape_xml(text);
        assert!(escaped.contains("&amp;"));
        assert!(escaped.contains("&lt;"));
        assert!(escaped.contains("&gt;"));
        assert!(escaped.contains("&quot;"));
    }

    #[test]
    fn test_to_presentation_xml() {
        let fh = FooterHeader::new()
            .set_footer("Footer")
            .enable_slide_number();
        let xml = fh.to_presentation_xml();
        assert!(xml.contains("<p:notesMasterIdLst>"));
        assert!(xml.contains("</p:notesMasterIdLst>"));
    }

    #[test]
    fn test_to_slide_xml() {
        let fh = FooterHeader::new()
            .set_footer("Footer")
            .enable_slide_number();
        let xml = fh.to_slide_xml(1);
        assert!(xml.contains("<p:cSld>"));
        assert!(xml.contains("</p:cSld>"));
    }
}
