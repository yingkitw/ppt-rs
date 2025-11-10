//! Collection of slides in a presentation

use crate::error::Result;
use crate::parts::presentation::PresentationPart;
use crate::parts::slide::SlidePart;
use super::Slide;

/// Iterator over slides
pub struct SlideIterator<'a, 'b> {
    pub(super) slides: &'a mut Slides<'b>,
    pub(super) package: &'a mut crate::opc::package::Package,
    pub(super) index: usize,
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

/// Collection of slides in a presentation
pub struct Slides<'a> {
    pub(super) presentation_part: &'a mut PresentationPart,
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
            let mut slide_part = crate::parts::slide::SlidePart::new(slide_uri.clone(), slide_layout as &dyn Part)?;
            
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
            
            // Update package with slide part (replace if exists to ensure relationships are current)
            package.add_part(Box::new(OwnedSlidePart {
                content_type: slide_content_type.to_string(),
                uri: slide_uri.clone(),
                blob: slide_blob,
                relationships: slide_rels.clone(),
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
        // Use to_xml() to get the current XML state
        if let Ok(xml) = Part::to_xml(self.presentation_part) {
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
        None
    }

    /// Get the number of slides
    pub fn len(&self) -> usize {
        use crate::opc::part::Part;
        // Parse presentation.xml to count slides in sldIdLst
        // Read blob directly to ensure we get the updated XML
        // Try blob() first, then fallback to to_xml() if blob is empty
        let xml = if let Ok(blob) = Part::blob(self.presentation_part) {
            if blob.is_empty() {
                // Blob is empty, try to_xml() which might generate default XML
                Part::to_xml(self.presentation_part).unwrap_or_default()
            } else {
                // Use blob directly
                String::from_utf8(blob).unwrap_or_default()
            }
        } else {
            // Fallback: try to_xml()
            Part::to_xml(self.presentation_part).unwrap_or_default()
        };
        
        // Count occurrences of <p:sldId> tags (not <p:sldIdLst>)
        // Match <p:sldId with attributes, not just the list container
        // The pattern matches: <p:sldId id="256" r:id="rId1"/>
        let pattern = "<p:sldId ";
        xml.matches(pattern).count()
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

