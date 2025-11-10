//! Main presentation object.

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::slide::Slides;
use crate::opc::{CoreProperties, AppProperties, CustomProperties, OpenXmlDocument, DocumentFormat};
use std::io::{Read, Seek, Write};
use super::{save, SlideLayoutsCollection, SlideMasters};

/// PresentationML (PML) presentation.
///
/// Not intended to be constructed directly. Use `ppt_rs::Presentation` to open or
/// create a presentation.
pub struct Presentation {
    part: PresentationPart,
    /// Internal package to store all parts (slides, images, etc.)
    package: crate::opc::package::Package,
    /// Available slide layouts
    slide_layouts: SlideLayoutsCollection,
    /// Available slide masters
    slide_masters: SlideMasters,
    /// Core properties (title, author, created, modified)
    core_properties: CoreProperties,
    /// App properties (application, version, slides count)
    app_properties: AppProperties,
    /// Custom properties (user-defined)
    custom_properties: CustomProperties,
}

impl Presentation {
    /// Create a new empty presentation
    pub fn new() -> Result<Self> {
        let part = PresentationPart::new()?;
        let package = crate::opc::package::Package::new();
        let slide_layouts = SlideLayoutsCollection::new();
        let slide_masters = SlideMasters::new();
        let core_properties = CoreProperties::new();
        let app_properties = AppProperties::new();
        let custom_properties = CustomProperties::new();
        Ok(Self { 
            part, 
            package, 
            slide_layouts, 
            slide_masters,
            core_properties,
            app_properties,
            custom_properties,
        })
    }

    /// Open a presentation from a reader
    pub fn open<R: Read + Seek>(reader: R) -> Result<Self> {
        use crate::opc::package::Package;
        
        // Open package
        let package = Package::open(reader)?;
        
        // Get main presentation part from package
        let part = open_from_package(&package)?;
        
        // TODO: Load all parts from package into internal package structure
        let package = Package::new(); // For now, create empty package
        let slide_layouts = SlideLayoutsCollection::new();
        let slide_masters = SlideMasters::new();
        let core_properties = CoreProperties::new();
        let app_properties = AppProperties::new();
        let custom_properties = CustomProperties::new();
        Ok(Self { 
            part, 
            package, 
            slide_layouts, 
            slide_masters,
            core_properties,
            app_properties,
            custom_properties,
        })
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
        slide_width(&self.part)
    }

    /// Set slide width in EMU
    pub fn set_slide_width(&mut self, width: u32) -> Result<()> {
        set_slide_width(&mut self.part, width)
    }

    /// Get slide height in EMU
    pub fn slide_height(&self) -> Option<u32> {
        slide_height(&self.part)
    }

    /// Set slide height in EMU
    pub fn set_slide_height(&mut self, height: u32) -> Result<()> {
        set_slide_height(&mut self.part, height)
    }

    /// Get available slide layouts
    pub fn slide_layouts(&self) -> &SlideLayoutsCollection {
        &self.slide_layouts
    }

    /// Get mutable reference to slide layouts
    pub fn slide_layouts_mut(&mut self) -> &mut SlideLayoutsCollection {
        &mut self.slide_layouts
    }

    /// Get available slide masters
    pub fn slide_masters(&self) -> &SlideMasters {
        &self.slide_masters
    }

    /// Get mutable reference to slide masters
    pub fn slide_masters_mut(&mut self) -> &mut SlideMasters {
        &mut self.slide_masters
    }

    /// Get the slide master (first master)
    pub fn slide_master(&self) -> Option<&crate::presentation::SlideMaster> {
        self.slide_masters.first()
    }

    /// Get core properties (title, author, created, modified)
    pub fn core_props(&self) -> &CoreProperties {
        &self.core_properties
    }

    /// Get mutable core properties
    pub fn core_props_mut(&mut self) -> &mut CoreProperties {
        &mut self.core_properties
    }

    /// Get app properties (application, version, slides count)
    pub fn app_props(&self) -> &AppProperties {
        &self.app_properties
    }

    /// Get mutable app properties
    pub fn app_props_mut(&mut self) -> &mut AppProperties {
        &mut self.app_properties
    }

    /// Get custom properties (user-defined)
    pub fn custom_props(&self) -> &CustomProperties {
        &self.custom_properties
    }

