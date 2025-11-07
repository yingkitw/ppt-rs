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

        // Write [Content_Types].xml
        // TODO: Generate content types XML
        let content_types_xml = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\"></Types>";
        zip.start_file(CONTENT_TYPES_URI.trim_start_matches('/'), options)?;
        zip.write_all(content_types_xml)?;

        // Write package relationships
        // TODO: Generate relationships XML
        let pkg_rels_uri = PackURI::new("/_rels/.rels")?;
        let rels_xml = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"></Relationships>";
        zip.start_file(pkg_rels_uri.membername(), options)?;
        zip.write_all(rels_xml)?;

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
                // TODO: Generate relationships XML
                let rels_xml = b"<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\"></Relationships>";
                zip.start_file(rels_uri.membername(), options)?;
                zip.write_all(rels_xml)?;
            }
        }

        zip.finish()?;
        Ok(())
    }
}

