//! Error context for better error messages

use crate::error::{PptError, Result};

/// Error context trait for adding context to errors
pub trait ErrorContext<T> {
    /// Add context to an error
    fn context(self, context: &str) -> Result<T>;
}

impl<T> ErrorContext<T> for Result<T> {
    fn context(self, context: &str) -> Result<T> {
        self.map_err(|e| {
            match e {
                PptError::ValueError(msg) => {
                    PptError::ValueError(format!("{}: {}", context, msg))
                }
                PptError::Xml(msg) => {
                    PptError::Xml(format!("{}: {}", context, msg))
                }
                PptError::NotImplemented(msg) => {
                    PptError::NotImplemented(format!("{}: {}", context, msg))
                }
                PptError::InvalidPackage(msg) => {
                    PptError::InvalidPackage(format!("{}: {}", context, msg))
                }
                other => other,
            }
        })
    }
}

/// Validation helper
pub struct Validator;

impl Validator {
    /// Validate that a value is not empty
    pub fn not_empty(value: &str, field_name: &str) -> Result<()> {
        if value.is_empty() {
            Err(PptError::ValueError(format!("{} cannot be empty", field_name)))
        } else {
            Ok(())
        }
    }

    /// Validate that a value is within range
    pub fn in_range(value: u32, min: u32, max: u32, field_name: &str) -> Result<()> {
        if value < min || value > max {
            Err(PptError::ValueError(format!(
                "{} must be between {} and {}, got {}",
                field_name, min, max, value
            )))
        } else {
            Ok(())
        }
    }

    /// Validate that a value is positive
    pub fn positive(value: u32, field_name: &str) -> Result<()> {
        if value == 0 {
            Err(PptError::ValueError(format!(
                "{} must be positive, got {}",
                field_name, value
            )))
        } else {
            Ok(())
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_context() {
        let result: Result<()> = Err(PptError::ValueError("test error".to_string()));
        let contexted = result.context("operation failed");
        
        match contexted {
            Err(PptError::ValueError(msg)) => {
                assert!(msg.contains("operation failed"));
                assert!(msg.contains("test error"));
            }
            _ => panic!("Expected ValueError"),
        }
    }

    #[test]
    fn test_validator_not_empty() {
        assert!(Validator::not_empty("value", "field").is_ok());
        assert!(Validator::not_empty("", "field").is_err());
    }

    #[test]
    fn test_validator_in_range() {
        assert!(Validator::in_range(50, 0, 100, "value").is_ok());
        assert!(Validator::in_range(150, 0, 100, "value").is_err());
    }

    #[test]
    fn test_validator_positive() {
        assert!(Validator::positive(1, "value").is_ok());
        assert!(Validator::positive(0, "value").is_err());
    }

}
