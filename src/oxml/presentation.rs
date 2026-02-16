//! Presentation XML parsing and reading
//!
//! Parses presentation.xml and provides high-level access to presentation content.

use super::slide::{ParsedLayout, ParsedMaster, ParsedSlide, SlideParser};
use super::xmlchemy::XmlParser;
use crate::exc::PptxError;
use crate::opc::Package;
use crate::oxml::ParsedImage;
use crate::parts::{ContentTypesPart, Part, RelationshipType, Relationships};

/// Parsed presentation metadata
#[derive(Debug, Clone)]
pub struct PresentationInfo {
    pub title: Option<String>,
    pub creator: Option<String>,
    pub last_modified_by: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub revision: Option<u32>,
    pub slide_count: usize,
}

impl PresentationInfo {
    pub fn new() -> Self {
        PresentationInfo {
            title: None,
            creator: None,
            last_modified_by: None,
            created: None,
            modified: None,
            revision: None,
            slide_count: 0,
        }
    }
}

impl Default for PresentationInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Presentation reader for parsing PPTX files
pub struct PresentationReader {
    package: Package,
    info: PresentationInfo,
    slide_paths: Vec<String>,
    content_types: ContentTypesPart,
}

impl PresentationReader {
    /// Open a PPTX file for reading
    pub fn open(path: &str) -> Result<Self, PptxError> {
        let package = Package::open(path)?;

        // Load content types
        let content_types = if let Some(ct_xml) = package.get_part("[Content_Types].xml") {
            let xml_str = String::from_utf8_lossy(ct_xml);
            ContentTypesPart::from_xml(&xml_str)?
        } else {
            ContentTypesPart::new()
        };

        let mut reader = PresentationReader {
            package,
            info: PresentationInfo::new(),
            slide_paths: Vec::new(),
            content_types,
        };
        reader.parse_structure()?;
        Ok(reader)
    }

    /// Get presentation info
    pub fn info(&self) -> &PresentationInfo {
        &self.info
    }

    /// Get number of slides
    pub fn slide_count(&self) -> usize {
        self.slide_paths.len()
    }

    /// Get slide by index (0-based)
    pub fn get_slide(&self, index: usize) -> Result<ParsedSlide, PptxError> {
        let path = self
            .slide_paths
            .get(index)
            .ok_or_else(|| PptxError::NotFound(format!("Slide {index} not found")))?;

        let xml = self
            .package
            .get_part(path)
            .ok_or_else(|| PptxError::NotFound(format!("Slide file not found: {path}")))?;

        let xml_str = String::from_utf8_lossy(xml);
        let mut slide = SlideParser::parse(&xml_str)?;

        // Resolve relationships
        if let Ok(rels) = self.get_relationships(path) {
            // Resolve slide images
            self.resolve_images(&mut slide.images, path, &rels);

            // Parse layout (each slide has exactly one layout)
            if let Some(rel) = rels.get_by_type(&RelationshipType::SlideLayout).first() {
                let layout_path = self.resolve_rel_target(path, &rel.target);
                if let Some(mut layout) = self.parse_layout_at(&layout_path) {
                    layout.rel_id = rel.id.clone();
                    layout.path = Some(layout_path.clone());

                    // Get layout's master (each layout has exactly one master)
                    if let Ok(layout_rels) = self.get_relationships(&layout_path) {
                        if let Some(master_rel) = layout_rels
                            .get_by_type(&RelationshipType::SlideMaster)
                            .first()
                        {
                            let master_path =
                                self.resolve_rel_target(&layout_path, &master_rel.target);
                            if let Some(mut master) = self.parse_master_at(&master_path) {
                                master.rel_id = master_rel.id.clone();
                                master.path = Some(master_path);
                                slide.master = Some(master);
                            }
                        }
                    }
                    slide.layout = Some(layout);
                }
            }
        }
        Ok(slide)
    }

    fn parse_layout_at(&self, layout_path: &str) -> Option<ParsedLayout> {
        let xml = self.package.get_part(layout_path)?;
        let xml_str = String::from_utf8_lossy(xml);
        let mut layout = SlideParser::parse_layout(&xml_str).ok()?;

        // Resolve layout images
        if let Ok(rels) = self.get_relationships(layout_path) {
            self.resolve_images(&mut layout.images, layout_path, &rels);
        }
        Some(layout)
    }

