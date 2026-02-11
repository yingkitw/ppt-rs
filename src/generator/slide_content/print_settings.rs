//! Print settings and handout configuration for presentations
//!
//! Controls print layout, handout options, and page setup.
//! Generates `<p:prnPr>` and handout master XML.

/// Handout layout (slides per page)
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum HandoutLayout {
    SlidesPerPage1,
    SlidesPerPage2,
    #[default]
    SlidesPerPage3,
    SlidesPerPage4,
    SlidesPerPage6,
    SlidesPerPage9,
}

impl HandoutLayout {
    pub fn slides_per_page(&self) -> u32 {
        match self {
            HandoutLayout::SlidesPerPage1 => 1,
            HandoutLayout::SlidesPerPage2 => 2,
            HandoutLayout::SlidesPerPage3 => 3,
            HandoutLayout::SlidesPerPage4 => 4,
            HandoutLayout::SlidesPerPage6 => 6,
            HandoutLayout::SlidesPerPage9 => 9,
        }
    }

    pub fn to_xml_value(&self) -> &'static str {
        match self {
            HandoutLayout::SlidesPerPage1 => "handout1",
            HandoutLayout::SlidesPerPage2 => "handout2",
            HandoutLayout::SlidesPerPage3 => "handout3",
            HandoutLayout::SlidesPerPage4 => "handout4",
            HandoutLayout::SlidesPerPage6 => "handout6",
            HandoutLayout::SlidesPerPage9 => "handout9",
        }
    }
}

/// Print color mode
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum PrintColorMode {
    #[default]
    Color,
    Grayscale,
    BlackAndWhite,
}

impl PrintColorMode {
    pub fn to_xml_value(&self) -> &'static str {
        match self {
            PrintColorMode::Color => "clr",
            PrintColorMode::Grayscale => "gray",
            PrintColorMode::BlackAndWhite => "bw",
        }
    }
}

/// What to print
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum PrintWhat {
    #[default]
    Slides,
    Handouts,
    Notes,
    Outline,
}

impl PrintWhat {
    pub fn to_xml_value(&self) -> &'static str {
        match self {
            PrintWhat::Slides => "slides",
            PrintWhat::Handouts => "handouts",
            PrintWhat::Notes => "notes",
            PrintWhat::Outline => "outline",
        }
    }
}

/// Page orientation
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Landscape,
    Portrait,
}

impl Orientation {
    pub fn to_xml_value(&self) -> &'static str {
        match self {
            Orientation::Landscape => "landscape",
            Orientation::Portrait => "portrait",
        }
    }
}

/// Print settings for the presentation
#[derive(Clone, Debug, Default)]
pub struct PrintSettings {
    pub print_what: PrintWhat,
    pub color_mode: PrintColorMode,
    pub handout_layout: HandoutLayout,
    pub frame_slides: bool,
    pub scale_to_fit: bool,
    pub include_hidden_slides: bool,
    pub orientation: Orientation,
    pub header: Option<String>,
    pub footer: Option<String>,
    pub print_date: bool,
    pub print_page_numbers: bool,
}

impl PrintSettings {
    pub fn new() -> Self {
        Self {
            scale_to_fit: true,
            ..Default::default()
        }
    }

    pub fn print_what(mut self, what: PrintWhat) -> Self {
        self.print_what = what;
        self
    }

