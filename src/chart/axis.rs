//! Chart axis functionality

use crate::text::TextFrame;

/// Base axis trait - common properties for all axis types
pub trait Axis {
    /// Check if axis has a title
    fn has_title(&self) -> bool;
    
    /// Set whether axis has a title
    fn set_has_title(&mut self, has_title: bool);
    
    /// Get axis title text frame
    fn title_text_frame(&self) -> Option<&TextFrame>;
    
    /// Get mutable axis title text frame
    fn title_text_frame_mut(&mut self) -> Option<&mut TextFrame>;
    
    /// Check if axis has major gridlines
    fn has_major_gridlines(&self) -> bool;
    
    /// Set whether axis has major gridlines
    fn set_has_major_gridlines(&mut self, has_major: bool);
    
    /// Check if axis has minor gridlines
    fn has_minor_gridlines(&self) -> bool;
    
    /// Set whether axis has minor gridlines
    fn set_has_minor_gridlines(&mut self, has_minor: bool);
    
    /// Get minimum scale value
    fn minimum_scale(&self) -> Option<f64>;
    
    /// Set minimum scale value
    fn set_minimum_scale(&mut self, min: Option<f64>);
    
    /// Get maximum scale value
    fn maximum_scale(&self) -> Option<f64>;
    
    /// Set maximum scale value
    fn set_maximum_scale(&mut self, max: Option<f64>);
    
    /// Check if axis is visible
    fn visible(&self) -> bool;
    
    /// Set axis visibility
    fn set_visible(&mut self, visible: bool);
}

/// Category axis - for discrete categories
pub struct CategoryAxis {
    has_title: bool,
    title_text_frame: Option<TextFrame>,
    has_major_gridlines: bool,
    has_minor_gridlines: bool,
    minimum_scale: Option<f64>,
    maximum_scale: Option<f64>,
    visible: bool,
    reverse_order: bool,
}

impl CategoryAxis {
    /// Create a new category axis
    pub fn new() -> Self {
        Self {
            has_title: false,
            title_text_frame: None,
            has_major_gridlines: false,
            has_minor_gridlines: false,
            minimum_scale: None,
            maximum_scale: None,
            visible: true,
            reverse_order: false,
        }
    }

    /// Check if categories are in reverse order
    pub fn reverse_order(&self) -> bool {
        self.reverse_order
    }

    /// Set reverse order
    pub fn set_reverse_order(&mut self, reverse: bool) {
        self.reverse_order = reverse;
    }
}

impl Axis for CategoryAxis {
    fn has_title(&self) -> bool {
        self.has_title
    }

    fn set_has_title(&mut self, has_title: bool) {
        self.has_title = has_title;
        if has_title && self.title_text_frame.is_none() {
            self.title_text_frame = Some(TextFrame::new());
        } else if !has_title {
            self.title_text_frame = None;
        }
    }

    fn title_text_frame(&self) -> Option<&TextFrame> {
        self.title_text_frame.as_ref()
    }

