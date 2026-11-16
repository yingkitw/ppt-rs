//! Presentation XML parsing and reading
//!
//! Parses presentation.xml and provides high-level access to presentation content.

use super::comments::{normalize_opc_path, parse_comment_authors_xml, parse_slide_comments_xml};
use super::slide::{ParsedSlide, SlideParser};
use super::xmlchemy::XmlParser;
use crate::exc::{PptxError, messages};
use crate::generator::{CommentAuthorList, SectionManager, SlideComments};
use crate::opc::Package;
use std::collections::HashMap;

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
}

impl PresentationReader {
    /// Open a PPTX file for reading
    pub fn open(path: &str) -> Result<Self, PptxError> {
        let package = Package::open(path)?;
        let mut reader = PresentationReader {
            package,
            info: PresentationInfo::new(),
            slide_paths: Vec::new(),
        };
        reader.parse_structure()?;
        Ok(reader)
    }

    /// Open a PPTX from an in-memory byte buffer (e.g. for network/embedded
    /// decks where no filesystem path is available).
    pub fn open_bytes(bytes: &[u8]) -> Result<Self, PptxError> {
        let package = Package::open_reader(std::io::Cursor::new(bytes))?;
        let mut reader = PresentationReader {
            package,
            info: PresentationInfo::new(),
            slide_paths: Vec::new(),
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
            .ok_or_else(|| PptxError::NotFound(messages::slide_not_found(index)))?;

        let xml = self
            .package
            .get_part(path)
            .ok_or_else(|| PptxError::NotFound(messages::slide_file_not_found(path)))?;

        let xml_str = String::from_utf8_lossy(xml);
        SlideParser::parse(&xml_str)
    }

    /// Get all slides
    pub fn get_all_slides(&self) -> Result<Vec<ParsedSlide>, PptxError> {
        let mut slides = Vec::new();
        for i in 0..self.slide_paths.len() {
            slides.push(self.get_slide(i)?);
        }
        Ok(slides)
    }

    /// Parse comment authors from `ppt/commentAuthors.xml`.
    pub fn comment_authors(&self) -> CommentAuthorList {
        let Some(xml_bytes) = self.package.get_part("ppt/commentAuthors.xml") else {
            return CommentAuthorList::new();
        };
        parse_comment_authors_xml(&String::from_utf8_lossy(xml_bytes))
    }

    /// Parse review comments linked from a slide's relationships.
    pub fn slide_comments(&self, index: usize) -> SlideComments {
        let Some(slide_path) = self.slide_paths.get(index) else {
            return SlideComments::new();
        };
        let file_name = slide_path.rsplit('/').next().unwrap_or("slide.xml");
        let rels_path = format!("ppt/slides/_rels/{file_name}.rels");
        let Some(rels_xml) = self.package.get_part(&rels_path) else {
            return SlideComments::new();
        };
        let Ok(root) = XmlParser::parse_str(&String::from_utf8_lossy(rels_xml)) else {
            return SlideComments::new();
        };
        for rel in root.find_all("Relationship") {
            let rel_type = rel.attr("Type").unwrap_or("");
            if !rel_type.contains("relationships/comments") {
                continue;
            }
            let Some(target) = rel.attr("Target") else {
                continue;
            };
            let comment_path = normalize_opc_path(slide_path, target);
            let Some(comment_xml) = self.package.get_part(&comment_path) else {
                continue;
            };
            return parse_slide_comments_xml(&String::from_utf8_lossy(comment_xml));
        }
        SlideComments::new()
    }

    /// Parse slide sections from `presentation.xml` (`<p14:sectionLst>`).
    pub fn sections(&self) -> SectionManager {
        let Some(pres_xml) = self.package.get_part("ppt/presentation.xml") else {
            return SectionManager::new();
        };
        let xml = String::from_utf8_lossy(pres_xml);
        let Ok(root) = XmlParser::parse_str(&xml) else {
            return SectionManager::new();
        };
        parse_sections(&root)
    }

    /// Get all text from presentation
    pub fn extract_all_text(&self) -> Result<Vec<String>, PptxError> {
        let mut all_text = Vec::new();
        for slide in self.get_all_slides()? {
            all_text.extend(slide.all_text());
        }
        Ok(all_text)
    }

    /// Get all text from a slide as a `Vec<String>` (paragraphs / shape texts).
    pub fn slide_text(&self, index: usize) -> Result<Vec<String>, PptxError> {
        let slide = self.get_slide(index)?;
        Ok(slide.all_text())
    }

    /// Get per-slide text from the whole presentation.
    /// `result[i]` is the text extracted from slide `i` (0-based). Each inner
    /// `Vec<String>` is the list of paragraphs / shape texts on that slide.
    pub fn extract_slide_texts(&self) -> Result<Vec<Vec<String>>, PptxError> {
        let mut out = Vec::with_capacity(self.slide_paths.len());
        for slide in self.get_all_slides()? {
            out.push(slide.all_text());
        }
        Ok(out)
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
                            let full_path = if let Some(stripped) = target.strip_prefix('/') {
                                stripped.to_string()
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

/// Extract per-slide text from a PPTX byte buffer.
///
/// `result[i]` is the list of paragraphs / shape texts on slide `i`. Use this
/// when you have the raw bytes (e.g. from a network response, a database, or
/// a memory-mapped file) and don't want to round-trip through
/// `PresentationReader` / `Presentation::from_path`. Pure content extraction —
/// no XML details leak to the caller.
///
/// ```rust,no_run
/// # use ppt_rs::oxml::extract_slide_texts_from_bytes;
/// let bytes: Vec<u8> = std::fs::read("deck.pptx").unwrap();
/// let slides = extract_slide_texts_from_bytes(&bytes).unwrap();
/// for (i, slide_text) in slides.iter().enumerate() {
///     println!("slide {i}: {} text fragments", slide_text.len());
/// }
/// ```
pub fn extract_slide_texts_from_bytes(bytes: &[u8]) -> Result<Vec<Vec<String>>, PptxError> {
    let reader = PresentationReader::open_bytes(bytes)?;
    reader.extract_slide_texts()
}

/// Build a [`SectionManager`] from parsed `presentation.xml`.
fn parse_sections(root: &super::xmlchemy::XmlElement) -> SectionManager {
    let mut mgr = SectionManager::new();

    let mut id_to_index = HashMap::new();
    if let Some(sld_id_lst) = root.find_descendant("sldIdLst") {
        for (idx, sld) in sld_id_lst.find_all("sldId").into_iter().enumerate() {
            if let Some(id) = sld.attr("id").and_then(|s| s.parse::<usize>().ok()) {
                id_to_index.insert(id, idx);
            }
        }
    }

    for section in root.find_all_descendants("section") {
        let Some(name) = section.attr("name").filter(|n| !n.is_empty()) else {
            continue;
        };
        let mut indices: Vec<usize> = section
            .find("sldIdLst")
            .map(|lst| {
                lst.find_all("sldId")
                    .into_iter()
                    .filter_map(|sld| {
                        sld.attr("id")?
                            .parse::<usize>()
                            .ok()
                            .and_then(|id| id_to_index.get(&id).copied())
                    })
                    .collect()
            })
            .unwrap_or_default();
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            continue;
        }
        let contiguous = indices.windows(2).all(|w| w[1] == w[0] + 1);
        if !contiguous {
            continue;
        }
        let _ = mgr.add_section(name, indices[0], indices.len());
    }

    mgr
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
    fn test_read_sections_from_generated_pptx() {
        use crate::generator::PresentationSettings;
        use crate::generator::SlideContent;
        use crate::generator::create_pptx_with_settings;

        let mut settings = PresentationSettings::new();
        settings.sections.add_section("Intro", 0, 1).unwrap();
        settings.sections.add_section("Body", 1, 1).unwrap();
        let slides = vec![SlideContent::new("Slide 1"), SlideContent::new("Slide 2")];
        let bytes = create_pptx_with_settings("Sections", &slides, Some(settings)).unwrap();
        fs::write("test_sections_read.pptx", &bytes).unwrap();

        let reader = PresentationReader::open("test_sections_read.pptx").unwrap();
        let sections = reader.sections();
        assert_eq!(sections.len(), 2);
        assert!(sections.get_section("Intro").is_some());
        assert!(sections.get_section("Body").is_some());

        fs::remove_file("test_sections_read.pptx").ok();
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

    #[test]
    fn test_extract_slide_texts_per_slide() {
        let slides = vec![
            SlideContent::new("Alpha Title")
                .add_bullet("Alpha bullet one")
                .add_bullet("Alpha bullet two"),
            SlideContent::new("Beta Title").add_bullet("Beta bullet"),
            SlideContent::new("Gamma Title").add_bullet("Gamma one"),
        ];

        let bytes = create_pptx_with_content("Per-slide", slides).unwrap();
        let texts = extract_slide_texts_from_bytes(&bytes).unwrap();

        assert_eq!(texts.len(), 3);
        assert!(texts[0].iter().any(|t| t.contains("Alpha Title")));
        assert!(texts[0].iter().any(|t| t.contains("Alpha bullet one")));
        assert!(texts[1].iter().any(|t| t.contains("Beta Title")));
        assert!(texts[2].iter().any(|t| t.contains("Gamma Title")));

        // No raw XML namespaces reach the caller.
        for slide_text in &texts {
            for t in slide_text {
                assert!(!t.contains("<p:"));
                assert!(!t.contains("xmlns"));
            }
        }
    }

    #[test]
    fn test_open_bytes_round_trip() {
        let slides = vec![SlideContent::new("Round-trip")
            .add_bullet("Survives open_bytes")
            .add_bullet("Survives extract_slide_texts")];
        let bytes = create_pptx_with_content("Round-trip", slides).unwrap();

        let reader = PresentationReader::open_bytes(&bytes).unwrap();
        assert_eq!(reader.slide_count(), 1);
        let title = reader.info().title.as_deref().unwrap_or("");
        assert!(title.contains("Round-trip"));
    }

    #[test]
    fn test_extract_slide_texts_rejects_invalid_bytes() {
        // Random bytes that are not a valid PPTX (ZIP) must surface an error
        // instead of panicking or returning garbage.
        let bad = b"this is not a zip file";
        let result = extract_slide_texts_from_bytes(bad);
        assert!(result.is_err());
    }
}
