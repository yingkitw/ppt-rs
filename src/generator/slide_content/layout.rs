//! Slide layout types

/// Slide layout types
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum SlideLayout {
    /// Title slide (centered title + subtitle)
    CenteredTitle,
    /// Title and content (bullets)
    TitleAndContent,
    /// Two columns of content
    TwoColumn,
    /// Section divider (large title)
    SectionHeader,
    /// Blank slide
    Blank,
    /// Title only (no content area)
    TitleOnly,
    /// Title at top, content fills rest
    TitleAndBigContent,
}

impl SlideLayout {
    /// 1-based `slideLayoutN.xml` index on slide master 1.
    pub fn layout_number(&self) -> usize {
        match self {
            SlideLayout::CenteredTitle => 1,
            SlideLayout::TitleAndContent => 2,
            SlideLayout::TwoColumn => 3,
            SlideLayout::SectionHeader => 4,
            SlideLayout::Blank => 5,
            SlideLayout::TitleOnly => 6,
            SlideLayout::TitleAndBigContent => 7,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SlideLayout::TitleOnly => "titleOnly",
            SlideLayout::TitleAndContent => "titleAndContent",
            SlideLayout::TitleAndBigContent => "titleAndBigContent",
            SlideLayout::Blank => "blank",
            SlideLayout::CenteredTitle => "centeredTitle",
            SlideLayout::TwoColumn => "twoColumn",
            SlideLayout::SectionHeader => "sectionHeader",
        }
    }
}
