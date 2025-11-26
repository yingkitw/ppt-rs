//! Configuration module - centralized settings

/// Default configuration values
pub struct Config {
    /// Default slide count
    pub default_slides: usize,
    /// Default presentation title
    pub default_title: String,
    /// Default output directory
    pub default_output_dir: String,
    /// Default file extension
    pub default_extension: String,
}

impl Config {
    /// Create default configuration
    pub fn default() -> Self {
        Config {
            default_slides: 1,
            default_title: "Presentation".to_string(),
            default_output_dir: "output".to_string(),
            default_extension: "pptx".to_string(),
        }
    }

    /// Create custom configuration
    pub fn new(
        default_slides: usize,
        default_title: &str,
        default_output_dir: &str,
        default_extension: &str,
    ) -> Self {
        Config {
            default_slides,
            default_title: default_title.to_string(),
            default_output_dir: default_output_dir.to_string(),
            default_extension: default_extension.to_string(),
        }
    }

    /// Get full output path
    pub fn output_path(&self, filename: &str) -> String {
        format!("{}/{}.{}", self.default_output_dir, filename, self.default_extension)
    }
}

/// CLI configuration
pub struct CliConfig {
    pub app_name: String,
    pub version: String,
    pub author: String,
}

impl CliConfig {
    /// Create CLI configuration
    pub fn default() -> Self {
        CliConfig {
            app_name: "pptx-cli".to_string(),
            version: "1.0.2".to_string(),
            author: "PPTX Generator".to_string(),
        }
    }
}

/// Generator configuration
pub struct GeneratorConfig {
    pub default_theme: String,
    pub default_layout: String,
    pub default_master: String,
}

impl GeneratorConfig {
    /// Create generator configuration
    pub fn default() -> Self {
        GeneratorConfig {
            default_theme: "theme1".to_string(),
            default_layout: "slideLayout1".to_string(),
            default_master: "slideMaster1".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_slides, 1);
        assert_eq!(config.default_title, "Presentation");
    }

    #[test]
    fn test_custom_config() {
        let config = Config::new(5, "Custom", "out", "pptx");
        assert_eq!(config.default_slides, 5);
        assert_eq!(config.default_title, "Custom");
    }

    #[test]
    fn test_output_path() {
        let config = Config::default();
        let path = config.output_path("test");
        assert_eq!(path, "output/test.pptx");
    }
}
