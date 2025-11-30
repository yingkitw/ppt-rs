//! Web page fetcher for Web2PPT

use super::{Web2PptError, Result, Web2PptConfig};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL};
use std::time::Duration;

/// Fetches web pages
pub struct WebFetcher {
    client: Client,
    config: Web2PptConfig,
}

impl WebFetcher {
    /// Create a new web fetcher with default config
    pub fn new() -> Result<Self> {
        Self::with_config(Web2PptConfig::default())
    }

    /// Create a new web fetcher with custom config
    pub fn with_config(config: Web2PptConfig) -> Result<Self> {
        // Build headers to look like a real browser
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .timeout(Duration::from_secs(config.timeout_secs))
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .map_err(|e| Web2PptError::FetchError(e.to_string()))?;

        Ok(WebFetcher { client, config })
    }

    /// Fetch HTML content from a URL
    pub fn fetch(&self, url: &str) -> Result<String> {
        // Validate URL
        let parsed_url = url::Url::parse(url)
            .map_err(|e| Web2PptError::InvalidUrl(e.to_string()))?;

        // Only allow http/https
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err(Web2PptError::InvalidUrl(
                "Only HTTP and HTTPS URLs are supported".to_string()
            ));
        }

        // Fetch the page with referer header
        let response = self.client
            .get(url)
            .header("Referer", url)
            .send()
            .map_err(|e| Web2PptError::FetchError(e.to_string()))?;

        // Check status
        if !response.status().is_success() {
            return Err(Web2PptError::FetchError(
                format!("HTTP {}: {}", response.status().as_u16(), response.status().as_str())
            ));
        }

        // Get text content
        response.text()
            .map_err(|e| Web2PptError::FetchError(e.to_string()))
    }

    /// Fetch and return both URL and HTML
    pub fn fetch_with_url(&self, url: &str) -> Result<(String, String)> {
        let html = self.fetch(url)?;
        Ok((url.to_string(), html))
    }

    /// Get the config
    pub fn config(&self) -> &Web2PptConfig {
        &self.config
    }
}

impl Default for WebFetcher {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebFetcher")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url() {
        let fetcher = WebFetcher::new().unwrap();
        let result = fetcher.fetch("not-a-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_scheme() {
        let fetcher = WebFetcher::new().unwrap();
        let result = fetcher.fetch("ftp://example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_config() {
        let config = Web2PptConfig::new().timeout(60);
        let fetcher = WebFetcher::with_config(config).unwrap();
        assert_eq!(fetcher.config().timeout_secs, 60);
    }
}
