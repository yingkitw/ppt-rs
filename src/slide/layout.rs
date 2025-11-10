//! Slide Layout - Predefined layout templates for slides

/// Collection of all 11 predefined slide layouts
#[derive(Clone, Debug)]
pub struct SlideLayouts {
    layouts: Vec<SlideLayout>,
}

impl SlideLayouts {
    /// Create a new collection with all 11 predefined layouts
    pub fn new() -> Self {
        let layouts = vec![
            SlideLayout::new(LayoutType::TitleSlide),
            SlideLayout::new(LayoutType::TitleAndContent),
            SlideLayout::new(LayoutType::TitleOnly),
            SlideLayout::new(LayoutType::CenteredTitle),
            SlideLayout::new(LayoutType::TitleAndTwoContent),
            SlideLayout::new(LayoutType::Blank),
            SlideLayout::new(LayoutType::Comparison),
            SlideLayout::new(LayoutType::TitleContentCaption),
            SlideLayout::new(LayoutType::PictureCaption),
            SlideLayout::new(LayoutType::BlankWithTitle),
            SlideLayout::new(LayoutType::TitleAndVerticalContent),
        ];

        Self { layouts }
    }

    /// Get layout by index (1-11)
    pub fn get(&self, index: u32) -> Option<&SlideLayout> {
        if index >= 1 && index <= 11 {
            self.layouts.get((index - 1) as usize)
        } else {
            None
        }
    }

    /// Get layout by type
    pub fn get_by_type(&self, layout_type: &LayoutType) -> Option<&SlideLayout> {
        self.layouts.iter().find(|l| l.layout_type() == layout_type)
    }

    /// Get all layouts
    pub fn all(&self) -> &[SlideLayout] {
        &self.layouts
    }

    /// Get number of layouts
    pub fn len(&self) -> usize {
        self.layouts.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.layouts.is_empty()
    }

    /// Get layout IDs for master
    pub fn layout_ids(&self) -> Vec<u32> {
        self.layouts.iter().map(|l| 256 + l.index() - 1).collect()
    }
}

impl Default for SlideLayouts {
    fn default() -> Self {
        Self::new()
    }
}

/// Slide layout type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LayoutType {
    /// Blank layout
    Blank,
    /// Title slide layout
    TitleSlide,
    /// Title and content layout
    TitleAndContent,
    /// Title only layout
    TitleOnly,
    /// Centered title layout
    CenteredTitle,
    /// Title and two content layout
    TitleAndTwoContent,
    /// Comparison layout
    Comparison,
    /// Title, content and caption layout
    TitleContentCaption,
    /// Picture with caption layout
    PictureCaption,
    /// Blank with title layout
    BlankWithTitle,
    /// Title and vertical content layout
    TitleAndVerticalContent,
}

impl LayoutType {
    /// Get layout name
    pub fn name(&self) -> &str {
        match self {
            LayoutType::Blank => "Blank",
            LayoutType::TitleSlide => "Title Slide",
            LayoutType::TitleAndContent => "Title and Content",
            LayoutType::TitleOnly => "Title Only",
            LayoutType::CenteredTitle => "Centered Title",
            LayoutType::TitleAndTwoContent => "Title and Two Content",
            LayoutType::Comparison => "Comparison",
            LayoutType::TitleContentCaption => "Title, Content and Caption",
            LayoutType::PictureCaption => "Picture with Caption",
            LayoutType::BlankWithTitle => "Blank with Title",
            LayoutType::TitleAndVerticalContent => "Title and Vertical Content",
        }
    }

    /// Get layout index (1-11)
    pub fn index(&self) -> u32 {
        match self {
            LayoutType::Blank => 6,
            LayoutType::TitleSlide => 1,
            LayoutType::TitleAndContent => 2,
            LayoutType::TitleOnly => 3,
            LayoutType::CenteredTitle => 4,
            LayoutType::TitleAndTwoContent => 5,
            LayoutType::Comparison => 7,
            LayoutType::TitleContentCaption => 8,
            LayoutType::PictureCaption => 9,
            LayoutType::BlankWithTitle => 10,
            LayoutType::TitleAndVerticalContent => 11,
        }
    }
}

