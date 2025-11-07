//! Open presentation from reader

use crate::error::{PptError, Result};
use crate::parts::presentation::PresentationPart;
use crate::opc::package::Package;

/// Open a presentation part from a package
pub fn open_from_package(package: &Package) -> Result<PresentationPart> {
    use crate::opc::constants::RELATIONSHIP_TYPE;
    use crate::opc::part::Part;
    
    // Get main presentation part from package relationships
    let pkg_rels = package.relationships();
    if let Some(rel) = pkg_rels.iter().find(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::OFFICE_DOCUMENT) {
        let target = &rel.1.target;
        let partname = if target.starts_with('/') {
            crate::opc::packuri::PackURI::new(target)?
        } else {
            crate::opc::packuri::PackURI::new(&format!("/{}", target))?
        };
        
        if let Some(part) = package.get_part(&partname) {
            // Get blob and create PresentationPart
            let blob = Part::blob(part)?;
            let xml = String::from_utf8(blob)
                .map_err(|e| PptError::ValueError(format!("Invalid UTF-8: {}", e)))?;
            
            PresentationPart::from_xml(std::io::Cursor::new(xml.as_bytes()))
        } else {
            // Fallback: create new presentation
            PresentationPart::new()
        }
    } else {
        // No main document found, create new presentation
        PresentationPart::new()
    }
}

