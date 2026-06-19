//! Customizable presentation themes for PPTX output
//!
//! Maps semantic color roles and fonts into ECMA-376 `ppt/theme/theme1.xml`.

/// ECMA-376 color scheme (12 slots used by PowerPoint theme)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeColorScheme {
    /// Dark 1 — primary text
    pub dk1: String,
    /// Light 1 — primary background
    pub lt1: String,
    /// Dark 2 — secondary dark
    pub dk2: String,
    /// Light 2 — secondary light
    pub lt2: String,
    pub accent1: String,
    pub accent2: String,
    pub accent3: String,
    pub accent4: String,
    pub accent5: String,
    pub accent6: String,
    pub hlink: String,
    pub fol_hlink: String,
}

impl ThemeColorScheme {
    /// Default Microsoft Office color scheme
    pub fn office() -> Self {
        Self {
            dk1: "000000".into(),
            lt1: "FFFFFF".into(),
            dk2: "1F497D".into(),
            lt2: "EEECE1".into(),
            accent1: "4F81BD".into(),
            accent2: "C0504D".into(),
            accent3: "9BBB59".into(),
            accent4: "8064A2".into(),
            accent5: "4BACC6".into(),
            accent6: "F79646".into(),
            hlink: "0000FF".into(),
            fol_hlink: "800080".into(),
        }
    }

    /// Build a scheme from semantic palette roles (matches `prelude::themes::Theme`)
    pub fn from_palette(
        primary: &str,
        secondary: &str,
        accent: &str,
        background: &str,
        text: &str,
        light: &str,
        dark: &str,
    ) -> Self {
        let primary = normalize_hex(primary);
        let secondary = normalize_hex(secondary);
        let accent = normalize_hex(accent);
        Self {
            dk1: normalize_hex(text),
            lt1: normalize_hex(background),
            dk2: normalize_hex(dark),
            lt2: normalize_hex(light),
            accent1: primary.clone(),
            accent2: secondary,
            accent3: accent.clone(),
            accent4: primary.clone(),
            accent5: accent,
            accent6: primary.clone(),
            hlink: primary,
            fol_hlink: "954F72".into(),
        }
    }

    pub fn accent1(mut self, hex: impl AsRef<str>) -> Self {
        self.accent1 = normalize_hex(hex.as_ref());
        self
    }

    pub fn accent2(mut self, hex: impl AsRef<str>) -> Self {
        self.accent2 = normalize_hex(hex.as_ref());
        self
    }

    pub fn accent3(mut self, hex: impl AsRef<str>) -> Self {
        self.accent3 = normalize_hex(hex.as_ref());
        self
    }

    pub fn hyperlink(mut self, hex: impl AsRef<str>) -> Self {
        self.hlink = normalize_hex(hex.as_ref());
        self
    }
}

/// Theme font pair (major = headings, minor = body)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ThemeFonts {
    pub major: String,
    pub minor: String,
}

impl ThemeFonts {
    pub fn office() -> Self {
        Self {
            major: "Calibri Light".into(),
            minor: "Calibri".into(),
        }
    }

    pub fn new(major: impl Into<String>, minor: impl Into<String>) -> Self {
        Self {
            major: major.into(),
            minor: minor.into(),
        }
    }
}

/// Full presentation theme embedded in generated PPTX files
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresentationTheme {
    pub name: String,
    pub colors: ThemeColorScheme,
    pub fonts: ThemeFonts,
}

impl Default for PresentationTheme {
    fn default() -> Self {
        Self::office()
    }
}

