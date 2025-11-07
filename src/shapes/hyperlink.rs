//! Hyperlink functionality for shapes and text

use regex::Regex;

/// Hyperlink - represents a hyperlink on a shape or text run
pub struct Hyperlink {
    address: Option<String>,
    screen_tip: Option<String>,
}

impl Hyperlink {
    /// Create a new hyperlink
    pub fn new() -> Self {
        Self {
            address: None,
            screen_tip: None,
        }
    }
    
    /// Create a hyperlink with an address
    pub fn with_address(address: String) -> Self {
        Self {
            address: Some(address),
            screen_tip: None,
        }
    }
    
    /// Get the hyperlink address (URL)
    pub fn address(&self) -> Option<&str> {
        self.address.as_deref()
    }
    
    /// Set the hyperlink address
    pub fn set_address(&mut self, address: Option<String>) {
        self.address = address;
    }
    
    /// Get the screen tip (tooltip text)
    pub fn screen_tip(&self) -> Option<&str> {
        self.screen_tip.as_deref()
    }
    
    /// Set the screen tip
    pub fn set_screen_tip(&mut self, tip: Option<String>) {
        self.screen_tip = tip;
    }
    
    /// Check if hyperlink has an address
    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }
}

impl Default for Hyperlink {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate hyperlink XML for shape
pub fn hyperlink_to_xml(hyperlink: &Hyperlink, r_id: &str) -> String {
    if let Some(ref address) = hyperlink.address {
        let screen_tip_attr = hyperlink.screen_tip
            .as_ref()
            .map(|tip| format!(r#" tooltip="{}"#, escape_xml(tip)))
            .unwrap_or_default();
        format!(r#"<a:hlinkClick r:id="{}"{} />"#, r_id, screen_tip_attr)
    } else {
        String::new()
    }
}

/// Parse hyperlink from XML
pub fn parse_hyperlink_from_xml(xml: &str) -> Option<(String, Option<String>)> {
    // Look for <a:hlinkClick r:id="..." tooltip="..."/>
    let hlink_re = regex::Regex::new(r#"<a:hlinkClick\s+r:id="([^"]+)"(?:\s+tooltip="([^"]+)")?"#).ok()?;
    hlink_re.captures(xml).map(|cap| {
        let r_id = cap.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
        let tooltip = cap.get(2).map(|m| unescape_xml(m.as_str()).to_string());
        (r_id, tooltip)
    })
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn unescape_xml(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyperlink_new() {
        let hlink = Hyperlink::new();
        assert!(!hlink.has_address());
        assert_eq!(hlink.address(), None);
        assert_eq!(hlink.screen_tip(), None);
    }

    #[test]
    fn test_hyperlink_with_address() {
        let hlink = Hyperlink::with_address("https://example.com".to_string());
        assert!(hlink.has_address());
        assert_eq!(hlink.address(), Some("https://example.com"));
    }

    #[test]
    fn test_hyperlink_set_address() {
        let mut hlink = Hyperlink::new();
        hlink.set_address(Some("https://rust-lang.org".to_string()));
        assert_eq!(hlink.address(), Some("https://rust-lang.org"));
        
        hlink.set_address(None);
        assert!(!hlink.has_address());
    }

    #[test]
    fn test_hyperlink_screen_tip() {
        let mut hlink = Hyperlink::new();
        hlink.set_screen_tip(Some("Click here".to_string()));
        assert_eq!(hlink.screen_tip(), Some("Click here"));
        
        hlink.set_screen_tip(None);
        assert_eq!(hlink.screen_tip(), None);
    }

    #[test]
    fn test_hyperlink_to_xml() {
        let mut hlink = Hyperlink::with_address("https://example.com".to_string());
        let xml = hyperlink_to_xml(&hlink, "rId1");
        assert!(xml.contains("r:id=\"rId1\""));
        assert!(xml.contains("a:hlinkClick"));
        
        hlink.set_screen_tip(Some("Example Site".to_string()));
        let xml_with_tip = hyperlink_to_xml(&hlink, "rId2");
        assert!(xml_with_tip.contains("tooltip="));
        assert!(xml_with_tip.contains("Example Site"));
    }

    #[test]
    fn test_hyperlink_to_xml_no_address() {
        let hlink = Hyperlink::new();
        let xml = hyperlink_to_xml(&hlink, "rId1");
        assert_eq!(xml, "");
    }

    #[test]
    fn test_parse_hyperlink_from_xml() {
        let xml = r#"<a:hlinkClick r:id="rId1" />"#;
        let result = parse_hyperlink_from_xml(xml);
        assert!(result.is_some());
        let (r_id, tooltip) = result.unwrap();
        assert_eq!(r_id, "rId1");
        assert_eq!(tooltip, None);
    }

    #[test]
    fn test_parse_hyperlink_from_xml_with_tooltip() {
        let xml = r#"<a:hlinkClick r:id="rId2" tooltip="Click me" />"#;
        let result = parse_hyperlink_from_xml(xml);
        assert!(result.is_some());
        let (r_id, tooltip) = result.unwrap();
        assert_eq!(r_id, "rId2");
        assert_eq!(tooltip, Some("Click me".to_string()));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a&b"), "a&amp;b");
        assert_eq!(escape_xml("a<b"), "a&lt;b");
        assert_eq!(escape_xml("a>b"), "a&gt;b");
        assert_eq!(escape_xml("a\"b"), "a&quot;b");
        assert_eq!(escape_xml("a'b"), "a&apos;b");
    }

    #[test]
    fn test_unescape_xml() {
        assert_eq!(unescape_xml("a&amp;b"), "a&b");
        assert_eq!(unescape_xml("a&lt;b"), "a<b");
        assert_eq!(unescape_xml("a&gt;b"), "a>b");
        assert_eq!(unescape_xml("a&quot;b"), "a\"b");
        assert_eq!(unescape_xml("a&apos;b"), "a'b");
    }
}

