//! XML utility functions
//!
//! Centralized XML utilities to avoid duplication across modules.

/// Escape special XML characters
pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// XML writer helper for building XML strings efficiently
#[allow(dead_code)]
pub struct XmlWriter {
    buffer: String,
    indent_level: usize,
    indent_str: &'static str,
}

impl XmlWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            indent_level: 0,
            indent_str: "  ",
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
            indent_level: 0,
            indent_str: "  ",
        }
    }

    /// Write XML declaration
    pub fn xml_declaration(&mut self) -> &mut Self {
        self.buffer
            .push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        self.buffer.push('\n');
        self
    }

    /// Start an element with attributes
    pub fn start_element(&mut self, name: &str, attrs: &[(&str, &str)]) -> &mut Self {
        self.buffer.push('<');
        self.buffer.push_str(name);
        for (key, value) in attrs {
            self.buffer.push(' ');
            self.buffer.push_str(key);
            self.buffer.push_str("=\"");
            self.buffer.push_str(&escape_xml(value));
            self.buffer.push('"');
        }
        self.buffer.push('>');
        self.indent_level += 1;
        self
    }

    /// End an element
    pub fn end_element(&mut self, name: &str) -> &mut Self {
        self.indent_level = self.indent_level.saturating_sub(1);
        self.buffer.push_str("</");
        self.buffer.push_str(name);
        self.buffer.push('>');
        self
    }

    /// Write a self-closing element
    pub fn empty_element(&mut self, name: &str, attrs: &[(&str, &str)]) -> &mut Self {
        self.buffer.push('<');
        self.buffer.push_str(name);
        for (key, value) in attrs {
            self.buffer.push(' ');
            self.buffer.push_str(key);
            self.buffer.push_str("=\"");
            self.buffer.push_str(&escape_xml(value));
            self.buffer.push('"');
        }
        self.buffer.push_str("/>");
        self
    }

    /// Write text content
    pub fn text(&mut self, content: &str) -> &mut Self {
        self.buffer.push_str(&escape_xml(content));
        self
    }

    /// Write raw XML (no escaping)
    pub fn raw(&mut self, xml: &str) -> &mut Self {
        self.buffer.push_str(xml);
        self
    }

    /// Get the built XML string
    pub fn finish(self) -> String {
        self.buffer
    }

    /// Get a reference to the buffer
    pub fn as_str(&self) -> &str {
        &self.buffer
    }
}

impl Default for XmlWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_xml_writer() {
        let mut writer = XmlWriter::new();
        writer
            .start_element("root", &[("attr", "value")])
            .text("content")
            .end_element("root");
        assert_eq!(writer.finish(), r#"<root attr="value">content</root>"#);
    }

    #[test]
    fn test_xml_writer_empty_element() {
        let mut writer = XmlWriter::new();
        writer.empty_element("br", &[]);
        assert_eq!(writer.finish(), "<br/>");
    }
}
