//! Shared helpers for integration tests.

use std::io::Cursor;

use ppt_rs::core::{validate_package_bytes, PackageValidationReport, ValidationCategory};
use zip::ZipArchive;

/// Assert that PPTX bytes pass all package validation rules.
pub fn assert_package_valid(bytes: &[u8], label: &str) {
    let report = validate_package_bytes(bytes);
    assert_package_report_valid(&report, label);
}

/// Assert that an on-disk PPTX passes all package validation rules.
pub fn assert_package_file_valid(path: &std::path::Path, label: &str) {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("{label}: read {}: {e}", path.display()));
    assert_package_valid(&bytes, label);
}

/// Assert a validation report has no errors (warnings are allowed).
pub fn assert_package_report_valid(report: &PackageValidationReport, label: &str) {
    if report.is_valid() {
        return;
    }
    let errors: Vec<_> = report
        .issues
        .iter()
        .filter(|i| i.severity == ppt_rs::core::ValidationSeverity::Error)
        .map(|i| format!("{:?}: {}", i.category, i.message))
        .collect();
    panic!("{label} failed package validation:\n  - {}", errors.join("\n  - "));
}

/// List part paths in a PPTX ZIP (test debugging helper).
pub fn list_parts(bytes: &[u8]) -> Vec<String> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor).expect("valid zip");
    let mut names = Vec::new();
    for i in 0..archive.len() {
        if let Ok(f) = archive.by_index(i) {
            if !f.is_dir() {
                names.push(f.name().to_string());
            }
        }
    }
    names.sort();
    names
}

/// Return issues in a specific category (for focused rule tests).
pub fn issues_in_category<'a>(
    report: &'a PackageValidationReport,
    category: ValidationCategory,
) -> Vec<&'a ppt_rs::core::PackageValidationIssue> {
    report.issues_in_category(category).collect()
}
