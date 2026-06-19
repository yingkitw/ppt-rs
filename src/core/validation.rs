//! Shared validation helpers used across generators, repair, and CLI tools.

use crate::exc::{messages, PptxError, Result};
use std::collections::HashSet;

/// A required PPTX package part and its human-readable description.
pub type RequiredPart = (&'static str, &'static str);

/// Minimum parts required for a readable PPTX (used by CLI validate and tests).
pub const REQUIRED_PARTS_MINIMAL: &[&'static str] = &[
    "[Content_Types].xml",
    "_rels/.rels",
    "ppt/presentation.xml",
    "docProps/core.xml",
];

/// Parts required for structural repair validation.
pub const REQUIRED_PARTS_REPAIR: &[RequiredPart] = &[
    ("[Content_Types].xml", "Content types definition"),
    ("_rels/.rels", "Package relationships"),
    ("ppt/presentation.xml", "Presentation document"),
    (
        "ppt/_rels/presentation.xml.rels",
        "Presentation relationships",
    ),
];

/// A validation issue found in a PPTX package or input value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationIssue {
    MissingPart {
        path: String,
        description: Option<String>,
    },
    EmptyXml {
        path: String,
    },
    InvalidXml {
        path: String,
        error: String,
    },
}

impl ValidationIssue {
    pub fn message(&self) -> String {
        match self {
            ValidationIssue::MissingPart { path, description } => match description {
                Some(desc) => format!("Missing required part '{path}' ({desc})"),
                None => messages::missing_part(path),
            },
            ValidationIssue::EmptyXml { path } => messages::empty_xml(path),
            ValidationIssue::InvalidXml { path, error } => messages::invalid_xml(path, error),
        }
    }
}

/// Clamp a ratio to the 0.0–1.0 range.
pub fn clamp_ratio(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Clamp an opacity/alpha value to the 0.0–1.0 range.
pub fn clamp_unit_interval(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

/// Validate that a string is non-empty after trimming.
pub fn validate_non_empty_str(value: &str, field: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(PptxError::InvalidValue(messages::must_not_be_empty(field)));
    }
    Ok(())
}

/// Validate that a collection is non-empty.
pub fn validate_non_empty<T>(items: &[T], field: &str) -> Result<()> {
    if items.is_empty() {
        return Err(PptxError::InvalidState(messages::must_not_be_empty(field)));
    }
    Ok(())
}

/// Validate that a usize index is within `[0, count)`.
pub fn validate_index(index: usize, count: usize, field: &str) -> Result<()> {
    if index >= count {
        return Err(PptxError::NotFound(messages::index_out_of_range(
            field, index, count,
        )));
    }
    Ok(())
}

/// Validate that a value is strictly positive.
pub fn validate_positive(value: u32, field: &str) -> Result<()> {
    if value == 0 {
        return Err(PptxError::InvalidValue(messages::must_be_positive(field)));
    }
    Ok(())
}

/// Basic well-formedness check for XML / RELS content.
pub fn validate_well_formed_xml(xml: &str) -> Result<()> {
    let trimmed = xml.trim();
    if trimmed.is_empty() {
        return Err(PptxError::InvalidXml(messages::empty_xml_content()));
    }

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
            _ => {}
        }
    }

    Ok(())
}

/// Check that all required part paths exist in `found`.
pub fn check_required_parts(
    found: &HashSet<String>,
    required: &[&str],
) -> Vec<ValidationIssue> {
    required
        .iter()
        .filter(|path| !found.contains(**path))
        .map(|path| ValidationIssue::MissingPart {
            path: (*path).to_string(),
            description: None,
        })
        .collect()
}

/// Check required parts with descriptions (repair workflow).
pub fn check_required_parts_with_descriptions(
    has_part: impl Fn(&str) -> bool,
    required: &[RequiredPart],
) -> Vec<ValidationIssue> {
    required
        .iter()
        .filter(|(path, _)| !has_part(path))
        .map(|(path, description)| ValidationIssue::MissingPart {
            path: (*path).to_string(),
            description: Some((*description).to_string()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp_ratio() {
        assert_eq!(clamp_ratio(0.5), 0.5);
        assert_eq!(clamp_ratio(-1.0), 0.0);
        assert_eq!(clamp_ratio(2.0), 1.0);
    }

    #[test]
    fn test_validate_non_empty_str() {
        assert!(validate_non_empty_str("hello", "title").is_ok());
        assert!(validate_non_empty_str("  ", "title").is_err());
    }

    #[test]
    fn test_validate_index() {
        assert!(validate_index(0, 3, "slide").is_ok());
        assert!(validate_index(3, 3, "slide").is_err());
    }

    #[test]
    fn test_validate_well_formed_xml() {
        assert!(validate_well_formed_xml("<root/>").is_ok());
        assert!(validate_well_formed_xml("").is_err());
        assert!(validate_well_formed_xml("   ").is_err());
    }

    #[test]
    fn test_check_required_parts() {
        let found: HashSet<_> = ["a.xml", "b.xml"].into_iter().map(str::to_string).collect();
        let issues = check_required_parts(&found, &["a.xml", "c.xml"]);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].message(), "Missing required part: c.xml");
    }
}
