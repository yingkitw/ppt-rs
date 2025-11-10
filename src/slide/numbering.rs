//! Slide Numbering Support
//!
//! This module provides support for slide numbering in presentations.
//! Slide numbers can be added to slides using placeholders or custom shapes.

use crate::error::Result;

/// Slide numbering format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberingFormat {
    /// Arabic numerals (1, 2, 3, ...)
    Arabic,
    /// Roman numerals uppercase (I, II, III, ...)
    RomanUpper,
    /// Roman numerals lowercase (i, ii, iii, ...)
    RomanLower,
    /// Alphabetic uppercase (A, B, C, ...)
    AlphaUpper,
    /// Alphabetic lowercase (a, b, c, ...)
    AlphaLower,
}

impl NumberingFormat {
    /// Convert a slide number to formatted string
    pub fn format(&self, slide_number: usize) -> String {
        match self {
            NumberingFormat::Arabic => slide_number.to_string(),
            NumberingFormat::RomanUpper => Self::to_roman_upper(slide_number),
            NumberingFormat::RomanLower => Self::to_roman_lower(slide_number),
            NumberingFormat::AlphaUpper => Self::to_alpha_upper(slide_number),
            NumberingFormat::AlphaLower => Self::to_alpha_lower(slide_number),
        }
    }

    fn to_roman_upper(num: usize) -> String {
        Self::to_roman(num, true)
    }

    fn to_roman_lower(num: usize) -> String {
        Self::to_roman(num, false)
    }

    fn to_roman(mut num: usize, uppercase: bool) -> String {
        let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let numerals_upper = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let numerals_lower = ["m", "cm", "d", "cd", "c", "xc", "l", "xl", "x", "ix", "v", "iv", "i"];
        let numerals = if uppercase { numerals_upper } else { numerals_lower };

        let mut result = String::new();
        for (i, &value) in values.iter().enumerate() {
            while num >= value {
                result.push_str(numerals[i]);
                num -= value;
            }
        }
        result
    }

    fn to_alpha_upper(num: usize) -> String {
        Self::to_alpha(num, true)
    }

    fn to_alpha_lower(num: usize) -> String {
        Self::to_alpha(num, false)
    }

    fn to_alpha(mut num: usize, uppercase: bool) -> String {
        if num == 0 {
            return String::new();
        }

        let mut result = String::new();
        let base = if uppercase { b'A' } else { b'a' };

        while num > 0 {
            num -= 1;
            result.insert(0, (base + (num % 26) as u8) as char);
            num /= 26;
        }

        result
    }
}

/// Slide numbering configuration
#[derive(Debug, Clone)]
pub struct SlideNumbering {
    /// Enable slide numbering
    enabled: bool,
    /// Numbering format
    format: NumberingFormat,
    /// Starting number
    start_number: usize,
    /// Include in footer
    include_in_footer: bool,
    /// Custom prefix
    prefix: Option<String>,
    /// Custom suffix
    suffix: Option<String>,
}

impl Default for SlideNumbering {
    fn default() -> Self {
        Self::new()
    }
}

impl SlideNumbering {
    /// Create a new slide numbering configuration
    pub fn new() -> Self {
        Self {
            enabled: false,
            format: NumberingFormat::Arabic,
            start_number: 1,
            include_in_footer: false,
            prefix: None,
            suffix: None,
        }
    }

    /// Enable slide numbering
    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    /// Disable slide numbering
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Check if slide numbering is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Set the numbering format
    pub fn set_format(mut self, format: NumberingFormat) -> Self {
        self.format = format;
        self
    }

    /// Get the numbering format
    pub fn format(&self) -> NumberingFormat {
        self.format
    }

    /// Set the starting number
    pub fn set_start_number(mut self, start: usize) -> Self {
        self.start_number = start;
        self
    }

    /// Get the starting number
    pub fn start_number(&self) -> usize {
        self.start_number
    }

    /// Set whether to include in footer
    pub fn set_include_in_footer(mut self, include: bool) -> Self {
        self.include_in_footer = include;
        self
    }

    /// Check if included in footer
    pub fn include_in_footer(&self) -> bool {
        self.include_in_footer
    }

    /// Set a custom prefix
    pub fn set_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Get the prefix
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Set a custom suffix
    pub fn set_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Get the suffix
    pub fn suffix(&self) -> Option<&str> {
        self.suffix.as_deref()
    }

