//! Presentation editing capabilities
//!
//! Provides functionality to modify existing PPTX files:
//! - Add new slides
//! - Update slide content
//! - Remove slides
//! - Modify presentation properties

use super::slide::{ParsedSlide, SlideParser};
use crate::exc::PptxError;
use crate::generator::slide_content::SlideContent;
use crate::generator::slide_xml::{create_slide_xml_with_content, create_slide_rels_xml};
use crate::opc::Package;

/// Presentation editor for modifying PPTX files
pub struct PresentationEditor {
    package: Package,
    slide_count: usize,
}

impl PresentationEditor {
    /// Open a PPTX file for editing
    pub fn open(path: &str) -> Result<Self, PptxError> {
        let package = Package::open(path)?;
        let slide_count = Self::count_slides(&package);
        
        Ok(PresentationEditor {
            package,
            slide_count,
        })
    }

    /// Create a new presentation for editing
    pub fn new() -> Self {
        PresentationEditor {
            package: Package::new(),
            slide_count: 0,
        }
    }

    /// Get number of slides
    pub fn slide_count(&self) -> usize {
        self.slide_count
    }

    /// Get a parsed slide by index (0-based)
    pub fn get_slide(&self, index: usize) -> Result<ParsedSlide, PptxError> {
        let slide_num = index + 1;
        let path = format!("ppt/slides/slide{slide_num}.xml");
        let xml = self.package.get_part(&path)
            .ok_or_else(|| PptxError::NotFound(format!("Slide {index} not found")))?;
        
        let xml_str = String::from_utf8_lossy(xml);
        SlideParser::parse(&xml_str)
    }

    /// Add a new slide at the end
    pub fn add_slide(&mut self, content: SlideContent) -> Result<usize, PptxError> {
        let new_index = self.slide_count + 1;
        
        // Generate slide XML
        let slide_xml = create_slide_xml_with_content(new_index, &content);
        let slide_rels_xml = create_slide_rels_xml();
        
        // Add slide file
        let slide_path = format!("ppt/slides/slide{new_index}.xml");
        self.package.add_part(slide_path, slide_xml.into_bytes());
        
        // Add slide relationships
        let rels_path = format!("ppt/slides/_rels/slide{new_index}.xml.rels");
        self.package.add_part(rels_path, slide_rels_xml.into_bytes());
        
        // Update presentation.xml to include new slide
        self.update_presentation_xml(new_index)?;
        
        // Update presentation.xml.rels
        self.update_presentation_rels(new_index)?;
        
        // Update [Content_Types].xml
        self.update_content_types(new_index)?;
        
        self.slide_count = new_index;
        Ok(new_index - 1) // Return 0-based index
    }

    /// Update slide content at index
    pub fn update_slide(&mut self, index: usize, content: SlideContent) -> Result<(), PptxError> {
        if index >= self.slide_count {
            return Err(PptxError::NotFound(format!("Slide {index} not found")));
        }
        
        let slide_num = index + 1;
        let slide_xml = create_slide_xml_with_content(slide_num, &content);
        let slide_path = format!("ppt/slides/slide{slide_num}.xml");
        
        self.package.add_part(slide_path, slide_xml.into_bytes());
        Ok(())
    }

    /// Remove a slide by index
    pub fn remove_slide(&mut self, index: usize) -> Result<(), PptxError> {
        if index >= self.slide_count {
            return Err(PptxError::NotFound(format!("Slide {index} not found")));
        }
        
        let slide_num = index + 1;
        
        // Remove slide file
        let slide_path = format!("ppt/slides/slide{slide_num}.xml");
        self.package.remove_part(&slide_path);
        
        // Remove slide relationships
        let rels_path = format!("ppt/slides/_rels/slide{slide_num}.xml.rels");
        self.package.remove_part(&rels_path);
        
        // Renumber remaining slides
        for i in (slide_num + 1)..=self.slide_count {
            self.renumber_slide(i, i - 1)?;
        }
        
        self.slide_count -= 1;
        
        // Update presentation files
        self.rebuild_presentation_xml()?;
        self.rebuild_presentation_rels()?;
        self.rebuild_content_types()?;
        
        Ok(())
    }

    /// Save the modified presentation
    pub fn save(&self, path: &str) -> Result<(), PptxError> {
        self.package.save(path)?;
        Ok(())
    }

