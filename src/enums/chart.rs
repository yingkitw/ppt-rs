//! Chart-related enumerations

/// Chart types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Area,
    AreaStacked,
    AreaStacked100,
    BarClustered,
    BarStacked,
    BarStacked100,
    ColumnClustered,
    ColumnStacked,
    ColumnStacked100,
    Doughnut,
    Line,
    LineMarkers,
    Pie,
    Scatter,
    Surface,
    // TODO: Add more chart types
}