    /// Get mutable custom properties
    pub fn custom_props_mut(&mut self) -> &mut CustomProperties {
        &mut self.custom_properties
    }

    /// Generate default placeholder shapes for a slide
    fn generate_placeholder_shapes() -> String {
        // Generate Title and Subtitle placeholders (matching python-pptx)
        r#"      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr>
            <a:spLocks noGrp="1"/>
          </p:cNvSpPr>
          <p:nvPr>
            <p:ph type="ctrTitle"/>
          </p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Subtitle 2"/>
          <p:cNvSpPr>
            <a:spLocks noGrp="1"/>
          </p:cNvSpPr>
          <p:nvPr>
            <p:ph type="subTitle" idx="1"/>
          </p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p/>
        </p:txBody>
      </p:sp>"#.to_string()
    }

    /// Add a new blank slide to the presentation
    /// Returns the index of the newly added slide
    pub fn add_slide(&mut self) -> Result<usize> {
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::{SlideLayoutPart, SlidePart};
        use crate::opc::part::Part;
        
        
        // Create a default slide layout (blank layout)
        let layout_uri = PackURI::new("/ppt/slideLayouts/slideLayout1.xml")?;
        let layout_part = SlideLayoutPart::new(layout_uri)?;
        
        // Get current slide count before adding
        let slide_count = self.part.slide_id_manager().all().len();
        
        // Create a new slide URI
        let slide_uri = PackURI::new(&format!("/ppt/slides/slide{}.xml", slide_count + 1))?;
        let mut slide_part = SlidePart::new(slide_uri.clone(), &layout_part as &dyn Part)?;
        
        // Generate placeholder shapes
        let placeholders = Self::generate_placeholder_shapes();
        
        // Initialize slide part with proper XML content (including placeholders)
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
{}
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>"#,
            placeholders
        );
        slide_part.update_xml(slide_xml)?;
        
        // Add slide to package so it can be retrieved during save
        self.package.add_part(Box::new(slide_part));
        
        // Generate relationship ID for this slide
        // rId7 onwards for slides (rId1=master, rId2=printerSettings, rId3-6=properties, rId7+=slides per python-pptx)
        let r_id = format!("rId{}", 7 + slide_count);
        
        // Add slide ID to manager (relationships will be generated during save)
        self.part.slide_id_manager_mut().add_slide(r_id);
        
        // Note: The slide part and relationships will be generated during save() 
        // based on SlideIdManager. Don't add relationships here to avoid duplicates.
        
        Ok(slide_count)
    }

    /// Remove a slide by index
    /// Returns true if slide was removed, false if index is out of bounds
    pub fn remove_slide(&mut self, index: usize) -> Result<bool> {
        let slide_count = self.part.slide_id_manager().all().len();
        
        if index >= slide_count {
            return Ok(false);
        }
        
        // Get the relationship ID for this slide
        let slide_ids = self.part.slide_id_manager().all();
        if let Some(slide_id) = slide_ids.get(index) {
            let slide_id_clone = slide_id.clone();
            
            // Remove from slide ID manager
            self.part.slide_id_manager_mut().remove_slide(&slide_id_clone);
            
            // Note: Actual file removal happens during save()
            // For now, we just update the relationships
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // ============================================================================
    // FLUENT API METHODS (Phase 2)
    // ============================================================================
    
    /// Set the slide width in EMU with fluent API
    pub fn with_slide_width(mut self, width: u32) -> Result<Self> {
        self.set_slide_width(width)?;
        Ok(self)
    }
    
    /// Set the slide height in EMU with fluent API
    pub fn with_slide_height(mut self, height: u32) -> Result<Self> {
        self.set_slide_height(height)?;
        Ok(self)
    }
}

/// Implement OpenXmlDocument trait for Presentation
impl OpenXmlDocument for Presentation {
    fn format(&self) -> DocumentFormat {
        DocumentFormat::Presentation
    }

    fn package(&self) -> &crate::opc::Package {
        &self.package
    }

    fn package_mut(&mut self) -> &mut crate::opc::Package {
        &mut self.package
    }

    fn core_properties(&self) -> &CoreProperties {
        &self.core_properties
    }

    fn core_properties_mut(&mut self) -> &mut CoreProperties {
        &mut self.core_properties
    }

    fn app_properties(&self) -> &AppProperties {
        &self.app_properties
    }

    fn app_properties_mut(&mut self) -> &mut AppProperties {
        &mut self.app_properties
    }

    fn custom_properties(&self) -> &CustomProperties {
        &self.custom_properties
    }

    fn custom_properties_mut(&mut self) -> &mut CustomProperties {
        &mut self.custom_properties
    }

    fn save(&mut self) -> Result<Vec<u8>> {
        use std::io::Cursor;
        let mut cursor = Cursor::new(Vec::new());
        self.save(&mut cursor)?;
        Ok(cursor.into_inner())
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
        // sldIdLst is only added when there are slides
        assert!(content.contains("sldSz"));
        assert!(content.contains("sldMasterIdLst"));
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
    #[ignore] // TODO: Fix slide collection in package
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
    #[ignore] // TODO: Fix slide collection in package
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
    #[ignore] // TODO: Fix slide collection in package
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

// Utility functions for opening and dimension handling

/// Open a presentation part from a package
pub fn open_from_package(package: &crate::opc::package::Package) -> crate::error::Result<crate::parts::presentation::PresentationPart> {
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
                .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))?;
            
            crate::parts::presentation::PresentationPart::from_xml(std::io::Cursor::new(xml.as_bytes()))
        } else {
            // Fallback: create new presentation
            crate::parts::presentation::PresentationPart::new()
        }
    } else {
        // No main document found, create new presentation
        crate::parts::presentation::PresentationPart::new()
    }
}