    /// Get the underlying package for advanced operations
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Get mutable reference to package
    pub fn package_mut(&mut self) -> &mut Package {
        &mut self.package
    }

    // Helper methods

    fn count_slides(package: &Package) -> usize {
        package.part_paths()
            .iter()
            .filter(|p| p.starts_with("ppt/slides/slide") && p.ends_with(".xml") && !p.contains("_rels"))
            .count()
    }

    fn update_presentation_xml(&mut self, new_slide_count: usize) -> Result<(), PptxError> {
        if let Some(xml) = self.package.get_part_string("ppt/presentation.xml") {
            // Parse and update the presentation XML
            let updated = self.add_slide_to_presentation_xml(&xml, new_slide_count)?;
            self.package.add_part("ppt/presentation.xml".to_string(), updated.into_bytes());
        }
        Ok(())
    }

    fn add_slide_to_presentation_xml(&self, xml: &str, slide_num: usize) -> Result<String, PptxError> {
        // Find </p:sldIdLst> and insert new slide reference before it
        let slide_id = 256 + slide_num;
        let r_id = slide_num + 2; // rId1=master, rId2=theme, rId3+=slides
        
        let new_slide_ref = format!(
            "\n<p:sldId id=\"{slide_id}\" r:id=\"rId{r_id}\"/>"
        );
        
        if let Some(pos) = xml.find("</p:sldIdLst>") {
            let mut result = xml.to_string();
            result.insert_str(pos, &new_slide_ref);
            Ok(result)
        } else {
            // If no sldIdLst, return as-is (shouldn't happen for valid PPTX)
            Ok(xml.to_string())
        }
    }

    fn update_presentation_rels(&mut self, new_slide_count: usize) -> Result<(), PptxError> {
        if let Some(xml) = self.package.get_part_string("ppt/_rels/presentation.xml.rels") {
            let updated = self.add_slide_to_presentation_rels(&xml, new_slide_count)?;
            self.package.add_part("ppt/_rels/presentation.xml.rels".to_string(), updated.into_bytes());
        }
        Ok(())
    }

    fn add_slide_to_presentation_rels(&self, xml: &str, slide_num: usize) -> Result<String, PptxError> {
        let r_id = slide_num + 2;
        let new_rel = format!(
            "\n    <Relationship Id=\"rId{r_id}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{slide_num}.xml\"/>"
        );
        
        if let Some(pos) = xml.find("</Relationships>") {
            let mut result = xml.to_string();
            result.insert_str(pos, &new_rel);
            Ok(result)
        } else {
            Ok(xml.to_string())
        }
    }

    fn update_content_types(&mut self, new_slide_count: usize) -> Result<(), PptxError> {
        if let Some(xml) = self.package.get_part_string("[Content_Types].xml") {
            let updated = self.add_slide_to_content_types(&xml, new_slide_count)?;
            self.package.add_part("[Content_Types].xml".to_string(), updated.into_bytes());
        }
        Ok(())
    }

    fn add_slide_to_content_types(&self, xml: &str, slide_num: usize) -> Result<String, PptxError> {
        let new_override = format!(
            "\n<Override PartName=\"/ppt/slides/slide{slide_num}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
        );
        
        if let Some(pos) = xml.find("</Types>") {
            let mut result = xml.to_string();
            result.insert_str(pos, &new_override);
            Ok(result)
        } else {
            Ok(xml.to_string())
        }
    }

    fn renumber_slide(&mut self, old_num: usize, new_num: usize) -> Result<(), PptxError> {
        // Move slide file
        let old_path = format!("ppt/slides/slide{old_num}.xml");
        let new_path = format!("ppt/slides/slide{new_num}.xml");
        
        if let Some(content) = self.package.remove_part(&old_path) {
            self.package.add_part(new_path, content);
        }
        
        // Move slide rels
        let old_rels = format!("ppt/slides/_rels/slide{old_num}.xml.rels");
        let new_rels = format!("ppt/slides/_rels/slide{new_num}.xml.rels");
        
        if let Some(content) = self.package.remove_part(&old_rels) {
            self.package.add_part(new_rels, content);
        }
        
        Ok(())
    }

