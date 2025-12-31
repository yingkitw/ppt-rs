//! Text XML elements for OOXML
//!
//! Provides types for parsing and generating DrawingML text elements.

use super::xmlchemy::XmlElement;
use crate::util::format_lang_attributes;

/// Text body properties (a:bodyPr)
#[derive(Debug, Clone, Default)]
pub struct BodyProperties {
    pub wrap: Option<String>,
    pub anchor: Option<String>,
    pub anchor_ctr: bool,
    pub rtl_col: bool,
    pub left_inset: Option<u32>,
    pub right_inset: Option<u32>,
    pub top_inset: Option<u32>,
    pub bottom_inset: Option<u32>,
}

impl BodyProperties {
    pub fn parse(elem: &XmlElement) -> Self {
        BodyProperties {
            wrap: elem.attr("wrap").map(|s| s.to_string()),
            anchor: elem.attr("anchor").map(|s| s.to_string()),
            anchor_ctr: elem.attr("anchorCtr").map(|v| v == "1").unwrap_or(false),
            rtl_col: elem.attr("rtlCol").map(|v| v == "1").unwrap_or(false),
            left_inset: elem.attr("lIns").and_then(|v| v.parse().ok()),
            right_inset: elem.attr("rIns").and_then(|v| v.parse().ok()),
            top_inset: elem.attr("tIns").and_then(|v| v.parse().ok()),
            bottom_inset: elem.attr("bIns").and_then(|v| v.parse().ok()),
        }
    }

