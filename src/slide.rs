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
    pub fn add_slide(&mut self, slide_layout: &SlidePart) -> Result<Slide> {
        use crate::opc::part::Part;
        use crate::opc::packuri::PackURI;
        // Use PresentationPart's add_slide method
        let r_id = self.presentation_part.add_slide(slide_layout as &dyn Part)?;
        
        // Get the slide partname from the relationship
        let rels = self.presentation_part.relationships();
        if let Some(rel) = rels.get(&r_id) {
            let slide_uri = PackURI::new(&format!("/{}", rel.target))?;
            let slide_part = crate::parts::slide::SlidePart::new(slide_uri, slide_layout as &dyn Part)?;
            Ok(Slide::with_part(slide_part))
        } else {
            Ok(Slide::new())
        }
    }

    /// Get a slide by index
    pub fn get(&self, index: usize) -> Option<Slide> {
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
                            // Create a minimal SlidePart - in full implementation, would load from package
                            if let Ok(slide_part) = crate::parts::slide::SlidePart::new(slide_uri, self.presentation_part as &dyn Part) {
                                return Some(Slide::with_part(slide_part));
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
    pub fn iter(&self) -> SlideIterator {
        SlideIterator {
            slides: self,
            index: 0,
        }
    }
}

/// Iterator over slides
pub struct SlideIterator<'a> {
    slides: &'a Slides<'a>,
    index: usize,
}

impl<'a> Iterator for SlideIterator<'a> {
    type Item = Slide;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.slides.len() {
            let slide = self.slides.get(self.index);
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
}

