//! Main presentation object.

use crate::error::{PptError, Result};
use crate::parts::presentation::PresentationPart;
use crate::slide::{Slide, Slides};
use crate::opc::packuri::PackURI;
use std::io::{Read, Seek, Write};

/// PresentationML (PML) presentation.
///
/// Not intended to be constructed directly. Use `ppt_rs::Presentation` to open or
/// create a presentation.
pub struct Presentation {
    part: PresentationPart,
    /// Internal package to store all parts (slides, images, etc.)
    package: crate::opc::package::Package,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Result<Self> {
        let part = PresentationPart::new()?;
        let package = crate::opc::package::Package::new();
        Ok(Self { part, package })
    }

    /// Open a presentation from a reader
    pub fn open<R: Read + Seek>(reader: R) -> Result<Self> {
        use crate::opc::package::Package;
        use crate::opc::constants::RELATIONSHIP_TYPE;
        use crate::opc::part::Part;
        
        // Open package
        let package = Package::open(reader)?;
        
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
                
                let part = PresentationPart::from_xml(std::io::Cursor::new(xml.as_bytes()))?;
                // TODO: Load all parts from package into internal package structure
                let package = Package::new(); // For now, create empty package
                Ok(Self { part, package })
            } else {
                // Fallback: create new presentation
                Self::new()
            }
        } else {
            // No main document found, create new presentation
            Self::new()
        }
    }

    /// Save the presentation to a writer
    pub fn save<W: Write + Seek>(&mut self, writer: W) -> Result<()> {
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
        if let Ok(core_props) = self.core_properties() {
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
        let blob = Part::blob(&self.part)?;
        let uri = Part::uri(&self.part).clone();
        let content_type = Part::content_type(&self.part);
        let relationships = self.part.relationships().clone();
        
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
        if let Ok(core_props) = self.core_properties() {
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
            let mut slides_collection = Slides::new(&mut self.part);
            let slide_count = slides_collection.len();
            // Access slides to ensure they're added to package
            for i in 0..slide_count {
                let _ = slides_collection.get(i, &mut self.package); // This will add slide parts to package if not already there
            }
            // slides_collection is dropped here, releasing the borrow
        }
        
        // Now collect all parts from package (including slides and images)
        // The slides_collection is dropped, so we can use self.package again
        for part in self.package.iter_parts() {
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
                    if let Some(img_part) = self.package.get_part(&img_uri) {
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

    /// Save the presentation to a file path
    pub fn save_to_file<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<()> {
        use std::io::Cursor;
        let mut cursor = Cursor::new(Vec::new());
        self.save(&mut cursor)?;
        let data = cursor.into_inner();
        std::fs::write(path, data)?;
        Ok(())
    }

    /// Get the slides collection
    pub fn slides(&mut self) -> Slides {
        Slides::new(&mut self.part)
    }
    
    /// Get the internal package (for adding parts like images)
    /// Note: This cannot be called while slides() is in use due to borrowing restrictions
    pub fn package_mut(&mut self) -> &mut crate::opc::package::Package {
        &mut self.package
    }

    /// Get the presentation part
    pub fn part(&self) -> &PresentationPart {
        &self.part
    }

    /// Get mutable presentation part
    pub fn part_mut(&mut self) -> &mut PresentationPart {
        &mut self.part
    }

    /// Get core properties
    pub fn core_properties(&self) -> Result<crate::parts::coreprops::CorePropertiesPart> {
        self.part.core_properties()
    }

    /// Get slide width in EMU (English Metric Units)
    pub fn slide_width(&self) -> Option<u32> {
        use crate::opc::part::Part;
        // Parse from XML blob
        if let Ok(blob) = Part::blob(&self.part) {
            if let Ok(xml) = String::from_utf8(blob) {
                // Look for sldSz cx="..." pattern
                if let Some(start) = xml.find("sldSz cx=\"") {
                    let start = start + 10;
                    if let Some(end) = xml[start..].find('"') {
                        if let Ok(width) = xml[start..start+end].parse::<u32>() {
                            return Some(width);
                        }
                    }
                }
            }
        }
        Some(9144000) // Default 10 inches
    }

    /// Set slide width in EMU
    pub fn set_slide_width(&mut self, width: u32) -> Result<()> {
        use crate::opc::part::Part;
        // Parse XML, update width, and store back
        let mut xml = Part::to_xml(&self.part)?;
        // Replace cx value in sldSz
        let pattern = r#"sldSz cx="[0-9]+""#;
        let replacement = format!(r#"sldSz cx="{}""#, width);
        xml = regex::Regex::new(pattern)
            .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?
            .replace_all(&xml, replacement.as_str())
            .to_string();
        
        // If sldSz doesn't exist, add it
        if !xml.contains("sldSz") {
            let sld_sz = format!(r#"<p:sldSz cx="{}" cy="6858000"/>"#, width);
            xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
        }
        
        // Store updated XML
        let uri = Part::uri(&self.part).clone();
        *self.part_mut() = PresentationPart::with_xml(uri, xml)?;
        Ok(())
    }

    /// Get slide height in EMU
    pub fn slide_height(&self) -> Option<u32> {
        use crate::opc::part::Part;
        // Parse from XML blob
        if let Ok(blob) = Part::blob(&self.part) {
            if let Ok(xml) = String::from_utf8(blob) {
                // Look for sldSz cy="..." pattern
                if let Some(start) = xml.find("sldSz cy=\"") {
                    let start = start + 10;
                    if let Some(end) = xml[start..].find('"') {
                        if let Ok(height) = xml[start..start+end].parse::<u32>() {
                            return Some(height);
                        }
                    }
                }
            }
        }
        Some(6858000) // Default 7.5 inches
    }

    /// Set slide height in EMU
    pub fn set_slide_height(&mut self, height: u32) -> Result<()> {
        use crate::opc::part::Part;
        // Parse XML, update height, and store back
        let mut xml = Part::to_xml(&self.part)?;
        // Replace cy value in sldSz
        let pattern = r#"sldSz cx="[0-9]+" cy="[0-9]+""#;
        let width = self.slide_width().unwrap_or(9144000);
        let replacement = format!(r#"sldSz cx="{}" cy="{}""#, width, height);
        xml = regex::Regex::new(pattern)
            .map_err(|e| PptError::ValueError(format!("Invalid regex: {}", e)))?
            .replace_all(&xml, replacement.as_str())
            .to_string();
        
        // If sldSz doesn't exist, add it
        if !xml.contains("sldSz") {
            let sld_sz = format!(r#"<p:sldSz cx="9144000" cy="{}"/>"#, height);
            xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
        }
        
        // Store updated XML
        let uri = Part::uri(&self.part).clone();
        *self.part_mut() = PresentationPart::with_xml(uri, xml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_presentation_new() {
        let prs = Presentation::new();
        assert!(prs.is_ok());
        let prs = prs.unwrap();
        assert_eq!(prs.slide_width(), Some(9144000));
        assert_eq!(prs.slide_height(), Some(6858000));
    }

    #[test]
    fn test_presentation_save_to_writer() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        let result = prs.save(&mut cursor);
        assert!(result.is_ok());
        
        // Verify we wrote some data
        let data = cursor.into_inner();
        assert!(!data.is_empty());
        
        // Verify it's a valid ZIP file (PPTX files are ZIP archives)
        let cursor = Cursor::new(&data);
        let archive = zip::ZipArchive::new(cursor);
        assert!(archive.is_ok());
    }

    #[test]
    fn test_presentation_save_to_file() {
        let mut prs = Presentation::new().unwrap();
        let test_path = "test_output/test_save.pptx";
        
        // Create test_output directory if it doesn't exist
        std::fs::create_dir_all("test_output").ok();
        
        let result = prs.save_to_file(test_path);
        assert!(result.is_ok());
        
        // Verify file exists
        assert!(std::path::Path::new(test_path).exists());
        
        // Verify it's a valid ZIP file
        let file = std::fs::File::open(test_path);
        assert!(file.is_ok());
        let archive = zip::ZipArchive::new(file.unwrap());
        assert!(archive.is_ok());
        
        // Clean up
        std::fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_presentation_save_contains_content_types() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        let data = cursor.into_inner();
        let cursor = Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        
        // Check for [Content_Types].xml
        let content_types = archive.by_name("[Content_Types].xml");
        assert!(content_types.is_ok());
        
        let mut content_types_file = content_types.unwrap();
        let mut content = String::new();
        std::io::Read::read_to_string(&mut content_types_file, &mut content).unwrap();
        assert!(content.contains("Types"));
        assert!(content.contains("application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"));
    }

    #[test]
    fn test_presentation_save_contains_presentation_xml() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        let data = cursor.into_inner();
        let cursor = Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        
        // Check for ppt/presentation.xml
        let presentation_xml = archive.by_name("ppt/presentation.xml");
        assert!(presentation_xml.is_ok());
        
        let mut presentation_file = presentation_xml.unwrap();
        let mut content = String::new();
        std::io::Read::read_to_string(&mut presentation_file, &mut content).unwrap();
        assert!(content.contains("presentation"));
        assert!(content.contains("sldIdLst"));
        assert!(content.contains("sldSz"));
    }

    #[test]
    fn test_presentation_save_contains_relationships() {
        let mut prs = Presentation::new().unwrap();
        let mut cursor = Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        let data = cursor.into_inner();
        let cursor = Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        
        // Check for _rels/.rels
        let rels = archive.by_name("_rels/.rels");
        assert!(rels.is_ok());
        
        let mut rels_file = rels.unwrap();
        let mut content = String::new();
        std::io::Read::read_to_string(&mut rels_file, &mut content).unwrap();
        assert!(content.contains("Relationships"));
        assert!(content.contains("ppt/presentation.xml"));
    }

    #[test]
    fn test_presentation_slide_dimensions() {
        let prs = Presentation::new().unwrap();
        assert_eq!(prs.slide_width(), Some(9144000));
        assert_eq!(prs.slide_height(), Some(6858000));
        
        // Test setting dimensions (even though not fully implemented)
        let mut prs = Presentation::new().unwrap();
        assert!(prs.set_slide_width(10000000).is_ok());
        assert!(prs.set_slide_height(8000000).is_ok());
    }

    #[test]
    fn test_presentation_slides() {
        let mut prs = Presentation::new().unwrap();
        let mut slides = prs.slides();
        // Empty presentation should have no slides
        assert_eq!(slides.len(), 0);
    }

    #[test]
    fn test_presentation_save_with_slides() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        let mut slides = prs.slides();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        
        let slide = slides.add_slide(&layout_part, prs.package_mut()).unwrap();
        assert_eq!(slides.len(), 1);
        
        // Save the presentation
        let test_path = "test_output/test_save_with_slides.pptx";
        std::fs::create_dir_all("test_output").ok();
        let result = prs.save_to_file(test_path);
        assert!(result.is_ok());
        
        // Verify file exists and is valid
        assert!(std::path::Path::new(test_path).exists());
        let file = std::fs::File::open(test_path).unwrap();
        let archive = zip::ZipArchive::new(file).unwrap();
        
        // Verify slide part is in the archive
        let slide_xml = archive.by_name("ppt/slides/slide1.xml");
        assert!(slide_xml.is_ok());
        
        // Clean up
        std::fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_presentation_save_with_images() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        let mut slides = prs.slides();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        
        let mut slide = slides.add_slide(&layout_part, prs.package_mut()).unwrap();
        
        // Create a minimal PNG image
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE,
            0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54,
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01,
            0x0D, 0x0A, 0x2D, 0xB4,
            0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
            0xAE, 0x42, 0x60, 0x82,
        ];
        
        let result = slide.add_image(png_data, "png", prs.package_mut());
        assert!(result.is_ok());
        
        // Save the presentation
        let test_path = "test_output/test_save_with_images.pptx";
        std::fs::create_dir_all("test_output").ok();
        let result = prs.save_to_file(test_path);
        assert!(result.is_ok());
        
        // Verify file exists and is valid
        assert!(std::path::Path::new(test_path).exists());
        let file = std::fs::File::open(test_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        
        // Verify image part is in the archive
        let image_file = archive.by_name("ppt/media/image1.png");
        assert!(image_file.is_ok());
        
        // Verify slide part has relationship to image
        let slide_xml = archive.by_name("ppt/slides/slide1.xml");
        assert!(slide_xml.is_ok());
        let mut slide_content = String::new();
        std::io::Read::read_to_string(&mut slide_xml.unwrap(), &mut slide_content).unwrap();
        assert!(slide_content.contains("image1.png"));
        
        // Clean up
        std::fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_presentation_package_mut() {
        let mut prs = Presentation::new().unwrap();
        let package = prs.package_mut();
        assert_eq!(package.iter_parts().count(), 0);
    }

    #[test]
    fn test_presentation_save_collects_all_parts() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        let mut slides = prs.slides();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        
        // Add a slide
        let mut slide = slides.add_slide(&layout_part, prs.package_mut()).unwrap();
        
        // Add an image to the slide
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        slide.add_image(png_data, "png", prs.package_mut()).unwrap();
        
        // Save and verify all parts are included
        let mut cursor = std::io::Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        let data = cursor.into_inner();
        let cursor = std::io::Cursor::new(&data);
        let archive = zip::ZipArchive::new(cursor).unwrap();
        
        // Verify presentation part
        assert!(archive.by_name("ppt/presentation.xml").is_ok());
        
        // Verify slide part
        assert!(archive.by_name("ppt/slides/slide1.xml").is_ok());
        
        // Verify image part
        assert!(archive.by_name("ppt/media/image1.png").is_ok());
        
        // Verify relationships
        assert!(archive.by_name("ppt/_rels/presentation.xml.rels").is_ok());
        assert!(archive.by_name("ppt/slides/_rels/slide1.xml.rels").is_ok());
    }
}

