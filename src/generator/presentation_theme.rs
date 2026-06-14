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

    /// Generate `ppt/theme/theme1.xml` content
    pub fn to_theme_xml(&self) -> String {
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
        .join("\n");

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" name="{name}">
<a:themeElements>
<a:clrScheme name="{scheme_name}">
{colors_xml}
</a:clrScheme>
<a:fontScheme name="Office">
<a:majorFont>
<a:latin typeface="{major}"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:majorFont>
<a:minorFont>
<a:latin typeface="{minor}"/>
<a:ea typeface=""/>
<a:cs typeface=""/>
</a:minorFont>
</a:fontScheme>
<a:fmtScheme name="Office">
<a:fillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="50000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="35000"><a:schemeClr val="phClr"><a:tint val="37000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:tint val="15000"/><a:satMod val="350000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="1"/></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:shade val="51000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="80000"><a:schemeClr val="phClr"><a:shade val="93000"/><a:satMod val="130000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="94000"/><a:satMod val="135000"/></a:schemeClr></a:gs></a:gsLst><a:lin ang="16200000" scaled="0"/></a:gradFill>
</a:fillStyleLst>
<a:lnStyleLst>
<a:ln w="9525" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"><a:shade val="95000"/><a:satMod val="105000"/></a:schemeClr></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="25400" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
<a:ln w="38100" cap="flat" cmpd="sng" algn="ctr"><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:prstDash val="solid"/></a:ln>
</a:lnStyleLst>
<a:effectStyleLst>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
<a:effectStyle><a:effectLst/></a:effectStyle>
</a:effectStyleLst>
<a:bgFillStyleLst>
<a:solidFill><a:schemeClr val="phClr"/></a:solidFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="40000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="40000"><a:schemeClr val="phClr"><a:tint val="45000"/><a:shade val="99000"/><a:satMod val="350000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="20000"/><a:satMod val="255000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="-80000" r="50000" b="180000"/></a:path></a:gradFill>
<a:gradFill rotWithShape="1"><a:gsLst><a:gs pos="0"><a:schemeClr val="phClr"><a:tint val="80000"/><a:satMod val="300000"/></a:schemeClr></a:gs><a:gs pos="100000"><a:schemeClr val="phClr"><a:shade val="30000"/><a:satMod val="200000"/></a:schemeClr></a:gs></a:gsLst><a:path path="circle"><a:fillToRect l="50000" t="50000" r="50000" b="50000"/></a:path></a:gradFill>
</a:bgFillStyleLst>
</a:fmtScheme>
</a:themeElements>
<a:objectDefaults/>
<a:extraClrSchemeLst/>
</a:theme>"#,
            name = escape_xml_attr(&self.name),
            scheme_name = escape_xml_attr(&self.name),
            colors_xml = colors_xml,
            major = escape_xml_attr(&self.fonts.major),
            minor = escape_xml_attr(&self.fonts.minor),
        )
    }
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
