//! Slide-related types and functionality

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::parts::slide::SlidePart;
use crate::shapes::Shape;

/// Collection of slides in a presentation
pub struct Slides<'a> {
    presentation_part: &'a mut PresentationPart,
}

impl<'a> Slides<'a> {
    pub fn new(presentation_part: &'a mut PresentationPart) -> Self {
        Self {
            presentation_part,
        }
    }

    /// Add a new slide
    pub fn add_slide(&mut self, slide_layout: &SlidePart, package: &mut crate::opc::package::Package) -> Result<Slide> {
        use crate::opc::part::Part;
        use crate::opc::packuri::PackURI;
        // Use PresentationPart's add_slide method
        let r_id = self.presentation_part.add_slide(slide_layout as &dyn Part)?;
        
        // Get the slide partname from the relationship
        let rels = self.presentation_part.relationships();
        if let Some(rel) = rels.get(&r_id) {
            let slide_uri = PackURI::new(&format!("/{}", rel.target))?;
            let slide_part = crate::parts::slide::SlidePart::new(slide_uri.clone(), slide_layout as &dyn Part)?;
            
            // Add slide part to package
            use crate::opc::part::Part;
            let slide_blob = Part::blob(&slide_part)?;
            let slide_content_type = Part::content_type(&slide_part);
            let slide_rels = Part::relationships(&slide_part).clone();
            
            // Create OwnedPart wrapper for package
            struct OwnedSlidePart {
                content_type: String,
                uri: PackURI,
                blob: Vec<u8>,
                relationships: crate::opc::relationships::Relationships,
            }
            
            impl crate::opc::part::Part for OwnedSlidePart {
                fn content_type(&self) -> &str { &self.content_type }
                fn uri(&self) -> &crate::opc::packuri::PackURI { &self.uri }
                fn relationships(&self) -> &crate::opc::relationships::Relationships { &self.relationships }
                fn relationships_mut(&mut self) -> &mut crate::opc::relationships::Relationships { &mut self.relationships }
                fn blob(&self) -> crate::error::Result<Vec<u8>> { Ok(self.blob.clone()) }
                fn to_xml(&self) -> crate::error::Result<String> {
                    String::from_utf8(self.blob.clone())
                        .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))
                }
                fn from_xml<R: std::io::Read>(_reader: R) -> crate::error::Result<Self> {
                    Err(crate::error::PptError::NotImplemented("OwnedSlidePart::from_xml".to_string()))
                }
            }
            
            package.add_part(Box::new(OwnedSlidePart {
                content_type: slide_content_type.to_string(),
                uri: slide_uri.clone(),
                blob: slide_blob,
                relationships: slide_rels,
            }));
            
