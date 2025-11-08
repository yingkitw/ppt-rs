//! Freeform Shapes - Custom path-based shapes

use crate::error::Result;

/// Path segment type
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PathSegmentType {
    /// Move to point
    MoveTo,
    /// Line to point
    LineTo,
    /// Cubic Bezier curve
    CubicBezierTo,
    /// Quadratic Bezier curve
    QuadraticBezierTo,
    /// Arc to point
    ArcTo,
    /// Close path
    Close,
}

/// Path segment
#[derive(Clone, Debug)]
pub struct PathSegment {
    /// Segment type
    segment_type: PathSegmentType,
    /// X coordinate (in EMUs)
    x: i32,
    /// Y coordinate (in EMUs)
    y: i32,
    /// Control point 1 X (for curves)
    cp1_x: Option<i32>,
    /// Control point 1 Y (for curves)
    cp1_y: Option<i32>,
    /// Control point 2 X (for curves)
    cp2_x: Option<i32>,
    /// Control point 2 Y (for curves)
    cp2_y: Option<i32>,
}

impl PathSegment {
    /// Create a move-to segment
    pub fn move_to(x: i32, y: i32) -> Self {
        Self {
            segment_type: PathSegmentType::MoveTo,
            x,
            y,
            cp1_x: None,
            cp1_y: None,
            cp2_x: None,
            cp2_y: None,
        }
    }

    /// Create a line-to segment
    pub fn line_to(x: i32, y: i32) -> Self {
        Self {
            segment_type: PathSegmentType::LineTo,
            x,
            y,
            cp1_x: None,
            cp1_y: None,
            cp2_x: None,
            cp2_y: None,
        }
    }

    /// Create a cubic Bezier segment
    pub fn cubic_bezier(cp1_x: i32, cp1_y: i32, cp2_x: i32, cp2_y: i32, x: i32, y: i32) -> Self {
        Self {
            segment_type: PathSegmentType::CubicBezierTo,
            x,
            y,
            cp1_x: Some(cp1_x),
            cp1_y: Some(cp1_y),
            cp2_x: Some(cp2_x),
            cp2_y: Some(cp2_y),
        }
    }

    /// Create a close path segment
    pub fn close() -> Self {
        Self {
            segment_type: PathSegmentType::Close,
            x: 0,
            y: 0,
            cp1_x: None,
            cp1_y: None,
            cp2_x: None,
            cp2_y: None,
        }
    }

    /// Get segment type
    pub fn segment_type(&self) -> &PathSegmentType {
        &self.segment_type
    }

    /// Generate XML for path segment
    pub fn to_xml(&self) -> String {
        match self.segment_type {
            PathSegmentType::MoveTo => {
                format!(r#"<a:moveTo><a:pt x="{}" y="{}"/></a:moveTo>"#, self.x, self.y)
            }
            PathSegmentType::LineTo => {
                format!(r#"<a:lnTo><a:pt x="{}" y="{}"/></a:lnTo>"#, self.x, self.y)
            }
            PathSegmentType::CubicBezierTo => {
                format!(
                    r#"<a:cubicBezTo><a:pt x="{}" y="{}"/><a:pt x="{}" y="{}"/><a:pt x="{}" y="{}"/></a:cubicBezTo>"#,
                    self.cp1_x.unwrap_or(0),
                    self.cp1_y.unwrap_or(0),
                    self.cp2_x.unwrap_or(0),
                    self.cp2_y.unwrap_or(0),
                    self.x,
                    self.y
                )
            }
            PathSegmentType::Close => r#"<a:close/>"#.to_string(),
            _ => String::new(),
        }
    }
}

/// Freeform shape
#[derive(Clone, Debug)]
pub struct Freeform {
    /// Path segments
    segments: Vec<PathSegment>,
    /// Shape width
    width: i32,
    /// Shape height
    height: i32,
}

impl Freeform {
    /// Create a new freeform shape
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            segments: vec![],
            width,
            height,
        }
    }

    /// Add a path segment
    pub fn add_segment(&mut self, segment: PathSegment) {
        self.segments.push(segment);
    }

