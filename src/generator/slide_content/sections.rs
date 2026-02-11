//! Slide sections and organization
//!
//! Provides section management for grouping slides into logical sections.
//! Generates proper OOXML `<p:extLst>` section data in presentation.xml.

/// A section that groups consecutive slides
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlideSection {
    pub name: String,
    pub first_slide: usize,
    pub slide_count: usize,
}

impl SlideSection {
    /// Create a new section starting at a given slide index (0-based)
    pub fn new(name: &str, first_slide: usize, slide_count: usize) -> Self {
        Self {
            name: name.to_string(),
            first_slide,
            slide_count,
        }
    }

    /// Last slide index (inclusive, 0-based)
    pub fn last_slide(&self) -> usize {
        if self.slide_count == 0 {
            self.first_slide
        } else {
            self.first_slide + self.slide_count - 1
        }
    }

    /// Check if a slide index belongs to this section
    pub fn contains_slide(&self, slide_index: usize) -> bool {
        slide_index >= self.first_slide && slide_index < self.first_slide + self.slide_count
    }
}

/// Manages sections across the presentation
#[derive(Clone, Debug, Default)]
pub struct SectionManager {
    sections: Vec<SlideSection>,
}

impl SectionManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a section. Returns error if it overlaps with existing sections.
    pub fn add_section(&mut self, name: &str, first_slide: usize, slide_count: usize) -> Result<(), String> {
        let new_section = SlideSection::new(name, first_slide, slide_count);

        // Check for overlaps
        for existing in &self.sections {
            if sections_overlap(existing, &new_section) {
                return Err(format!(
                    "Section '{}' (slides {}-{}) overlaps with '{}' (slides {}-{})",
                    name, first_slide, new_section.last_slide(),
                    existing.name, existing.first_slide, existing.last_slide(),
                ));
            }
        }

        self.sections.push(new_section);
        // Keep sorted by first_slide
        self.sections.sort_by_key(|s| s.first_slide);
        Ok(())
    }

    /// Remove a section by name
    pub fn remove_section(&mut self, name: &str) -> bool {
        let before = self.sections.len();
        self.sections.retain(|s| s.name != name);
        self.sections.len() < before
    }

    /// Get section by name
    pub fn get_section(&self, name: &str) -> Option<&SlideSection> {
        self.sections.iter().find(|s| s.name == name)
    }

    /// Find which section a slide belongs to
    pub fn section_for_slide(&self, slide_index: usize) -> Option<&SlideSection> {
        self.sections.iter().find(|s| s.contains_slide(slide_index))
    }

    /// Get all sections
    pub fn sections(&self) -> &[SlideSection] {
        &self.sections
    }

    /// Number of sections
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Whether there are no sections
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Clear all sections
    pub fn clear(&mut self) {
        self.sections.clear();
    }

    /// Rename a section. Returns false if not found.
    pub fn rename_section(&mut self, old_name: &str, new_name: &str) -> bool {
        if let Some(section) = self.sections.iter_mut().find(|s| s.name == old_name) {
            section.name = new_name.to_string();
            true
        } else {
            false
        }
    }

    /// Generate OOXML extension XML for sections (used in presentation.xml `<p:extLst>`)
    pub fn to_xml(&self, total_slides: usize) -> String {
        if self.sections.is_empty() {
            return String::new();
        }

        let mut xml = String::from(
            r#"<p:extLst><p:ext uri="{521415D9-36F7-43E2-AB2F-B90AF26B5E84}"><p14:sectionLst xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main">"#,
        );

        for section in &self.sections {
            xml.push_str(&format!(
                r#"<p14:section name="{}" id="{{{}}}">"#,
                xml_escape(&section.name),
                generate_section_id(&section.name),
            ));
            xml.push_str("<p14:sldIdLst>");
            for i in 0..section.slide_count {
                let slide_id = 256 + section.first_slide + i;
                if section.first_slide + i < total_slides {
                    xml.push_str(&format!(r#"<p14:sldId id="{}"/>"#, slide_id));
                }
            }
            xml.push_str("</p14:sldIdLst>");
            xml.push_str("</p14:section>");
        }

        xml.push_str("</p14:sectionLst></p:ext></p:extLst>");
        xml
    }
}

/// Check if two sections overlap
fn sections_overlap(a: &SlideSection, b: &SlideSection) -> bool {
    if a.slide_count == 0 || b.slide_count == 0 {
        return false;
    }
    a.first_slide < b.first_slide + b.slide_count && b.first_slide < a.first_slide + a.slide_count
}

/// Generate a deterministic GUID-like ID from a section name
fn generate_section_id(name: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in name.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    let a = (hash >> 32) as u32;
    let b = (hash & 0xFFFF) as u16;
    let c = ((hash >> 16) & 0xFFFF) as u16;
    let d = hash.wrapping_mul(0x9e3779b97f4a7c15);
    format!(
        "{:08X}-{:04X}-{:04X}-{:04X}-{:012X}",
        a,
        b,
        c,
        (d & 0xFFFF) as u16,
        d >> 16,
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_section_new() {
        let section = SlideSection::new("Introduction", 0, 3);
        assert_eq!(section.name, "Introduction");
        assert_eq!(section.first_slide, 0);
        assert_eq!(section.slide_count, 3);
    }

    #[test]
    fn test_slide_section_last_slide() {
        let section = SlideSection::new("Intro", 0, 3);
        assert_eq!(section.last_slide(), 2);

        let empty = SlideSection::new("Empty", 5, 0);
        assert_eq!(empty.last_slide(), 5);
    }

    #[test]
    fn test_slide_section_contains() {
        let section = SlideSection::new("Body", 3, 5);
        assert!(!section.contains_slide(2));
        assert!(section.contains_slide(3));
        assert!(section.contains_slide(7));
        assert!(!section.contains_slide(8));
    }

    #[test]
    fn test_section_manager_new() {
        let mgr = SectionManager::new();
        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
    }

    #[test]
    fn test_section_manager_add() {
        let mut mgr = SectionManager::new();
        assert!(mgr.add_section("Intro", 0, 3).is_ok());
        assert!(mgr.add_section("Body", 3, 5).is_ok());
        assert_eq!(mgr.len(), 2);
    }

    #[test]
    fn test_section_manager_overlap_detection() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 3).unwrap();
        let result = mgr.add_section("Overlap", 2, 3);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("overlaps"));
    }

    #[test]
    fn test_section_manager_no_overlap_adjacent() {
        let mut mgr = SectionManager::new();
        mgr.add_section("A", 0, 3).unwrap();
        assert!(mgr.add_section("B", 3, 2).is_ok());
    }

    #[test]
    fn test_section_manager_remove() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 3).unwrap();
        assert!(mgr.remove_section("Intro"));
        assert!(mgr.is_empty());
        assert!(!mgr.remove_section("NonExistent"));
    }

    #[test]
    fn test_section_manager_get_section() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 3).unwrap();
        let section = mgr.get_section("Intro");
        assert!(section.is_some());
        assert_eq!(section.unwrap().first_slide, 0);
        assert!(mgr.get_section("Missing").is_none());
    }

    #[test]
    fn test_section_manager_section_for_slide() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 3).unwrap();
        mgr.add_section("Body", 3, 5).unwrap();
        assert_eq!(mgr.section_for_slide(0).unwrap().name, "Intro");
        assert_eq!(mgr.section_for_slide(4).unwrap().name, "Body");
        assert!(mgr.section_for_slide(10).is_none());
    }

    #[test]
    fn test_section_manager_sorted() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Body", 3, 5).unwrap();
        mgr.add_section("Intro", 0, 3).unwrap();
        assert_eq!(mgr.sections()[0].name, "Intro");
        assert_eq!(mgr.sections()[1].name, "Body");
    }

    #[test]
    fn test_section_manager_clear() {
        let mut mgr = SectionManager::new();
        mgr.add_section("A", 0, 2).unwrap();
        mgr.clear();
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_section_manager_rename() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Old", 0, 3).unwrap();
        assert!(mgr.rename_section("Old", "New"));
        assert!(mgr.get_section("New").is_some());
        assert!(mgr.get_section("Old").is_none());
        assert!(!mgr.rename_section("Missing", "X"));
    }

    #[test]
    fn test_section_manager_xml_empty() {
        let mgr = SectionManager::new();
        assert_eq!(mgr.to_xml(10), "");
    }

    #[test]
    fn test_section_manager_xml() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 2).unwrap();
        mgr.add_section("Body", 2, 3).unwrap();
        let xml = mgr.to_xml(5);
        assert!(xml.contains("<p:extLst>"));
        assert!(xml.contains("p14:sectionLst"));
        assert!(xml.contains("Intro"));
        assert!(xml.contains("Body"));
        assert!(xml.contains("p14:sldId"));
        assert!(xml.contains("</p:extLst>"));
    }

    #[test]
    fn test_section_manager_xml_slide_ids() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Intro", 0, 2).unwrap();
        let xml = mgr.to_xml(5);
        // Slide IDs start at 256
        assert!(xml.contains(r#"id="256""#));
        assert!(xml.contains(r#"id="257""#));
    }

    #[test]
    fn test_sections_overlap_fn() {
        let a = SlideSection::new("A", 0, 3);
        let b = SlideSection::new("B", 2, 3);
        assert!(sections_overlap(&a, &b));

        let c = SlideSection::new("C", 3, 2);
        assert!(!sections_overlap(&a, &c));
    }

    #[test]
    fn test_sections_overlap_empty() {
        let a = SlideSection::new("A", 0, 0);
        let b = SlideSection::new("B", 0, 3);
        assert!(!sections_overlap(&a, &b));
    }

    #[test]
    fn test_generate_section_id_deterministic() {
        let id1 = generate_section_id("Test");
        let id2 = generate_section_id("Test");
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_generate_section_id_unique() {
        let id1 = generate_section_id("Intro");
        let id2 = generate_section_id("Body");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_section_xml_escaping() {
        let mut mgr = SectionManager::new();
        mgr.add_section("Q&A <Session>", 0, 1).unwrap();
        let xml = mgr.to_xml(1);
        assert!(xml.contains("Q&amp;A &lt;Session&gt;"));
    }
}
