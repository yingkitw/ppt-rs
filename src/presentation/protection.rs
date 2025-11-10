//! Document Protection - Password protection and editing restrictions


/// Document protection settings
#[derive(Clone, Debug)]
pub struct DocumentProtection {
    /// Edit password hash (optional)
    edit_password_hash: Option<String>,
    /// View password hash (optional)
    view_password_hash: Option<String>,
    /// Lock structure (prevent adding/removing slides)
    lock_structure: bool,
    /// Lock windows (prevent resizing)
    lock_windows: bool,
    /// Require password to modify
    require_password_to_modify: bool,
}

impl DocumentProtection {
    /// Create new document protection settings
    pub fn new() -> Self {
        Self {
            edit_password_hash: None,
            view_password_hash: None,
            lock_structure: false,
            lock_windows: false,
            require_password_to_modify: false,
        }
    }

    /// Set edit password (hashed)
    pub fn set_edit_password(&mut self, hash: String) {
        self.edit_password_hash = Some(hash);
    }

    /// Get edit password hash
    pub fn edit_password_hash(&self) -> Option<&str> {
        self.edit_password_hash.as_deref()
    }

    /// Set view password (hashed)
    pub fn set_view_password(&mut self, hash: String) {
        self.view_password_hash = Some(hash);
    }

    /// Get view password hash
    pub fn view_password_hash(&self) -> Option<&str> {
        self.view_password_hash.as_deref()
    }

    /// Set lock structure
    pub fn set_lock_structure(&mut self, lock: bool) {
        self.lock_structure = lock;
    }

    /// Get lock structure
    pub fn lock_structure(&self) -> bool {
        self.lock_structure
    }

    /// Set lock windows
    pub fn set_lock_windows(&mut self, lock: bool) {
        self.lock_windows = lock;
    }

    /// Get lock windows
    pub fn lock_windows(&self) -> bool {
        self.lock_windows
    }

    /// Set require password to modify
    pub fn set_require_password_to_modify(&mut self, require: bool) {
        self.require_password_to_modify = require;
    }

    /// Get require password to modify
    pub fn require_password_to_modify(&self) -> bool {
        self.require_password_to_modify
    }

    /// Check if any protection is enabled
    pub fn is_protected(&self) -> bool {
        self.edit_password_hash.is_some()
            || self.view_password_hash.is_some()
            || self.lock_structure
            || self.lock_windows
    }

    /// Generate XML for document protection
    pub fn to_xml(&self) -> String {
        if !self.is_protected() {
            return String::new();
        }

        let mut xml = String::from("<p:protection");

        if let Some(hash) = &self.edit_password_hash {
            xml.push_str(&format!(r#" editId="{}""#, hash));
        }

        if let Some(hash) = &self.view_password_hash {
            xml.push_str(&format!(r#" viewId="{}""#, hash));
        }

        if self.lock_structure {
            xml.push_str(r#" lockStructure="1""#);
        }

        if self.lock_windows {
            xml.push_str(r#" lockWindows="1""#);
        }

        xml.push_str("/>");
        xml
    }
}

impl Default for DocumentProtection {
    fn default() -> Self {
        Self::new()
    }
}

/// Editing restrictions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditingRestriction {
    /// No restrictions
    None,
    /// Read-only
    ReadOnly,
    /// Comments only
    CommentsOnly,
    /// Tracked changes only
    TrackedChangesOnly,
}

impl EditingRestriction {
    /// Get restriction string
    pub fn as_str(&self) -> &str {
        match self {
            EditingRestriction::None => "none",
            EditingRestriction::ReadOnly => "readOnly",
            EditingRestriction::CommentsOnly => "commentsOnly",
            EditingRestriction::TrackedChangesOnly => "trackedChangesOnly",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_protection_creation() {
        let protection = DocumentProtection::new();
        assert!(!protection.is_protected());
        assert!(!protection.lock_structure());
        assert!(!protection.lock_windows());
    }

    #[test]
    fn test_document_protection_edit_password() {
        let mut protection = DocumentProtection::new();
        protection.set_edit_password("ABC123".to_string());
        assert_eq!(protection.edit_password_hash(), Some("ABC123"));
        assert!(protection.is_protected());
    }

    #[test]
    fn test_document_protection_view_password() {
        let mut protection = DocumentProtection::new();
        protection.set_view_password("XYZ789".to_string());
        assert_eq!(protection.view_password_hash(), Some("XYZ789"));
        assert!(protection.is_protected());
    }

    #[test]
    fn test_document_protection_lock_structure() {
        let mut protection = DocumentProtection::new();
        protection.set_lock_structure(true);
        assert!(protection.lock_structure());
        assert!(protection.is_protected());
    }

    #[test]
    fn test_document_protection_lock_windows() {
        let mut protection = DocumentProtection::new();
        protection.set_lock_windows(true);
        assert!(protection.lock_windows());
        assert!(protection.is_protected());
    }

    #[test]
    fn test_document_protection_require_password() {
        let mut protection = DocumentProtection::new();
        protection.set_require_password_to_modify(true);
        assert!(protection.require_password_to_modify());
    }

    #[test]
    fn test_document_protection_xml_empty() {
        let protection = DocumentProtection::new();
        assert_eq!(protection.to_xml(), "");
    }

    #[test]
    fn test_document_protection_xml_with_edit_password() {
        let mut protection = DocumentProtection::new();
        protection.set_edit_password("ABC123".to_string());
        let xml = protection.to_xml();
        assert!(xml.contains("<p:protection"));
        assert!(xml.contains(r#"editId="ABC123""#));
        assert!(xml.contains("/>"));
    }

    #[test]
    fn test_document_protection_xml_with_locks() {
        let mut protection = DocumentProtection::new();
        protection.set_lock_structure(true);
        protection.set_lock_windows(true);
        let xml = protection.to_xml();
        assert!(xml.contains(r#"lockStructure="1""#));
        assert!(xml.contains(r#"lockWindows="1""#));
    }

    #[test]
    fn test_editing_restriction_as_str() {
        assert_eq!(EditingRestriction::None.as_str(), "none");
        assert_eq!(EditingRestriction::ReadOnly.as_str(), "readOnly");
        assert_eq!(EditingRestriction::CommentsOnly.as_str(), "commentsOnly");
        assert_eq!(EditingRestriction::TrackedChangesOnly.as_str(), "trackedChangesOnly");
    }

    #[test]
    fn test_editing_restriction_equality() {
        assert_eq!(EditingRestriction::ReadOnly, EditingRestriction::ReadOnly);
        assert_ne!(EditingRestriction::ReadOnly, EditingRestriction::CommentsOnly);
    }
}
