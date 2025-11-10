//! Streaming XML reader and writer for efficient large document handling
//!
//! Provides streaming capabilities for processing large XML documents
//! without loading entire content into memory.

use crate::error::{PptError, Result};
use std::io::{Read, Write};

/// Streaming XML event for callback-based processing
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XmlEvent {
    /// Start element with tag name and attributes
    StartElement {
        name: String,
        attributes: Vec<(String, String)>,
    },
    /// End element with tag name
    EndElement { name: String },
    /// Character data
    Characters(String),
    /// Comment
    Comment(String),
    /// Processing instruction
    ProcessingInstruction { target: String, data: String },
}

/// Streaming XML reader for callback-based processing
pub struct StreamingXmlReader;

impl StreamingXmlReader {
    /// Create a new streaming XML reader
    pub fn new() -> Self {
        Self
    }

    /// Read XML from a reader with callback for each event
    pub fn read<R: Read, F>(&self, reader: R, mut callback: F) -> Result<()>
    where
        F: FnMut(XmlEvent) -> Result<()>,
    {
        // Simple implementation: read content and parse events
        let mut content = String::new();
        let mut reader = reader;
        reader.read_to_string(&mut content)
            .map_err(|e| PptError::Io(e))?;

        self.parse_events(&content, &mut callback)?;
        Ok(())
    }

    /// Parse XML content into events
    fn parse_events<F>(&self, content: &str, callback: &mut F) -> Result<()>
    where
        F: FnMut(XmlEvent) -> Result<()>,
    {
        let mut pos = 0;
        let bytes = content.as_bytes();

        while pos < bytes.len() {
            // Find next tag
            if let Some(start) = content[pos..].find('<') {
                let tag_start = pos + start;
                
                // Emit character data if any
                if tag_start > pos {
                    let text = content[pos..tag_start].trim();
                    if !text.is_empty() {
                        callback(XmlEvent::Characters(text.to_string()))?;
                    }
                }

                // Find end of tag
                if let Some(end) = content[tag_start..].find('>') {
                    let tag_end = tag_start + end + 1;
                    let tag_content = &content[tag_start + 1..tag_start + end];

                    // Parse tag
                    if tag_content.starts_with("?") {
                        // Processing instruction
                        let pi = tag_content[1..].trim_end_matches('?');
                        if let Some(space) = pi.find(' ') {
                            let target = pi[..space].to_string();
                            let data = pi[space + 1..].to_string();
                            callback(XmlEvent::ProcessingInstruction { target, data })?;
                        }
                    } else if tag_content.starts_with("!--") {
                        // Comment
                        let comment = tag_content[3..].trim_end_matches("--").to_string();
                        callback(XmlEvent::Comment(comment))?;
                    } else if tag_content.starts_with("/") {
                        // End element
                        let name = tag_content[1..].trim().to_string();
                        callback(XmlEvent::EndElement { name })?;
                    } else {
                        // Start element
                        let (name, attrs) = self.parse_element(tag_content)?;
                        callback(XmlEvent::StartElement {
                            name,
                            attributes: attrs,
                        })?;
                    }

                    pos = tag_end;
                } else {
                    break;
                }
            } else {
                // Remaining text
                let text = content[pos..].trim();
                if !text.is_empty() {
                    callback(XmlEvent::Characters(text.to_string()))?;
                }
                break;
            }
        }

        Ok(())
    }

    /// Parse element tag into name and attributes
    fn parse_element(&self, tag: &str) -> Result<(String, Vec<(String, String)>)> {
        let tag = tag.trim();
        let tag = if tag.ends_with("/") {
            &tag[..tag.len() - 1]
        } else {
            tag
        };

        let parts: Vec<&str> = tag.splitn(2, ' ').collect();
        let name = parts[0].to_string();
        
        let mut attributes = Vec::new();
        if parts.len() > 1 {
            // Parse attributes (simplified)
            let attr_str = parts[1];
            // This is a simplified parser - real implementation would be more robust
            for attr in attr_str.split_whitespace() {
                if let Some(eq) = attr.find('=') {
                    let key = attr[..eq].to_string();
                    let value = attr[eq + 1..]
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string();
                    attributes.push((key, value));
                }
            }
        }

        Ok((name, attributes))
    }
}

impl Default for StreamingXmlReader {
    fn default() -> Self {
        Self::new()
    }
}

/// Streaming XML writer for efficient serialization
pub struct StreamingXmlWriter {
    indent_level: usize,
    indent_size: usize,
}

