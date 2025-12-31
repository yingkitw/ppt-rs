//! Utility functions and types for length conversions

/// Base class for length values in English Metric Units (EMUs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Length(i32);

impl Length {
    const EMUS_PER_INCH: i32 = 914400;
    const EMUS_PER_CENTIPOINT: i32 = 127;
    const EMUS_PER_CM: i32 = 360000;
    const EMUS_PER_MM: i32 = 36000;
    const EMUS_PER_PT: i32 = 12700;

    /// Create a Length from EMUs
    pub fn new(emu: i32) -> Self {
        Length(emu)
    }

    /// Get length in inches
    pub fn inches(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_INCH as f64
    }

    /// Get length in centipoints (hundredths of a point)
    pub fn centipoints(&self) -> i32 {
        self.0 / Self::EMUS_PER_CENTIPOINT
    }

    /// Get length in centimeters
    pub fn cm(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_CM as f64
    }

    /// Get length in EMUs
    pub fn emu(&self) -> i32 {
        self.0
    }

    /// Get length in millimeters
    pub fn mm(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_MM as f64
    }

    /// Get length in points
    pub fn pt(&self) -> f64 {
        self.0 as f64 / Self::EMUS_PER_PT as f64
    }
}

impl From<i32> for Length {
    fn from(emu: i32) -> Self {
        Length(emu)
    }
}

impl From<Length> for i32 {
    fn from(length: Length) -> Self {
        length.0
    }
}

/// Create a Length from inches
pub fn inches(value: f64) -> Length {
    Length((value * Length::EMUS_PER_INCH as f64) as i32)
}

/// Create a Length from centipoints
pub fn centipoints(value: i32) -> Length {
    Length(value * Length::EMUS_PER_CENTIPOINT)
}

/// Create a Length from centimeters
pub fn cm(value: f64) -> Length {
    Length((value * Length::EMUS_PER_CM as f64) as i32)
}

/// Create a Length from English Metric Units
pub fn emu(value: i32) -> Length {
    Length(value)
}

/// Create a Length from millimeters
pub fn mm(value: f64) -> Length {
    Length((value * Length::EMUS_PER_MM as f64) as i32)
}

