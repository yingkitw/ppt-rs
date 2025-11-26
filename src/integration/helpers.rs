//! Utility functions for presentation generation

/// Utility functions for length conversions
pub mod utils {
    use crate::util;

    /// Convert inches to EMU
    pub fn inches_to_emu(inches: f64) -> i32 {
        util::inches(inches).into()
    }

    /// Convert centimeters to EMU
    pub fn cm_to_emu(cm: f64) -> i32 {
        util::cm(cm).into()
    }

    /// Convert points to EMU
    pub fn pt_to_emu(pt: f64) -> i32 {
        util::pt(pt).into()
    }

    /// Format file size in human-readable format
    pub fn format_size(bytes: usize) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        }
    }
}

/// Enumeration helpers
pub mod enum_helpers {
    use crate::enums;

    /// Get action type description
    pub fn action_description(action: &enums::base::BaseEnum) -> String {
        format!("{} ({})", action.name, action.value)
    }

    /// Get chart type description
    pub fn chart_description(chart: &enums::base::BaseEnum) -> String {
        format!("{} ({})", chart.name, chart.value)
    }
}