            Ok(Slide::with_part(slide_part))
        } else {
            Ok(Slide::new())
        }
    }

    /// Get a slide by index
    pub fn get(&mut self, index: usize, package: &mut crate::opc::package::Package) -> Option<Slide> {
        use crate::opc::part::Part;
        
        // Parse presentation.xml to get slide relationships
        if let Ok(blob) = Part::blob(self.presentation_part) {
            if let Ok(xml) = String::from_utf8(blob) {
                // Extract all r:id values from sldId entries
                let re = regex::Regex::new(r#"<p:sldId\s+[^>]*r:id="([^"]+)""#).ok()?;
                let r_ids: Vec<&str> = re.captures_iter(&xml)
                    .map(|cap| cap.get(1).map(|m| m.as_str()))
                    .collect::<Option<Vec<_>>>()?;
                
                if index < r_ids.len() {
                    let r_id = r_ids[index];
                    let rels = self.presentation_part.relationships();
                    if let Some(rel) = rels.get(r_id) {
                        use crate::opc::packuri::PackURI;
                        if let Ok(slide_uri) = PackURI::new(&format!("/{}", rel.target)) {
                            // Check if slide part is already in package
                            if package.get_part(&slide_uri).is_none() {
                                // Create a minimal SlidePart - in full implementation, would load from package
                                if let Ok(slide_part) = crate::parts::slide::SlidePart::new(slide_uri.clone(), self.presentation_part as &dyn Part) {
                                    // Add slide part to package
                                    let slide_blob = Part::blob(&slide_part).ok()?;
                                    let slide_content_type = Part::content_type(&slide_part);
                                    let slide_rels = Part::relationships(&slide_part).clone();
                                    
                                    struct OwnedSlidePart {
                                        content_type: String,
                                        uri: PackURI,
                                        blob: Vec<u8>,
                                        relationships: crate::opc::relationships::Relationships,
                                    }
                                    
                                    impl crate::opc::part::Part for OwnedSlidePart {
                                        fn content_type(&self) -> &str { &self.content_type }
                                        fn uri(&self) -> &crate::opc::packuri::PackURI { &self.uri }
                                        fn relationships(&self) -> &crate::opc::relationships::Relationships { &self.relationships }
                                        fn relationships_mut(&mut self) -> &mut crate::opc::relationships::Relationships { &mut self.relationships }
                                        fn blob(&self) -> crate::error::Result<Vec<u8>> { Ok(self.blob.clone()) }
                                        fn to_xml(&self) -> crate::error::Result<String> {
                                            String::from_utf8(self.blob.clone())
                                                .map_err(|e| crate::error::PptError::ValueError(format!("Invalid UTF-8: {}", e)))
                                        }
                                        fn from_xml<R: std::io::Read>(_reader: R) -> crate::error::Result<Self> {
                                            Err(crate::error::PptError::NotImplemented("OwnedSlidePart::from_xml".to_string()))
                                        }
                                    }
                                    
                                    package.add_part(Box::new(OwnedSlidePart {
                                        content_type: slide_content_type.to_string(),
                                        uri: slide_uri.clone(),
                                        blob: slide_blob,
                                        relationships: slide_rels,
                                    }));
                                    
                                    return Some(Slide::with_part(slide_part));
                                }
                            } else {
                                // Slide part already in package, create Slide from it
                                if let Some(part) = package.get_part(&slide_uri) {
                                    // Create SlidePart from the part in package
                                    let blob = Part::blob(part).ok()?;
                                    let xml = String::from_utf8(blob).ok()?;
                                    // Create BaseSlidePart from XML, then wrap in SlidePart
                                    if let Ok(_base_slide_part) = crate::parts::slide::BaseSlidePart::from_xml(std::io::Cursor::new(xml.as_bytes())) {
                                        // Create a new SlidePart with the same URI and content
                                        if let Ok(mut slide_part) = crate::parts::slide::SlidePart::new(slide_uri.clone(), self.presentation_part as &dyn Part) {
                                            // Update the slide part with the XML from package
                                            if slide_part.update_xml(xml).is_ok() {
                                                return Some(Slide::with_part(slide_part));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Get the number of slides
    pub fn len(&self) -> usize {
        use crate::opc::part::Part;
        // Parse presentation.xml to count slides in sldIdLst
        if let Ok(blob) = Part::blob(self.presentation_part) {
            if let Ok(xml) = String::from_utf8(blob) {
                // Count occurrences of <p:sldId> tags (not <p:sldIdLst>)
                // Match <p:sldId with attributes, not just the list container
                let pattern = r#"<p:sldId\s"#;
                return xml.matches(pattern).count();
            }
        }
        0
    }

    /// Check if slides collection is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over slides
    pub fn iter<'b>(&'b mut self, package: &'b mut crate::opc::package::Package) -> SlideIterator<'b, 'a> {
        SlideIterator {
            slides: self,
            package,
            index: 0,
        }
    }
}

/// Iterator over slides
pub struct SlideIterator<'a, 'b> {
    slides: &'a mut Slides<'b>,
    package: &'a mut crate::opc::package::Package,
    index: usize,
}

impl<'a, 'b> Iterator for SlideIterator<'a, 'b> {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slides.len() {
            let slide = self.slides.get(self.index, self.package);
            self.index += 1;
            slide
        } else {
            None
        }
    }
}

/// Individual slide
pub struct Slide {
    part: Option<SlidePart>,
    name: String,
}

impl Slide {
    pub fn new() -> Self {
        Self {
            part: None,
            name: String::new(),
        }
    }

    pub fn with_part(part: SlidePart) -> Self {
        Self {
            part: Some(part),
            name: String::new(),
        }
    }

    /// Get the slide name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the slide name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Get the slide part
    pub fn part(&self) -> Option<&SlidePart> {
        self.part.as_ref()
    }
    
    /// Get mutable slide part
    pub fn part_mut(&mut self) -> Option<&mut SlidePart> {
        self.part.as_mut()
    }

    /// Get shapes on this slide
    pub fn shapes(&self) -> Vec<Box<dyn Shape>> {
        use crate::opc::part::Part;
        use crate::shapes::xml::parse_shapes_from_xml;
        
        // Parse slide XML to extract shapes
        if let Some(ref part) = self.part {
            if let Ok(blob) = Part::blob(part) {
                if let Ok(xml) = String::from_utf8(blob) {
                    // Parse shapes from XML spTree
                    if let Ok(shapes) = parse_shapes_from_xml(&xml) {
                        return shapes;
                    }
                }
            }
        }
        Vec::new()
    }

    /// Add a shape to the slide
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) -> Result<()> {
        use crate::opc::part::Part;
        use crate::shapes::xml::{shape_to_xml, next_shape_id};
        
        if let Some(ref mut part) = self.part {
            // Get current XML
            let mut xml = Part::to_xml(part)?;
            
            // Find next shape ID
            let next_id = next_shape_id(&xml);
            
            // Generate shape XML
            let shape_xml = shape_to_xml(shape.as_ref(), next_id);
            
            // Insert shape XML into spTree (before closing </p:spTree>)
            if let Some(pos) = xml.find("</p:spTree>") {
                xml.insert_str(pos, &format!("      {}\n    ", shape_xml));
            } else {
                // If spTree doesn't exist, add it
                if let Some(pos) = xml.find("<p:cSld>") {
                    let sp_tree = format!(
                        r#"<p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr/>
      {}
    </p:spTree>"#,
                        shape_xml
                    );
                    xml.insert_str(pos + 8, &format!("\n    {}", sp_tree));
                }
            }
            
            // Update slide part with new XML
            part.update_xml(xml)?;
        }
        Ok(())
    }
    
    /// Add an image to the slide
    /// This creates an ImagePart, adds it to the package, creates a relationship, and adds a Picture shape
    pub fn add_image(
        &mut self,
        image_blob: Vec<u8>,
        image_ext: &str,
        package: &mut crate::opc::package::Package,
    ) -> Result<crate::shapes::Picture> {
        use crate::opc::part::Part;
        use crate::opc::constants::{CONTENT_TYPE, RELATIONSHIP_TYPE};
        use crate::opc::packuri::PackURI;
        use crate::parts::image::ImagePart;
        use crate::shapes::{Picture, Shape};
        
        // Determine content type from extension
        let content_type = match image_ext.to_lowercase().as_str() {
            "png" => CONTENT_TYPE::PNG,
            "jpg" | "jpeg" => CONTENT_TYPE::JPEG,
            "gif" => CONTENT_TYPE::GIF,
            "bmp" => CONTENT_TYPE::BMP,
            _ => CONTENT_TYPE::PNG, // Default to PNG
        };
        
        // Get next image partname
        let image_count = package.iter_parts()
            .filter(|p| p.uri().as_str().starts_with("/ppt/media/image"))
            .count();
        let image_uri = PackURI::new(&format!("/ppt/media/image{}.{}", image_count + 1, image_ext))?;
        
        // Create ImagePart
        let image_part = ImagePart::new(image_uri.clone(), content_type, image_blob)?;
        
        // Add image part to package
        struct OwnedImagePart {
            content_type: String,
            uri: PackURI,
            blob: Vec<u8>,
            relationships: crate::opc::relationships::Relationships,
        }
        
        impl crate::opc::part::Part for OwnedImagePart {
            fn content_type(&self) -> &str { &self.content_type }
            fn uri(&self) -> &crate::opc::packuri::PackURI { &self.uri }
            fn relationships(&self) -> &crate::opc::relationships::Relationships { &self.relationships }
            fn relationships_mut(&mut self) -> &mut crate::opc::relationships::Relationships { &mut self.relationships }
            fn blob(&self) -> crate::error::Result<Vec<u8>> { Ok(self.blob.clone()) }
            fn to_xml(&self) -> crate::error::Result<String> { Ok(String::new()) }
            fn from_xml<R: std::io::Read>(_reader: R) -> crate::error::Result<Self> {
                Err(crate::error::PptError::NotImplemented("OwnedImagePart::from_xml".to_string()))
            }
        }
        
        let img_blob = Part::blob(&image_part)?;
        let img_content_type = Part::content_type(&image_part);
        package.add_part(Box::new(OwnedImagePart {
            content_type: img_content_type.to_string(),
            uri: image_uri.clone(),
            blob: img_blob,
            relationships: crate::opc::relationships::Relationships::new(),
        }));
        
        // Add relationship from slide to image
        if let Some(ref mut part) = self.part {
            let r_id = part.relationships_mut().next_r_id();
            part.relationships_mut().add(
                r_id.clone(),
                RELATIONSHIP_TYPE::IMAGE.to_string(),
                image_uri.membername().to_string(),
                false,
            );
            
            // Create Picture shape
            use crate::shapes::xml::next_shape_id;
            let slide_xml = Part::to_xml(part)?;
            let shape_id = next_shape_id(&slide_xml);
            let mut picture = Picture::with_image(
                shape_id,
                format!("Picture {}", shape_id),
                r_id.clone(),
            );
            
            // Set default size based on image dimensions
            if let Ok((width, height)) = image_part.dimensions() {
                picture.set_width(width);
                picture.set_height(height);
            }
            
            // Add picture shape to slide
            self.add_shape(Box::new(picture))?;
            
            // Return the picture (need to get it back from the shape)
            // For now, create a new one with the same properties
            Ok(Picture::with_image(
                shape_id,
                format!("Picture {}", shape_id),
                r_id,
            ))
        } else {
            Err(crate::error::PptError::ValueError("Slide has no part".to_string()))
        }
    }
}

/// Slide masters collection
pub struct SlideMasters {
    presentation_part: *const PresentationPart, // Using raw pointer to avoid lifetime issues
}

impl SlideMasters {
    pub fn new(_presentation_part: &PresentationPart) -> Self {
        Self {
            presentation_part: std::ptr::null(),
        }
    }
    
    /// Get the number of slide masters
    pub fn len(&self) -> usize {
        // Parse presentation.xml to count slide masters
        // For now, return 0 as this requires XML parsing
        0
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Slide layouts collection
pub struct SlideLayouts {
    slide_master_part: *const crate::parts::slide::SlideMasterPart,
}

impl SlideLayouts {
    pub fn new(_slide_master_part: &crate::parts::slide::SlideMasterPart) -> Self {
        Self {
            slide_master_part: std::ptr::null(),
        }
    }

    /// Get a layout by name
    pub fn get_by_name(&self, _name: &str) -> Option<crate::parts::slide::SlideLayoutPart> {
        // Parse slide master XML to find layout by name
        // For now, return None as this requires XML parsing
        None
    }
    
    /// Get the number of layouts
    pub fn len(&self) -> usize {
        // Parse slide master XML to count layouts
        // For now, return 0
        0
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parts::presentation::PresentationPart;

    #[test]
    fn test_slide_new() {
        let slide = Slide::new();
        assert_eq!(slide.name(), "");
        assert_eq!(slide.shapes().len(), 0);
    }

    #[test]
    fn test_slide_name() {
        let mut slide = Slide::new();
        slide.set_name("Test Slide".to_string());
        assert_eq!(slide.name(), "Test Slide");
    }

    #[test]
    fn test_slide_masters_new() {
        let part = PresentationPart::new().unwrap();
        let masters = SlideMasters::new(&part);
        assert_eq!(masters.len(), 0);
        assert!(masters.is_empty());
    }

    #[test]
    fn test_slide_layouts_new() {
        use crate::parts::slide::SlideMasterPart;
        use crate::opc::packuri::PackURI;
        
        let master_part = SlideMasterPart::new(PackURI::new("/ppt/slideMasters/slideMaster1.xml").unwrap()).unwrap();
        let layouts = SlideLayouts::new(&master_part);
        assert_eq!(layouts.len(), 0);
        assert!(layouts.is_empty());
    }

    #[test]
    fn test_slides_len_empty() {
        let mut part = PresentationPart::new().unwrap();
        let slides = Slides::new(&mut part);
        assert_eq!(slides.len(), 0);
        assert!(slides.is_empty());
    }

    #[test]
    fn test_slides_add_slide_with_package() {
        let mut part = PresentationPart::new().unwrap();
        let mut package = crate::opc::package::Package::new();
        let mut slides = Slides::new(&mut part);
        
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        
        let result = slides.add_slide(&layout_part, &mut package);
        assert!(result.is_ok());
        assert_eq!(slides.len(), 1);
    }

    #[test]
    fn test_slides_get_with_package() {
        let mut part = PresentationPart::new().unwrap();
        let mut package = crate::opc::package::Package::new();
        let mut slides = Slides::new(&mut part);
        
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        
        slides.add_slide(&layout_part, &mut package).unwrap();
        
        let slide = slides.get(0, &mut package);
        assert!(slide.is_some());
        let slide = slide.unwrap();
        assert_eq!(slide.name(), "");
    }

    #[test]
    fn test_slide_add_image() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let mut package = crate::opc::package::Package::new();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        // Create a minimal PNG image (1x1 pixel PNG)
        let png_data = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, // IHDR chunk length
            0x49, 0x48, 0x44, 0x52, // IHDR
            0x00, 0x00, 0x00, 0x01, // width: 1
            0x00, 0x00, 0x00, 0x01, // height: 1
            0x08, 0x02, 0x00, 0x00, 0x00, // bit depth, color type, etc.
            0x90, 0x77, 0x53, 0xDE, // CRC
            0x00, 0x00, 0x00, 0x0A, // IDAT chunk length
            0x49, 0x44, 0x41, 0x54, // IDAT
            0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, // compressed data
            0x0D, 0x0A, 0x2D, 0xB4, // CRC
            0x00, 0x00, 0x00, 0x00, // IEND chunk length
            0x49, 0x45, 0x4E, 0x44, // IEND
            0xAE, 0x42, 0x60, 0x82, // CRC
        ];
        
        let result = slide.add_image(png_data, "png", &mut package);
        assert!(result.is_ok());
        
        // Verify image was added to package
        let image_uri = crate::opc::packuri::PackURI::new("/ppt/media/image1.png").unwrap();
        assert!(package.get_part(&image_uri).is_some());
        
        // Verify slide has a relationship to the image
        if let Some(ref part) = slide.part() {
            use crate::opc::part::Part;
            use crate::opc::constants::RELATIONSHIP_TYPE;
            let rels = Part::relationships(part);
            let has_image_rel = rels.iter().any(|(_, r)| r.rel_type == RELATIONSHIP_TYPE::IMAGE);
            assert!(has_image_rel);
        }
    }

    #[test]
    fn test_slide_add_image_different_formats() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let mut package = crate::opc::package::Package::new();
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        // Test JPEG
        let jpeg_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // Minimal JPEG header
        let result = slide.add_image(jpeg_data.clone(), "jpg", &mut package);
        assert!(result.is_ok());
        
        // Test GIF
        let gif_data = vec![0x47, 0x49, 0x46, 0x38]; // GIF header
        let result = slide.add_image(gif_data.clone(), "gif", &mut package);
        assert!(result.is_ok());
        
        // Verify both images are in package
        let jpeg_uri = crate::opc::packuri::PackURI::new("/ppt/media/image1.jpg").unwrap();
        let gif_uri = crate::opc::packuri::PackURI::new("/ppt/media/image2.gif").unwrap();
        assert!(package.get_part(&jpeg_uri).is_some());
        assert!(package.get_part(&gif_uri).is_some());
    }

    #[test]
    fn test_slide_add_image_no_part() {
        let mut package = crate::opc::package::Package::new();
        let mut slide = Slide::new();
        let image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG header
        
        let result = slide.add_image(image_data, "png", &mut package);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Slide has no part"));
    }

    #[test]
    fn test_slide_part_accessors() {
        use crate::parts::slide::SlideLayoutPart;
        use crate::opc::packuri::PackURI;
        use crate::parts::slide::SlidePart;
        use crate::opc::part::Part;
        
        let layout_part = SlideLayoutPart::new(PackURI::new("/ppt/slideLayouts/slideLayout1.xml").unwrap()).unwrap();
        let slide_part = SlidePart::new(PackURI::new("/ppt/slides/slide1.xml").unwrap(), &layout_part as &dyn Part).unwrap();
        let mut slide = Slide::with_part(slide_part);
        
        assert!(slide.part().is_some());
        assert!(slide.part_mut().is_some());
        
        // Test that we can modify through part_mut
        if let Some(ref mut part) = slide.part_mut() {
            use crate::opc::part::Part;
            let uri = Part::uri(part);
            assert_eq!(uri.as_str(), "/ppt/slides/slide1.xml");
        }
    }
}

