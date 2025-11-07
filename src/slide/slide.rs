//! Individual slide

use crate::error::Result;
use crate::parts::slide::SlidePart;
use crate::shapes::Shape;

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
        if self.part.is_none() {
            return Err(crate::error::PptError::ValueError("Slide has no part".to_string()));
        }
        
        let r_id = {
            let part = self.part.as_ref().unwrap();
            let rels = part.relationships();
            rels.next_r_id()
        };
        
        self.part.as_mut().unwrap().relationships_mut().add(
            r_id.clone(),
            RELATIONSHIP_TYPE::IMAGE.to_string(),
            image_uri.membername().to_string(),
            false,
        );
        
        // Create Picture shape
        use crate::shapes::xml::next_shape_id;
        let slide_xml = {
            let part = self.part.as_ref().unwrap();
            Part::to_xml(part)?
        };
        let shape_id = next_shape_id(&slide_xml);
        let mut picture = Picture::with_image(
            shape_id,
            format!("image{}.{}", image_count + 1, image_ext),
            r_id.clone(),
        );
        
        // Set default size based on image dimensions
        if let Ok((width, height)) = image_part.dimensions() {
            picture.set_width(width);
            picture.set_height(height);
        }
        
        // Get slide info before calling add_shape
        let slide_uri = {
            let part = self.part.as_ref().unwrap();
            Part::uri(part).clone()
        };
        let slide_content_type = {
            let part = self.part.as_ref().unwrap();
            Part::content_type(part).to_string()
        };
        
        // Add picture shape to slide (this updates self.part with new XML)
        self.add_shape(Box::new(picture))?;
        
        // Now update the slide part in the package with the new relationships and blob
        // Get the updated relationships and blob from the part
        let updated_rels = {
            let part = self.part.as_ref().unwrap();
            Part::relationships(part).clone()
        };
        let slide_blob = {
            let part = self.part.as_ref().unwrap();
            Part::blob(part)?
        };
        
        // Create OwnedSlidePart wrapper with updated relationships and blob
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
        
        // Replace the part in package with updated relationships and blob
        package.add_part(Box::new(OwnedSlidePart {
            content_type: slide_content_type,
            uri: slide_uri,
            blob: slide_blob,
            relationships: updated_rels,
        }));
        
        // Return the picture (need to get it back from the shape)
        // For now, create a new one with the same properties
        Ok(Picture::with_image(
            shape_id,
            format!("Picture {}", shape_id),
            r_id,
        ))
    }
}