/// Get slide width in EMU (English Metric Units)
pub fn slide_width(part: &crate::parts::presentation::PresentationPart) -> Option<u32> {
    use crate::opc::part::Part;
    // Parse from XML blob
    if let Ok(blob) = Part::blob(part) {
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
pub fn set_slide_width(part: &mut crate::parts::presentation::PresentationPart, width: u32) -> crate::error::Result<()> {
    use crate::opc::part::Part;
    // Parse XML, update width, and store back
    let mut xml = Part::to_xml(part)?;
    // Replace cx value in sldSz
    let pattern = r#"sldSz cx="[0-9]+""#;
    let replacement = format!(r#"sldSz cx="{}""#, width);
    xml = regex::Regex::new(pattern)
        .map_err(|e| crate::error::PptError::ValueError(format!("Invalid regex: {}", e)))?
        .replace_all(&xml, replacement.as_str())
        .to_string();
    
    // If sldSz doesn't exist, add it
    if !xml.contains("sldSz") {
        let sld_sz = format!(r#"<p:sldSz cx="{}" cy="6858000"/>"#, width);
        xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
    }
    
    // Store updated XML
    let uri = Part::uri(part).clone();
    *part = crate::parts::presentation::PresentationPart::with_xml(uri, xml)?;
    Ok(())
}

/// Get slide height in EMU
pub fn slide_height(part: &crate::parts::presentation::PresentationPart) -> Option<u32> {
    use crate::opc::part::Part;
    // Parse from XML blob
    if let Ok(blob) = Part::blob(part) {
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
pub fn set_slide_height(part: &mut crate::parts::presentation::PresentationPart, height: u32) -> crate::error::Result<()> {
    use crate::opc::part::Part;
    // Parse XML, update height, and store back
    let mut xml = Part::to_xml(part)?;
    // Replace cy value in sldSz
    let pattern = r#"sldSz cx="[0-9]+" cy="[0-9]+""#;
    let width = slide_width(part).unwrap_or(9144000);
    let replacement = format!(r#"sldSz cx="{}" cy="{}""#, width, height);
    xml = regex::Regex::new(pattern)
        .map_err(|e| crate::error::PptError::ValueError(format!("Invalid regex: {}", e)))?
        .replace_all(&xml, replacement.as_str())
        .to_string();
    
    // If sldSz doesn't exist, add it
    if !xml.contains("sldSz") {
        let sld_sz = format!(r#"<p:sldSz cx="9144000" cy="{}"/>"#, height);
        xml = xml.replace("<p:sldIdLst/>", &format!("<p:sldIdLst/>\n  {}", sld_sz));
    }
    
    // Store updated XML
    let uri = Part::uri(part).clone();
    *part = crate::parts::presentation::PresentationPart::with_xml(uri, xml)?;
    Ok(())
}