/// Create a Length from points
pub fn pt(value: f64) -> Length {
    Length((value * Length::EMUS_PER_PT as f64) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_from_inches() {
        let len = inches(1.0);
        assert_eq!(len.emu(), 914400);
        assert_eq!(len.inches(), 1.0);
    }

    #[test]
    fn test_length_from_cm() {
        let len = cm(2.54);
        assert!((len.inches() - 1.0).abs() < 0.01);
        assert!((len.cm() - 2.54).abs() < 0.001);
    }

    #[test]
    fn test_length_from_mm() {
        let len = mm(25.4);
        assert!((len.inches() - 1.0).abs() < 0.01);
        assert!((len.mm() - 25.4).abs() < 0.01);
    }

    #[test]
    fn test_length_from_pt() {
        let len = pt(72.0);
        assert!((len.inches() - 1.0).abs() < 0.01);
        assert!((len.pt() - 72.0).abs() < 0.01);
    }

    #[test]
    fn test_length_from_emu() {
        let len = emu(914400);
        assert_eq!(len.emu(), 914400);
        assert_eq!(len.inches(), 1.0);
    }

    #[test]
    fn test_length_from_centipoints() {
        let len = centipoints(7200);
        assert_eq!(len.centipoints(), 7200);
        assert!((len.pt() - 72.0).abs() < 0.1);
    }

    #[test]
    fn test_length_new() {
        let len = Length::new(914400);
        assert_eq!(len.emu(), 914400);
    }

    #[test]
    fn test_length_from_i32() {
        let len: Length = 914400.into();
        assert_eq!(len.emu(), 914400);
    }

    #[test]
    fn test_i32_from_length() {
        let len = inches(1.0);
        let emu_val: i32 = len.into();
        assert_eq!(emu_val, 914400);
    }

    #[test]
    fn test_length_comparison() {
        let len1 = inches(1.0);
        let len2 = inches(2.0);
        assert!(len1 < len2);
        assert!(len2 > len1);
        assert_eq!(len1, inches(1.0));
    }

    #[test]
    fn test_length_clone() {
        let len1 = inches(1.0);
        let len2 = len1;
        assert_eq!(len1, len2);
    }

    #[test]
    fn test_zero_length() {
        let len = emu(0);
        assert_eq!(len.emu(), 0);
        assert_eq!(len.inches(), 0.0);
        assert_eq!(len.cm(), 0.0);
        assert_eq!(len.mm(), 0.0);
        assert_eq!(len.pt(), 0.0);
    }

    #[test]
    fn test_negative_length() {
        let len = emu(-914400);
        assert_eq!(len.emu(), -914400);
        assert_eq!(len.inches(), -1.0);
    }

    #[test]
    fn test_common_slide_dimensions() {
        // Standard slide width: 10 inches
        let width = inches(10.0);
        assert_eq!(width.emu(), 9144000);
        
        // Standard slide height: 7.5 inches
        let height = inches(7.5);
        assert_eq!(height.emu(), 6858000);
    }
}

/// Detects the system language and returns the appropriate language code
pub fn get_system_language() -> String {
    // Try to get the system locale from environment variables
    if let Ok(locale) = std::env::var("LANG") {
        // Parse locale string like "en_US.UTF-8" or "zh_CN.UTF-8"
        if locale.starts_with("zh_CN") {
            return "zh-CN".to_string();
        } else if locale.starts_with("en_US") {
            return "en-US".to_string();
        } else if locale.starts_with("en_GB") {
            return "en-GB".to_string();
        } else if locale.starts_with("es") {
            return "es-ES".to_string();
        } else if locale.starts_with("fr") {
            return "fr-FR".to_string();
        } else if locale.starts_with("de") {
            return "de-DE".to_string();
        } else if locale.starts_with("it") {
            return "it-IT".to_string();
        } else if locale.starts_with("pt") {
            return "pt-PT".to_string();
        } else if locale.starts_with("nl") {
            return "nl-NL".to_string();
        } else if locale.starts_with("ru") {
            return "ru-RU".to_string();
        } else if locale.starts_with("ja") {
            return "ja-JP".to_string();
        } else if locale.starts_with("ko") {
            return "ko-KR".to_string();
        }
    }

    // Try LC_ALL environment variable
    if let Ok(lc_all) = std::env::var("LC_ALL") {
        if lc_all.starts_with("zh_CN") {
            return "zh-CN".to_string();
        } else if lc_all.starts_with("en_US") {
            return "en-US".to_string();
        }
    }

    // Try Windows API on Windows systems
    #[cfg(all(windows, feature = "windows-lang"))]
    {
        if let Ok(lang) = get_windows_language() {
            return lang;
        }
    }

    // Default to en-US if we can't detect the language
    "en-US".to_string()
}

/// Get Windows system language using Windows API
#[cfg(all(windows, feature = "windows-lang"))]
fn get_windows_language() -> Result<String, ()> {
    use windows_sys::Win32::System::SystemInformation::*;
    
    unsafe {
        let mut buffer = [0u16; 256];
        let result = GetUserDefaultLocaleName(buffer.as_mut_ptr(), buffer.len() as i32);
        
        if result > 0 {
            let locale = String::from_utf16_lossy(&buffer[..result as usize - 1]);
            
            // Convert Windows locale format to XML language format
            match locale.as_str() {
                "zh-CN" => Ok("zh-CN".to_string()),
                "en-US" => Ok("en-US".to_string()),
                "en-GB" => Ok("en-GB".to_string()),
                "es-ES" => Ok("es-ES".to_string()),
                "fr-FR" => Ok("fr-FR".to_string()),
                "de-DE" => Ok("de-DE".to_string()),
                "it-IT" => Ok("it-IT".to_string()),
                "pt-PT" => Ok("pt-PT".to_string()),
                "pt-BR" => Ok("pt-BR".to_string()),
                "nl-NL" => Ok("nl-NL".to_string()),
                "ru-RU" => Ok("ru-RU".to_string()),
                "ja-JP" => Ok("ja-JP".to_string()),
                "ko-KR" => Ok("ko-KR".to_string()),
                _ => Ok("en-US".to_string()),
            }
        } else {
            Err(())
        }
    }
}

/// Get the alternative language (usually English) for XML altLang attribute
pub fn get_alt_language() -> String {
    let system_lang = get_system_language();
    
    // If system language is already English, return empty string
    if system_lang.starts_with("en") {
        String::new()
    } else {
        "en-US".to_string()
    }
}

/// Format language attributes for XML
pub fn format_lang_attributes() -> String {
    let lang = get_system_language();
    let alt_lang = get_alt_language();
    
    if alt_lang.is_empty() {
        format!(r#" lang="{}""#, lang)
    } else {
        format!(r#" lang="{}" altLang="{}""#, lang, alt_lang)
    }
}

/// Format language attributes for XML with additional attributes
pub fn format_lang_attributes_with(additional_attrs: &str) -> String {
    let lang = get_system_language();
    let alt_lang = get_alt_language();
    
    if alt_lang.is_empty() {
        format!(r#" lang="{}"{}"#, lang, additional_attrs)
    } else {
        format!(r#" lang="{}" altLang="{}"{}"#, lang, alt_lang, additional_attrs)
    }
}