    fn parse_master_at(&self, master_path: &str) -> Option<ParsedMaster> {
        let xml = self.package.get_part(master_path)?;
        let xml_str = String::from_utf8_lossy(xml);
        let mut master = SlideParser::parse_master(&xml_str).ok()?;

        // Resolve master images
        if let Ok(rels) = self.get_relationships(master_path) {
            self.resolve_images(&mut master.images, master_path, &rels);
        }
        Some(master)
    }

    fn resolve_images(&self, images: &mut Vec<ParsedImage>, base_path: &str, rels: &Relationships) {
        for image in images {
            if let Some(rel) = rels.get(&image.rel_id) {
                if matches!(rel.rel_type, RelationshipType::Image) {
                    let image_path = self.resolve_rel_target(base_path, &rel.target);
                    if self.package.has_part(&image_path) {
                        // Get content type from [Content_Types].xml
                        let format = if let Some(content_type) =
                            self.content_types.get_content_type(&image_path)
                        {
                            // Extract format from MIME type (e.g., "image/png" -> "png")
                            content_type.rsplit('/').next().map(|s| s.to_lowercase())
                        } else {
                            // Fallback: extract format from file extension
                            image_path.rsplit('.').next().map(|s| s.to_lowercase())
                        };
                        image.format = format;
                        image.path = Some(image_path);
                    }
                }
            }
        }
    }

    fn get_relationships(&self, part_path: &str) -> Result<Relationships, PptxError> {
        // Convert part path to rels path
        // e.g., ppt/slides/slide1.xml -> ppt/slides/_rels/slide1.xml.rels
        // e.g., ppt/slideLayouts/slideLayout1.xml -> ppt/slideLayouts/_rels/slideLayout1.xml.rels
        let rels_path = if let Some((dir, file)) = part_path.rsplit_once('/') {
            format!("{}/_rels/{}.rels", dir, file)
        } else {
            format!("_rels/{}.rels", part_path)
        };

        if let Some(rels_xml) = self.package.get_part(&rels_path) {
            let xml_str = String::from_utf8_lossy(rels_xml);
            return Relationships::from_xml(&xml_str);
        }
        Err(PptxError::NotFound(format!(
            "Relationships file not found: {rels_path}"
        )))
    }

    fn resolve_rel_target(&self, base_path: &str, target: &str) -> String {
        if target.starts_with('/') {
            return target[1..].to_string();
        }

        // Get directory of base path
        let base_dir = base_path.rsplit_once('/').map(|(d, _)| d).unwrap_or("");

        // Handle relative paths like ../media/image1.png
        let mut parts: Vec<&str> = base_dir.split('/').collect();
        for segment in target.split('/') {
            match segment {
                ".." => {
                    parts.pop();
                }
                "." | "" => {}
                s => parts.push(s),
            }
        }
        parts.join("/")
    }

    /// Get image data by path
    pub fn get_image_data(&self, image: &ParsedImage) -> Option<&[u8]> {
        if let Some(path) = &image.path {
            self.package.get_part(path)
        } else {
            None
        }
    }

    /// Get all slides
    pub fn get_all_slides(&self) -> Result<Vec<ParsedSlide>, PptxError> {
        let mut slides = Vec::new();
        for i in 0..self.slide_paths.len() {
            slides.push(self.get_slide(i)?);
        }
        Ok(slides)
    }

    /// Get all text from presentation
    pub fn extract_all_text(&self) -> Result<Vec<String>, PptxError> {
        let mut all_text = Vec::new();
        for slide in self.get_all_slides()? {
            all_text.extend(slide.all_text());
        }
        Ok(all_text)
    }

    /// Parse presentation structure
    fn parse_structure(&mut self) -> Result<(), PptxError> {
        // Parse core properties
        self.parse_core_properties()?;

        // Parse presentation.xml to get slide list
        self.parse_presentation_xml()?;

        Ok(())
    }

