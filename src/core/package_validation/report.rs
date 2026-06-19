//! Structured validation report types.

/// Severity of a package validation finding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

/// Category grouping for validation findings (used in tests and filtering).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValidationCategory {
    MissingPart,
    Relationship,
    ContentType,
    Presentation,
    SlideMaster,
    Slide,
    Chart,
    Xml,
    Theme,
}

/// A single structural issue found in a PPTX package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageValidationIssue {
    pub category: ValidationCategory,
    pub severity: ValidationSeverity,
    pub message: String,
    /// Part path most relevant to the issue, when applicable.
    pub path: Option<String>,
}

impl PackageValidationIssue {
    pub fn error(
        category: ValidationCategory,
        message: impl Into<String>,
        path: Option<&str>,
    ) -> Self {
        Self {
            category,
            severity: ValidationSeverity::Error,
            message: message.into(),
            path: path.map(str::to_string),
        }
    }

    pub fn warning(
        category: ValidationCategory,
        message: impl Into<String>,
        path: Option<&str>,
    ) -> Self {
        Self {
            category,
            severity: ValidationSeverity::Warning,
            message: message.into(),
            path: path.map(str::to_string),
        }
    }
}

/// Aggregated result of running all package validation rules.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PackageValidationReport {
    pub issues: Vec<PackageValidationIssue>,
}

impl PackageValidationReport {
    pub fn is_valid(&self) -> bool {
        self.issues
            .iter()
            .all(|i| i.severity != ValidationSeverity::Error)
    }

    pub fn is_ok(&self) -> bool {
        self.is_valid()
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .count()
    }

    pub fn push(&mut self, issue: PackageValidationIssue) {
        self.issues.push(issue);
    }

    pub fn extend(&mut self, issues: impl IntoIterator<Item = PackageValidationIssue>) {
        self.issues.extend(issues);
    }

    /// Flat string list of error messages (compat with legacy `CompatReport`).
    pub fn error_messages(&self) -> Vec<String> {
        self.issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .map(|i| i.message.clone())
            .collect()
    }

    pub fn issues_in_category(
        &self,
        category: ValidationCategory,
    ) -> impl Iterator<Item = &PackageValidationIssue> {
        self.issues
            .iter()
            .filter(move |i| i.category == category)
    }
}
