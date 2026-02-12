//! Slide XML parsing and content extraction
//!
//! Parses slide XML to extract text, shapes, tables, and other content.

use super::xmlchemy::{XmlElement, XmlParser};
use crate::exc::PptxError;

/// Parsed text run with formatting
#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub font_size: Option<u32>,
    pub color: Option<String>,
}

impl TextRun {
    pub fn new(text: &str) -> Self {
        TextRun {
            text: text.to_string(),
            bold: false,
            italic: false,
            underline: false,
            font_size: None,
            color: None,
        }
    }
}

/// Parsed paragraph with text runs
#[derive(Debug, Clone)]
pub struct Paragraph {
    pub runs: Vec<TextRun>,
    pub level: u32,
}

impl Paragraph {
    pub fn new() -> Self {
        Paragraph {
            runs: Vec::new(),
            level: 0,
        }
    }

    /// Get full text content
    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

impl Default for Paragraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed shape from slide
#[derive(Debug, Clone)]
pub struct ParsedShape {
    pub name: String,
    pub shape_type: Option<String>,
    pub paragraphs: Vec<Paragraph>,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub is_title: bool,
    pub is_body: bool,
}

impl ParsedShape {
    pub fn new(name: &str) -> Self {
        ParsedShape {
            name: name.to_string(),
            shape_type: None,
            paragraphs: Vec::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            is_title: false,
            is_body: false,
        }
    }