    pub fn color_mode(mut self, mode: PrintColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    pub fn handout_layout(mut self, layout: HandoutLayout) -> Self {
        self.handout_layout = layout;
        self
    }

    pub fn frame_slides(mut self, frame: bool) -> Self {
        self.frame_slides = frame;
        self
    }

    pub fn scale_to_fit(mut self, scale: bool) -> Self {
        self.scale_to_fit = scale;
        self
    }

    pub fn include_hidden_slides(mut self, include: bool) -> Self {
        self.include_hidden_slides = include;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn header(mut self, header: &str) -> Self {
        self.header = Some(header.to_string());
        self
    }

    pub fn footer(mut self, footer: &str) -> Self {
        self.footer = Some(footer.to_string());
        self
    }

    pub fn print_date(mut self, val: bool) -> Self {
        self.print_date = val;
        self
    }

    pub fn print_page_numbers(mut self, val: bool) -> Self {
        self.print_page_numbers = val;
        self
    }

    /// Generate `<p:prnPr>` XML for presProps.xml
    pub fn to_prnpr_xml(&self) -> String {
        let mut attrs = Vec::new();
        attrs.push(format!(r#"prnWhat="{}""#, self.print_what.to_xml_value()));
        attrs.push(format!(r#"clrMode="{}""#, self.color_mode.to_xml_value()));

        if self.frame_slides {
            attrs.push("frameSlides=\"1\"".to_string());
        }
        if self.include_hidden_slides {
            attrs.push("hiddenSlides=\"1\"".to_string());
        }
        if self.scale_to_fit {
            attrs.push("scaleToFitPaper=\"1\"".to_string());
        }

        format!(r#"<p:prnPr {}/>"#, attrs.join(" "))
    }

    /// Generate handout master XML
    pub fn to_handout_master_xml(&self) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
        );
        xml.push_str(
            r#"<p:handoutMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
        );
        xml.push_str("<p:cSld><p:spTree>");
        xml.push_str(r#"<p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr>"#);
        xml.push_str("<p:grpSpPr/>");

        // Header placeholder
        if let Some(ref header) = self.header {
            xml.push_str(&format!(
                r#"<p:sp><p:nvSpPr><p:cNvPr id="2" name="Header"/><p:cNvSpPr/><p:nvPr><p:ph type="hdr"/></p:nvPr></p:nvSpPr><p:spPr/><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>{}</a:t></a:r></a:p></p:txBody></p:sp>"#,
                xml_escape(header)
            ));
        }

        // Footer placeholder
        if let Some(ref footer) = self.footer {
            xml.push_str(&format!(
                r#"<p:sp><p:nvSpPr><p:cNvPr id="3" name="Footer"/><p:cNvSpPr/><p:nvPr><p:ph type="ftr"/></p:nvPr></p:nvSpPr><p:spPr/><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:t>{}</a:t></a:r></a:p></p:txBody></p:sp>"#,
                xml_escape(footer)
            ));
        }

        // Date placeholder
        if self.print_date {
            xml.push_str(
                r#"<p:sp><p:nvSpPr><p:cNvPr id="4" name="Date"/><p:cNvSpPr/><p:nvPr><p:ph type="dt"/></p:nvPr></p:nvSpPr><p:spPr/><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:fld id="{B6F15528-F159-4107-2D14-000000000000}" type="datetimeFigureOut"><a:t/></a:fld></a:p></p:txBody></p:sp>"#,
            );
        }

        // Page number placeholder
        if self.print_page_numbers {
            xml.push_str(
                r#"<p:sp><p:nvSpPr><p:cNvPr id="5" name="Slide Number"/><p:cNvSpPr/><p:nvPr><p:ph type="sldNum"/></p:nvPr></p:nvSpPr><p:spPr/><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:fld id="{B6F15528-F159-4107-2D14-000000000001}" type="slidenum"><a:t/></a:fld></a:p></p:txBody></p:sp>"#,
            );
        }

        xml.push_str("</p:spTree></p:cSld>");
        xml.push_str("<p:clrMap bg1=\"lt1\" tx1=\"dk1\" bg2=\"lt2\" tx2=\"dk2\" accent1=\"accent1\" accent2=\"accent2\" accent3=\"accent3\" accent4=\"accent4\" accent5=\"accent5\" accent6=\"accent6\" hlink=\"hlink\" folHlink=\"folHlink\"/>");
        xml.push_str("</p:handoutMaster>");
        xml
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handout_layout_default() {
        assert_eq!(HandoutLayout::default(), HandoutLayout::SlidesPerPage3);
        assert_eq!(HandoutLayout::SlidesPerPage3.slides_per_page(), 3);
    }

    #[test]
    fn test_handout_layout_variants() {
        assert_eq!(HandoutLayout::SlidesPerPage1.slides_per_page(), 1);
        assert_eq!(HandoutLayout::SlidesPerPage2.slides_per_page(), 2);
        assert_eq!(HandoutLayout::SlidesPerPage4.slides_per_page(), 4);
        assert_eq!(HandoutLayout::SlidesPerPage6.slides_per_page(), 6);
        assert_eq!(HandoutLayout::SlidesPerPage9.slides_per_page(), 9);
    }

    #[test]
    fn test_handout_layout_xml() {
        assert_eq!(HandoutLayout::SlidesPerPage1.to_xml_value(), "handout1");
        assert_eq!(HandoutLayout::SlidesPerPage6.to_xml_value(), "handout6");
    }

    #[test]
    fn test_print_color_mode() {
        assert_eq!(PrintColorMode::default(), PrintColorMode::Color);
        assert_eq!(PrintColorMode::Color.to_xml_value(), "clr");
        assert_eq!(PrintColorMode::Grayscale.to_xml_value(), "gray");
        assert_eq!(PrintColorMode::BlackAndWhite.to_xml_value(), "bw");
    }

    #[test]
    fn test_print_what() {
        assert_eq!(PrintWhat::default(), PrintWhat::Slides);
        assert_eq!(PrintWhat::Slides.to_xml_value(), "slides");
        assert_eq!(PrintWhat::Handouts.to_xml_value(), "handouts");
        assert_eq!(PrintWhat::Notes.to_xml_value(), "notes");
        assert_eq!(PrintWhat::Outline.to_xml_value(), "outline");
    }

    #[test]
    fn test_orientation() {
        assert_eq!(Orientation::default(), Orientation::Landscape);
        assert_eq!(Orientation::Landscape.to_xml_value(), "landscape");
        assert_eq!(Orientation::Portrait.to_xml_value(), "portrait");
    }

    #[test]
    fn test_print_settings_new() {
        let s = PrintSettings::new();
        assert_eq!(s.print_what, PrintWhat::Slides);
        assert_eq!(s.color_mode, PrintColorMode::Color);
        assert!(s.scale_to_fit);
        assert!(!s.frame_slides);
    }

    #[test]
    fn test_print_settings_builder() {
        let s = PrintSettings::new()
            .print_what(PrintWhat::Handouts)
            .color_mode(PrintColorMode::Grayscale)
            .handout_layout(HandoutLayout::SlidesPerPage6)
            .frame_slides(true)
            .scale_to_fit(false)
            .include_hidden_slides(true)
            .orientation(Orientation::Portrait)
            .header("My Header")
            .footer("Page Footer")
            .print_date(true)
            .print_page_numbers(true);
        assert_eq!(s.print_what, PrintWhat::Handouts);
        assert_eq!(s.color_mode, PrintColorMode::Grayscale);
        assert_eq!(s.handout_layout, HandoutLayout::SlidesPerPage6);
        assert!(s.frame_slides);
        assert!(!s.scale_to_fit);
        assert!(s.include_hidden_slides);
        assert_eq!(s.header.as_deref(), Some("My Header"));
        assert_eq!(s.footer.as_deref(), Some("Page Footer"));
        assert!(s.print_date);
        assert!(s.print_page_numbers);
    }

    #[test]
    fn test_prnpr_xml() {
        let s = PrintSettings::new();
        let xml = s.to_prnpr_xml();
        assert!(xml.contains("<p:prnPr"));
        assert!(xml.contains(r#"prnWhat="slides""#));
        assert!(xml.contains(r#"clrMode="clr""#));
    }

    #[test]
    fn test_prnpr_xml_handouts() {
        let s = PrintSettings::new()
            .print_what(PrintWhat::Handouts)
            .color_mode(PrintColorMode::BlackAndWhite)
            .frame_slides(true);
        let xml = s.to_prnpr_xml();
        assert!(xml.contains(r#"prnWhat="handouts""#));
        assert!(xml.contains(r#"clrMode="bw""#));
        assert!(xml.contains("frameSlides=\"1\""));
    }

    #[test]
    fn test_handout_master_xml_basic() {
        let s = PrintSettings::new();
        let xml = s.to_handout_master_xml();
        assert!(xml.contains("<p:handoutMaster"));
        assert!(xml.contains("</p:handoutMaster>"));
        assert!(xml.contains("<p:spTree>"));
    }

    #[test]
    fn test_handout_master_xml_with_header_footer() {
        let s = PrintSettings::new()
            .header("Report")
            .footer("Confidential")
            .print_date(true)
            .print_page_numbers(true);
        let xml = s.to_handout_master_xml();
        assert!(xml.contains("Report"));
        assert!(xml.contains("Confidential"));
        assert!(xml.contains(r#"type="hdr""#));
        assert!(xml.contains(r#"type="ftr""#));
        assert!(xml.contains(r#"type="dt""#));
        assert!(xml.contains(r#"type="sldNum""#));
    }

    #[test]
    fn test_prnpr_xml_wellformed() {
        // Regression: raw string r#"frameSlides="1"# ate the closing quote
        let s = PrintSettings::new()
            .print_what(PrintWhat::Handouts)
            .color_mode(PrintColorMode::Grayscale)
            .frame_slides(true)
            .include_hidden_slides(true)
            .scale_to_fit(true);
        let xml = s.to_prnpr_xml();
        // Every attribute must have properly closed quotes
        assert!(xml.contains("frameSlides=\"1\""));
        assert!(xml.contains("hiddenSlides=\"1\""));
        assert!(xml.contains("scaleToFitPaper=\"1\""));
        // Must be self-closing and end with />
        assert!(xml.ends_with("/>"));
        // Wrap in root to validate XML well-formedness
        let wrapped = format!("<root {}/>", &xml[9..xml.len()-2]);
        assert!(!wrapped.contains("=\"1 "), "attribute value missing closing quote");
    }

    #[test]
    fn test_handout_master_xml_escaping() {
        let s = PrintSettings::new().header("Q&A <Report>");
        let xml = s.to_handout_master_xml();
        assert!(xml.contains("Q&amp;A &lt;Report&gt;"));
    }
}