    /// Format a slide number with the current configuration
    pub fn format_slide_number(&self, slide_number: usize) -> String {
        let mut result = String::new();

        if let Some(prefix) = &self.prefix {
            result.push_str(prefix);
        }

        result.push_str(&self.format.format(slide_number));

        if let Some(suffix) = &self.suffix {
            result.push_str(suffix);
        }

        result
    }

    /// Generate XML for slide number placeholder
    pub fn to_xml(&self, slide_number: usize) -> Result<String> {
        if !self.enabled {
            return Ok(String::new());
        }

        let formatted = self.format_slide_number(slide_number);
        let xml = format!(
            r#"<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Slide Number"/>
    <p:cNvSpPr/>
    <p:nvPr>
      <p:ph type="sldNum"/>
    </p:nvPr>
  </p:nvSpPr>
  <p:spPr/>
  <p:txBody>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p>
      <a:r>
        <a:rPr lang="en-US" dirty="0" smtClean="0"/>
        <a:t>{}</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>"#,
            formatted
        );

        Ok(xml)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_format() {
        let format = NumberingFormat::Arabic;
        assert_eq!(format.format(1), "1");
        assert_eq!(format.format(10), "10");
        assert_eq!(format.format(100), "100");
    }

    #[test]
    fn test_roman_upper_format() {
        let format = NumberingFormat::RomanUpper;
        assert_eq!(format.format(1), "I");
        assert_eq!(format.format(4), "IV");
        assert_eq!(format.format(9), "IX");
        assert_eq!(format.format(27), "XXVII");
        assert_eq!(format.format(49), "XLIX");
    }

    #[test]
    fn test_roman_lower_format() {
        let format = NumberingFormat::RomanLower;
        assert_eq!(format.format(1), "i");
        assert_eq!(format.format(4), "iv");
        assert_eq!(format.format(9), "ix");
        assert_eq!(format.format(27), "xxvii");
    }

    #[test]
    fn test_alpha_upper_format() {
        let format = NumberingFormat::AlphaUpper;
        assert_eq!(format.format(1), "A");
        assert_eq!(format.format(26), "Z");
        assert_eq!(format.format(27), "AA");
    }

    #[test]
    fn test_alpha_lower_format() {
        let format = NumberingFormat::AlphaLower;
        assert_eq!(format.format(1), "a");
        assert_eq!(format.format(26), "z");
        assert_eq!(format.format(27), "aa");
    }

    #[test]
    fn test_slide_numbering_default() {
        let numbering = SlideNumbering::new();
        assert!(!numbering.is_enabled());
        assert_eq!(numbering.format(), NumberingFormat::Arabic);
        assert_eq!(numbering.start_number(), 1);
        assert!(!numbering.include_in_footer());
    }

    #[test]
    fn test_slide_numbering_enable() {
        let numbering = SlideNumbering::new().enable();
        assert!(numbering.is_enabled());
    }

    #[test]
    fn test_slide_numbering_format_slide_number() {
        let numbering = SlideNumbering::new()
            .enable()
            .set_format(NumberingFormat::Arabic)
            .set_prefix("Slide ")
            .set_suffix(" of 10");

        assert_eq!(numbering.format_slide_number(1), "Slide 1 of 10");
        assert_eq!(numbering.format_slide_number(5), "Slide 5 of 10");
    }

    #[test]
    fn test_slide_numbering_roman_with_prefix() {
        let numbering = SlideNumbering::new()
            .enable()
            .set_format(NumberingFormat::RomanUpper)
            .set_prefix("Page ");

        assert_eq!(numbering.format_slide_number(1), "Page I");
        assert_eq!(numbering.format_slide_number(5), "Page V");
    }

    #[test]
    fn test_slide_numbering_to_xml() {
        let numbering = SlideNumbering::new()
            .enable()
            .set_format(NumberingFormat::Arabic);

        let xml = numbering.to_xml(1).unwrap();
        assert!(xml.contains("Slide Number"));
        assert!(xml.contains("sldNum"));
        assert!(xml.contains("<a:t>1</a:t>"));
    }

    #[test]
    fn test_slide_numbering_disabled_to_xml() {
        let numbering = SlideNumbering::new();
        let xml = numbering.to_xml(1).unwrap();
        assert!(xml.is_empty());
    }

    #[test]
    fn test_slide_numbering_footer_option() {
        let numbering = SlideNumbering::new()
            .enable()
            .set_include_in_footer(true);

        assert!(numbering.include_in_footer());
    }
}