    /// Get all text from shape
    pub fn text(&self) -> String {
        self.paragraphs
            .iter()
            .map(|p| p.text())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Parsed table cell
#[derive(Debug, Clone)]
pub struct ParsedTableCell {
    pub text: String,
    pub row_span: u32,
    pub col_span: u32,
}

/// Parsed table
#[derive(Debug, Clone)]
pub struct ParsedTable {
    pub rows: Vec<Vec<ParsedTableCell>>,
}

impl ParsedTable {
    pub fn new() -> Self {
        ParsedTable { rows: Vec::new() }
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn col_count(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or(0)
    }
}

impl Default for ParsedTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed image reference from slide
#[derive(Debug, Clone)]
pub struct ParsedImage {
    pub path: Option<String>,
    pub name: String,
    pub rel_id: String,
    pub format: Option<String>,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub description: Option<String>,
}

impl ParsedImage {
    pub fn new(name: &str, rel_id: &str) -> Self {
        ParsedImage {
            name: name.to_string(),
            rel_id: rel_id.to_string(),
            format: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            description: None,
            path: None,
        }
    }

    pub fn mime_type(&self) -> &str {
        match self.format.as_deref() {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("bmp") => "image/bmp",
            Some("tiff") | Some("tif") => "image/tiff",
            Some("emf") => "image/x-emf",
            Some("wmf") => "image/x-wmf",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        }
    }
}

/// Parsed slide layout reference
#[derive(Debug, Clone)]
pub struct ParsedLayout {
    pub path: Option<String>,
    pub rel_id: String,
    pub name: Option<String>,
    pub layout_type: Option<String>,
    pub images: Vec<ParsedImage>,
}

impl ParsedLayout {
    pub fn new(rel_id: &str) -> Self {
        ParsedLayout {
            path: None,
            rel_id: rel_id.to_string(),
            name: None,
            layout_type: None,
            images: Vec::new(),
        }
    }
}

/// Parsed slide master reference
#[derive(Debug, Clone)]
pub struct ParsedMaster {
    pub path: Option<String>,
    pub rel_id: String,
    pub name: Option<String>,
    pub images: Vec<ParsedImage>,
}

impl ParsedMaster {
    pub fn new(rel_id: &str) -> Self {
        ParsedMaster {
            path: None,
            rel_id: rel_id.to_string(),
            name: None,
            images: Vec::new(),
        }
    }
}

/// Parsed slide content
#[derive(Debug, Clone)]
pub struct ParsedSlide {
    pub shapes: Vec<ParsedShape>,
    pub tables: Vec<ParsedTable>,
    pub images: Vec<ParsedImage>,
    pub layout: Option<ParsedLayout>,
    pub master: Option<ParsedMaster>,
    pub title: Option<String>,
    pub body_text: Vec<String>,
}

impl ParsedSlide {
    pub fn new() -> Self {
        ParsedSlide {
            shapes: Vec::new(),
            tables: Vec::new(),
            images: Vec::new(),
            layout: None,
            master: None,
            title: None,
            body_text: Vec::new(),
        }
    }

    /// Get all text from slide
    pub fn all_text(&self) -> Vec<String> {
        let mut texts = Vec::new();
        if let Some(ref title) = self.title {
            texts.push(title.clone());
        }
        texts.extend(self.body_text.clone());
        for shape in &self.shapes {
            let text = shape.text();
            if !text.is_empty() {
                texts.push(text);
            }
        }
        texts
    }

    /// Get all images from slide (including layout and master)
    pub fn all_images(&self) -> Vec<&ParsedImage> {
        let mut images = Vec::new();
        images.extend(self.images.iter());
        if let Some(ref layout) = self.layout {
            images.extend(layout.images.iter());
        }
        if let Some(ref master) = self.master {
            images.extend(master.images.iter());
        }
        images
    }
}

impl Default for ParsedSlide {
    fn default() -> Self {
        Self::new()
    }
}

/// Slide parser
pub struct SlideParser;

impl SlideParser {
    /// Parse slide XML content
    pub fn parse(xml: &str) -> Result<ParsedSlide, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut slide = ParsedSlide::new();

        // Find shape tree (spTree)
        if let Some(sp_tree) = root.find_descendant("spTree") {
            // Parse shapes
            for sp in sp_tree.find_all("sp") {
                if let Some(mut shape) = Self::parse_shape(sp) {
                    // Check if this is title or body
                    if Self::is_title_shape(sp) {
                        shape.is_title = true;
                        slide.title = Some(shape.text());
                    } else if Self::is_body_shape(sp) {
                        shape.is_body = true;
                        for para in &shape.paragraphs {
                            let text = para.text();
                            if !text.is_empty() {
                                slide.body_text.push(text);
                            }
                        }
                    }
                    slide.shapes.push(shape);
                }
            }

            // Parse graphic frames (tables, charts)
            for gf in sp_tree.find_all("graphicFrame") {
                if let Some(table) = Self::parse_table_from_graphic_frame(gf) {
                    slide.tables.push(table);
                }
            }

            // Parse pictures
            for pic in sp_tree.find_all("pic") {
                if let Some(image) = Self::parse_picture(pic) {
                    slide.images.push(image);
                }
            }
        }

        Ok(slide)
    }

    fn parse_picture(pic: &XmlElement) -> Option<ParsedImage> {
        // Get picture name and description from nvPicPr/cNvPr
        let nv_pic_pr = pic.find_descendant("nvPicPr")?;
        let cnv_pr = nv_pic_pr.find_descendant("cNvPr")?;

        let name = cnv_pr.attr("name").unwrap_or("Picture");
        let description = cnv_pr.attr("descr").map(|s| s.to_string());

        // Get relationship ID from blipFill/blip
        let blip_fill = pic.find_descendant("blipFill")?;
        let blip = blip_fill.find_descendant("blip")?;

        // r:embed attribute contains the relationship ID
        let rel_id = blip.attr("embed").or_else(|| blip.attr("r:embed"))?;

        let mut image = ParsedImage::new(name, rel_id);
        image.description = description;

        // Get position and size from spPr/xfrm
        if let Some(sp_pr) = pic.find_descendant("spPr") {
            if let Some(xfrm) = sp_pr.find_descendant("xfrm") {
                if let Some(off) = xfrm.find("off") {
                    image.x = off.attr("x").and_then(|v| v.parse().ok()).unwrap_or(0);
                    image.y = off.attr("y").and_then(|v| v.parse().ok()).unwrap_or(0);
                }
                if let Some(ext) = xfrm.find("ext") {
                    image.width = ext.attr("cx").and_then(|v| v.parse().ok()).unwrap_or(0);
                    image.height = ext.attr("cy").and_then(|v| v.parse().ok()).unwrap_or(0);
                }
            }
        }

        Some(image)
    }

    fn parse_shape(sp: &XmlElement) -> Option<ParsedShape> {
        // Get shape name from nvSpPr/cNvPr
        let name = sp
            .find_descendant("cNvPr")
            .and_then(|e| e.attr("name"))
            .unwrap_or("Shape");

        let mut shape = ParsedShape::new(name);

        // Get position and size from spPr/xfrm
        if let Some(xfrm) = sp.find_descendant("xfrm") {
            if let Some(off) = xfrm.find("off") {
                shape.x = off.attr("x").and_then(|v| v.parse().ok()).unwrap_or(0);
                shape.y = off.attr("y").and_then(|v| v.parse().ok()).unwrap_or(0);
            }
            if let Some(ext) = xfrm.find("ext") {
                shape.width = ext.attr("cx").and_then(|v| v.parse().ok()).unwrap_or(0);
                shape.height = ext.attr("cy").and_then(|v| v.parse().ok()).unwrap_or(0);
            }
        }

        // Get shape type from prstGeom
        if let Some(prst_geom) = sp.find_descendant("prstGeom") {
            shape.shape_type = prst_geom.attr("prst").map(|s| s.to_string());
        }

        // Parse text body
        if let Some(tx_body) = sp.find_descendant("txBody") {
            shape.paragraphs = Self::parse_text_body(tx_body);
        }

        Some(shape)
    }

    fn parse_text_body(tx_body: &XmlElement) -> Vec<Paragraph> {
        let mut paragraphs = Vec::new();

        for p in tx_body.find_all("p") {
            let mut para = Paragraph::new();

            // Get paragraph level
            if let Some(ppr) = p.find("pPr") {
                para.level = ppr.attr("lvl").and_then(|v| v.parse().ok()).unwrap_or(0);
            }

            // Parse text runs
            for r in p.find_all("r") {
                let text = r.find("t").map(|t| t.text_content()).unwrap_or_default();
                if text.is_empty() {
                    continue;
                }

                let mut run = TextRun::new(&text);

                // Parse run properties
                if let Some(rpr) = r.find("rPr") {
                    run.bold = rpr
                        .attr("b")
                        .map(|v| v == "1" || v == "true")
                        .unwrap_or(false);
                    run.italic = rpr
                        .attr("i")
                        .map(|v| v == "1" || v == "true")
                        .unwrap_or(false);
                    run.underline = rpr.attr("u").is_some();
                    run.font_size = rpr.attr("sz").and_then(|v| v.parse().ok());

                    // Get color from solidFill/srgbClr
                    if let Some(solid_fill) = rpr.find_descendant("solidFill") {
                        if let Some(srgb) = solid_fill.find("srgbClr") {
                            run.color = srgb.attr("val").map(|s| s.to_string());
                        }
                    }
                }

                para.runs.push(run);
            }

            if !para.runs.is_empty() {
                paragraphs.push(para);
            }
        }

        paragraphs
    }

    fn is_title_shape(sp: &XmlElement) -> bool {
        // Check placeholder type first
        if let Some(nv_pr) = sp.find_descendant("nvPr") {
            if let Some(ph) = nv_pr.find("ph") {
                let ph_type = ph.attr("type").unwrap_or("");
                if ph_type == "title" || ph_type == "ctrTitle" {
                    return true;
                }
            }
        }
        // Also check shape name for textbox-based titles
        if let Some(cnv_pr) = sp.find_descendant("cNvPr") {
            if let Some(name) = cnv_pr.attr("name") {
                let name_lower = name.to_lowercase();
                if name_lower == "title" || name_lower.contains("title") {
                    return true;
                }
            }
        }
        false
    }

    fn is_body_shape(sp: &XmlElement) -> bool {
        // Check placeholder type first
        if let Some(nv_pr) = sp.find_descendant("nvPr") {
            if let Some(ph) = nv_pr.find("ph") {
                let ph_type = ph.attr("type").unwrap_or("body");
                if ph_type == "body" || ph_type.is_empty() {
                    return true;
                }
            }
        }
        // Also check shape name for textbox-based content
        if let Some(cnv_pr) = sp.find_descendant("cNvPr") {
            if let Some(name) = cnv_pr.attr("name") {
                let name_lower = name.to_lowercase();
                if name_lower == "content" || name_lower.contains("content") {
                    return true;
                }
            }
        }
        false
    }

    fn parse_table_from_graphic_frame(gf: &XmlElement) -> Option<ParsedTable> {
        // Find table element (a:tbl)
        let tbl = gf.find_descendant("tbl")?;
        let mut table = ParsedTable::new();

        for tr in tbl.find_all("tr") {
            let mut row = Vec::new();
            for tc in tr.find_all("tc") {
                let text = tc
                    .find_descendant("t")
                    .map(|t| t.text_content())
                    .unwrap_or_default();

                let row_span = tc.attr("rowSpan").and_then(|v| v.parse().ok()).unwrap_or(1);
                let col_span = tc
                    .attr("gridSpan")
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1);

                row.push(ParsedTableCell {
                    text,
                    row_span,
                    col_span,
                });
            }
            if !row.is_empty() {
                table.rows.push(row);
            }
        }

        if table.rows.is_empty() {
            None
        } else {
            Some(table)
        }
    }

    /// Parse slide layout XML to extract images
    pub fn parse_layout(xml: &str) -> Result<ParsedLayout, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut layout = ParsedLayout::new("");

        // Get layout name and type from cSld
        if let Some(c_sld) = root.find_descendant("cSld") {
            layout.name = c_sld.attr("name").map(|s| s.to_string());
        }

        // Get layout type from sldLayout
        layout.layout_type = root.attr("type").map(|s| s.to_string());

        // Parse images from spTree
        if let Some(sp_tree) = root.find_descendant("spTree") {
            for pic in sp_tree.find_all("pic") {
                if let Some(image) = Self::parse_picture(pic) {
                    layout.images.push(image);
                }
            }
        }

        Ok(layout)
    }