    /// Add move-to segment
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.add_segment(PathSegment::move_to(x, y));
    }

    /// Add line-to segment
    pub fn line_to(&mut self, x: i32, y: i32) {
        self.add_segment(PathSegment::line_to(x, y));
    }

    /// Add cubic Bezier segment
    pub fn cubic_bezier(&mut self, cp1_x: i32, cp1_y: i32, cp2_x: i32, cp2_y: i32, x: i32, y: i32) {
        self.add_segment(PathSegment::cubic_bezier(cp1_x, cp1_y, cp2_x, cp2_y, x, y));
    }

    /// Close the path
    pub fn close(&mut self) {
        self.add_segment(PathSegment::close());
    }

    /// Get all segments
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Get width
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Generate XML for path
    pub fn to_xml(&self) -> String {
        let mut xml = String::new();
        xml.push_str(&format!(r#"<a:custGeom><a:avLst/><a:gdLst/><a:ahLst/><a:cxnSpLst/><a:prstTxBody/><a:pathLst><a:path w="{}" h="{}">"#, self.width, self.height));
        xml.push('\n');

        for segment in &self.segments {
            xml.push_str(&segment.to_xml());
            xml.push('\n');
        }

        xml.push_str(r#"</a:path></a:pathLst></a:custGeom>"#);
        xml
    }
}

/// Freeform Manager
#[derive(Clone, Debug)]
pub struct FreeformManager {
    /// Freeforms
    freeforms: Vec<Freeform>,
}

impl FreeformManager {
    /// Create a new freeform manager
    pub fn new() -> Self {
        Self {
            freeforms: vec![],
        }
    }

    /// Add a freeform
    pub fn add_freeform(&mut self, freeform: Freeform) -> usize {
        self.freeforms.push(freeform);
        self.freeforms.len() - 1
    }

    /// Create and add a new freeform
    pub fn create_freeform(&mut self, width: i32, height: i32) -> usize {
        self.add_freeform(Freeform::new(width, height))
    }

    /// Get freeform by index
    pub fn get(&self, index: usize) -> Option<&Freeform> {
        self.freeforms.get(index)
    }

    /// Get mutable freeform by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Freeform> {
        self.freeforms.get_mut(index)
    }

    /// Get all freeforms
    pub fn all(&self) -> &[Freeform] {
        &self.freeforms
    }

    /// Get number of freeforms
    pub fn len(&self) -> usize {
        self.freeforms.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.freeforms.is_empty()
    }
}

impl Default for FreeformManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_segment_move_to() {
        let seg = PathSegment::move_to(100, 200);
        assert_eq!(seg.segment_type, PathSegmentType::MoveTo);
        assert_eq!(seg.x, 100);
        assert_eq!(seg.y, 200);
    }

    #[test]
    fn test_path_segment_line_to() {
        let seg = PathSegment::line_to(300, 400);
        assert_eq!(seg.segment_type, PathSegmentType::LineTo);
    }

    #[test]
    fn test_path_segment_cubic_bezier() {
        let seg = PathSegment::cubic_bezier(100, 100, 200, 200, 300, 300);
        assert_eq!(seg.segment_type, PathSegmentType::CubicBezierTo);
        assert_eq!(seg.cp1_x, Some(100));
        assert_eq!(seg.cp1_y, Some(100));
        assert_eq!(seg.cp2_x, Some(200));
        assert_eq!(seg.cp2_y, Some(200));
    }

    #[test]
    fn test_path_segment_close() {
        let seg = PathSegment::close();
        assert_eq!(seg.segment_type, PathSegmentType::Close);
    }

    #[test]
    fn test_path_segment_to_xml() {
        let seg = PathSegment::move_to(100, 200);
        let xml = seg.to_xml();
        assert!(xml.contains(r#"<a:moveTo>"#));
        assert!(xml.contains(r#"x="100""#));
        assert!(xml.contains(r#"y="200""#));
    }

    #[test]
    fn test_freeform_creation() {
        let freeform = Freeform::new(1000, 1000);
        assert_eq!(freeform.width(), 1000);
        assert_eq!(freeform.height(), 1000);
        assert_eq!(freeform.segments().len(), 0);
    }

    #[test]
    fn test_freeform_add_segments() {
        let mut freeform = Freeform::new(1000, 1000);
        freeform.move_to(100, 100);
        freeform.line_to(200, 200);
        freeform.close();

        assert_eq!(freeform.segments().len(), 3);
    }

    #[test]
    fn test_freeform_to_xml() {
        let mut freeform = Freeform::new(1000, 1000);
        freeform.move_to(100, 100);
        freeform.line_to(200, 200);
        freeform.close();

        let xml = freeform.to_xml();
        assert!(xml.contains(r#"<a:custGeom>"#));
        assert!(xml.contains(r#"w="1000""#));
        assert!(xml.contains(r#"h="1000""#));
        assert!(xml.contains(r#"<a:moveTo>"#));
        assert!(xml.contains(r#"<a:lnTo>"#));
        assert!(xml.contains(r#"<a:close/>"#));
    }

    #[test]
    fn test_freeform_manager_creation() {
        let manager = FreeformManager::new();
        assert!(manager.is_empty());
    }

    #[test]
    fn test_freeform_manager_add() {
        let mut manager = FreeformManager::new();
        let freeform = Freeform::new(1000, 1000);
        manager.add_freeform(freeform);

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_freeform_manager_create() {
        let mut manager = FreeformManager::new();
        manager.create_freeform(1000, 1000);

        assert_eq!(manager.len(), 1);
    }

    #[test]
    fn test_freeform_manager_get() {
        let mut manager = FreeformManager::new();
        manager.create_freeform(1000, 1000);

        let freeform = manager.get(0);
        assert!(freeform.is_some());
        assert_eq!(freeform.unwrap().width(), 1000);
    }

    #[test]
    fn test_freeform_manager_get_mut() {
        let mut manager = FreeformManager::new();
        manager.create_freeform(1000, 1000);

        if let Some(freeform) = manager.get_mut(0) {
            freeform.move_to(100, 100);
        }

        assert_eq!(manager.get(0).unwrap().segments().len(), 1);
    }

    #[test]
    fn test_freeform_manager_default() {
        let manager = FreeformManager::default();
        assert!(manager.is_empty());
    }
}
