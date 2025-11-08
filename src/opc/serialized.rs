//! API for reading/writing serialized Open Packaging Convention (OPC) package

use crate::error::{PptError, Result};
use crate::opc::packuri::{PackURI, CONTENT_TYPES_URI};
use crate::opc::part::Part;
use crate::opc::relationships::Relationships;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};
use zip::ZipArchive;
use zip::ZipWriter;
use zip::write::FileOptions;

/// Package reader - provides access to package parts
pub struct PackageReader {
    blobs: HashMap<PackURI, Vec<u8>>,
}

impl PackageReader {
    /// Create a new PackageReader from a file path or reader
    pub fn new<R: Read + Seek>(reader: R) -> Result<Self> {
        let mut archive = ZipArchive::new(reader)?;
        let mut blobs = HashMap::new();
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();
            let uri = if name.starts_with('/') {
                PackURI::new(&name)?
            } else {
                PackURI::new(&format!("/{}", name))?
            };
            
            let mut data = Vec::new();
            std::io::copy(&mut file, &mut data)?;
            blobs.insert(uri, data);
        }
        
        Ok(Self { blobs })
    }

    /// Check if a part exists
    pub fn contains(&self, uri: &PackURI) -> bool {
        self.blobs.contains_key(uri)
    }

    /// Get the blob for a part
    pub fn get(&self, uri: &PackURI) -> Result<&[u8]> {
        self.blobs.get(uri)
            .map(|v| v.as_slice())
            .ok_or_else(|| PptError::PartNotFound(uri.to_string()))
    }

    /// Get relationships XML for a partname
    pub fn rels_xml_for(&self, partname: &PackURI) -> Result<Option<Vec<u8>>> {
        let rels_uri = partname.rels_uri()?;
        if self.contains(&rels_uri) {
            Ok(Some(self.get(&rels_uri)?.to_vec()))
        } else {
            Ok(None)
        }
    }
}

/// Package writer - writes a zip-format OPC package
pub struct PackageWriter;

