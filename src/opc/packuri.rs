//! Package URI handling

/// Represents a URI within a package
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackUri {
    uri: String,
}

impl PackUri {
    /// Create a new PackUri
    pub fn new(uri: &str) -> Self {
        PackUri {
            uri: uri.to_string(),
        }
    }

    /// Get the URI as a string
    pub fn as_str(&self) -> &str {
        &self.uri
    }

    /// Get the base URI (directory part)
    pub fn base_uri(&self) -> PackUri {
        if let Some(pos) = self.uri.rfind('/') {
            PackUri {
                uri: self.uri[..=pos].to_string(),
            }
        } else {
            PackUri {
                uri: "/".to_string(),
            }
        }
    }

    /// Get the filename part
    pub fn filename(&self) -> &str {
        if let Some(pos) = self.uri.rfind('/') {
            &self.uri[pos + 1..]
        } else {
            &self.uri
        }
    }

    /// Resolve a relative URI against this URI
    pub fn resolve(&self, relative: &str) -> PackUri {
        let base = self.base_uri();
        PackUri {
            uri: format!("{}{}", base.uri, relative),
        }
    }
}

impl std::fmt::Display for PackUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl From<&str> for PackUri {
    fn from(uri: &str) -> Self {
        PackUri::new(uri)
    }
}

impl From<String> for PackUri {
    fn from(uri: String) -> Self {
        PackUri { uri }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packuri_creation() {
        let uri = PackUri::new("/ppt/slides/slide1.xml");
        assert_eq!(uri.as_str(), "/ppt/slides/slide1.xml");
    }

    #[test]
    fn test_packuri_filename() {
        let uri = PackUri::new("/ppt/slides/slide1.xml");
        assert_eq!(uri.filename(), "slide1.xml");
    }

    #[test]
    fn test_packuri_base_uri() {
        let uri = PackUri::new("/ppt/slides/slide1.xml");
        assert_eq!(uri.base_uri().as_str(), "/ppt/slides/");
    }

    #[test]
    fn test_packuri_resolve() {
        let uri = PackUri::new("/ppt/slides/slide1.xml");
        let resolved = uri.resolve("../theme/theme1.xml");
        assert_eq!(resolved.as_str(), "/ppt/slides/../theme/theme1.xml");
    }
}