/// Slide Layout - defines layout template for slides
#[derive(Clone, Debug)]
pub struct SlideLayout {
    /// Layout type
    layout_type: LayoutType,
}

impl SlideLayout {
    /// Create a new slide layout
    pub fn new(layout_type: LayoutType) -> Self {
        Self {
            layout_type,
        }
    }

    /// Get layout type
    pub fn layout_type(&self) -> &LayoutType {
        &self.layout_type
    }

    /// Get layout name
    pub fn name(&self) -> &str {
        self.layout_type.name()
    }

    /// Get layout index
    pub fn index(&self) -> u32 {
        self.layout_type.index()
    }

    /// Generate slideLayout*.xml content
    pub fn to_xml(&self) -> String {
        // Use predefined layout XML templates that are PowerPoint-compatible
        match self.layout_type {
            LayoutType::TitleSlide => self.layout_title_slide(),
            LayoutType::TitleAndContent => self.layout_title_and_content(),
            LayoutType::TitleOnly => self.layout_title_only(),
            LayoutType::CenteredTitle => self.layout_centered_title(),
            LayoutType::TitleAndTwoContent => self.layout_title_and_two_content(),
            LayoutType::Blank => self.layout_blank(),
            LayoutType::Comparison => self.layout_comparison(),
            LayoutType::TitleContentCaption => self.layout_title_content_caption(),
            LayoutType::PictureCaption => self.layout_picture_caption(),
            LayoutType::BlankWithTitle => self.layout_blank_with_title(),
            LayoutType::TitleAndVerticalContent => self.layout_title_and_vertical_content(),
        }
    }

