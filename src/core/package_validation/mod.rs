//! Systematic PPTX package validation for generated decks.
//!
//! Run [`validate_package_bytes`] after generation (or in tests) to catch
//! structural issues before opening in PowerPoint.

mod context;
mod rels;
mod report;
mod rules;

pub use report::{
    PackageValidationIssue, PackageValidationReport, ValidationCategory, ValidationSeverity,
};
pub use rules::{validate_package, validate_package_bytes, REQUIRED_PACKAGE_PARTS};