impl PackageWriter {
    /// Write a package to a writer
    pub fn write<W: Write + Seek>(
        writer: W,
        pkg_rels: &Relationships,
        parts: &[Box<dyn Part>],
    ) -> Result<()> {
        let mut zip = ZipWriter::new(writer);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // Generate Content_Types.xml
        let mut content_types = Vec::new();
        content_types.push(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string());
        content_types.push(r#"<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">"#.to_string());
        
        // Add default extensions (must match python-pptx)
        content_types.push(r#"  <Default Extension="bin" ContentType="application/vnd.openxmlformats-officedocument.presentationml.printerSettings"/>"#.to_string());
        content_types.push(r#"  <Default Extension="jpeg" ContentType="image/jpeg"/>"#.to_string());
        content_types.push(r#"  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>"#.to_string());
        content_types.push(r#"  <Default Extension="xml" ContentType="application/xml"/>"#.to_string());
        
        // Track which extensions we've already added as defaults
        let mut added_extensions = std::collections::HashSet::new();
        
        // Add content types for each part
        for part in parts {
            let ext = part.uri().ext();
            let content_type = part.content_type();
            let membername = part.uri().membername();
            
            // Always use Override for specific parts (required by OPC spec)
            // Override takes precedence over Default
            content_types.push(format!(
                r#"  <Override PartName="{}" ContentType="{}"/>"#,
                membername,
                content_type
            ));
            
            // Add default extension if not already added and not xml/rels
            if !ext.is_empty() && ext != "xml" && ext != "rels" && !added_extensions.contains(ext) {
                // Only add default if we have a reasonable content type
                // For now, skip adding defaults for non-standard extensions
                added_extensions.insert(ext);
            }
        }
        
        content_types.push("</Types>".to_string());
        let content_types_xml = content_types.join("\n");
        
        // CONTENT_TYPES_URI is "/[Content_Types].xml", need to remove leading slash
        let content_types_filename = CONTENT_TYPES_URI.trim_start_matches('/');
        zip.start_file(content_types_filename, options)?;
        zip.write_all(content_types_xml.as_bytes())?;

        // Generate package relationships XML
        let mut rels_xml = Vec::new();
        rels_xml.push(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string());
        rels_xml.push(r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#.to_string());
        
        // Iterate over relationships using iter() method
        for (r_id, rel) in pkg_rels.iter() {
            rels_xml.push(format!(
                r#"  <Relationship Id="{}" Type="{}" Target="{}"/>"#,
                r_id,
                rel.rel_type,
                rel.target
            ));
        }
        
        rels_xml.push("</Relationships>".to_string());
        let pkg_rels_xml = rels_xml.join("\n");
        
        let pkg_rels_uri = PackURI::new("/_rels/.rels")?;
        zip.start_file(pkg_rels_uri.membername(), options)?;
        zip.write_all(pkg_rels_xml.as_bytes())?;

        // Write parts
        for part in parts {
            let uri = part.uri();
            zip.start_file(uri.membername(), options)?;
            let blob = part.blob()?;
            zip.write_all(&blob)?;
            
            // Write part relationships if they exist
            let rels = part.relationships();
            if !rels.is_empty() {
                let rels_uri = uri.rels_uri()?;
                let mut part_rels_xml = Vec::new();
                part_rels_xml.push(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string());
                part_rels_xml.push(r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">"#.to_string());
                
                // Iterate over relationships using iter() method
                for (r_id, rel) in rels.iter() {
                    part_rels_xml.push(format!(
                        r#"  <Relationship Id="{}" Type="{}" Target="{}"/>"#,
                        r_id,
                        rel.rel_type,
                        rel.target
                    ));
                }
                
                part_rels_xml.push("</Relationships>".to_string());
                let part_rels_xml_str = part_rels_xml.join("\n");
                
                zip.start_file(rels_uri.membername(), options)?;
                zip.write_all(part_rels_xml_str.as_bytes())?;
            }
        }

        // Finish writing the ZIP archive
        let mut finished_writer = zip.finish()?;
        // Ensure all data is flushed to the underlying writer
        finished_writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opc::part::Part;
    use crate::opc::packuri::PackURI;
    use crate::opc::constants::CONTENT_TYPE;
    use std::io::Cursor;

    #[test]
    fn test_package_writer_writes_content_types() {
        let mut pkg_rels = Relationships::new();
        pkg_rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "ppt/presentation.xml".to_string(),
            false,
        );
        
        // Create a simple test part
        struct TestPart {
            uri: PackURI,
            blob: Vec<u8>,
        }
        
        impl Part for TestPart {
            fn content_type(&self) -> &str {
                CONTENT_TYPE::PML_PRESENTATION_MAIN
            }
            fn uri(&self) -> &PackURI {
                &self.uri
            }
            fn relationships(&self) -> &Relationships {
                use std::sync::OnceLock;
                static EMPTY: OnceLock<Relationships> = OnceLock::new();
                EMPTY.get_or_init(Relationships::new)
            }
            fn relationships_mut(&mut self) -> &mut Relationships {
                panic!("Not implemented")
            }
            fn blob(&self) -> Result<Vec<u8>> {
                Ok(self.blob.clone())
            }
            fn to_xml(&self) -> Result<String> {
                Ok(String::from_utf8(self.blob.clone()).unwrap())
            }
            fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
                Err(PptError::NotImplemented("TestPart::from_xml".to_string()))
            }
        }
        
        let part = TestPart {
            uri: PackURI::new("/ppt/presentation.xml").unwrap(),
            blob: b"<test>content</test>".to_vec(),
        };
        
        let parts: Vec<Box<dyn Part>> = vec![Box::new(part)];
        let mut cursor = Cursor::new(Vec::new());
        
        let result = PackageWriter::write(&mut cursor, &pkg_rels, &parts);
        assert!(result.is_ok());
        
        // Verify ZIP structure
        let data = cursor.into_inner();
        let cursor = Cursor::new(&data);
        let archive = ZipArchive::new(cursor);
        assert!(archive.is_ok());
        
        let mut archive = archive.unwrap();
        
        // Check Content_Types.xml exists
        let content_types = archive.by_name("[Content_Types].xml");
        assert!(content_types.is_ok());
        
        let mut content_types_file = content_types.unwrap();
        let mut content = String::new();
        std::io::Read::read_to_string(&mut content_types_file, &mut content).unwrap();
        assert!(content.contains("Types"));
        assert!(content.contains("ppt/presentation.xml"));
    }

    #[test]
    fn test_package_writer_writes_relationships() {
        let mut pkg_rels = Relationships::new();
        pkg_rels.add(
            "rId1".to_string(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument".to_string(),
            "ppt/presentation.xml".to_string(),
            false,
        );
        
        let parts: Vec<Box<dyn Part>> = vec![];
        let mut cursor = Cursor::new(Vec::new());
        
        let result = PackageWriter::write(&mut cursor, &pkg_rels, &parts);
        assert!(result.is_ok());
        
        // Verify ZIP structure
        let data = cursor.into_inner();
        let cursor = Cursor::new(&data);
        let archive = ZipArchive::new(cursor);
        assert!(archive.is_ok());
        
        let mut archive = archive.unwrap();
        
        // Check _rels/.rels exists
        let rels = archive.by_name("_rels/.rels");
        assert!(rels.is_ok());
        
        let mut rels_file = rels.unwrap();
        let mut content = String::new();
        std::io::Read::read_to_string(&mut rels_file, &mut content).unwrap();
        assert!(content.contains("Relationships"));
        assert!(content.contains("rId1"));
        assert!(content.contains("ppt/presentation.xml"));
    }

    #[test]
    fn test_save_and_validate_pptx() {
        // Create a presentation and save it
        use crate::presentation::Presentation;
        
        let mut prs = Presentation::new().unwrap();
        
        // Save to memory
        let mut cursor = Cursor::new(Vec::new());
        let result = prs.save(&mut cursor);
        assert!(result.is_ok(), "Failed to save presentation");
        
        // Validate the saved file can be opened as ZIP
        cursor.set_position(0);
        let archive = ZipArchive::new(cursor);
        assert!(archive.is_ok(), "Saved file is not a valid ZIP");
        
        let mut archive = archive.unwrap();
        
        // Verify essential files exist
        assert!(archive.by_name("[Content_Types].xml").is_ok(), "Missing [Content_Types].xml");
        assert!(archive.by_name("_rels/.rels").is_ok(), "Missing _rels/.rels");
        assert!(archive.by_name("ppt/presentation.xml").is_ok(), "Missing ppt/presentation.xml");
        
        // Verify Content_Types.xml contains proper entries
        {
            let mut file = archive.by_name("[Content_Types].xml").unwrap();
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content).unwrap();
            assert!(content.contains("ppt/presentation.xml"), "Content_Types.xml missing presentation entry");
        }
        
        // Verify _rels/.rels contains proper relationships
        {
            let mut file = archive.by_name("_rels/.rels").unwrap();
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content).unwrap();
            assert!(content.contains("ppt/presentation.xml"), "_rels/.rels missing presentation relationship");
            assert!(content.contains("officeDocument"), "_rels/.rels missing officeDocument relationship type");
        }
        
        // Verify presentation.xml has valid structure
        {
            let mut file = archive.by_name("ppt/presentation.xml").unwrap();
            let mut content = String::new();
            std::io::Read::read_to_string(&mut file, &mut content).unwrap();
            assert!(content.contains("<?xml"), "presentation.xml missing XML declaration");
            assert!(content.contains("<p:presentation"), "presentation.xml missing presentation element");
            assert!(content.contains("xmlns:p="), "presentation.xml missing namespace");
        }
        
    }

    #[test]
    fn test_package_reader_new() {
        // Create a minimal ZIP file
        let mut cursor = Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut cursor);
            let options = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated);
            
            zip.start_file("test.txt", options).unwrap();
            zip.write_all(b"test content").unwrap();
            zip.finish().unwrap();
        }
        
        cursor.set_position(0);
        let reader = PackageReader::new(cursor);
        assert!(reader.is_ok());
        
        let reader = reader.unwrap();
        let uri = PackURI::new("/test.txt").unwrap();
        assert!(reader.contains(&uri));
        
        let blob = reader.get(&uri);
        assert!(blob.is_ok());
        assert_eq!(blob.unwrap(), b"test content");
    }
}

