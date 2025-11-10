//! XML writer for OpenXML

use crate::error::Result;
use std::io::Write;

/// Serialize XML string to a writer
pub fn serialize_xml<W: Write>(mut writer: W, content: &str) -> Result<()> {
    writer.write_all(content.as_bytes())?;
    Ok(())
}

/// Serialize XML bytes to a writer
pub fn serialize_xml_bytes<W: Write>(mut writer: W, content: &[u8]) -> Result<()> {
    writer.write_all(content)?;
    Ok(())
}