    fn title_text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        self.title_text_frame.as_mut()
    }

    fn has_major_gridlines(&self) -> bool {
        self.has_major_gridlines
    }

    fn set_has_major_gridlines(&mut self, has_major: bool) {
        self.has_major_gridlines = has_major;
    }

    fn has_minor_gridlines(&self) -> bool {
        self.has_minor_gridlines
    }

    fn set_has_minor_gridlines(&mut self, has_minor: bool) {
        self.has_minor_gridlines = has_minor;
    }

    fn minimum_scale(&self) -> Option<f64> {
        self.minimum_scale
    }

    fn set_minimum_scale(&mut self, min: Option<f64>) {
        self.minimum_scale = min;
    }

    fn maximum_scale(&self) -> Option<f64> {
        self.maximum_scale
    }

    fn set_maximum_scale(&mut self, max: Option<f64>) {
        self.maximum_scale = max;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Value axis - for continuous numeric values
pub struct ValueAxis {
    has_title: bool,
    title_text_frame: Option<TextFrame>,
    has_major_gridlines: bool,
    has_minor_gridlines: bool,
    minimum_scale: Option<f64>,
    maximum_scale: Option<f64>,
    visible: bool,
    major_unit: Option<f64>,
    minor_unit: Option<f64>,
    crosses_at: Option<f64>,
}

impl ValueAxis {
    /// Create a new value axis
    pub fn new() -> Self {
        Self {
            has_title: false,
            title_text_frame: None,
            has_major_gridlines: false,
            has_minor_gridlines: false,
            minimum_scale: None,
            maximum_scale: None,
            visible: true,
            major_unit: None,
            minor_unit: None,
            crosses_at: None,
        }
    }

    /// Get major unit (spacing between major tick marks)
    pub fn major_unit(&self) -> Option<f64> {
        self.major_unit
    }

    /// Set major unit
    pub fn set_major_unit(&mut self, unit: Option<f64>) {
        self.major_unit = unit;
    }

    /// Get minor unit (spacing between minor tick marks)
    pub fn minor_unit(&self) -> Option<f64> {
        self.minor_unit
    }

    /// Set minor unit
    pub fn set_minor_unit(&mut self, unit: Option<f64>) {
        self.minor_unit = unit;
    }

    /// Get the value where the perpendicular axis crosses
    pub fn crosses_at(&self) -> Option<f64> {
        self.crosses_at
    }

    /// Set the value where the perpendicular axis crosses
    pub fn set_crosses_at(&mut self, value: Option<f64>) {
        self.crosses_at = value;
    }
}

impl Axis for ValueAxis {
    fn has_title(&self) -> bool {
        self.has_title
    }

    fn set_has_title(&mut self, has_title: bool) {
        self.has_title = has_title;
        if has_title && self.title_text_frame.is_none() {
            self.title_text_frame = Some(TextFrame::new());
        } else if !has_title {
            self.title_text_frame = None;
        }
    }

    fn title_text_frame(&self) -> Option<&TextFrame> {
        self.title_text_frame.as_ref()
    }

    fn title_text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        self.title_text_frame.as_mut()
    }

    fn has_major_gridlines(&self) -> bool {
        self.has_major_gridlines
    }

    fn set_has_major_gridlines(&mut self, has_major: bool) {
        self.has_major_gridlines = has_major;
    }

    fn has_minor_gridlines(&self) -> bool {
        self.has_minor_gridlines
    }

    fn set_has_minor_gridlines(&mut self, has_minor: bool) {
        self.has_minor_gridlines = has_minor;
    }

    fn minimum_scale(&self) -> Option<f64> {
        self.minimum_scale
    }

    fn set_minimum_scale(&mut self, min: Option<f64>) {
        self.minimum_scale = min;
    }

    fn maximum_scale(&self) -> Option<f64> {
        self.maximum_scale
    }

    fn set_maximum_scale(&mut self, max: Option<f64>) {
        self.maximum_scale = max;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

/// Date axis - category axis with dates as category labels
pub struct DateAxis {
    category_axis: CategoryAxis,
}

impl DateAxis {
    /// Create a new date axis
    pub fn new() -> Self {
        Self {
            category_axis: CategoryAxis::new(),
        }
    }
}

impl Axis for DateAxis {
    fn has_title(&self) -> bool {
        self.category_axis.has_title()
    }

    fn set_has_title(&mut self, has_title: bool) {
        self.category_axis.set_has_title(has_title);
    }

    fn title_text_frame(&self) -> Option<&TextFrame> {
        self.category_axis.title_text_frame()
    }

    fn title_text_frame_mut(&mut self) -> Option<&mut TextFrame> {
        self.category_axis.title_text_frame_mut()
    }

    fn has_major_gridlines(&self) -> bool {
        self.category_axis.has_major_gridlines()
    }

    fn set_has_major_gridlines(&mut self, has_major: bool) {
        self.category_axis.set_has_major_gridlines(has_major);
    }

    fn has_minor_gridlines(&self) -> bool {
        self.category_axis.has_minor_gridlines()
    }

    fn set_has_minor_gridlines(&mut self, has_minor: bool) {
        self.category_axis.set_has_minor_gridlines(has_minor);
    }

    fn minimum_scale(&self) -> Option<f64> {
        self.category_axis.minimum_scale()
    }

    fn set_minimum_scale(&mut self, min: Option<f64>) {
        self.category_axis.set_minimum_scale(min);
    }

    fn maximum_scale(&self) -> Option<f64> {
        self.category_axis.maximum_scale()
    }

    fn set_maximum_scale(&mut self, max: Option<f64>) {
        self.category_axis.set_maximum_scale(max);
    }

    fn visible(&self) -> bool {
        self.category_axis.visible()
    }

    fn set_visible(&mut self, visible: bool) {
        self.category_axis.set_visible(visible);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_axis_new() {
        let axis = CategoryAxis::new();
        assert!(!axis.has_title());
        assert!(!axis.has_major_gridlines());
        assert!(axis.visible());
    }

    #[test]
    fn test_category_axis_title() {
        let mut axis = CategoryAxis::new();
        axis.set_has_title(true);
        assert!(axis.has_title());
        assert!(axis.title_text_frame().is_some());
    }

    #[test]
    fn test_value_axis_new() {
        let axis = ValueAxis::new();
        assert!(!axis.has_title());
        assert!(axis.major_unit().is_none());
        assert!(axis.crosses_at().is_none());
    }

    #[test]
    fn test_value_axis_units() {
        let mut axis = ValueAxis::new();
        axis.set_major_unit(Some(10.0));
        axis.set_minor_unit(Some(5.0));
        axis.set_crosses_at(Some(0.0));
        
        assert_eq!(axis.major_unit(), Some(10.0));
        assert_eq!(axis.minor_unit(), Some(5.0));
        assert_eq!(axis.crosses_at(), Some(0.0));
    }

    #[test]
    fn test_date_axis_new() {
        let axis = DateAxis::new();
        assert!(!axis.has_title());
        assert!(axis.visible());
    }
}

