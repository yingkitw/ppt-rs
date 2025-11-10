//! Chart Data Table - Display data table in chart


/// Data table display options
#[derive(Clone, Debug)]
pub struct DataTable {
    /// Show legend keys
    show_legend_keys: bool,
    /// Show horizontal border
    show_h_border: bool,
    /// Show vertical border
    show_v_border: bool,
    /// Show outline
    show_outline: bool,
}

impl DataTable {
    /// Create a new data table
    pub fn new() -> Self {
        Self {
            show_legend_keys: true,
            show_h_border: true,
            show_v_border: true,
            show_outline: true,
        }
    }

    /// Set show legend keys
    pub fn set_show_legend_keys(&mut self, show: bool) {
        self.show_legend_keys = show;
    }

    /// Get show legend keys
    pub fn show_legend_keys(&self) -> bool {
        self.show_legend_keys
    }

    /// Set show horizontal border
    pub fn set_show_h_border(&mut self, show: bool) {
        self.show_h_border = show;
    }

    /// Get show horizontal border
    pub fn show_h_border(&self) -> bool {
        self.show_h_border
    }

    /// Set show vertical border
    pub fn set_show_v_border(&mut self, show: bool) {
        self.show_v_border = show;
    }

    /// Get show vertical border
    pub fn show_v_border(&self) -> bool {
        self.show_v_border
    }

    /// Set show outline
    pub fn set_show_outline(&mut self, show: bool) {
        self.show_outline = show;
    }

    /// Get show outline
    pub fn show_outline(&self) -> bool {
        self.show_outline
    }

    /// Generate XML for data table
    pub fn to_xml(&self) -> String {
        let mut xml = String::from("<c:dTable>");
        
        if self.show_legend_keys {
            xml.push_str("<c:showLegendKey val=\"1\"/>");
        }
        if self.show_h_border {
            xml.push_str("<c:showHBorder val=\"1\"/>");
        }
        if self.show_v_border {
            xml.push_str("<c:showVBorder val=\"1\"/>");
        }
        if self.show_outline {
            xml.push_str("<c:showOutline val=\"1\"/>");
        }
        
        xml.push_str("</c:dTable>");
        xml
    }
}

impl Default for DataTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_table_creation() {
        let table = DataTable::new();
        assert!(table.show_legend_keys());
        assert!(table.show_h_border());
        assert!(table.show_v_border());
        assert!(table.show_outline());
    }

    #[test]
    fn test_data_table_toggle_options() {
        let mut table = DataTable::new();
        table.set_show_legend_keys(false);
        assert!(!table.show_legend_keys());
        
        table.set_show_h_border(false);
        assert!(!table.show_h_border());
        
        table.set_show_v_border(false);
        assert!(!table.show_v_border());
        
        table.set_show_outline(false);
        assert!(!table.show_outline());
    }

    #[test]
    fn test_data_table_xml_generation() {
        let table = DataTable::new();
        let xml = table.to_xml();
        assert!(xml.contains("<c:dTable>"));
        assert!(xml.contains("<c:showLegendKey"));
        assert!(xml.contains("</c:dTable>"));
    }

    #[test]
    fn test_data_table_xml_minimal() {
        let mut table = DataTable::new();
        table.set_show_legend_keys(false);
        table.set_show_h_border(false);
        table.set_show_v_border(false);
        table.set_show_outline(false);
        
        let xml = table.to_xml();
        assert_eq!(xml, "<c:dTable></c:dTable>");
    }
}
