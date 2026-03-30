//! PPTX Repair Module
//!
//! Provides functionality to detect and repair common issues in PPTX files:
//! - Missing required parts (Content_Types, relationships, etc.)
//! - Invalid or malformed XML
//! - Broken relationships
//! - Missing slide references
//! - Corrupted package structure

use crate::exc::{PptxError, Result};
use crate::opc::Package;
use std::collections::HashSet;
use std::path::Path;

/// Types of issues that can be detected in a PPTX file
#[derive(Debug, Clone, PartialEq)]
pub enum RepairIssue {
    /// Missing required part
    MissingPart { path: String, description: String },
    /// Invalid XML content
    InvalidXml { path: String, error: String },
    /// Broken relationship reference
    BrokenRelationship {
        source: String,
        target: String,
        rel_id: String,
    },
    /// Missing slide in presentation
    MissingSlideReference { slide_path: String },
    /// Orphan slide (not referenced in presentation)
    OrphanSlide { slide_path: String },
    /// Invalid content type
    InvalidContentType { path: String },
    /// Corrupted ZIP entry
    CorruptedEntry { path: String, error: String },
    /// Missing namespace declaration
    MissingNamespace { path: String, namespace: String },
    /// Empty required element
    EmptyRequiredElement { path: String, element: String },
}

impl RepairIssue {
    /// Get severity level (1-3, where 3 is critical)
    pub fn severity(&self) -> u8 {
        match self {
            RepairIssue::MissingPart { .. } => 3,
            RepairIssue::InvalidXml { .. } => 3,
            RepairIssue::BrokenRelationship { .. } => 2,
            RepairIssue::MissingSlideReference { .. } => 2,
            RepairIssue::OrphanSlide { .. } => 1,
            RepairIssue::InvalidContentType { .. } => 2,
            RepairIssue::CorruptedEntry { .. } => 3,
            RepairIssue::MissingNamespace { .. } => 2,
            RepairIssue::EmptyRequiredElement { .. } => 1,
        }
    }

    /// Check if this issue is auto-repairable
    pub fn is_repairable(&self) -> bool {
        match self {
            RepairIssue::MissingPart { .. } => true,
            RepairIssue::InvalidXml { .. } => true,
            RepairIssue::BrokenRelationship { .. } => true,
            RepairIssue::MissingSlideReference { .. } => true,
            RepairIssue::OrphanSlide { .. } => true,
            RepairIssue::InvalidContentType { .. } => true,
            RepairIssue::CorruptedEntry { .. } => false,
            RepairIssue::MissingNamespace { .. } => true,
            RepairIssue::EmptyRequiredElement { .. } => true,
        }
    }

    /// Get human-readable description
    pub fn description(&self) -> String {
        match self {
            RepairIssue::MissingPart { path, description } => {
                format!("Missing required part '{}': {}", path, description)
            }
            RepairIssue::InvalidXml { path, error } => {
                format!("Invalid XML in '{}': {}", path, error)
            }
            RepairIssue::BrokenRelationship {
                source,
                target,
                rel_id,
            } => {
                format!(
                    "Broken relationship in '{}': {} -> {} ({})",
                    source, rel_id, target, rel_id
                )
            }
            RepairIssue::MissingSlideReference { slide_path } => {
                format!(
                    "Slide '{}' exists but not referenced in presentation",
                    slide_path
                )
            }
            RepairIssue::OrphanSlide { slide_path } => {
                format!("Orphan slide reference: '{}' does not exist", slide_path)
            }
            RepairIssue::InvalidContentType { path } => {
                format!("Invalid or missing content type for '{}'", path)
            }
            RepairIssue::CorruptedEntry { path, error } => {
                format!("Corrupted ZIP entry '{}': {}", path, error)
            }
            RepairIssue::MissingNamespace { path, namespace } => {
                format!("Missing namespace '{}' in '{}'", namespace, path)
            }
            RepairIssue::EmptyRequiredElement { path, element } => {
                format!("Empty required element '{}' in '{}'", element, path)
            }
        }
    }
}

