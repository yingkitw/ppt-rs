//! Multi-level validation framework inspired by Open-XML-SDK
//!
//! Provides comprehensive validation at multiple levels:
//! - Schema validation (XML structure)
//! - Semantic validation (business rules)
//! - Document validation (document-level rules)
//! - Package validation (package-level rules)

use crate::error::Result;
use std::fmt;

/// Validation error type classification
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ValidationErrorType {
    /// Critical error that prevents valid document
    Error,
    /// Warning that may indicate issues
    Warning,
    /// Informational notice
    Notice,
}

impl fmt::Display for ValidationErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationErrorType::Error => write!(f, "Error"),
            ValidationErrorType::Warning => write!(f, "Warning"),
            ValidationErrorType::Notice => write!(f, "Notice"),
        }
    }
}

/// Comprehensive validation error with context information
#[derive(Clone, Debug)]
pub struct ValidationError {
    /// Error type classification
    pub error_type: ValidationErrorType,
    /// Human-readable error description
    pub description: String,
    /// XPath to the problematic element
    pub path: String,
    /// Line number in XML (if available)
    pub line: Option<usize>,
    /// Column number in XML (if available)
    pub column: Option<usize>,
    /// URI of the part containing the error
    pub part_uri: Option<String>,
    /// Element ID if available
    pub element_id: Option<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(error_type: ValidationErrorType, description: String, path: String) -> Self {
        Self {
            error_type,
            description,
            path,
            line: None,
            column: None,
            part_uri: None,
            element_id: None,
        }
    }

    /// Create an error with line and column information
    pub fn with_location(
        error_type: ValidationErrorType,
        description: String,
        path: String,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            error_type,
            description,
            path,
            line: Some(line),
            column: Some(column),
            part_uri: None,
            element_id: None,
        }
    }

    /// Set the part URI
    pub fn with_part_uri(mut self, part_uri: String) -> Self {
        self.part_uri = Some(part_uri);
        self
    }

    /// Set the element ID
    pub fn with_element_id(mut self, element_id: String) -> Self {
        self.element_id = Some(element_id);
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.description)?;
        if let Some(part_uri) = &self.part_uri {
            write!(f, " in {}", part_uri)?;
        }
        if let Some(line) = self.line {
            write!(f, " at line {}", line)?;
            if let Some(column) = self.column {
                write!(f, ", column {}", column)?;
            }
        }
        if !self.path.is_empty() {
            write!(f, " ({})", self.path)?;
        }
        Ok(())
    }
}

/// Validator trait for implementing custom validation logic
pub trait Validator {
    /// Perform validation and return list of errors
    fn validate(&self) -> Result<Vec<ValidationError>>;
}

/// Schema validator - validates XML structure and attributes
pub struct SchemaValidator;

impl SchemaValidator {
    /// Create a new schema validator
    pub fn new() -> Self {
        Self
    }

    /// Validate element structure
    pub fn validate_element_structure(
        tag_name: &str,
        required_attributes: &[&str],
        actual_attributes: &[&str],
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for required in required_attributes {
            if !actual_attributes.contains(required) {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::Error,
                        format!("Required attribute '{}' missing from element '{}'", required, tag_name),
                        format!("/{}", tag_name),
                    )
                );
            }
        }

        errors
    }

    /// Validate attribute value
    pub fn validate_attribute_value(
        tag_name: &str,
        attr_name: &str,
        attr_value: &str,
        valid_values: &[&str],
    ) -> Option<ValidationError> {
        if !valid_values.contains(&attr_value) {
            return Some(
                ValidationError::new(
                    ValidationErrorType::Error,
                    format!(
                        "Invalid value '{}' for attribute '{}' in element '{}'",
                        attr_value, attr_name, tag_name
                    ),
                    format!("/{}/{}", tag_name, attr_name),
                )
            );
        }
        None
    }
}

impl Default for SchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic validator - validates business rules and constraints
pub struct SemanticValidator;

impl SemanticValidator {
    /// Create a new semantic validator
    pub fn new() -> Self {
        Self
    }

    /// Validate numeric range
    pub fn validate_range(
        value: i32,
        min: i32,
        max: i32,
        field_name: &str,
    ) -> Option<ValidationError> {
        if value < min || value > max {
            return Some(
                ValidationError::new(
                    ValidationErrorType::Error,
                    format!(
                        "Value {} for '{}' is outside valid range [{}, {}]",
                        value, field_name, min, max
                    ),
                    format!("/{}", field_name),
                )
            );
        }
        None
    }

    /// Validate required field
    pub fn validate_required(value: Option<&str>, field_name: &str) -> Option<ValidationError> {
        if value.is_none() || value.map(|v| v.is_empty()).unwrap_or(false) {
            return Some(
                ValidationError::new(
                    ValidationErrorType::Error,
                    format!("Required field '{}' is empty", field_name),
                    format!("/{}", field_name),
                )
            );
        }
        None
    }
}

impl Default for SemanticValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Document validator - validates document-level constraints
pub struct DocumentValidator;

impl DocumentValidator {
    /// Create a new document validator
    pub fn new() -> Self {
        Self
    }