    /// Parse slide master XML to extract images
    pub fn parse_master(xml: &str) -> Result<ParsedMaster, PptxError> {
        let root = XmlParser::parse_str(xml)?;
        let mut master = ParsedMaster::new("");

        // Get master name
        if let Some(c_sld) = root.find_descendant("cSld") {
            master.name = c_sld.attr("name").map(|s| s.to_string());
        }

        // Parse images from spTree
        if let Some(sp_tree) = root.find_descendant("spTree") {
            for pic in sp_tree.find_all("pic") {
                if let Some(image) = Self::parse_picture(pic) {
                    master.images.push(image);
                }
            }
        }

        Ok(master)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_slide() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" 
               xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld>
                <p:spTree>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="2" name="Title"/>
                            <p:nvPr><p:ph type="title"/></p:nvPr>
                        </p:nvSpPr>
                        <p:txBody>
                            <a:p>
                                <a:r><a:t>Test Title</a:t></a:r>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="3" name="Content"/>
                            <p:nvPr><p:ph type="body"/></p:nvPr>
                        </p:nvSpPr>
                        <p:txBody>
                            <a:p>
                                <a:r><a:t>Bullet 1</a:t></a:r>
                            </a:p>
                            <a:p>
                                <a:r><a:t>Bullet 2</a:t></a:r>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                </p:spTree>
            </p:cSld>
        </p:sld>"#;

        let slide = SlideParser::parse(xml).unwrap();
        assert_eq!(slide.title, Some("Test Title".to_string()));
        assert_eq!(slide.body_text.len(), 2);
        assert_eq!(slide.body_text[0], "Bullet 1");
        assert_eq!(slide.body_text[1], "Bullet 2");
    }

