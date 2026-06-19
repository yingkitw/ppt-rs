//! Structural compatibility checks aligned with PowerPoint package expectations.
//!
//! This module is a thin wrapper around [`crate::core::package_validation`] for
//! backward compatibility. Prefer [`validate_package_bytes`] for new code.

use std::io::{Read, Seek};

use zip::ZipArchive;

pub use crate::core::package_validation::{validate_package, PackageValidationReport};

/// Result of a PowerPoint structural compatibility scan (legacy).
#[derive(Debug, Default)]
pub struct CompatReport {
    pub issues: Vec<String>,
}

impl CompatReport {
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn push(&mut self, issue: impl Into<String>) {
        self.issues.push(issue.into());
    }
}

impl From<PackageValidationReport> for CompatReport {
    fn from(report: PackageValidationReport) -> Self {
        Self {
            issues: report.error_messages(),
        }
    }
}

/// Validate structural compatibility of a PPTX ZIP archive.
pub fn validate_powerpoint_structure<R: Read + Seek>(
    archive: &mut ZipArchive<R>,
) -> CompatReport {
    validate_package(archive).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::create_pptx;

    #[test]
    fn minimal_deck_passes_compat_gate() {
        let bytes = create_pptx("Compat", 1).unwrap();
        let cursor = std::io::Cursor::new(bytes);
        let mut archive = ZipArchive::new(cursor).unwrap();
        let report = validate_powerpoint_structure(&mut archive);
        assert!(report.is_ok(), "issues: {:?}", report.issues);
    }

    #[test]
    fn compat_report_matches_package_validation() {
        use crate::validate_package_bytes;
        let bytes = create_pptx("Compat", 3).unwrap();
        let package_report = validate_package_bytes(&bytes);
        let compat: CompatReport = package_report.clone().into();
        assert_eq!(compat.issues, package_report.error_messages());
    }
}