    fn parse_core_properties(&mut self) -> Result<(), PptxError> {
        if let Some(core_xml) = self.package.get_part("docProps/core.xml") {
            let xml_str = String::from_utf8_lossy(core_xml);
            if let Ok(root) = XmlParser::parse_str(&xml_str) {
                self.info.title = root
                    .find_descendant("title")
                    .map(|e| e.text_content())
                    .filter(|s| !s.is_empty());

                self.info.creator = root
                    .find_descendant("creator")
                    .map(|e| e.text_content())
                    .filter(|s| !s.is_empty());

                self.info.last_modified_by = root
                    .find_descendant("lastModifiedBy")
                    .map(|e| e.text_content())
                    .filter(|s| !s.is_empty());

                self.info.created = root
                    .find_descendant("created")
                    .map(|e| e.text_content())
                    .filter(|s| !s.is_empty());

                self.info.modified = root
                    .find_descendant("modified")
                    .map(|e| e.text_content())
                    .filter(|s| !s.is_empty());

                self.info.revision = root
                    .find_descendant("revision")
                    .and_then(|e| e.text_content().parse().ok());
            }
        }
        Ok(())
    }

    fn parse_presentation_xml(&mut self) -> Result<(), PptxError> {
        // First, find slide references from presentation.xml.rels
        if let Some(rels_xml) = self.package.get_part("ppt/_rels/presentation.xml.rels") {
            let xml_str = String::from_utf8_lossy(rels_xml);
            if let Ok(root) = XmlParser::parse_str(&xml_str) {
                let mut slide_rels: Vec<(String, String)> = Vec::new();

                for rel in root.find_all("Relationship") {
                    let rel_type = rel.attr("Type").unwrap_or("");
                    if rel_type.contains("/slide")
                        && !rel_type.contains("Layout")
                        && !rel_type.contains("Master")
                    {
                        if let (Some(id), Some(target)) = (rel.attr("Id"), rel.attr("Target")) {
                            let full_path = if target.starts_with('/') {
                                target[1..].to_string()
                            } else {
                                format!("ppt/{target}")
                            };
                            slide_rels.push((id.to_string(), full_path));
                        }
                    }
                }

                // Sort by relationship ID to maintain slide order
                slide_rels.sort_by(|a, b| {
                    let num_a: u32 = a.0.trim_start_matches("rId").parse().unwrap_or(0);
                    let num_b: u32 = b.0.trim_start_matches("rId").parse().unwrap_or(0);
                    num_a.cmp(&num_b)
                });

                self.slide_paths = slide_rels.into_iter().map(|(_, path)| path).collect();
            }
        }

        // Fallback: scan for slide files
        if self.slide_paths.is_empty() {
            let paths = self.package.part_paths();
            let mut slides: Vec<String> = paths
                .into_iter()
                .filter(|p| {
                    p.starts_with("ppt/slides/slide") && p.ends_with(".xml") && !p.contains("_rels")
                })
                .map(|s| s.to_string())
                .collect();
            slides.sort();
            self.slide_paths = slides;
        }

        self.info.slide_count = self.slide_paths.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::SlideContent;
    use crate::generator::create_pptx_with_content;
    use std::fs;

    #[test]
    fn test_read_generated_pptx() {
        // Create a test PPTX
        let slides = vec![
            SlideContent::new("Test Title")
                .add_bullet("Bullet 1")
                .add_bullet("Bullet 2"),
            SlideContent::new("Second Slide").add_bullet("More content"),
        ];

        let pptx_data = create_pptx_with_content("Test Presentation", slides).unwrap();
        fs::write("test_read.pptx", &pptx_data).unwrap();

        // Read it back
        let reader = PresentationReader::open("test_read.pptx").unwrap();

        assert_eq!(reader.slide_count(), 2);
        assert!(reader.info().title.is_some());

        let slide1 = reader.get_slide(0).unwrap();
        assert!(slide1.title.is_some());

        // Cleanup
        fs::remove_file("test_read.pptx").ok();
    }

    #[test]
    fn test_extract_all_text() {
        let slides = vec![
            SlideContent::new("Title One")
                .add_bullet("Point A")
                .add_bullet("Point B"),
            SlideContent::new("Title Two").add_bullet("Point C"),
        ];

        let pptx_data = create_pptx_with_content("Text Extract Test", slides).unwrap();
        fs::write("test_extract.pptx", &pptx_data).unwrap();

        let reader = PresentationReader::open("test_extract.pptx").unwrap();
        let all_text = reader.extract_all_text().unwrap();

        assert!(all_text.iter().any(|t| t.contains("Title One")));
        assert!(all_text.iter().any(|t| t.contains("Point A")));

        fs::remove_file("test_extract.pptx").ok();
    }
}