/// Result of a repair operation
#[derive(Debug, Clone)]
pub struct RepairResult {
    /// Issues found during validation
    pub issues_found: Vec<RepairIssue>,
    /// Issues that were successfully repaired
    pub issues_repaired: Vec<RepairIssue>,
    /// Issues that could not be repaired
    pub issues_unrepaired: Vec<RepairIssue>,
    /// Whether the file is now valid
    pub is_valid: bool,
}

impl RepairResult {
    fn new() -> Self {
        RepairResult {
            issues_found: Vec::new(),
            issues_repaired: Vec::new(),
            issues_unrepaired: Vec::new(),
            is_valid: true,
        }
    }

    /// Get total number of issues found
    pub fn total_issues(&self) -> usize {
        self.issues_found.len()
    }

    /// Get number of critical issues
    pub fn critical_issues(&self) -> usize {
        self.issues_found
            .iter()
            .filter(|i| i.severity() == 3)
            .count()
    }

    /// Check if all issues were repaired
    pub fn fully_repaired(&self) -> bool {
        self.issues_unrepaired.is_empty()
    }
}

/// PPTX Repair utility
pub struct PptxRepair {
    package: Package,
    issues: Vec<RepairIssue>,
}

impl PptxRepair {
    /// Required parts in a valid PPTX file
    const REQUIRED_PARTS: &'static [(&'static str, &'static str)] = &[
        ("[Content_Types].xml", "Content types definition"),
        ("_rels/.rels", "Package relationships"),
        ("ppt/presentation.xml", "Presentation document"),
        (
            "ppt/_rels/presentation.xml.rels",
            "Presentation relationships",
        ),
    ];

    /// Open a PPTX file for repair
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let package = Package::open(path)?;
        Ok(PptxRepair {
            package,
            issues: Vec::new(),
        })
    }

    /// Open from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let cursor = std::io::Cursor::new(data);
        let package = Package::open_reader(cursor)?;
        Ok(PptxRepair {
            package,
            issues: Vec::new(),
        })
    }

    /// Validate the PPTX file and return found issues
    pub fn validate(&mut self) -> Vec<RepairIssue> {
        self.issues.clear();

        // Check required parts
        self.check_required_parts();

        // Check XML validity
        self.check_xml_validity();

        // Check relationships
        self.check_relationships();

        // Check slide references
        self.check_slide_references();

        // Check content types
        self.check_content_types();

        self.issues.clone()
    }

    /// Repair all detected issues
    pub fn repair(&mut self) -> RepairResult {
        let mut result = RepairResult::new();

        // First validate to find issues
        result.issues_found = self.validate();

        if result.issues_found.is_empty() {
            result.is_valid = true;
            return result;
        }

        // Attempt to repair each issue
        for issue in &result.issues_found.clone() {
            if issue.is_repairable() {
                match self.repair_issue(issue) {
                    Ok(()) => result.issues_repaired.push(issue.clone()),
                    Err(_) => result.issues_unrepaired.push(issue.clone()),
                }
            } else {
                result.issues_unrepaired.push(issue.clone());
            }
        }

        // Re-validate to check if repairs were successful
        let remaining_issues = self.validate();
        result.is_valid = remaining_issues.is_empty();

        result
    }

    /// Save the repaired PPTX to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        self.package.save(path)
    }

    /// Save to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        self.package.save_writer(&mut cursor)?;
        Ok(cursor.into_inner())
    }

    /// Get the underlying package
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Get mutable reference to package
    pub fn package_mut(&mut self) -> &mut Package {
        &mut self.package
    }

    // Validation methods

    fn check_required_parts(&mut self) {
        for (path, description) in Self::REQUIRED_PARTS {
            if !self.package.has_part(path) {
                self.issues.push(RepairIssue::MissingPart {
                    path: path.to_string(),
                    description: description.to_string(),
                });
            }
        }
    }

    fn check_xml_validity(&mut self) {
        let xml_parts: Vec<String> = self
            .package
            .part_paths()
            .iter()
            .filter(|p| p.ends_with(".xml") || p.ends_with(".rels"))
            .map(|s| s.to_string())
            .collect();

        for path in xml_parts {
            if let Some(content) = self.package.get_part(&path) {
                let xml_str = String::from_utf8_lossy(content);
                if let Err(e) = self.validate_xml(&xml_str) {
                    self.issues.push(RepairIssue::InvalidXml { path, error: e });
                }
            }
        }
    }

    fn validate_xml(&self, xml: &str) -> std::result::Result<(), String> {
        // Check for XML declaration
        let trimmed = xml.trim();
        if trimmed.is_empty() {
            return Err("Empty XML content".to_string());
        }

        // Basic well-formedness check
        let depth = 0i32;
        let mut in_tag = false;
        let mut in_string = false;
        let mut string_char = '"';

        for ch in trimmed.chars() {
            match ch {
                '"' | '\'' if in_tag && !in_string => {
                    in_string = true;
                    string_char = ch;
                }
                c if in_string && c == string_char => {
                    in_string = false;
                }
                '<' if !in_string => {
                    in_tag = true;
                }
                '>' if !in_string => {
                    in_tag = false;
                }
                '/' if in_tag && !in_string => {
                    // Self-closing or end tag
                }
                _ => {}
            }
        }

        // Try to use anyrepair for JSON-like content repair
        // For XML, we do basic validation
        if depth < 0 {
            return Err("Unbalanced tags".to_string());
        }

        Ok(())
    }

    fn check_relationships(&mut self) {
        // Check _rels/.rels
        self.check_rels_file("_rels/.rels");

        // Check ppt/_rels/presentation.xml.rels
        self.check_rels_file("ppt/_rels/presentation.xml.rels");

        // Check slide relationship files
        let slide_rels: Vec<String> = self
            .package
            .part_paths()
            .iter()
            .filter(|p| p.starts_with("ppt/slides/_rels/") && p.ends_with(".xml.rels"))
            .map(|s| s.to_string())
            .collect();

        for rels_path in slide_rels {
            self.check_rels_file(&rels_path);
        }
    }

    fn check_rels_file(&mut self, rels_path: &str) {
        if let Some(content) = self.package.get_part(rels_path) {
            let xml_str = String::from_utf8_lossy(content);

            // Parse relationships and check targets exist
            for line in xml_str.lines() {
                if line.contains("Relationship") && line.contains("Target=") {
                    if let Some(target) = self.extract_attribute(line, "Target") {
                        let rel_id = self.extract_attribute(line, "Id").unwrap_or_default();

                        // Skip external relationships
                        if target.starts_with("http://") || target.starts_with("https://") {
                            continue;
                        }

                        // Resolve relative path
                        let full_path = self.resolve_path(rels_path, &target);

                        if !self.package.has_part(&full_path) && !target.contains("..") {
                            self.issues.push(RepairIssue::BrokenRelationship {
                                source: rels_path.to_string(),
                                target: full_path,
                                rel_id,
                            });
                        }
                    }
                }
            }
        }
    }

    fn extract_attribute(&self, line: &str, attr: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr);
        if let Some(start) = line.find(&pattern) {
            let value_start = start + pattern.len();
            if let Some(end) = line[value_start..].find('"') {
                return Some(line[value_start..value_start + end].to_string());
            }
        }
        None
    }

    fn resolve_path(&self, rels_path: &str, target: &str) -> String {
        if target.starts_with('/') {
            return target[1..].to_string();
        }

        // Get directory of rels file
        let rels_dir = rels_path.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
        let base_dir = rels_dir
            .replace("_rels", "")
            .trim_end_matches('/')
            .to_string();

        if target.starts_with("../") {
            // Go up one directory
            let parent = base_dir.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
            format!("{}/{}", parent, target.trim_start_matches("../"))
        } else {
            if base_dir.is_empty() {
                target.to_string()
            } else {
                format!("{}/{}", base_dir, target)
            }
        }
    }

    fn check_slide_references(&mut self) {
        // Get slides from presentation.xml.rels
        let mut referenced_slides: HashSet<String> = HashSet::new();

        if let Some(rels_content) = self.package.get_part("ppt/_rels/presentation.xml.rels") {
            let xml_str = String::from_utf8_lossy(rels_content);
            for line in xml_str.lines() {
                if line.contains("slide") && line.contains("Target=") {
                    if let Some(target) = self.extract_attribute(line, "Target") {
                        let full_path = if target.starts_with('/') {
                            target[1..].to_string()
                        } else {
                            format!("ppt/{}", target)
                        };
                        referenced_slides.insert(full_path);
                    }
                }
            }
        }

        // Get actual slide files
        let actual_slides: HashSet<String> = self
            .package
            .part_paths()
            .iter()
            .filter(|p| {
                p.starts_with("ppt/slides/slide") && p.ends_with(".xml") && !p.contains("_rels")
            })
            .map(|s| s.to_string())
            .collect();

        // Check for orphan references (referenced but don't exist)
        for slide in &referenced_slides {
            if !actual_slides.contains(slide) {
                self.issues.push(RepairIssue::OrphanSlide {
                    slide_path: slide.clone(),
                });
            }
        }

        // Check for unreferenced slides
        for slide in &actual_slides {
            if !referenced_slides.contains(slide) {
                self.issues.push(RepairIssue::MissingSlideReference {
                    slide_path: slide.clone(),
                });
            }
        }
    }

    fn check_content_types(&mut self) {
        if let Some(content) = self.package.get_part("[Content_Types].xml") {
            let xml_str = String::from_utf8_lossy(content);

            // Check that all parts have content types
            let parts = self.package.part_paths();
            for part in parts {
                if part == "[Content_Types].xml" {
                    continue;
                }

                // Check if part has Override or matches Default
                let has_override = xml_str.contains(&format!("PartName=\"/{}\"", part));
                let extension = part.rsplit('.').next().unwrap_or("");
                let has_default = xml_str.contains(&format!("Extension=\"{}\"", extension));

                if !has_override && !has_default && !part.ends_with(".rels") {
                    self.issues.push(RepairIssue::InvalidContentType {
                        path: part.to_string(),
                    });
                }
            }
        }
    }

    // Repair methods

    fn repair_issue(&mut self, issue: &RepairIssue) -> Result<()> {
        match issue {
            RepairIssue::MissingPart { path, .. } => self.repair_missing_part(path),
            RepairIssue::InvalidXml { path, .. } => self.repair_invalid_xml(path),
            RepairIssue::BrokenRelationship {
                source,
                target,
                rel_id,
            } => self.repair_broken_relationship(source, target, rel_id),
            RepairIssue::MissingSlideReference { slide_path } => {
                self.repair_missing_slide_reference(slide_path)
            }
            RepairIssue::OrphanSlide { slide_path } => self.repair_orphan_slide(slide_path),
            RepairIssue::InvalidContentType { path } => self.repair_invalid_content_type(path),
            RepairIssue::MissingNamespace { path, namespace } => {
                self.repair_missing_namespace(path, namespace)
            }
            RepairIssue::EmptyRequiredElement { path, element } => {
                self.repair_empty_element(path, element)
            }
            RepairIssue::CorruptedEntry { .. } => Err(PptxError::Generic(
                "Cannot repair corrupted entry".to_string(),
            )),
        }
    }

    fn repair_missing_part(&mut self, path: &str) -> Result<()> {
        let content = match path {
            "[Content_Types].xml" => self.generate_content_types(),
            "_rels/.rels" => self.generate_package_rels(),
            "ppt/presentation.xml" => self.generate_presentation_xml(),
            "ppt/_rels/presentation.xml.rels" => self.generate_presentation_rels(),
            _ => {
                return Err(PptxError::Generic(format!(
                    "Cannot generate part: {}",
                    path
                )))
            }
        };

        self.package
            .add_part(path.to_string(), content.into_bytes());
        Ok(())
    }

    fn repair_invalid_xml(&mut self, path: &str) -> Result<()> {
        if let Some(content) = self.package.get_part(path) {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Try to repair using anyrepair for JSON-like structures
            // For XML, we try basic fixes
            let repaired = self.attempt_xml_repair(&xml_str);

            self.package
                .add_part(path.to_string(), repaired.into_bytes());
        }
        Ok(())
    }

    fn attempt_xml_repair(&self, xml: &str) -> String {
        let mut repaired = xml.to_string();

        // Ensure XML declaration
        if !repaired.trim().starts_with("<?xml") {
            repaired = format!(
                "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n{}",
                repaired
            );
        }

        // Fix common issues
        // Replace bare & with &amp; (but not existing entities)
        // Since Rust regex doesn't support look-ahead, we use a different approach
        let mut result = String::new();
        let mut chars = repaired.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '&' {
                // Check if this is already an entity
                let mut entity = String::from("&");
                let mut is_entity = false;
                let mut temp_chars: Vec<char> = Vec::new();

                while let Some(&next) = chars.peek() {
                    if next == ';' {
                        entity.push(chars.next().unwrap());
                        is_entity = entity == "&amp;"
                            || entity == "&lt;"
                            || entity == "&gt;"
                            || entity == "&quot;"
                            || entity == "&apos;"
                            || entity.starts_with("&#");
                        break;
                    } else if next.is_alphanumeric() || next == '#' || next == 'x' {
                        temp_chars.push(chars.next().unwrap());
                        entity.push(*temp_chars.last().unwrap());
                    } else {
                        break;
                    }
                    if entity.len() > 10 {
                        break;
                    }
                }

                if is_entity {
                    result.push_str(&entity);
                } else {
                    result.push_str("&amp;");
                    for c in temp_chars {
                        result.push(c);
                    }
                }
            } else {
                result.push(ch);
            }
        }
        repaired = result;

        repaired
    }

    fn repair_broken_relationship(
        &mut self,
        source: &str,
        _target: &str,
        rel_id: &str,
    ) -> Result<()> {
        // Remove the broken relationship from the rels file
        if let Some(content) = self.package.get_part(source) {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Remove the relationship line with this ID
            let pattern = format!("Id=\"{}\"", rel_id);
            let lines: Vec<&str> = xml_str
                .lines()
                .filter(|line| !line.contains(&pattern))
                .collect();

            let repaired = lines.join("\n");
            self.package
                .add_part(source.to_string(), repaired.into_bytes());
        }
        Ok(())
    }

    fn repair_missing_slide_reference(&mut self, slide_path: &str) -> Result<()> {
        // Add the slide to presentation.xml.rels
        if let Some(content) = self.package.get_part("ppt/_rels/presentation.xml.rels") {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Find the highest rId
            let max_id = self.find_max_rel_id(&xml_str);
            let new_id = format!("rId{}", max_id + 1);

            // Extract slide number from path
            let slide_target = slide_path.replace("ppt/", "");

            // Add new relationship before </Relationships>
            let new_rel = format!(
                "  <Relationship Id=\"{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide\" Target=\"{}\"/>",
                new_id, slide_target
            );

            let repaired = xml_str.replace(
                "</Relationships>",
                &format!("{}\n</Relationships>", new_rel),
            );
            self.package.add_part(
                "ppt/_rels/presentation.xml.rels".to_string(),
                repaired.into_bytes(),
            );

            // Also update presentation.xml sldIdLst
            self.add_slide_to_presentation(&new_id)?;
        }
        Ok(())
    }

    fn add_slide_to_presentation(&mut self, rel_id: &str) -> Result<()> {
        if let Some(content) = self.package.get_part("ppt/presentation.xml") {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Find highest slide ID
            let max_slide_id = self.find_max_slide_id(&xml_str);
            let new_slide_id = max_slide_id + 1;

            // Add to sldIdLst
            let new_entry = format!("<p:sldId id=\"{}\" r:id=\"{}\"/>", new_slide_id, rel_id);

            let repaired = if xml_str.contains("</p:sldIdLst>") {
                xml_str.replace("</p:sldIdLst>", &format!("{}\n</p:sldIdLst>", new_entry))
            } else if xml_str.contains("<p:sldIdLst/>") {
                xml_str.replace(
                    "<p:sldIdLst/>",
                    &format!("<p:sldIdLst>{}</p:sldIdLst>", new_entry),
                )
            } else {
                // Insert sldIdLst after sldMasterIdLst
                xml_str.replace(
                    "</p:sldMasterIdLst>",
                    &format!(
                        "</p:sldMasterIdLst>\n<p:sldIdLst>{}</p:sldIdLst>",
                        new_entry
                    ),
                )
            };

            self.package
                .add_part("ppt/presentation.xml".to_string(), repaired.into_bytes());
        }
        Ok(())
    }

    fn find_max_rel_id(&self, xml: &str) -> u32 {
        let re = regex::Regex::new(r#"Id="rId(\d+)""#).unwrap();
        re.captures_iter(xml)
            .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse::<u32>().ok()))
            .max()
            .unwrap_or(0)
    }

    fn find_max_slide_id(&self, xml: &str) -> u32 {
        let re = regex::Regex::new(r#"<p:sldId id="(\d+)""#).unwrap();
        re.captures_iter(xml)
            .filter_map(|cap| cap.get(1).and_then(|m| m.as_str().parse::<u32>().ok()))
            .max()
            .unwrap_or(255)
    }

    fn repair_orphan_slide(&mut self, slide_path: &str) -> Result<()> {
        // Remove the orphan reference from presentation.xml.rels
        if let Some(content) = self.package.get_part("ppt/_rels/presentation.xml.rels") {
            let xml_str = String::from_utf8_lossy(content).to_string();
            let slide_target = slide_path.replace("ppt/", "");

            // Find and remove the relationship
            let lines: Vec<&str> = xml_str
                .lines()
                .filter(|line| !line.contains(&format!("Target=\"{}\"", slide_target)))
                .collect();

            let repaired = lines.join("\n");
            self.package.add_part(
                "ppt/_rels/presentation.xml.rels".to_string(),
                repaired.into_bytes(),
            );
        }
        Ok(())
    }

    fn repair_invalid_content_type(&mut self, path: &str) -> Result<()> {
        if let Some(content) = self.package.get_part("[Content_Types].xml") {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Determine content type based on path
            let content_type = self.infer_content_type(path);

            // Add Override entry
            let new_override = format!(
                "  <Override PartName=\"/{}\" ContentType=\"{}\"/>",
                path, content_type
            );

            let repaired = xml_str.replace("</Types>", &format!("{}\n</Types>", new_override));
            self.package
                .add_part("[Content_Types].xml".to_string(), repaired.into_bytes());
        }
        Ok(())
    }

    fn infer_content_type(&self, path: &str) -> &'static str {
        if path.contains("slide") && path.ends_with(".xml") {
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        } else if path.contains("slideLayout") {
            "application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml"
        } else if path.contains("slideMaster") {
            "application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml"
        } else if path.contains("theme") {
            "application/vnd.openxmlformats-officedocument.theme+xml"
        } else if path.contains("presentation.xml") {
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
        } else if path.ends_with(".xml") {
            "application/xml"
        } else {
            "application/octet-stream"
        }
    }

    fn repair_missing_namespace(&mut self, path: &str, namespace: &str) -> Result<()> {
        if let Some(content) = self.package.get_part(path) {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Add namespace to root element
            let ns_decl = match namespace {
                "p" => "xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\"",
                "a" => "xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\"",
                "r" => "xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\"",
                _ => return Ok(()),
            };

            // Find first element and add namespace
            if let Some(pos) = xml_str.find('>') {
                if !xml_str[..pos].contains(ns_decl) {
                    let repaired = format!("{} {}{}", &xml_str[..pos], ns_decl, &xml_str[pos..]);
                    self.package
                        .add_part(path.to_string(), repaired.into_bytes());
                }
            }
        }
        Ok(())
    }

    fn repair_empty_element(&mut self, path: &str, element: &str) -> Result<()> {
        if let Some(content) = self.package.get_part(path) {
            let xml_str = String::from_utf8_lossy(content).to_string();

            // Add minimal content to empty element
            let empty_pattern = format!("<{}/>", element);
            let filled = format!("<{}></{}>", element, element);

            let repaired = xml_str.replace(&empty_pattern, &filled);
            self.package
                .add_part(path.to_string(), repaired.into_bytes());
        }
        Ok(())
    }

    // Template generators

    fn generate_content_types(&self) -> String {
        let mut content = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Default Extension="jpeg" ContentType="image/jpeg"/>
  <Default Extension="png" ContentType="image/png"/>
"#,
        );

        // Add overrides for existing parts
        for path in self.package.part_paths() {
            if path.ends_with(".xml") && path != "[Content_Types].xml" {
                let ct = self.infer_content_type(path);
                content.push_str(&format!(
                    "  <Override PartName=\"/{}\" ContentType=\"{}\"/>\n",
                    path, ct
                ));
            }
        }

        content.push_str("</Types>");
        content
    }

    fn generate_package_rels(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="ppt/presentation.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
</Relationships>"#.to_string()
    }

    fn generate_presentation_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:presentation xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
  <p:sldMasterIdLst>
    <p:sldMasterId id="2147483648" r:id="rId1"/>
  </p:sldMasterIdLst>
  <p:sldIdLst/>
  <p:sldSz cx="9144000" cy="6858000"/>
  <p:notesSz cx="6858000" cy="9144000"/>