    fn rebuild_presentation_xml(&mut self) -> Result<(), PptxError> {
        let mut slide_refs = String::new();
        for i in 1..=self.slide_count {
            let slide_id = 256 + i;
            let r_id = i + 2;
            slide_refs.push_str(&format!(
                "\n<p:sldId id=\"{slide_id}\" r:id=\"rId{r_id}\"/>"
            ));
        }
        
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" saveSubsetFonts="1">
<p:sldMasterIdLst>
<p:sldMasterId id="2147483648" r:id="rId1"/>
</p:sldMasterIdLst>
<p:sldIdLst>{slide_refs}
</p:sldIdLst>
<p:sldSz cx="9144000" cy="6858000" type="screen4x3"/>
<p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#
        );
        
        self.package.add_part("ppt/presentation.xml".to_string(), xml.into_bytes());
        Ok(())
    }

    fn rebuild_presentation_rels(&mut self) -> Result<(), PptxError> {
        let mut slide_rels = String::new();
        for i in 1..=self.slide_count {
            let r_id = i + 2;
            slide_rels.push_str(&format!(
                "\n    <Relationship Id=\"rId{r_id}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"slides/slide{i}.xml\"/>"
            ));
        }
        
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>{slide_rels}
</Relationships>"#
        );
        
        self.package.add_part("ppt/_rels/presentation.xml.rels".to_string(), xml.into_bytes());
        Ok(())
    }

    fn rebuild_content_types(&mut self) -> Result<(), PptxError> {
        let mut slide_overrides = String::new();
        for i in 1..=self.slide_count {
            slide_overrides.push_str(&format!(
                "\n<Override PartName=\"/ppt/slides/slide{i}.xml\" ContentType=\"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\"/>"
            ));
        }
        
        let xml = format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml" ContentType="application/xml"/>
<Default Extension="xlsx" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"/>

<Override PartName="/ppt/presentation.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"/>{slide_overrides}
<Override PartName="/ppt/slideLayouts/slideLayout1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"/>
<Override PartName="/ppt/slideMasters/slideMaster1.xml" ContentType="application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"/>
<Override PartName="/ppt/theme/theme1.xml" ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
<Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
<Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
</Types>"#
        );
        
        self.package.add_part("[Content_Types].xml".to_string(), xml.into_bytes());
        Ok(())
    }
}

impl Default for PresentationEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::create_pptx_with_content;
    use crate::oxml::PresentationReader;
    use std::fs;

    #[test]
    fn test_open_and_add_slide() {
        // Create initial presentation
        let slides = vec![
            SlideContent::new("Original Slide 1")
                .add_bullet("Original content"),
        ];
        let pptx_data = create_pptx_with_content("Test", slides).unwrap();
        fs::write("test_edit.pptx", &pptx_data).unwrap();
        
        // Open and add a slide
        let mut editor = PresentationEditor::open("test_edit.pptx").unwrap();
        assert_eq!(editor.slide_count(), 1);
        
        let new_slide = SlideContent::new("New Slide")
            .add_bullet("Added via editor");
        editor.add_slide(new_slide).unwrap();
        
        assert_eq!(editor.slide_count(), 2);
        
        // Save and verify
        editor.save("test_edit_modified.pptx").unwrap();
        
        let reader = PresentationReader::open("test_edit_modified.pptx").unwrap();
        assert_eq!(reader.slide_count(), 2);
        
        // Cleanup
        fs::remove_file("test_edit.pptx").ok();
        fs::remove_file("test_edit_modified.pptx").ok();
    }

    #[test]
    fn test_update_slide() {
        let slides = vec![
            SlideContent::new("Original Title")
                .add_bullet("Original bullet"),
        ];
        let pptx_data = create_pptx_with_content("Test", slides).unwrap();
        fs::write("test_update.pptx", &pptx_data).unwrap();
        
        let mut editor = PresentationEditor::open("test_update.pptx").unwrap();
        
        let updated = SlideContent::new("Updated Title")
            .add_bullet("Updated bullet");
        editor.update_slide(0, updated).unwrap();
        
        editor.save("test_update_modified.pptx").unwrap();
        
        let reader = PresentationReader::open("test_update_modified.pptx").unwrap();
        let slide = reader.get_slide(0).unwrap();
        assert_eq!(slide.title, Some("Updated Title".to_string()));
        
        fs::remove_file("test_update.pptx").ok();
        fs::remove_file("test_update_modified.pptx").ok();
    }
}