impl PresentationTheme {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            colors: ThemeColorScheme::office(),
            fonts: ThemeFonts::office(),
        }
    }

    pub fn office() -> Self {
        Self::new("Office Theme")
    }

    pub fn colors(mut self, colors: ThemeColorScheme) -> Self {
        self.colors = colors;
        self
    }

    pub fn fonts(mut self, fonts: ThemeFonts) -> Self {
        self.fonts = fonts;
        self
    }

    pub fn major_font(mut self, typeface: impl Into<String>) -> Self {
        self.fonts.major = typeface.into();
        self
    }

    pub fn minor_font(mut self, typeface: impl Into<String>) -> Self {
        self.fonts.minor = typeface.into();
        self
    }

    pub fn from_palette(
        name: impl Into<String>,
        primary: &str,
        secondary: &str,
        accent: &str,
        background: &str,
        text: &str,
        light: &str,
        dark: &str,
    ) -> Self {
        Self {
            name: name.into(),
            colors: ThemeColorScheme::from_palette(
                primary, secondary, accent, background, text, light, dark,
            ),
            fonts: ThemeFonts::office(),
        }
    }

    pub fn corporate() -> Self {
        Self::from_palette(
            "Corporate",
            "1565C0", "1976D2", "FF6F00", "FFFFFF", "212121", "E3F2FD", "0D47A1",
        )
    }

    pub fn modern() -> Self {
        Self::from_palette(
            "Modern",
            "212121", "757575", "00BCD4", "FAFAFA", "212121", "F5F5F5", "424242",
        )
    }

    pub fn vibrant() -> Self {
        Self::from_palette(
            "Vibrant",
            "E91E63", "9C27B0", "FF9800", "FFFFFF", "212121", "FCE4EC", "880E4F",
        )
    }

    pub fn dark() -> Self {
        Self::from_palette(
            "Dark",
            "BB86FC", "03DAC6", "CF6679", "121212", "FFFFFF", "1E1E1E", "000000",
        )
    }

    pub fn nature() -> Self {
        Self::from_palette(
            "Nature",
            "2E7D32", "4CAF50", "8BC34A", "FFFFFF", "1B5E20", "E8F5E9", "1B5E20",
        )
    }

    pub fn tech() -> Self {
        Self::from_palette(
            "Tech",
            "0D47A1", "1976D2", "00E676", "FAFAFA", "263238", "E3F2FD", "01579B",
        )
    }

    pub fn carbon() -> Self {
        Self::from_palette(
            "Carbon",
            "0043CE", "4589FF", "24A148", "FFFFFF", "161616", "E0E0E0", "161616",
        )
    }

    /// Generate `ppt/theme/theme1.xml` content using the full Office theme template.
    pub fn to_theme_xml(&self) -> String {
        if self.name == "Office Theme" && self.colors == ThemeColorScheme::office() {
            return office_theme_xml().to_string();
        }

        let c = &self.colors;
        let color_slot = |tag: &str, hex: &str| {
            format!(r#"<a:{tag}><a:srgbClr val="{hex}"/></a:{tag}>"#)
        };
        let colors_xml = [
            color_slot("dk1", &c.dk1),
            color_slot("lt1", &c.lt1),
            color_slot("dk2", &c.dk2),
            color_slot("lt2", &c.lt2),
            color_slot("accent1", &c.accent1),
            color_slot("accent2", &c.accent2),
            color_slot("accent3", &c.accent3),
            color_slot("accent4", &c.accent4),
            color_slot("accent5", &c.accent5),
            color_slot("accent6", &c.accent6),
            color_slot("hlink", &c.hlink),
            color_slot("folHlink", &c.fol_hlink),
        ]
        .join("");
        let scheme_name = escape_xml_attr(&self.name);
        let clr_scheme = format!(r#"<a:clrScheme name="{scheme_name}">{colors_xml}</a:clrScheme>"#);

        let mut xml = office_theme_xml().to_string();
        if let (Some(start), Some(end)) = (
            xml.find("<a:clrScheme"),
            xml.find("</a:clrScheme>").map(|i| i + "</a:clrScheme>".len()),
        ) {
            xml.replace_range(start..end, &clr_scheme);
        }

        let theme_name = escape_xml_attr(&self.name);
        if let Some(start) = xml.find(r#"name=""#) {
            let name_start = start + 6;
            if let Some(end) = xml[name_start..].find('"') {
                xml.replace_range(name_start..name_start + end, &theme_name);
            }
        }

        replace_font_latin(&mut xml, "majorFont", &self.fonts.major);
        replace_font_latin(&mut xml, "minorFont", &self.fonts.minor);

        xml
    }
}

fn replace_font_latin(xml: &mut String, font_tag: &str, typeface: &str) {
    const LATIN_PREFIX: &str = r#"<a:latin typeface=""#;
    let marker = format!("<a:{font_tag}>");
    let Some(start) = xml.find(&marker) else {
        return;
    };
    let section = &xml[start..];
    let Some(rel) = section.find(LATIN_PREFIX) else {
        return;
    };
    let abs = start + rel + LATIN_PREFIX.len();
    if let Some(end) = xml[abs..].find('"') {
        let escaped = escape_xml_attr(typeface);
        xml.replace_range(abs..abs + end, &escaped);
    }
}

/// Full Office theme XML extracted from a PowerPoint-compatible reference file.
pub fn office_theme_xml() -> &'static str {
    include_str!("office_theme.xml")
}

fn normalize_hex(hex: &str) -> String {
    hex.trim().trim_start_matches('#').to_uppercase()
}

fn escape_xml_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corporate_theme_xml_contains_colors() {
        let theme = PresentationTheme::corporate().major_font("Arial").minor_font("Arial");
        let xml = theme.to_theme_xml();
        assert!(xml.contains("1565C0"));
        assert!(xml.contains("FF6F00"));
        assert!(xml.contains(r#"name="Corporate""#));
        assert!(xml.contains(r#"typeface="Arial""#));
    }

    #[test]
    fn test_dark_theme_background() {
        let theme = PresentationTheme::dark();
        let xml = theme.to_theme_xml();
        assert!(xml.contains("121212"));
        assert!(xml.contains("FFFFFF"));
    }

    #[test]
    fn test_custom_accent_override() {
        let colors = ThemeColorScheme::office().accent1("AABBCC");
        let theme = PresentationTheme::new("Custom").colors(colors);
        let xml = theme.to_theme_xml();
        assert!(xml.contains("AABBCC"));
    }

    #[test]
    fn test_normalize_hex_strips_hash() {
        assert_eq!(normalize_hex("#ff8040"), "FF8040");
    }
}