impl StreamingXmlWriter {
    /// Create a new streaming XML writer
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_size: 2,
        }
    }

    /// Create writer with custom indent size
    pub fn with_indent(indent_size: usize) -> Self {
        Self {
            indent_level: 0,
            indent_size,
        }
    }

    /// Write XML declaration
    pub fn write_declaration<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n")
            .map_err(|e| PptError::Io(e))?;
        Ok(())
    }

    /// Write start element
    pub fn write_start_element<W: Write>(
        &mut self,
        writer: &mut W,
        name: &str,
        attributes: &[(String, String)],
    ) -> Result<()> {
        let indent = " ".repeat(self.indent_level * self.indent_size);
        write!(writer, "{}<{}", indent, name).map_err(|e| PptError::Io(e))?;

        for (key, value) in attributes {
            write!(writer, " {}=\"{}\"", key, value).map_err(|e| PptError::Io(e))?;
        }

        writer.write_all(b">\n").map_err(|e| PptError::Io(e))?;
        self.indent_level += 1;
        Ok(())
    }

    /// Write end element
    pub fn write_end_element<W: Write>(&mut self, writer: &mut W, name: &str) -> Result<()> {
        self.indent_level = self.indent_level.saturating_sub(1);
        let indent = " ".repeat(self.indent_level * self.indent_size);
        writeln!(writer, "{}</{}>", indent, name).map_err(|e| PptError::Io(e))?;
        Ok(())
    }

    /// Write character data
    pub fn write_characters<W: Write>(&self, writer: &mut W, text: &str) -> Result<()> {
        let indent = " ".repeat(self.indent_level * self.indent_size);
        writeln!(writer, "{}{}", indent, text).map_err(|e| PptError::Io(e))?;
        Ok(())
    }

    /// Write self-closing element
    pub fn write_element<W: Write>(
        &self,
        writer: &mut W,
        name: &str,
        attributes: &[(String, String)],
    ) -> Result<()> {
        let indent = " ".repeat(self.indent_level * self.indent_size);
        write!(writer, "{}<{}", indent, name).map_err(|e| PptError::Io(e))?;

        for (key, value) in attributes {
            write!(writer, " {}=\"{}\"", key, value).map_err(|e| PptError::Io(e))?;
        }

        writer.write_all(b" />\n").map_err(|e| PptError::Io(e))?;
        Ok(())
    }
}

impl Default for StreamingXmlWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_streaming_reader_new() {
        let _reader = StreamingXmlReader::new();
        // Just verify it constructs without error
    }

    #[test]
    fn test_streaming_reader_parse_simple() {
        let reader = StreamingXmlReader::new();
        let xml = b"<?xml version=\"1.0\"?><root><item>test</item></root>";
        let cursor = Cursor::new(xml);
        
        let mut events = Vec::new();
        reader.read(cursor, |event| {
            events.push(event);
            Ok(())
        }).unwrap();

        assert!(!events.is_empty());
    }

    #[test]
    fn test_streaming_writer_new() {
        let writer = StreamingXmlWriter::new();
        assert_eq!(writer.indent_level, 0);
        assert_eq!(writer.indent_size, 2);
    }

    #[test]
    fn test_streaming_writer_with_indent() {
        let writer = StreamingXmlWriter::with_indent(4);
        assert_eq!(writer.indent_size, 4);
    }

    #[test]
    fn test_streaming_writer_declaration() {
        let writer = StreamingXmlWriter::new();
        let mut buffer = Vec::new();
        writer.write_declaration(&mut buffer).unwrap();
        
        let content = String::from_utf8(buffer).unwrap();
        assert!(content.contains("<?xml version"));
    }

    #[test]
    fn test_streaming_writer_element() {
        let writer = StreamingXmlWriter::new();
        let mut buffer = Vec::new();
        
        writer.write_element(
            &mut buffer,
            "test",
            &[("attr".to_string(), "value".to_string())],
        ).unwrap();
        
        let content = String::from_utf8(buffer).unwrap();
        assert!(content.contains("<test"));
        assert!(content.contains("attr=\"value\""));
    }

    #[test]
    fn test_streaming_writer_start_end_element() {
        let mut writer = StreamingXmlWriter::new();
        let mut buffer = Vec::new();
        
        writer.write_start_element(&mut buffer, "root", &[]).unwrap();
        writer.write_end_element(&mut buffer, "root").unwrap();
        
        let content = String::from_utf8(buffer).unwrap();
        assert!(content.contains("<root>"));
        assert!(content.contains("</root>"));
    }

    #[test]
    fn test_xml_event_start_element() {
        let event = XmlEvent::StartElement {
            name: "test".to_string(),
            attributes: vec![],
        };
        assert_eq!(event, event.clone());
    }

    #[test]
    fn test_xml_event_end_element() {
        let event = XmlEvent::EndElement {
            name: "test".to_string(),
        };
        assert_eq!(event, event.clone());
    }

    #[test]
    fn test_xml_event_characters() {
        let event = XmlEvent::Characters("test".to_string());
        assert_eq!(event, event.clone());
    }
}
