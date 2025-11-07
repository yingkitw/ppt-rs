//! Chart-related enumerations

/// Chart types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    // Area charts
    Area,
    AreaStacked,
    AreaStacked100,
    ThreeDArea,
    ThreeDAreaStacked,
    ThreeDAreaStacked100,
    
    // Bar charts
    BarClustered,
    BarStacked,
    BarStacked100,
    ThreeDBarClustered,
    ThreeDBarStacked,
    ThreeDBarStacked100,
    BarOfPie,
    
    // Column charts
    ColumnClustered,
    ColumnStacked,
    ColumnStacked100,
    ThreeDColumn,
    ThreeDColumnClustered,
    ThreeDColumnStacked,
    ThreeDColumnStacked100,
    
    // Line charts
    Line,
    LineMarkers,
    LineMarkersStacked,
    LineMarkersStacked100,
    LineStacked,
    LineStacked100,
    ThreeDLine,
    
    // Pie charts
    Pie,
    PieExploded,
    PieOfPie,
    ThreeDPie,
    ThreeDPieExploded,
    
    // Doughnut charts
    Doughnut,
    DoughnutExploded,
    
    // Scatter/Bubble charts
    Scatter,
    ScatterSmooth,
    ScatterSmoothMarkers,
    ScatterStraight,
    ScatterStraightMarkers,
    Bubble,
    BubbleThreeDEffect,
    
    // Radar charts
    Radar,
    RadarFilled,
    RadarMarkers,
    
    // Stock charts
    StockHLC,
    StockOHLC,
    StockVHLC,
    StockVOHLC,
    
    // Surface charts
    Surface,
    SurfaceWireframe,
    SurfaceTopView,
    SurfaceTopViewWireframe,
    
    // Cone charts
    ConeBarClustered,
    ConeBarStacked,
    ConeBarStacked100,
    ConeCol,
    ConeColClustered,
    ConeColStacked,
    ConeColStacked100,
    
    // Cylinder charts
    CylinderBarClustered,
    CylinderBarStacked,
    CylinderBarStacked100,
    CylinderCol,
    CylinderColClustered,
    CylinderColStacked,
    CylinderColStacked100,
    
    // Pyramid charts
    PyramidBarClustered,
    PyramidBarStacked,
    PyramidBarStacked100,
    PyramidCol,
    PyramidColClustered,
    PyramidColStacked,
    PyramidColStacked100,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_type_equality() {
        assert_eq!(ChartType::Area, ChartType::Area);
        assert_ne!(ChartType::Area, ChartType::BarClustered);
    }

    #[test]
    fn test_chart_type_copy() {
        let ct1 = ChartType::ColumnClustered;
        let ct2 = ct1;
        assert_eq!(ct1, ct2);
    }

    #[test]
    fn test_chart_type_debug() {
        let ct = ChartType::Pie;
        let debug_str = format!("{:?}", ct);
        assert!(debug_str.contains("Pie"));
    }

    #[test]
    fn test_all_chart_types() {
        // Test that all chart types can be created and compared
        let types = vec![
            ChartType::Area,
            ChartType::AreaStacked,
            ChartType::AreaStacked100,
            ChartType::ThreeDArea,
            ChartType::BarClustered,
            ChartType::BarStacked,
            ChartType::ColumnClustered,
            ChartType::Line,
            ChartType::LineMarkers,
            ChartType::Pie,
            ChartType::Doughnut,
            ChartType::Scatter,
            ChartType::Bubble,
            ChartType::Radar,
            ChartType::StockHLC,
            ChartType::Surface,
            ChartType::ConeBarClustered,
            ChartType::CylinderBarClustered,
            ChartType::PyramidBarClustered,
        ];
        
        for chart_type in types {
            assert_eq!(chart_type, chart_type);
        }
    }
}