    #[test]
    fn test_parse_formatted_text() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" 
               xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld>
                <p:spTree>
                    <p:sp>
                        <p:nvSpPr>
                            <p:cNvPr id="2" name="Title"/>
                            <p:nvPr><p:ph type="title"/></p:nvPr>
                        </p:nvSpPr>
                        <p:txBody>
                            <a:p>
                                <a:r>
                                    <a:rPr b="1" i="1" sz="4400"/>
                                    <a:t>Bold Italic</a:t>
                                </a:r>
                            </a:p>
                        </p:txBody>
                    </p:sp>
                </p:spTree>
            </p:cSld>
        </p:sld>"#;

        let slide = SlideParser::parse(xml).unwrap();
        assert!(slide.shapes.len() > 0);
        let shape = &slide.shapes[0];
        assert!(shape.paragraphs.len() > 0);
        let run = &shape.paragraphs[0].runs[0];
        assert!(run.bold);
        assert!(run.italic);
        assert_eq!(run.font_size, Some(4400));
    }

    #[test]
    fn test_parse_slide_with_image() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" 
               xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
               xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld>
                <p:spTree>
                    <p:pic>
                        <p:nvPicPr>
                            <p:cNvPr id="4" name="Picture 3" descr="Test image"/>
                            <p:cNvPicPr/>
                            <p:nvPr/>
                        </p:nvPicPr>
                        <p:blipFill>
                            <a:blip r:embed="rId2"/>
                        </p:blipFill>
                        <p:spPr>
                            <a:xfrm>
                                <a:off x="1000" y="2000"/>
                                <a:ext cx="5000" cy="3000"/>
                            </a:xfrm>
                        </p:spPr>
                    </p:pic>
                </p:spTree>
            </p:cSld>
        </p:sld>"#;

