//! XML parser for OpenXML

use crate::error::{PptError, Result};
use quick_xml::Reader;
use std::io::Read;

/// Parse XML from a reader and return as string
pub fn parse_xml<R: Read>(mut reader: R) -> Result<String> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(String::from_utf8(buf)
        .map_err(|e| PptError::Xml(format!("Invalid UTF-8: {}", e)))?)
}

/// Parse XML and return a Reader for event-based parsing
pub fn parse_xml_reader<R: Read>(reader: R) -> Reader<R> {
    Reader::from_reader(reader)
}

/// Parse XML from bytes
pub fn parse_xml_bytes(bytes: &[u8]) -> Result<String> {
    Ok(String::from_utf8(bytes.to_vec())
        .map_err(|e| PptError::Xml(format!("Invalid UTF-8: {}", e)))?)
}

