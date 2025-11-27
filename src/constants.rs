//! Global constants - no hardcoding

/// Version information
pub mod version {
    pub const MAJOR: u32 = 1;
    pub const MINOR: u32 = 0;
    pub const PATCH: u32 = 2;
    pub const VERSION_STRING: &str = "1.0.2";
}

/// Presentation defaults
pub mod presentation {
    pub const DEFAULT_TITLE: &str = "Presentation";
    pub const DEFAULT_SLIDES: usize = 1;
    pub const DEFAULT_WIDTH: i32 = 9144000;  // EMU
    pub const DEFAULT_HEIGHT: i32 = 6858000; // EMU
}

/// Slide defaults
pub mod slide {
    pub const DEFAULT_SLIDE_ID: i32 = 255;
    pub const SLIDE_WIDTH: i32 = 9144000;   // EMU
    pub const SLIDE_HEIGHT: i32 = 6858000;  // EMU
}

/// File system defaults
pub mod filesystem {
    pub const DEFAULT_OUTPUT_DIR: &str = "examples/output";
    pub const DEFAULT_EXTENSION: &str = "pptx";
    pub const DEFAULT_FILENAME: &str = "presentation";
}

/// XML defaults
pub mod xml {
    pub const XML_VERSION: &str = "1.0";
    pub const XML_ENCODING: &str = "UTF-8";
    pub const XML_STANDALONE: &str = "yes";
}

/// Theme defaults
pub mod theme {
    pub const DEFAULT_THEME_NAME: &str = "Office Theme";
    pub const DEFAULT_THEME_FILE: &str = "theme1";
    pub const DEFAULT_COLOR_SCHEME: &str = "Office";
    pub const DEFAULT_FONT_SCHEME: &str = "Office";
}

/// Layout defaults
pub mod layout {
    pub const DEFAULT_LAYOUT_FILE: &str = "slideLayout1";
    pub const DEFAULT_MASTER_FILE: &str = "slideMaster1";
}

/// Color defaults
pub mod colors {
    pub const WHITE: &str = "FFFFFF";
    pub const BLACK: &str = "000000";
    pub const DARK_BLUE: &str = "1F497D";
    pub const LIGHT_GRAY: &str = "EBEBEB";
    pub const ACCENT_BLUE: &str = "4472C4";
    pub const ACCENT_ORANGE: &str = "ED7D31";
}

/// Font defaults
pub mod fonts {
    pub const DEFAULT_FONT: &str = "Calibri";
    pub const MAJOR_FONT: &str = "Calibri";
    pub const MINOR_FONT: &str = "Calibri";
}

/// CLI defaults
pub mod cli {
    pub const APP_NAME: &str = "pptcli";
    pub const AUTHOR: &str = "PPTX Generator";
    pub const DEFAULT_SLIDES_CLI: usize = 1;
}

/// Size formatting
pub mod sizes {
    pub const BYTES_PER_KB: usize = 1024;
    pub const BYTES_PER_MB: usize = 1024 * 1024;
}

/// Relationship IDs
pub mod relationships {
    pub const PRESENTATION_REL_ID: &str = "rId1";
    pub const CORE_PROPS_REL_ID: &str = "rId2";
    pub const APP_PROPS_REL_ID: &str = "rId3";
    pub const SLIDE_MASTER_REL_ID: &str = "rId1";
    pub const THEME_REL_ID: &str = "rId2";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version::VERSION_STRING, "1.0.2");
    }

    #[test]
    fn test_presentation_defaults() {
        assert_eq!(presentation::DEFAULT_TITLE, "Presentation");
        assert_eq!(presentation::DEFAULT_SLIDES, 1);
    }

    #[test]
    fn test_colors() {
        assert_eq!(colors::WHITE, "FFFFFF");
        assert_eq!(colors::BLACK, "000000");
    }
}
