//! Table cell definition and formatting

/// Horizontal text alignment
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CellAlign {
    Left,
    #[default]
    Center,
    Right,
    Justify,
}

impl CellAlign {
    /// Get the OOXML alignment value
    pub fn as_str(&self) -> &'static str {
        match self {
            CellAlign::Left => "l",
            CellAlign::Center => "ctr",
            CellAlign::Right => "r",
            CellAlign::Justify => "just",
        }
    }
}

/// Vertical text alignment
#[derive(Clone, Debug, Default, PartialEq)]
pub enum CellVAlign {
    Top,
    #[default]
    Middle,
    Bottom,
}

impl CellVAlign {
    /// Get the OOXML anchor value
    pub fn as_str(&self) -> &'static str {
        match self {
            CellVAlign::Top => "t",
            CellVAlign::Middle => "ctr",
            CellVAlign::Bottom => "b",
        }
    }
}

/// Table cell content with formatting options
#[derive(Clone, Debug)]
pub struct TableCell {
    /// Cell text content
    pub text: String,
    /// Bold text
    pub bold: bool,
    /// Italic text
    pub italic: bool,
    /// Underlined text
    pub underline: bool,
    /// Text color (RGB hex, e.g., "FF0000")
    pub text_color: Option<String>,
    /// Background color (RGB hex, e.g., "0000FF")
    pub background_color: Option<String>,
    /// Font size in points
    pub font_size: Option<u32>,
    /// Font family name
    pub font_family: Option<String>,
    /// Horizontal text alignment
    pub align: CellAlign,
    /// Vertical text alignment
    pub valign: CellVAlign,
    /// Enable text wrapping
    pub wrap_text: bool,
    /// Number of columns this cell spans (gridSpan attribute)
    pub grid_span: Option<u32>,
    /// Number of rows this cell spans (rowSpan attribute)
    pub row_span: Option<u32>,
    /// Whether this cell is horizontally merged (covered by a gridSpan)
    pub h_merge: bool,
    /// Whether this cell is vertically merged (covered by a rowSpan)
    pub v_merge: bool,
}

impl TableCell {
    /// Create a new table cell with text
    pub fn new(text: &str) -> Self {
        TableCell {
            text: text.to_string(),
            bold: false,
            italic: false,
            underline: false,
            text_color: None,
            background_color: None,
            font_size: None,
            font_family: None,
            align: CellAlign::Center,
            valign: CellVAlign::Middle,
            wrap_text: true,
            grid_span: None,
            row_span: None,
            h_merge: false,
            v_merge: false,
        }
    }

    /// Set cell text as bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set cell text as italic
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    /// Set cell text as underlined
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    /// Set cell text color (RGB hex format, e.g., "FF0000" or "#FF0000")
    pub fn text_color(mut self, color: &str) -> Self {
        self.text_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set cell background color (RGB hex format, e.g., "FF0000" or "#FF0000")
    pub fn background_color(mut self, color: &str) -> Self {
        self.background_color = Some(color.trim_start_matches('#').to_uppercase());
        self
    }

    /// Set font size in points
    pub fn font_size(mut self, size: u32) -> Self {
        self.font_size = Some(size);
        self
    }

    /// Set font family name
    pub fn font_family(mut self, family: &str) -> Self {
        self.font_family = Some(family.to_string());
        self
    }

    /// Set horizontal text alignment
    pub fn align(mut self, align: CellAlign) -> Self {
        self.align = align;
        self
    }

    /// Set horizontal text alignment to left
    pub fn align_left(mut self) -> Self {
        self.align = CellAlign::Left;
        self
    }

    /// Set horizontal text alignment to right
    pub fn align_right(mut self) -> Self {
        self.align = CellAlign::Right;
        self
    }

    /// Set horizontal text alignment to center
    pub fn align_center(mut self) -> Self {
        self.align = CellAlign::Center;
        self
    }

    /// Set vertical text alignment
    pub fn valign(mut self, valign: CellVAlign) -> Self {
        self.valign = valign;
        self
    }

    /// Set vertical text alignment to top
    pub fn valign_top(mut self) -> Self {
        self.valign = CellVAlign::Top;
        self
    }

    /// Set vertical text alignment to bottom
    pub fn valign_bottom(mut self) -> Self {
        self.valign = CellVAlign::Bottom;
        self
    }

    /// Enable or disable text wrapping
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap_text = wrap;
        self
    }

    /// Set horizontal span (gridSpan) - this cell covers multiple columns
    pub fn grid_span(mut self, span: u32) -> Self {
        self.grid_span = Some(span);
        self
    }

    /// Set vertical span (rowSpan) - this cell covers multiple rows
    pub fn row_span(mut self, span: u32) -> Self {
        self.row_span = Some(span);
        self
    }

    /// Mark this cell as horizontally merged (covered by another cell's gridSpan)
    pub fn h_merge(mut self) -> Self {
        self.h_merge = true;
        self
    }

    /// Mark this cell as vertically merged (covered by another cell's rowSpan)
    pub fn v_merge(mut self) -> Self {
        self.v_merge = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_new() {
        let cell = TableCell::new("Hello");
        assert_eq!(cell.text, "Hello");
        assert!(!cell.bold);
        assert!(!cell.italic);
        assert_eq!(cell.align, CellAlign::Center);
        assert_eq!(cell.valign, CellVAlign::Middle);
    }

    #[test]
    fn test_cell_formatting() {
        let cell = TableCell::new("Test")
            .bold()
            .italic()
            .underline()
            .text_color("FF0000")
            .background_color("0000FF")
            .font_size(24)
            .font_family("Arial");

        assert!(cell.bold);
        assert!(cell.italic);
        assert!(cell.underline);
        assert_eq!(cell.text_color, Some("FF0000".to_string()));
        assert_eq!(cell.background_color, Some("0000FF".to_string()));
        assert_eq!(cell.font_size, Some(24));
        assert_eq!(cell.font_family, Some("Arial".to_string()));
    }

    #[test]
    fn test_cell_alignment() {
        let cell = TableCell::new("Test").align_left().valign_top();
        assert_eq!(cell.align, CellAlign::Left);
        assert_eq!(cell.valign, CellVAlign::Top);
    }

    #[test]
    fn test_cell_align_as_str() {
        assert_eq!(CellAlign::Left.as_str(), "l");
        assert_eq!(CellAlign::Center.as_str(), "ctr");
        assert_eq!(CellAlign::Right.as_str(), "r");
        assert_eq!(CellAlign::Justify.as_str(), "just");
    }

    #[test]
    fn test_cell_valign_as_str() {
        assert_eq!(CellVAlign::Top.as_str(), "t");
        assert_eq!(CellVAlign::Middle.as_str(), "ctr");
        assert_eq!(CellVAlign::Bottom.as_str(), "b");
    }
}
