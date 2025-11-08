//! Chart Shape - Chart rendering for slides

use crate::chart::Chart;
use crate::enums::chart::ChartType;

/// Chart shape for rendering charts on slides
#[derive(Clone, Debug)]
pub struct ChartShape {
    /// Shape ID
    id: u32,
    /// Shape name
    name: String,
    /// Chart data
    chart: Chart,
    /// Position X (EMU)
    x: i32,
    /// Position Y (EMU)
    y: i32,
    /// Width (EMU)
    width: u32,
    /// Height (EMU)
    height: u32,
    /// Relationship ID
    rel_id: String,
}

impl ChartShape {
    /// Create a new chart shape
    pub fn new(id: u32, name: String, chart: Chart, x: i32, y: i32, width: u32, height: u32, rel_id: String) -> Self {
        Self {
            id,
            name,
            chart,
            x,
            y,
            width,
            height,
            rel_id,
        }
    }

    /// Get shape ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Get shape name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get chart
    pub fn chart(&self) -> &Chart {
        &self.chart
    }

    /// Get mutable chart
    pub fn chart_mut(&mut self) -> &mut Chart {
        &mut self.chart
    }

    /// Get position X
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Get position Y
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Get width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Get relationship ID
    pub fn rel_id(&self) -> &str {
        &self.rel_id
    }

    /// Generate XML for chart shape (graphic frame)
    pub fn to_xml(&self) -> String {
        format!(
            r#"<p:graphicFrame><p:nvGraphicFramePr><p:cNvPr id="{}" name="{}"/><p:cNvGraphicFramePr/><p:nvPr/></p:nvGraphicFramePr><p:xfrm><a:off x="{}" y="{}"/><a:ext cx="{}" cy="{}"/></p:xfrm><a:graphic><a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart"><c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" r:id="{}"/></a:graphicData></a:graphic></p:graphicFrame>"#,
            self.id, self.name, self.x, self.y, self.width, self.height, self.rel_id
        )
    }
}

/// Chart shape manager
#[derive(Clone, Debug)]
pub struct ChartShapeManager {
    shapes: Vec<ChartShape>,
    next_id: u32,
}

impl ChartShapeManager {
    /// Create a new chart shape manager
    pub fn new() -> Self {
        Self {
            shapes: vec![],
            next_id: 2,
        }
    }

    /// Add a chart shape
    pub fn add_shape(&mut self, shape: ChartShape) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    /// Create and add a chart shape
    pub fn create_shape(&mut self, name: String, chart: Chart, x: i32, y: i32, width: u32, height: u32, rel_id: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.add_shape(ChartShape::new(id, name, chart, x, y, width, height, rel_id))
    }

    /// Get shape by index
    pub fn get(&self, index: usize) -> Option<&ChartShape> {
        self.shapes.get(index)
    }

    /// Get mutable shape by index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ChartShape> {
        self.shapes.get_mut(index)
    }

    /// Get all shapes
    pub fn all(&self) -> &[ChartShape] {
        &self.shapes
    }

    /// Get shape count
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.shapes.is_empty()
    }

    /// Generate XML for all shapes
    pub fn to_xml(&self) -> String {
        self.shapes.iter().map(|s| s.to_xml()).collect()
    }
}

impl Default for ChartShapeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_shape_creation() {
        let chart = Chart::new(ChartType::Area);
        let shape = ChartShape::new(2, "Chart".to_string(), chart, 100, 200, 2000, 1500, "rId3".to_string());
        assert_eq!(shape.id(), 2);
        assert_eq!(shape.name(), "Chart");
        assert_eq!(shape.rel_id(), "rId3");
    }

    #[test]
    fn test_chart_shape_to_xml() {
        let chart = Chart::new(ChartType::Area);
        let shape = ChartShape::new(2, "Chart".to_string(), chart, 100, 200, 2000, 1500, "rId3".to_string());
        let xml = shape.to_xml();
        assert!(xml.contains(r#"<p:graphicFrame>"#));
        assert!(xml.contains(r#"id="2""#));
        assert!(xml.contains(r#"r:id="rId3""#));
    }

    #[test]
    fn test_chart_shape_manager() {
        let mut manager = ChartShapeManager::new();
        let chart = Chart::new(ChartType::Area);
        manager.create_shape("Chart".to_string(), chart, 100, 200, 2000, 1500, "rId3".to_string());
        assert_eq!(manager.len(), 1);
        assert_eq!(manager.get(0).unwrap().name(), "Chart");
    }
}
