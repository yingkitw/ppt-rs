//! Ink annotations for slides
//!
//! Supports freehand ink strokes on slides using the OOXML `<p:inkGrp>` element.
//! Each stroke is a series of points with pen properties (color, width).

/// Pen tip style
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum PenTip {
    #[default]
    Ball,
    Flat,
}

impl PenTip {
    pub fn to_xml(&self) -> &'static str {
        match self {
            PenTip::Ball => "ball",
            PenTip::Flat => "flat",
        }
    }
}

/// Ink pen properties
#[derive(Clone, Debug)]
pub struct InkPen {
    pub color: String,
    pub width: u32,
    pub tip: PenTip,
    pub opacity: f32,
}

impl InkPen {
    /// Create a pen with color (RGB hex) and width in hundredths of a mm
    pub fn new(color: &str, width: u32) -> Self {
        Self {
            color: color.trim_start_matches('#').to_uppercase(),
            width,
            tip: PenTip::default(),
            opacity: 1.0,
        }
    }

    pub fn tip(mut self, tip: PenTip) -> Self {
        self.tip = tip;
        self
    }

    /// Set opacity (0.0 - 1.0)
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }

    /// Default red pen
    pub fn red() -> Self {
        Self::new("FF0000", 50)
    }

    /// Default blue pen
    pub fn blue() -> Self {
        Self::new("0000FF", 50)
    }

    /// Default black pen
    pub fn black() -> Self {
        Self::new("000000", 50)
    }

    /// Highlighter (wide, semi-transparent yellow)
    pub fn highlighter() -> Self {
        Self::new("FFFF00", 300).tip(PenTip::Flat).opacity(0.5)
    }
}

/// A single point in an ink stroke
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct InkPoint {
    pub x: f64,
    pub y: f64,
}

impl InkPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A single ink stroke (continuous pen path)
#[derive(Clone, Debug)]
pub struct InkStroke {
    pub points: Vec<InkPoint>,
    pub pen: InkPen,
}

impl InkStroke {
    pub fn new(pen: InkPen) -> Self {
        Self {
            points: Vec::new(),
            pen,
        }
    }

    /// Add a point to the stroke
    pub fn add_point(mut self, x: f64, y: f64) -> Self {
        self.points.push(InkPoint::new(x, y));
        self
    }

    /// Add multiple points
    pub fn add_points(mut self, points: &[(f64, f64)]) -> Self {
        for &(x, y) in points {
            self.points.push(InkPoint::new(x, y));
        }
        self
    }

    /// Number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Generate points string for ink XML (space-separated x y pairs)
    fn points_str(&self) -> String {
        self.points
            .iter()
            .map(|p| format!("{} {}", p.x as i64, p.y as i64))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Generate XML for this stroke as an `<ink:trace>` element
    pub fn to_xml(&self, trace_id: u32) -> String {
        format!(
            "<ink:trace contextRef=\"#{}\" brushRef=\"#br{}\" id=\"{}\">{}</ink:trace>",
            trace_id,
            trace_id,
            trace_id,
            self.points_str(),
        )
    }
}

/// Collection of ink annotations on a slide
#[derive(Clone, Debug, Default)]
pub struct InkAnnotations {
    strokes: Vec<InkStroke>,
}

impl InkAnnotations {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a stroke
    pub fn add_stroke(&mut self, stroke: InkStroke) {
        self.strokes.push(stroke);
    }

    /// Get all strokes
    pub fn strokes(&self) -> &[InkStroke] {
        &self.strokes
    }

    /// Number of strokes
    pub fn len(&self) -> usize {
        self.strokes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.strokes.is_empty()
    }

    /// Clear all strokes
    pub fn clear(&mut self) {
        self.strokes.clear();
    }

