//! XML utility functions
//!
//! Centralized XML utilities to avoid duplication across modules.

use std::fmt::Write;

/// Escape special XML characters in a single pass.
pub fn escape_xml(s: &str) -> String {
    // Fast path: most strings need no escaping.
    if !s
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        return s.to_string();
    }

    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Append escaped XML text into an existing buffer (avoids intermediate String).
pub fn append_escape_xml(buf: &mut String, s: &str) {
    if !s
        .bytes()
        .any(|b| matches!(b, b'&' | b'<' | b'>' | b'"' | b'\''))
    {
        buf.push_str(s);
        return;
    }
    for c in s.chars() {
        match c {
            '&' => buf.push_str("&amp;"),
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '"' => buf.push_str("&quot;"),
            '\'' => buf.push_str("&apos;"),
            _ => buf.push(c),
        }
    }
}

/// Append a decimal integer without allocating a temporary `format!` string.
pub fn append_usize(buf: &mut String, value: usize) {
    let _ = write!(buf, "{value}");
}

/// Append a signed integer without allocating a temporary `format!` string.
pub fn append_i32(buf: &mut String, value: i32) {
    let _ = write!(buf, "{value}");
}

/// XML writer helper for building XML strings efficiently
pub struct XmlWriter {
    buffer: String,
}

impl XmlWriter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: String::with_capacity(capacity),
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
            append_escape_xml(&mut self.buffer, value);
            self.buffer.push('"');
        }
        self.buffer.push('>');
        self
    }

    /// End an element
    pub fn end_element(&mut self, name: &str) -> &mut Self {
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
            append_escape_xml(&mut self.buffer, value);
            self.buffer.push('"');
        }
        self.buffer.push_str("/>");
        self
    }

    /// Write text content
    pub fn text(&mut self, content: &str) -> &mut Self {
        append_escape_xml(&mut self.buffer, content);
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
        assert_eq!(escape_xml("plain"), "plain");
        assert_eq!(escape_xml("a & b"), "a &amp; b");
        assert_eq!(escape_xml("<tag>"), "&lt;tag&gt;");
        assert_eq!(escape_xml("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(escape_xml("it's"), "it&apos;s");
    }

    #[test]
    fn test_append_escape_xml() {
        let mut buf = String::from("pre:");
        append_escape_xml(&mut buf, "a < b & \"c\"");
        assert_eq!(buf, "pre:a &lt; b &amp; &quot;c&quot;");
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

    #[test]
    fn test_append_usize() {
        let mut buf = String::new();
        append_usize(&mut buf, 42);
        assert_eq!(buf, "42");
    }
}
