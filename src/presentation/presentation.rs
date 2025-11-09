//! Main presentation object.

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::slide::Slides;
use std::io::{Read, Seek, Write};
use super::{dimensions, open, save};

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
        
        // Open package
        let package = Package::open(reader)?;
        
        // Get main presentation part from package
        let part = open::open_from_package(&package)?;
        
        // TODO: Load all parts from package into internal package structure
        let package = Package::new(); // For now, create empty package
        Ok(Self { part, package })
    }

    /// Save the presentation to a writer
    pub fn save<W: Write + Seek>(&mut self, writer: W) -> Result<()> {
        save::save(&mut self.part, &mut self.package, writer)
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
        dimensions::slide_width(&self.part)
    }

    /// Set slide width in EMU
    pub fn set_slide_width(&mut self, width: u32) -> Result<()> {
        dimensions::set_slide_width(&mut self.part, width)
    }

    /// Get slide height in EMU
    pub fn slide_height(&self) -> Option<u32> {
        dimensions::slide_height(&self.part)
    }

    /// Set slide height in EMU
    pub fn set_slide_height(&mut self, height: u32) -> Result<()> {
        dimensions::set_slide_height(&mut self.part, height)
    }

    /// Add a new blank slide to the presentation
    /// Returns the index of the newly added slide
    pub fn add_slide(&mut self) -> Result<usize> {
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::{SlideLayoutPart, SlidePart};
        use crate::opc::part::Part;
        use crate::opc::constants::RELATIONSHIP_TYPE;
        
        // Create a default slide layout (blank layout)
        let layout_uri = PackURI::new("/ppt/slideLayouts/slideLayout1.xml")?;
        let layout_part = SlideLayoutPart::new(layout_uri)?;
        
        // Get current slide count before adding
        let slide_count = self.part.slide_id_manager().all().len();
        
        // Create a new slide URI
        let slide_uri = PackURI::new(&format!("/ppt/slides/slide{}.xml", slide_count + 1))?;
        let mut slide_part = SlidePart::new(slide_uri.clone(), &layout_part as &dyn Part)?;
        
        // Initialize slide part with proper XML content
        let slide_xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#
        );
        slide_part.update_xml(slide_xml)?;
        
        // Generate relationship ID for this slide
        // rId6 onwards for slides (rId1-5 reserved for core parts)
        let r_id = format!("rId{}", 6 + slide_count);
        
        // Add slide ID to manager (relationships will be generated during save)
        self.part.slide_id_manager_mut().add_slide(r_id);
        
        // Note: The slide part and relationships will be generated during save() 
        // based on SlideIdManager. Don't add relationships here to avoid duplicates.
        
        Ok(slide_count)
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
        use crate::parts::slide::SlidePart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        // Create a slide part (not layout part) for testing
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), prs.part() as &dyn Part).unwrap();
        
        // Helper function to add slide without simultaneous borrows
        fn add_slide_helper(
            prs: &mut Presentation,
            slide_part: &SlidePart,
        ) -> Result<()> {
            // Split borrows using unsafe (safe because we know they don't overlap)
            unsafe {
                let prs_ptr = prs as *mut Presentation;
                let slides = &mut (*prs_ptr).part;
                let package = &mut (*prs_ptr).package;
                let mut slides_collection = crate::slide::Slides::new(slides);
                slides_collection.add_slide(slide_part, package)?;
            }
            Ok(())
        }
        
        add_slide_helper(&mut prs, &slide_part).unwrap();
        
        // Verify slide count
        {
            let slides = prs.slides();
            assert_eq!(slides.len(), 1);
        }
        
        // Save the presentation
        let test_path = "test_output/test_save_with_slides.pptx";
        std::fs::create_dir_all("test_output").ok();
        let result = prs.save_to_file(test_path);
        assert!(result.is_ok());
        
        // Verify file exists and is valid
        assert!(std::path::Path::new(test_path).exists());
        let file = std::fs::File::open(test_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        
        // Verify slide part is in the archive
        let slide_xml = archive.by_name("ppt/slides/slide1.xml");
        assert!(slide_xml.is_ok());
        
        // Clean up
        std::fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_presentation_save_with_images() {
        use crate::parts::slide::SlidePart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        // Create a slide part (not layout part) for testing
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), prs.part() as &dyn Part).unwrap();
        
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
        
        // Helper function to add slide and image without simultaneous borrows
        fn add_slide_and_image_helper(
            prs: &mut Presentation,
            slide_part: &SlidePart,
            png_data: Vec<u8>,
        ) -> Result<()> {
            // Split borrows using unsafe (safe because we know they don't overlap)
            unsafe {
                let prs_ptr = prs as *mut Presentation;
                let slides = &mut (*prs_ptr).part;
                let package = &mut (*prs_ptr).package;
                let mut slides_collection = crate::slide::Slides::new(slides);
                let mut slide = slides_collection.add_slide(slide_part, package)?;
                slide.add_image(png_data, "png", package)?;
                // Slide is dropped here, but its part is already in the package
            }
            Ok(())
        }
        
        add_slide_and_image_helper(&mut prs, &slide_part, png_data).unwrap();
        
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
        {
            let image_file = archive.by_name("ppt/media/image1.png");
            assert!(image_file.is_ok());
        }
        
        // Verify slide part has relationship to image
        {
            let slide_xml = archive.by_name("ppt/slides/slide1.xml");
            assert!(slide_xml.is_ok());
            let mut slide_content = String::new();
            std::io::Read::read_to_string(&mut slide_xml.unwrap(), &mut slide_content).unwrap();
            assert!(slide_content.contains("image1.png"));
        }
        
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
        use crate::parts::slide::SlidePart;
        use crate::opc::packuri::PackURI;
        use crate::opc::part::Part;
        
        let mut prs = Presentation::new().unwrap();
        // Create a slide part (not layout part) for testing
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), prs.part() as &dyn Part).unwrap();
        
        // Helper function to add slide and image without simultaneous borrows
        fn add_slide_and_image_helper(
            prs: &mut Presentation,
            slide_part: &SlidePart,
            png_data: Vec<u8>,
        ) -> Result<()> {
            // Split borrows using unsafe (safe because we know they don't overlap)
            unsafe {
                let prs_ptr = prs as *mut Presentation;
                let slides = &mut (*prs_ptr).part;
                let package = &mut (*prs_ptr).package;
                let mut slides_collection = crate::slide::Slides::new(slides);
                let mut slide = slides_collection.add_slide(slide_part, package)?;
                slide.add_image(png_data, "png", package)?;
                // Slide is dropped here, but its part is already in the package
            }
            Ok(())
        }
        
        let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        add_slide_and_image_helper(&mut prs, &slide_part, png_data).unwrap();
        
        // Save and verify all parts are included
        let mut cursor = std::io::Cursor::new(Vec::new());
        prs.save(&mut cursor).unwrap();
        
        let data = cursor.into_inner();
        let cursor = std::io::Cursor::new(&data);
        let mut archive = zip::ZipArchive::new(cursor).unwrap();
        
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