    /// Generate the ink XML for embedding in a slide's `<mc:AlternateContent>` block
    pub fn to_xml(&self) -> String {
        if self.strokes.is_empty() {
            return String::new();
        }

        let mut xml = String::from(
            r#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"><mc:Choice Requires="p14"><p:contentPart xmlns:p14="http://schemas.microsoft.com/office/powerpoint/2010/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">"#,
        );
        xml.push_str(r#"<ink:ink xmlns:ink="http://www.w3.org/2003/InkML">"#);

        // Brush definitions
        for (i, stroke) in self.strokes.iter().enumerate() {
            let opacity_attr = if (stroke.pen.opacity - 1.0).abs() > 0.01 {
                format!(r#" transparency="{:.0}""#, (1.0 - stroke.pen.opacity) * 255.0)
            } else {
                String::new()
            };
            xml.push_str(&format!(
                "<ink:brush id=\"br{}\" color=\"#{}\" width=\"{}\" tip=\"{}\"{}/>\n",
                i, stroke.pen.color, stroke.pen.width, stroke.pen.tip.to_xml(), opacity_attr,
            ));
        }

        // Traces
        for (i, stroke) in self.strokes.iter().enumerate() {
            xml.push_str(&stroke.to_xml(i as u32));
        }

        xml.push_str("</ink:ink>");
        xml.push_str("</p:contentPart></mc:Choice></mc:AlternateContent>");
        xml
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pen_tip_default() {
        assert_eq!(PenTip::default(), PenTip::Ball);
        assert_eq!(PenTip::Ball.to_xml(), "ball");
        assert_eq!(PenTip::Flat.to_xml(), "flat");
    }

    #[test]
    fn test_ink_pen_new() {
        let pen = InkPen::new("FF0000", 50);
        assert_eq!(pen.color, "FF0000");
        assert_eq!(pen.width, 50);
        assert_eq!(pen.tip, PenTip::Ball);
        assert!((pen.opacity - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ink_pen_presets() {
        let red = InkPen::red();
        assert_eq!(red.color, "FF0000");
        let blue = InkPen::blue();
        assert_eq!(blue.color, "0000FF");
        let black = InkPen::black();
        assert_eq!(black.color, "000000");
    }

    #[test]
    fn test_ink_pen_highlighter() {
        let h = InkPen::highlighter();
        assert_eq!(h.color, "FFFF00");
        assert_eq!(h.tip, PenTip::Flat);
        assert!((h.opacity - 0.5).abs() < f32::EPSILON);
        assert_eq!(h.width, 300);
    }

    #[test]
    fn test_ink_pen_opacity_clamp() {
        let pen = InkPen::new("000000", 10).opacity(2.0);
        assert!((pen.opacity - 1.0).abs() < f32::EPSILON);
        let pen2 = InkPen::new("000000", 10).opacity(-1.0);
        assert!((pen2.opacity - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ink_point() {
        let p = InkPoint::new(100.0, 200.0);
        assert!((p.x - 100.0).abs() < f64::EPSILON);
        assert!((p.y - 200.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ink_stroke_new() {
        let stroke = InkStroke::new(InkPen::black());
        assert!(stroke.is_empty());
        assert_eq!(stroke.len(), 0);
    }

    #[test]
    fn test_ink_stroke_add_points() {
        let stroke = InkStroke::new(InkPen::red())
            .add_point(0.0, 0.0)
            .add_point(100.0, 100.0)
            .add_point(200.0, 50.0);
        assert_eq!(stroke.len(), 3);
        assert!(!stroke.is_empty());
    }

    #[test]
    fn test_ink_stroke_add_points_batch() {
        let stroke = InkStroke::new(InkPen::blue())
            .add_points(&[(0.0, 0.0), (50.0, 50.0), (100.0, 0.0)]);
        assert_eq!(stroke.len(), 3);
    }

    #[test]
    fn test_ink_stroke_xml() {
        let stroke = InkStroke::new(InkPen::black())
            .add_point(10.0, 20.0)
            .add_point(30.0, 40.0);
        let xml = stroke.to_xml(0);
        assert!(xml.contains("ink:trace"));
        assert!(xml.contains("10 20"));
        assert!(xml.contains("30 40"));
    }

    #[test]
    fn test_ink_annotations_new() {
        let ann = InkAnnotations::new();
        assert!(ann.is_empty());
        assert_eq!(ann.len(), 0);
    }

    #[test]
    fn test_ink_annotations_add() {
        let mut ann = InkAnnotations::new();
        ann.add_stroke(InkStroke::new(InkPen::red()).add_point(0.0, 0.0));
        ann.add_stroke(InkStroke::new(InkPen::blue()).add_point(10.0, 10.0));
        assert_eq!(ann.len(), 2);
    }

    #[test]
    fn test_ink_annotations_clear() {
        let mut ann = InkAnnotations::new();
        ann.add_stroke(InkStroke::new(InkPen::black()).add_point(0.0, 0.0));
        ann.clear();
        assert!(ann.is_empty());
    }

    #[test]
    fn test_ink_annotations_xml_empty() {
        let ann = InkAnnotations::new();
        assert_eq!(ann.to_xml(), "");
    }

    #[test]
    fn test_ink_annotations_xml() {
        let mut ann = InkAnnotations::new();
        ann.add_stroke(
            InkStroke::new(InkPen::red())
                .add_point(0.0, 0.0)
                .add_point(100.0, 100.0),
        );
        let xml = ann.to_xml();
        assert!(xml.contains("mc:AlternateContent"));
        assert!(xml.contains("ink:ink"));
        assert!(xml.contains("ink:brush"));
        assert!(xml.contains("ink:trace"));
        assert!(xml.contains("FF0000"));
    }

    #[test]
    fn test_ink_annotations_xml_highlighter() {
        let mut ann = InkAnnotations::new();
        ann.add_stroke(
            InkStroke::new(InkPen::highlighter())
                .add_point(0.0, 0.0)
                .add_point(500.0, 0.0),
        );
        let xml = ann.to_xml();
        assert!(xml.contains("FFFF00"));
        assert!(xml.contains("flat"));
        assert!(xml.contains("transparency"));
    }
}
