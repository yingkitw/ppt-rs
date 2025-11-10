//! Package - main OPC package handler

use crate::error::{PptError, Result};
use crate::opc::constants::RELATIONSHIP_TYPE;
use crate::opc::part::Part;
use crate::opc::packuri::PackURI;
use crate::opc::relationships::Relationships;
use crate::opc::serialized::PackageReader;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::io::{Read, Seek, Write};

/// OPC Package - represents a .pptx file
pub struct Package {
    parts: LinkedHashMap<PackURI, Box<dyn Part>>,
    relationships: Relationships,
}

impl Package {
    /// Create a new empty package
    pub fn new() -> Self {
        Self {
            parts: LinkedHashMap::new(),
            relationships: Relationships::new(),
        }
    }

    /// Open a package from a reader
    pub fn open<R: Read + Seek>(mut reader: R) -> Result<Self> {
        use crate::opc::constants::RELATIONSHIP_TYPE;
        use crate::opc::packuri::CONTENT_TYPES_URI;
        use crate::parts::presentation::PresentationPart;
        
        
        let pkg_reader = PackageReader::new(&mut reader)?;
        
        // 1. Parse [Content_Types].xml to get content types
        let content_types_uri = PackURI::new(CONTENT_TYPES_URI)?;
        let content_types_xml = pkg_reader.get(&content_types_uri)?;
        let content_types_str = String::from_utf8(content_types_xml.to_vec())
            .map_err(|e| PptError::ValueError(format!("Invalid UTF-8 in Content_Types: {}", e)))?;
        
        // Parse content types - extract Override PartName and ContentType
        let mut content_type_map: HashMap<String, String> = HashMap::new();
        let override_re = regex::Regex::new(r#"<Override\s+PartName="([^"]+)"\s+ContentType="([^"]+)""#)
            .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?;
        for cap in override_re.captures_iter(&content_types_str) {
            let partname = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let content_type = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            content_type_map.insert(partname.to_string(), content_type.to_string());
        }
        
        // 2. Parse _rels/.rels to get package relationships
        let pkg_rels_uri = PackURI::new("/_rels/.rels")?;
        let mut pkg_rels = Relationships::new();
        
        if pkg_reader.contains(&pkg_rels_uri) {
            let rels_xml = pkg_reader.get(&pkg_rels_uri)?;
            let rels_str = String::from_utf8(rels_xml.to_vec())
                .map_err(|e| PptError::ValueError(format!("Invalid UTF-8 in relationships: {}", e)))?;
            
            // Parse relationships XML
            let rel_re = regex::Regex::new(r#"<Relationship\s+Id="([^"]+)"\s+Type="([^"]+)"\s+Target="([^"]+)""#)
                .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?;
            for cap in rel_re.captures_iter(&rels_str) {
                let r_id = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                let rel_type = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                let target = cap.get(3).map(|m| m.as_str()).unwrap_or("");
                let is_external = target.starts_with("http://") || target.starts_with("https://");
                pkg_rels.add(r_id.to_string(), rel_type.to_string(), target.to_string(), is_external);
            }
        }
        
        // 3. Load main presentation part
        let mut parts: LinkedHashMap<PackURI, Box<dyn Part>> = LinkedHashMap::new();
        
        // Find main document part from package relationships
        if let Some(rel) = pkg_rels.iter().find(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::OFFICE_DOCUMENT) {
            let target = &rel.1.target;
            let partname = if target.starts_with('/') {
                PackURI::new(target)?
            } else {
                PackURI::new(&format!("/{}", target))?
            };
            
            if pkg_reader.contains(&partname) {
                let blob = pkg_reader.get(&partname)?.to_vec();
                let xml = String::from_utf8(blob.clone())
                    .map_err(|e| PptError::ValueError(format!("Invalid UTF-8: {}", e)))?;
                
                // Create PresentationPart from XML
                let mut part = PresentationPart::from_xml(std::io::Cursor::new(xml.as_bytes()))?;
                
                // Parse part relationships
                if let Ok(Some(rels_blob)) = pkg_reader.rels_xml_for(&partname) {
                    let rels_str = String::from_utf8(rels_blob)
                        .map_err(|e| PptError::ValueError(format!("Invalid UTF-8: {}", e)))?;
                    let rel_re = regex::Regex::new(r#"<Relationship\s+Id="([^"]+)"\s+Type="([^"]+)"\s+Target="([^"]+)""#)
                        .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?;
                    for cap in rel_re.captures_iter(&rels_str) {
                        let r_id = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                        let rel_type = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                        let target = cap.get(3).map(|m| m.as_str()).unwrap_or("");
                        let is_external = target.starts_with("http://") || target.starts_with("https://");
                        part.relationships_mut().add(
                            r_id.to_string(),
                            rel_type.to_string(),
                            target.to_string(),
                            is_external,
                        );
                    }
                }
                
                parts.insert(partname, Box::new(part));
            }
        }
        
        Ok(Self {
            parts,
            relationships: pkg_rels,
        })
    }

    /// Save the package to a writer
    pub fn save<W: Write + Seek>(&self, writer: W) -> Result<()> {
        use crate::opc::serialized::PackageWriter;
        // Collect all parts into a vector by cloning their data
        let mut parts: Vec<Box<dyn Part>> = Vec::new();
        
        for part in self.parts.values() {
            // Create owned copies of parts by cloning their data
            struct OwnedPartWrapper {
                content_type: String,
                uri: PackURI,
                blob: Vec<u8>,
                relationships: Relationships,
            }
            
            impl Part for OwnedPartWrapper {
                fn content_type(&self) -> &str {
                    &self.content_type
                }
                fn uri(&self) -> &PackURI {
                    &self.uri
                }
                fn relationships(&self) -> &Relationships {
                    &self.relationships
                }
                fn relationships_mut(&mut self) -> &mut Relationships {
                    &mut self.relationships
                }
                fn blob(&self) -> Result<Vec<u8>> {
                    Ok(self.blob.clone())
                }
                fn to_xml(&self) -> Result<String> {
                    String::from_utf8(self.blob.clone())
                        .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))
                }
                fn from_xml<R: std::io::Read>(_reader: R) -> Result<Self> {
                    Err(crate::error::PptError::NotImplemented("OwnedPartWrapper::from_xml".to_string()))
                }
            }
            
            let content_type = part.content_type().to_string();
            let uri = part.uri().clone();
            let blob = part.blob().unwrap_or_default();
            // Clone relationships manually
            let mut relationships = Relationships::new();
            for (r_id, rel) in part.relationships().iter() {
                relationships.add(
                    r_id.clone(),
                    rel.rel_type.clone(),
                    rel.target.clone(),
                    rel.is_external,
                );
            }
            
            parts.push(Box::new(OwnedPartWrapper {
                content_type,
                uri,
                blob,
                relationships,
            }) as Box<dyn Part>);
        }
        
        PackageWriter::write(writer, &self.relationships, &parts)
    }

    /// Get the main document part
    pub fn main_document_part(&self) -> Result<&dyn Part> {
        self.relationships
            .part_with_reltype(RELATIONSHIP_TYPE::OFFICE_DOCUMENT, &self.parts)
    }

    /// Add a part to the package
    pub fn add_part(&mut self, part: Box<dyn Part>) {
        let uri = part.uri().clone();
        self.parts.insert(uri, part);
    }

    /// Get a part by URI
    pub fn get_part(&self, uri: &PackURI) -> Option<&dyn Part> {
        self.parts.get(uri).map(|p| p.as_ref())
    }

    /// Get relationships
    pub fn relationships(&self) -> &Relationships {
        &self.relationships
    }

    /// Get mutable relationships
    pub fn relationships_mut(&mut self) -> &mut Relationships {
        &mut self.relationships
    }

    /// Iterate over all parts
    pub fn iter_parts(&self) -> impl Iterator<Item = &Box<dyn Part>> {
        self.parts.values()
    }
}