    /// Validate slide count
    pub fn validate_slide_count(slide_count: usize) -> Option<ValidationError> {
        if slide_count == 0 {
            return Some(
                ValidationError::new(
                    ValidationErrorType::Warning,
                    "Document contains no slides".to_string(),
                    "/presentation/slides".to_string(),
                )
            );
        }
        None
    }

    /// Validate slide has content
    pub fn validate_slide_has_content(slide_index: usize, has_content: bool) -> Option<ValidationError> {
        if !has_content {
            return Some(
                ValidationError::new(
                    ValidationErrorType::Notice,
                    format!("Slide {} has no content", slide_index + 1),
                    format!("/presentation/slides/slide{}", slide_index + 1),
                )
            );
        }
        None
    }
}

impl Default for DocumentValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Package validator - validates package-level constraints
pub struct PackageValidator;

impl PackageValidator {
    /// Create a new package validator
    pub fn new() -> Self {
        Self
    }

    /// Validate package structure
    pub fn validate_required_parts(required_parts: &[&str], actual_parts: &[&str]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for required in required_parts {
            if !actual_parts.contains(required) {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::Error,
                        format!("Required part '{}' not found in package", required),
                        format!("/package/{}", required),
                    )
                );
            }
        }

        errors
    }

    /// Validate relationships
    pub fn validate_relationships(
        part_name: &str,
        required_rel_types: &[&str],
        actual_rel_types: &[&str],
    ) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for required in required_rel_types {
            if !actual_rel_types.contains(required) {
                errors.push(
                    ValidationError::new(
                        ValidationErrorType::Warning,
                        format!(
                            "Expected relationship type '{}' not found in part '{}'",
                            required, part_name
                        ),
                        format!("/package/{}/relationships", part_name),
                    )
                );
            }
        }

        errors
    }
}

impl Default for PackageValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new(
            ValidationErrorType::Error,
            "Test error".to_string(),
            "/test/path".to_string(),
        );
        assert_eq!(error.error_type, ValidationErrorType::Error);
        assert_eq!(error.description, "Test error");
        assert_eq!(error.path, "/test/path");
    }

    #[test]
    fn test_validation_error_with_location() {
        let error = ValidationError::with_location(
            ValidationErrorType::Warning,
            "Test warning".to_string(),
            "/test/path".to_string(),
            10,
            5,
        );
        assert_eq!(error.line, Some(10));
        assert_eq!(error.column, Some(5));
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new(
            ValidationErrorType::Error,
            "Test error".to_string(),
            "/test/path".to_string(),
        )
        .with_part_uri("/ppt/presentation.xml".to_string());
        
        let display = format!("{}", error);
        assert!(display.contains("Error"));
        assert!(display.contains("Test error"));
        assert!(display.contains("/ppt/presentation.xml"));
    }

    #[test]
    fn test_schema_validator_missing_attribute() {
        let errors = SchemaValidator::validate_element_structure(
            "slide",
            &["id", "name"],
            &["id"],
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].description.contains("name"));
    }

    #[test]
    fn test_schema_validator_invalid_attribute_value() {
        let error = SchemaValidator::validate_attribute_value(
            "shape",
            "type",
            "invalid",
            &["rectangle", "circle", "triangle"],
        );
        assert!(error.is_some());
        assert!(error.unwrap().description.contains("Invalid value"));
    }

    #[test]
    fn test_semantic_validator_range() {
        let error = SemanticValidator::validate_range(150, 0, 100, "opacity");
        assert!(error.is_some());
        assert!(error.unwrap().description.contains("outside valid range"));
    }

    #[test]
    fn test_semantic_validator_required() {
        let error = SemanticValidator::validate_required(None, "title");
        assert!(error.is_some());
        assert!(error.unwrap().description.contains("Required field"));
    }

    #[test]
    fn test_document_validator_no_slides() {
        let error = DocumentValidator::validate_slide_count(0);
        assert!(error.is_some());
        assert_eq!(error.unwrap().error_type, ValidationErrorType::Warning);
    }

    #[test]
    fn test_document_validator_empty_slide() {
        let error = DocumentValidator::validate_slide_has_content(0, false);
        assert!(error.is_some());
        assert_eq!(error.unwrap().error_type, ValidationErrorType::Notice);
    }

    #[test]
    fn test_package_validator_missing_part() {
        let errors = PackageValidator::validate_required_parts(
            &["presentation.xml", "theme.xml"],
            &["presentation.xml"],
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].description.contains("theme.xml"));
    }

    #[test]
    fn test_package_validator_relationships() {
        let errors = PackageValidator::validate_relationships(
            "presentation.xml",
            &["slide", "slideLayout"],
            &["slide"],
        );
        assert_eq!(errors.len(), 1);
        assert!(errors[0].description.contains("slideLayout"));
    }

    #[test]
    fn test_validation_error_type_display() {
        assert_eq!(format!("{}", ValidationErrorType::Error), "Error");
        assert_eq!(format!("{}", ValidationErrorType::Warning), "Warning");
        assert_eq!(format!("{}", ValidationErrorType::Notice), "Notice");
    }
}