    pub fn to_xml(&self) -> String {
        let mut attrs = Vec::new();
        
        if let Some(ref wrap) = self.wrap {
            attrs.push(format!(r#"wrap="{wrap}""#));
        }
        if let Some(ref anchor) = self.anchor {
            attrs.push(format!(r#"anchor="{anchor}""#));
        }
        if self.rtl_col {
            attrs.push(r#"rtlCol="1""#.to_string());
        }
        if let Some(l) = self.left_inset {
            attrs.push(format!(r#"lIns="{l}""#));
        }
        if let Some(r) = self.right_inset {
            attrs.push(format!(r#"rIns="{r}""#));
        }
        if let Some(t) = self.top_inset {
            attrs.push(format!(r#"tIns="{t}""#));
        }
        if let Some(b) = self.bottom_inset {
            attrs.push(format!(r#"bIns="{b}""#));
        }

        if attrs.is_empty() {
            "<a:bodyPr/>".to_string()
        } else {
            format!("<a:bodyPr {}/>", attrs.join(" "))
        }
    }
}

/// Paragraph properties (a:pPr)
#[derive(Debug, Clone, Default)]
pub struct ParagraphProperties {
    pub align: Option<String>,
    pub level: u32,
    pub indent: Option<i32>,
    pub margin_left: Option<i32>,
    pub rtl: bool,
}

impl ParagraphProperties {
    pub fn parse(elem: &XmlElement) -> Self {
        ParagraphProperties {
            align: elem.attr("algn").map(|s| s.to_string()),
            level: elem.attr("lvl").and_then(|v| v.parse().ok()).unwrap_or(0),
            indent: elem.attr("indent").and_then(|v| v.parse().ok()),
            margin_left: elem.attr("marL").and_then(|v| v.parse().ok()),
            rtl: elem.attr("rtl").map(|v| v == "1").unwrap_or(false),
        }
    }

    pub fn to_xml(&self) -> String {
        let mut attrs = Vec::new();
        
        if let Some(ref align) = self.align {
            attrs.push(format!(r#"algn="{align}""#));
        }
        if self.level > 0 {
            let level = self.level;
            attrs.push(format!(r#"lvl="{level}""#));
        }
        if let Some(indent) = self.indent {
            attrs.push(format!(r#"indent="{indent}""#));
        }
        if let Some(mar_l) = self.margin_left {
            attrs.push(format!(r#"marL="{mar_l}""#));
        }
        if self.rtl {
            attrs.push(r#"rtl="1""#.to_string());
        }

        if attrs.is_empty() {
            "<a:pPr/>".to_string()
        } else {
            format!("<a:pPr {}/>", attrs.join(" "))
        }
    }
}

/// Run properties (a:rPr)
#[derive(Debug, Clone, Default)]
pub struct RunProperties {
    pub lang: Option<String>,
    pub size: Option<u32>,
    pub bold: bool,
    pub italic: bool,
    pub underline: Option<String>,
    pub strike: Option<String>,
    pub color: Option<String>,
    pub font_family: Option<String>,
}

impl RunProperties {
    pub fn parse(elem: &XmlElement) -> Self {
        let mut props = RunProperties {
            lang: elem.attr("lang").map(|s| s.to_string()),
            size: elem.attr("sz").and_then(|v| v.parse().ok()),
            bold: elem.attr("b").map(|v| v == "1").unwrap_or(false),
            italic: elem.attr("i").map(|v| v == "1").unwrap_or(false),
            underline: elem.attr("u").map(|s| s.to_string()),
            strike: elem.attr("strike").map(|s| s.to_string()),
            color: None,
            font_family: None,
        };

        // Parse color from solidFill
        if let Some(solid_fill) = elem.find_descendant("solidFill") {
            if let Some(srgb) = solid_fill.find("srgbClr") {
                props.color = srgb.attr("val").map(|s| s.to_string());
            }
        }

        // Parse font family from latin
        if let Some(latin) = elem.find("latin") {
            props.font_family = latin.attr("typeface").map(|s| s.to_string());
        }

        props
    }

    pub fn to_xml(&self) -> String {
        let lang_attrs = format_lang_attributes();
        let mut attrs = vec![lang_attrs];
        
        if let Some(sz) = self.size {
            attrs.push(format!(r#"sz="{sz}""#));
        }
        let b = if self.bold { "1" } else { "0" };
        let i = if self.italic { "1" } else { "0" };
        attrs.push(format!(r#"b="{b}""#));
        attrs.push(format!(r#"i="{i}""#));
        
        if let Some(ref u) = self.underline {
            attrs.push(format!(r#"u="{u}""#));
        }
        if let Some(ref strike) = self.strike {
            attrs.push(format!(r#"strike="{strike}""#));
        }

        let mut inner = String::new();
        if let Some(ref color) = self.color {
            inner.push_str(&format!(r#"<a:solidFill><a:srgbClr val="{color}"/></a:solidFill>"#));
        }
        if let Some(ref font) = self.font_family {
            inner.push_str(&format!(r#"<a:latin typeface="{font}"/>"#));
        }

        if inner.is_empty() {
            format!("<a:rPr {}/>", attrs.join(" "))
        } else {
            format!("<a:rPr {}>{}</a:rPr>", attrs.join(" "), inner)
        }
    }
}

/// Text run (a:r)
#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub properties: RunProperties,
}

impl TextRun {
    pub fn new(text: &str) -> Self {
        TextRun {
            text: text.to_string(),
            properties: RunProperties::default(),
        }
    }

    pub fn parse(elem: &XmlElement) -> Option<Self> {
        let text = elem.find("t").map(|t| t.text_content())?;
        let properties = elem.find("rPr")
            .map(|rpr| RunProperties::parse(rpr))
            .unwrap_or_default();

        Some(TextRun { text, properties })
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<a:r>{}<a:t>{}</a:t></a:r>",
            self.properties.to_xml(),
            escape_xml(&self.text)
        )
    }
}

/// Paragraph (a:p)
#[derive(Debug, Clone)]
pub struct TextParagraph {
    pub properties: ParagraphProperties,
    pub runs: Vec<TextRun>,
}

impl TextParagraph {
    pub fn new() -> Self {
        TextParagraph {
            properties: ParagraphProperties::default(),
            runs: Vec::new(),
        }
    }

    pub fn parse(elem: &XmlElement) -> Self {
        let properties = elem.find("pPr")
            .map(|ppr| ParagraphProperties::parse(ppr))
            .unwrap_or_default();

        let runs = elem.find_all("r")
            .into_iter()
            .filter_map(|r| TextRun::parse(r))
            .collect();

        TextParagraph { properties, runs }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<a:p>");
        xml.push_str(&self.properties.to_xml());
        for run in &self.runs {
            xml.push_str(&run.to_xml());
        }
        xml.push_str("</a:p>");
        xml
    }

    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

impl Default for TextParagraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Text body (p:txBody or a:txBody)
#[derive(Debug, Clone)]
pub struct TextBody {
    pub body_properties: BodyProperties,
    pub paragraphs: Vec<TextParagraph>,
}

impl TextBody {
    pub fn new() -> Self {
        TextBody {
            body_properties: BodyProperties::default(),
            paragraphs: Vec::new(),
        }
    }

    pub fn parse(elem: &XmlElement) -> Self {
        let body_properties = elem.find("bodyPr")
            .map(|bp| BodyProperties::parse(bp))
            .unwrap_or_default();

        let paragraphs = elem.find_all("p")
            .into_iter()
            .map(|p| TextParagraph::parse(p))
            .collect();

        TextBody { body_properties, paragraphs }
    }

    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<p:txBody>");
        xml.push_str(&self.body_properties.to_xml());
        xml.push_str("<a:lstStyle/>");
        for para in &self.paragraphs {
            xml.push_str(&para.to_xml());
        }
        if self.paragraphs.is_empty() {
            xml.push_str("<a:p/>");
        }
        xml.push_str("</p:txBody>");
        xml
    }

    pub fn all_text(&self) -> String {
        self.paragraphs.iter()
            .map(|p| p.text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for TextBody {
    fn default() -> Self {
        Self::new()
    }
}

fn escape_xml(s: &str) -> String {
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
    fn test_run_properties_to_xml() {
        let mut props = RunProperties::default();
        props.bold = true;
        props.size = Some(2400);
        props.color = Some("FF0000".to_string());

        let xml = props.to_xml();
        assert!(xml.contains("b=\"1\""));
        assert!(xml.contains("sz=\"2400\""));
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_text_run_to_xml() {
        let run = TextRun::new("Hello World");
        let xml = run.to_xml();
        
        assert!(xml.contains("<a:r>"));
        assert!(xml.contains("Hello World"));
        assert!(xml.contains("</a:r>"));
    }

    #[test]
    fn test_paragraph_to_xml() {
        let mut para = TextParagraph::new();
        para.runs.push(TextRun::new("Test"));
        
        let xml = para.to_xml();
        assert!(xml.contains("<a:p>"));
        assert!(xml.contains("Test"));
    }

    #[test]
    fn test_text_body_to_xml() {
        let mut body = TextBody::new();
        let mut para = TextParagraph::new();
        para.runs.push(TextRun::new("Content"));
        body.paragraphs.push(para);

        let xml = body.to_xml();
        assert!(xml.contains("<p:txBody>"));
        assert!(xml.contains("Content"));
    }
}