    /// Title Slide layout
    fn layout_title_slide(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" type="title" preserve="1"><p:cSld name="Title Slide"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="ctrTitle"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="685800" y="2130425"/><a:ext cx="7772400" cy="1470025"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp><p:sp><p:nvSpPr><p:cNvPr id="3" name="Subtitle 2"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="subTitle" idx="1"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="1371600" y="3886200"/><a:ext cx="6400800" cy="1752600"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master subtitle style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree><p:extLst><p:ext uri="{BB962C8B-B14F-4D97-AF65-F5344CB8AC3E}"><p14:creationId val="1"/></p:ext></p:extLst></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Title and Content layout
    fn layout_title_and_content(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" type="titleOnly" preserve="1"><p:cSld name="Title and Content"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp><p:sp><p:nvSpPr><p:cNvPr id="3" name="Content Placeholder 2"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph idx="1"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="1463250"/><a:ext cx="8229600" cy="5105400"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p/></p:txBody></p:sp></p:spTree><p:extLst><p:ext uri="{BB962C8B-B14F-4D97-AF65-F5344CB8AC3E}"><p14:creationId val="2"/></p:ext></p:extLst></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Title Only layout
    fn layout_title_only(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" type="titleOnly" preserve="1"><p:cSld name="Title Only"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree><p:extLst><p:ext uri="{BB962C8B-B14F-4D97-AF65-F5344CB8AC3E}"><p14:creationId val="3"/></p:ext></p:extLst></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Centered Title layout
    fn layout_centered_title(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Centered Title"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="ctrTitle"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="2743200"/><a:ext cx="8229600" cy="1371600"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Title and Two Content layout
    fn layout_title_and_two_content(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Title and Two Content"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp><p:sp><p:nvSpPr><p:cNvPr id="3" name="Content Placeholder 2"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph idx="1"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="1463250"/><a:ext cx="3886200" cy="5105400"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p/></p:txBody></p:sp><p:sp><p:nvSpPr><p:cNvPr id="4" name="Content Placeholder 3"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph idx="2"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="4343400" y="1463250"/><a:ext cx="3886200" cy="5105400"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p/></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Blank layout
    fn layout_blank(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" type="blank" preserve="1"><p:cSld name="Blank"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr></p:spTree><p:extLst><p:ext uri="{BB962C8B-B14F-4D97-AF65-F5344CB8AC3E}"><p14:creationId val="6"/></p:ext></p:extLst></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Comparison layout
    fn layout_comparison(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Comparison"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Title, Content and Caption layout
    fn layout_title_content_caption(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Title, Content and Caption"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Picture with Caption layout
    fn layout_picture_caption(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Picture with Caption"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Blank with Title layout
    fn layout_blank_with_title(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Blank with Title"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }

    /// Title and Vertical Content layout
    fn layout_title_and_vertical_content(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><p:sldLayout xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main" type="titleOnly" preserve="1"><p:cSld name="Title and Vertical Content"><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr><a:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/><a:chOff x="0" y="0"/><a:chExt cx="0" cy="0"/></a:xfrm></p:grpSpPr><p:sp><p:nvSpPr><p:cNvPr id="2" name="Title 1"/><p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr><p:nvPr><p:ph type="title"/></p:nvPr></p:nvSpPr><p:spPr><a:xfrm><a:off x="457200" y="274638"/><a:ext cx="8229600" cy="1143000"/></a:xfrm></p:spPr><p:txBody><a:bodyPr/><a:lstStyle/><a:p><a:r><a:rPr lang="en-US" smtClean="0"/><a:t>Click to edit Master title style</a:t></a:r><a:endParaRPr lang="en-US"/></a:p></p:txBody></p:sp></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_type_name() {
        assert_eq!(LayoutType::Blank.name(), "Blank");
        assert_eq!(LayoutType::TitleSlide.name(), "Title Slide");
        assert_eq!(LayoutType::TitleAndContent.name(), "Title and Content");
    }

    #[test]
    fn test_layout_type_index() {
        assert_eq!(LayoutType::TitleSlide.index(), 1);
        assert_eq!(LayoutType::TitleAndContent.index(), 2);
        assert_eq!(LayoutType::Blank.index(), 6);
    }

    #[test]
    fn test_slide_layout_creation() {
        let layout = SlideLayout::new(LayoutType::TitleAndContent);
        assert_eq!(layout.layout_type(), &LayoutType::TitleAndContent);
        assert_eq!(layout.name(), "Title and Content");
        assert_eq!(layout.index(), 2);
    }

    #[test]
    fn test_slide_layout_to_xml() {
        let layout = SlideLayout::new(LayoutType::TitleAndContent);
        let xml = layout.to_xml();
        
        assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
        assert!(xml.contains(r#"<p:sldLayout"#));
        assert!(xml.contains(r#"<p:cSld"#)); // Changed to match new format with name attribute
        assert!(xml.contains(r#"<p:clrMapOvr>"#));
        assert!(xml.contains(r#"</p:sldLayout>"#));
    }

    #[test]
    fn test_slide_layout_xml_has_placeholders() {
        let layout = SlideLayout::new(LayoutType::TitleAndContent);
        let xml = layout.to_xml();
        
        // Should have title and content placeholders
        assert!(xml.contains(r#"<p:ph type="title"/>"#));
        assert!(xml.contains(r#"<p:ph idx="1"/>"#));
    }

    #[test]
    fn test_blank_layout_no_placeholders() {
        let layout = SlideLayout::new(LayoutType::Blank);
        let xml = layout.to_xml();
        
        // Blank layout should not have placeholders
        assert!(!xml.contains(r#"<p:ph type="title"/>"#));
    }

    #[test]
    fn test_title_slide_layout() {
        let layout = SlideLayout::new(LayoutType::TitleSlide);
        let xml = layout.to_xml();
        
        // Should have center title and subtitle
        assert!(xml.contains(r#"<p:ph type="ctrTitle"/>"#));
        assert!(xml.contains(r#"<p:ph type="subTitle" idx="1"/>"#));
    }

    #[test]
    fn test_all_layout_types() {
        let layouts = vec![
            LayoutType::Blank,
            LayoutType::TitleSlide,
            LayoutType::TitleAndContent,
            LayoutType::TitleOnly,
            LayoutType::CenteredTitle,
            LayoutType::TitleAndTwoContent,
            LayoutType::Comparison,
            LayoutType::TitleContentCaption,
            LayoutType::PictureCaption,
            LayoutType::BlankWithTitle,
            LayoutType::TitleAndVerticalContent,
        ];

        for layout_type in layouts {
            let layout = SlideLayout::new(layout_type);
            let xml = layout.to_xml();
            
            assert!(xml.contains(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#));
            assert!(xml.contains(r#"<p:sldLayout"#));
            assert!(xml.contains(r#"</p:sldLayout>"#));
        }
    }
}
