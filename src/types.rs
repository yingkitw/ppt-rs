//! Abstract types and traits used by pptx library

use crate::util::Length;

/// A trait for objects that have width and height extents
pub trait ProvidesExtents {
    /// Distance between left and right extents of shape in EMUs
    fn width(&self) -> Length;

    /// Distance between top and bottom extents of shape in EMUs
    fn height(&self) -> Length;
}

/// A trait for objects that provide access to their XmlPart
pub trait ProvidesPart {
    /// Get the XmlPart for this object
    fn part(&self) -> &dyn std::any::Any;
}