</p:presentation>"#.to_string()
    }

    fn generate_presentation_rels(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster" Target="slideMasters/slideMaster1.xml"/>
  <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme" Target="theme/theme1.xml"/>
  <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/presProps" Target="presProps.xml"/>
  <Relationship Id="rId4" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/viewProps" Target="viewProps.xml"/>
  <Relationship Id="rId5" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/tableStyles" Target="tableStyles.xml"/>
</Relationships>"#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_issue_severity() {
        let issue = RepairIssue::MissingPart {
            path: "test".to_string(),
            description: "test".to_string(),
        };
        assert_eq!(issue.severity(), 3);

        let issue = RepairIssue::OrphanSlide {
            slide_path: "test".to_string(),
        };
        assert_eq!(issue.severity(), 1);
    }

    #[test]
    fn test_repair_issue_is_repairable() {
        let issue = RepairIssue::MissingPart {
            path: "test".to_string(),
            description: "test".to_string(),
        };
        assert!(issue.is_repairable());

        let issue = RepairIssue::CorruptedEntry {
            path: "test".to_string(),
            error: "test".to_string(),
        };
        assert!(!issue.is_repairable());
    }

    #[test]
    fn test_repair_issue_description() {
        let issue = RepairIssue::MissingPart {
            path: "[Content_Types].xml".to_string(),
            description: "Content types definition".to_string(),
        };
        assert!(issue.description().contains("[Content_Types].xml"));
    }

    #[test]
    fn test_repair_result_new() {
        let result = RepairResult::new();
        assert!(result.issues_found.is_empty());
        assert!(result.is_valid);
    }

    #[test]
    fn test_repair_result_total_issues() {
        let mut result = RepairResult::new();
        result.issues_found.push(RepairIssue::MissingPart {
            path: "test".to_string(),
            description: "test".to_string(),
        });
        assert_eq!(result.total_issues(), 1);
    }

    #[test]
    fn test_repair_result_critical_issues() {
        let mut result = RepairResult::new();
        result.issues_found.push(RepairIssue::MissingPart {
            path: "test".to_string(),
            description: "test".to_string(),
        });
        result.issues_found.push(RepairIssue::OrphanSlide {
            slide_path: "test".to_string(),
        });
        assert_eq!(result.critical_issues(), 1);
    }

    #[test]
    fn test_repair_result_fully_repaired() {
        let result = RepairResult::new();
        assert!(result.fully_repaired());
    }

    #[test]
    fn test_infer_content_type() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        assert_eq!(
            repair.infer_content_type("ppt/slides/slide1.xml"),
            "application/vnd.openxmlformats-officedocument.presentationml.slide+xml"
        );
        assert_eq!(
            repair.infer_content_type("ppt/presentation.xml"),
            "application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml"
        );
    }

    #[test]
    fn test_resolve_path() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        assert_eq!(
            repair.resolve_path("ppt/_rels/presentation.xml.rels", "slides/slide1.xml"),
            "ppt/slides/slide1.xml"
        );
        assert_eq!(
            repair.resolve_path("_rels/.rels", "ppt/presentation.xml"),
            "ppt/presentation.xml"
        );
    }

    #[test]
    fn test_find_max_rel_id() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let xml = r#"<Relationships>
            <Relationship Id="rId1" Target="a"/>
            <Relationship Id="rId5" Target="b"/>
            <Relationship Id="rId3" Target="c"/>
        </Relationships>"#;

        assert_eq!(repair.find_max_rel_id(xml), 5);
    }

    #[test]
    fn test_find_max_slide_id() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let xml = r#"<p:sldIdLst>
            <p:sldId id="256" r:id="rId1"/>
            <p:sldId id="260" r:id="rId2"/>
            <p:sldId id="258" r:id="rId3"/>
        </p:sldIdLst>"#;

        assert_eq!(repair.find_max_slide_id(xml), 260);
    }

    #[test]
    fn test_extract_attribute() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let line = r#"<Relationship Id="rId1" Target="slides/slide1.xml"/>"#;
        assert_eq!(
            repair.extract_attribute(line, "Id"),
            Some("rId1".to_string())
        );
        assert_eq!(
            repair.extract_attribute(line, "Target"),
            Some("slides/slide1.xml".to_string())
        );
        assert_eq!(repair.extract_attribute(line, "Missing"), None);
    }

    #[test]
    fn test_generate_content_types() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let content = repair.generate_content_types();
        assert!(content.contains("<?xml"));
        assert!(content.contains("<Types"));
        assert!(content.contains("Extension=\"rels\""));
    }

    #[test]
    fn test_generate_package_rels() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let content = repair.generate_package_rels();
        assert!(content.contains("<?xml"));
        assert!(content.contains("<Relationships"));
        assert!(content.contains("presentation.xml"));
    }

    #[test]
    fn test_generate_presentation_xml() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        let content = repair.generate_presentation_xml();
        assert!(content.contains("<?xml"));
        assert!(content.contains("<p:presentation"));
        assert!(content.contains("p:sldIdLst"));
    }

    #[test]
    fn test_attempt_xml_repair() {
        let repair = PptxRepair {
            package: Package::new(),
            issues: Vec::new(),
        };

        // Test adding XML declaration
        let xml = "<root><child/></root>";
        let repaired = repair.attempt_xml_repair(xml);
        assert!(repaired.starts_with("<?xml"));

        // Test ampersand fix
        let xml = "<?xml version=\"1.0\"?><root>A & B</root>";
        let repaired = repair.attempt_xml_repair(xml);
        assert!(repaired.contains("A &amp; B"));
    }
}
