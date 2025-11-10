//! Package URI handling

use crate::error::{PptError, Result};
use std::path::Path;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref FILENAME_RE: Regex = Regex::new(r"([a-zA-Z]+)([0-9][0-9]*)?").unwrap();
}

/// Package URI - represents a part URI within a package
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackURI {
    uri: String,
}

impl PackURI {
    /// Create a new PackURI from a string
    pub fn new(uri: &str) -> Result<Self> {
        if !uri.starts_with('/') {
            return Err(PptError::ValueError(format!(
                "PackURI must begin with slash, got '{}'",
                uri
            )));
        }
        Ok(Self {
            uri: uri.to_string(),
        })
    }

    /// Get the URI as a string
    pub fn as_str(&self) -> &str {
        &self.uri
    }

    /// Create a PackURI from a relative reference
    pub fn from_rel_ref(relative_ref: &str, base: &PackURI) -> Result<Self> {
        let base_uri = base.base_uri();
        
        // Handle relative references with ../
        let mut parts: Vec<&str> = base_uri.split('/').filter(|s| !s.is_empty()).collect();
        let rel_parts: Vec<&str> = relative_ref.split('/').collect();
        
        for part in rel_parts {
            match part {
                ".." => {
                    if !parts.is_empty() {
                        parts.pop();
                    }
                }
                "." => {
                    // Current directory, do nothing
                }
                "" => {
                    // Empty part, skip
                }
                _ => {
                    parts.push(part);
                }
            }
        }
        
        let normalized = if parts.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", parts.join("/"))
        };
        
        Self::new(&normalized)
    }

    /// Get the base URI (directory portion)
    pub fn base_uri(&self) -> &str {
        if self.uri == "/" {
            return "/";
        }
        if let Some(pos) = self.uri.rfind('/') {
            if pos == 0 {
                "/"
            } else {
                &self.uri[..pos]
            }
        } else {
            "/"
        }
    }

    /// Get the file extension
    pub fn ext(&self) -> &str {
        if let Some(pos) = self.uri.rfind('.') {
            if let Some(slash_pos) = self.uri.rfind('/') {
                if pos > slash_pos {
                    &self.uri[pos + 1..]
                } else {
                    ""
                }
            } else {
                &self.uri[pos + 1..]
            }
        } else {
            ""
        }
    }

    /// Get the filename portion
    pub fn filename(&self) -> &str {
        if self.uri == "/" {
            return "";
        }
        if let Some(pos) = self.uri.rfind('/') {
            &self.uri[pos + 1..]
        } else {
            &self.uri
        }
    }

    /// Get the index from the filename (e.g., 21 from "slide21.xml")
    pub fn idx(&self) -> Option<u32> {
        let filename = self.filename();
        if filename.is_empty() {
            return None;
        }
        let name_part = filename.split('.').next().unwrap_or("");
        if let Some(captures) = FILENAME_RE.captures(name_part) {
            if let Some(num_str) = captures.get(2) {
                return num_str.as_str().parse().ok();
            }
        }
        None
    }

    /// Get the membername (URI without leading slash)
    pub fn membername(&self) -> &str {
        if self.uri == "/" {
            ""
        } else {
            &self.uri[1..]
        }
    }

    /// Get relative reference from base URI
    pub fn relative_ref(&self, base_uri: &str) -> String {
        if base_uri == "/" {
            return self.membername().to_string();
        }
        // Simple implementation - can be improved
        let base_path = Path::new(base_uri);
        let self_path = Path::new(&self.uri);
        self_path.strip_prefix(base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| self.membername().to_string())
    }

    /// Get the relationships URI for this part
    pub fn rels_uri(&self) -> Result<Self> {
        let filename = self.filename();
        if filename.is_empty() {
            return Self::new("/_rels/.rels");
        }
        let rels_filename = format!("{}.rels", filename);
        let base = self.base_uri();
        let rels_uri = if base == "/" {
            format!("/_rels/{}", rels_filename)
        } else {
            format!("{}/_rels/{}", base, rels_filename)
        };
        Self::new(&rels_uri)
    }
}

impl std::fmt::Display for PackURI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

/// Content types URI
pub const CONTENT_TYPES_URI: &str = "/[Content_Types].xml";

/// Package relationships URI
pub const PACKAGE_URI: &str = "/";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packuri_new() {
        let uri = PackURI::new("/ppt/presentation.xml").unwrap();
        assert_eq!(uri.as_str(), "/ppt/presentation.xml");
    }

    #[test]
    fn test_packuri_new_invalid() {
        assert!(PackURI::new("foobar").is_err());
    }

    #[test]
    fn test_packuri_from_rel_ref() {
        let base = PackURI::new("/ppt/slides/slide1.xml").unwrap();
        let rel = PackURI::from_rel_ref("slideLayout1.xml", &base).unwrap();
        assert_eq!(rel.as_str(), "/ppt/slides/slideLayout1.xml");
    }

    #[test]
    fn test_packuri_base_uri() {
        assert_eq!(PackURI::new("/").unwrap().base_uri(), "/");
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().base_uri(), "/ppt");
        assert_eq!(PackURI::new("/ppt/slides/slide1.xml").unwrap().base_uri(), "/ppt/slides");
    }

    #[test]
    fn test_packuri_ext() {
        assert_eq!(PackURI::new("/").unwrap().ext(), "");
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().ext(), "xml");
        assert_eq!(PackURI::new("/ppt/media/image.PnG").unwrap().ext(), "PnG");
    }

    #[test]
    fn test_packuri_filename() {
        assert_eq!(PackURI::new("/").unwrap().filename(), "");
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().filename(), "presentation.xml");
        assert_eq!(PackURI::new("/ppt/media/image.png").unwrap().filename(), "image.png");
    }

    #[test]
    fn test_packuri_idx() {
        assert_eq!(PackURI::new("/").unwrap().idx(), None);
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().idx(), None);
        assert_eq!(PackURI::new("/ppt/media/image42.png").unwrap().idx(), Some(42));
    }

    #[test]
    fn test_packuri_membername() {
        assert_eq!(PackURI::new("/").unwrap().membername(), "");
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().membername(), "ppt/presentation.xml");
    }

    #[test]
    fn test_packuri_rels_uri() {
        assert_eq!(PackURI::new("/").unwrap().rels_uri().unwrap().as_str(), "/_rels/.rels");
        assert_eq!(PackURI::new("/ppt/presentation.xml").unwrap().rels_uri().unwrap().as_str(), "/ppt/_rels/presentation.xml.rels");
        assert_eq!(PackURI::new("/ppt/slides/slide42.xml").unwrap().rels_uri().unwrap().as_str(), "/ppt/slides/_rels/slide42.xml.rels");
    }
}