        let slide = SlideParser::parse(xml).unwrap();
        assert_eq!(slide.images.len(), 1);

        let image = &slide.images[0];
        assert_eq!(image.name, "Picture 3");
        assert_eq!(image.rel_id, "rId2");
        assert_eq!(image.description, Some("Test image".to_string()));
        assert_eq!(image.x, 1000);
        assert_eq!(image.y, 2000);
        assert_eq!(image.width, 5000);
        assert_eq!(image.height, 3000);
    }

    #[test]
    fn test_parse_layout_with_image() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                     xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                     xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
                     type="title">
            <p:cSld name="Title Layout">
                <p:spTree>
                    <p:pic>
                        <p:nvPicPr>
                            <p:cNvPr id="5" name="Logo"/>
                            <p:cNvPicPr/>
                            <p:nvPr/>
                        </p:nvPicPr>
                        <p:blipFill>
                            <a:blip r:embed="rId3"/>
                        </p:blipFill>
                        <p:spPr/>
                    </p:pic>
                </p:spTree>
            </p:cSld>
        </p:sldLayout>"#;

        let layout = SlideParser::parse_layout(xml).unwrap();
        assert_eq!(layout.name, Some("Title Layout".to_string()));
        assert_eq!(layout.layout_type, Some("title".to_string()));
        assert_eq!(layout.images.len(), 1);
        assert_eq!(layout.images[0].name, "Logo");
        assert_eq!(layout.images[0].rel_id, "rId3");
    }

    #[test]
    fn test_parse_master_with_image() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <p:sldMaster xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
                     xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                     xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld name="Office Theme">
                <p:spTree>
                    <p:pic>
                        <p:nvPicPr>
                            <p:cNvPr id="6" name="Background"/>
                            <p:cNvPicPr/>
                            <p:nvPr/>
                        </p:nvPicPr>
                        <p:blipFill>
                            <a:blip r:embed="rId4"/>
                        </p:blipFill>
                        <p:spPr/>
                    </p:pic>
                </p:spTree>
            </p:cSld>
        </p:sldMaster>"#;

        let master = SlideParser::parse_master(xml).unwrap();
        assert_eq!(master.name, Some("Office Theme".to_string()));
        assert_eq!(master.images.len(), 1);
        assert_eq!(master.images[0].name, "Background");
        assert_eq!(master.images[0].rel_id, "rId4");
    }

    #[test]
    fn test_parsed_image_mime_type() {
        let mut image = ParsedImage::new("Test", "rId1");

        image.format = Some("png".to_string());
        assert_eq!(image.mime_type(), "image/png");

        image.format = Some("jpeg".to_string());
        assert_eq!(image.mime_type(), "image/jpeg");

        image.format = Some("gif".to_string());
        assert_eq!(image.mime_type(), "image/gif");

        image.format = None;
        assert_eq!(image.mime_type(), "application/octet-stream");
    }

    #[test]
    fn test_all_images() {
        let mut slide = ParsedSlide::new();

        // Add slide image
        slide.images.push(ParsedImage::new("SlideImage", "rId1"));

        // Add layout with image
        let mut layout = ParsedLayout::new("rId10");
        layout.images.push(ParsedImage::new("LayoutImage", "rId2"));
        slide.layout = Some(layout);

        // Add master with image
        let mut master = ParsedMaster::new("rId20");
        master.images.push(ParsedImage::new("MasterImage", "rId3"));
        slide.master = Some(master);

        let all = slide.all_images();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].name, "SlideImage");
        assert_eq!(all[1].name, "LayoutImage");
        assert_eq!(all[2].name, "MasterImage");
    }
}
