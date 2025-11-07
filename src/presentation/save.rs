//! Save presentation functionality

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::opc::packuri::PackURI;
use std::io::{Seek, Write};

/// Save the presentation to a writer
pub fn save<W: Write + Seek>(
    part: &mut PresentationPart,
    package: &mut crate::opc::package::Package,
    writer: W,
) -> Result<()> {
    use crate::opc::constants::RELATIONSHIP_TYPE;
    use crate::opc::serialized::PackageWriter;
    use crate::opc::relationships::Relationships;
    
    // Create package relationships
    let mut pkg_rels = Relationships::new();
    pkg_rels.add(
        "rId1".to_string(),
        RELATIONSHIP_TYPE::OFFICE_DOCUMENT.to_string(),
        "ppt/presentation.xml".to_string(),
        false,
    );
    
    // Add core properties relationship if it exists
    if let Ok(core_props) = part.core_properties() {
        use crate::opc::part::Part;
        let core_props_uri = Part::uri(&core_props);
        pkg_rels.add(
            "rId2".to_string(),
            RELATIONSHIP_TYPE::CORE_PROPERTIES.to_string(),
            core_props_uri.membername().to_string(),
            false,
        );
    }
    
    // Get the blob and URI directly instead of using trait objects
    use crate::opc::part::Part;
    let blob = Part::blob(part)?;
    let uri = Part::uri(part).clone();
    let content_type = Part::content_type(part);
    let relationships = part.relationships().clone();
    
    // Create a simple part wrapper that owns its data
    struct OwnedPart {
        content_type: String,
        uri: crate::opc::packuri::PackURI,
        blob: Vec<u8>,
        relationships: Relationships,
    }
    
    impl crate::opc::part::Part for OwnedPart {
        fn content_type(&self) -> &str {
            &self.content_type
        }
        fn uri(&self) -> &crate::opc::packuri::PackURI {
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
            Err(crate::error::PptError::NotImplemented("OwnedPart::from_xml".to_string()))
        }
    }
    
    // Collect all parts: presentation part, core properties, slides, and their related parts
    let mut parts_map: std::collections::HashMap<PackURI, OwnedPart> = std::collections::HashMap::new();
    
    // Add presentation part
    parts_map.insert(uri.clone(), OwnedPart {
        content_type: content_type.to_string(),
        uri: uri.clone(),
        blob,
        relationships: relationships.clone(),
    });
    
    // Add core properties part if it exists
    if let Ok(core_props) = part.core_properties() {
        use crate::opc::part::Part;
        let core_blob = Part::blob(&core_props)?;
        let core_uri = Part::uri(&core_props).clone();
        let core_content_type = Part::content_type(&core_props);
        parts_map.insert(core_uri.clone(), OwnedPart {
            content_type: core_content_type.to_string(),
            uri: core_uri,
            blob: core_blob,
            relationships: Relationships::new(),
        });
    }
    
    // Collect all parts from internal package
    // This includes slides, images, and other related parts
    // First, ensure all slides are loaded into the package by accessing them
    {
        use crate::slide::Slides;
        let mut slides_collection = Slides::new(part);
        let slide_count = slides_collection.len();
        // Access slides to ensure they're added to package
        for i in 0..slide_count {
            let _ = slides_collection.get(i, package); // This will add slide parts to package if not already there
        }
        // slides_collection is dropped here, releasing the borrow
    }
    
    // Now collect all parts from package (including slides and images)
    // The slides_collection is dropped, so we can use package again
    for part in package.iter_parts() {
        use crate::opc::part::Part;
        let part_blob = Part::blob(part.as_ref())?;
        let part_uri = Part::uri(part.as_ref()).clone();
        let part_content_type = Part::content_type(part.as_ref());
        let part_rels = Part::relationships(part.as_ref()).clone();
        
        // Add part to collection if not already there
        if !parts_map.contains_key(&part_uri) {
            parts_map.insert(part_uri.clone(), OwnedPart {
                content_type: part_content_type.to_string(),
                uri: part_uri.clone(),
                blob: part_blob,
                relationships: part_rels.clone(),
            });
        }
        
        // Collect image parts from slide relationships
        for (_img_r_id, img_rel) in part_rels.iter() {
            use crate::opc::constants::RELATIONSHIP_TYPE;
            if img_rel.rel_type == RELATIONSHIP_TYPE::IMAGE && !img_rel.is_external {
                // Resolve image URI
                let img_uri = if img_rel.target.starts_with('/') {
                    PackURI::new(&img_rel.target)?
                } else {
                    // Relative path - resolve from part's base URI
                    let base_uri_str = part_uri.base_uri();
                    let resolved = if base_uri_str == "/" {
                        format!("/{}", img_rel.target)
                    } else {
                        format!("{}/{}", base_uri_str, img_rel.target)
                    };
                    PackURI::new(&resolved)?
                };
                
                // Try to get image part from package
                if let Some(img_part) = package.get_part(&img_uri) {
                    let img_blob = Part::blob(img_part)?;
                    let img_content_type = Part::content_type(img_part);
                    
                    if !parts_map.contains_key(&img_uri) {
                        parts_map.insert(img_uri.clone(), OwnedPart {
                            content_type: img_content_type.to_string(),
                            uri: img_uri,
                            blob: img_blob,
                            relationships: Relationships::new(),
                        });
                    }
                }
            }
        }
    }
    
    // Convert parts_map to Vec
    let parts: Vec<Box<dyn crate::opc::part::Part>> = parts_map
        .into_values()
        .map(|p| Box::new(p) as Box<dyn crate::opc::part::Part>)
        .collect();
    
    // Write the package
    PackageWriter::write(writer, &pkg_rels, &parts)
}
