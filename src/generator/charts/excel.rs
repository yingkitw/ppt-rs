use rust_xlsxwriter::Workbook;
use crate::generator::charts::data::Chart;
use crate::generator::charts::ChartType;

/// Trait for Excel workbook writers that generate chart data
pub trait ExcelWriter {
    /// Generate Excel workbook bytes for the chart data
    fn generate_excel_bytes(&self, chart: &Chart) -> Vec<u8>;
    
    /// Get the Excel reference for categories
    fn categories_ref(&self) -> String;
    
    /// Get the Excel reference for categories with specific range
    fn categories_ref_with_range(&self, start_row: u32, end_row: u32) -> String;
    
    /// Get the Excel reference for series values
    fn values_ref(&self, series_index: usize) -> String;
    
    /// Get the Excel reference for series values with specific range
    fn values_ref_with_range(&self, series_index: usize, start_row: u32, end_row: u32) -> String;
    
    /// Get the Excel reference for series name
    fn series_name_ref(&self, series_index: usize) -> String;
    
    /// Get the Excel reference for bubble sizes (for bubble charts)
    fn bubble_sizes_ref(&self, series_index: usize) -> String {
        // Default implementation - bubble charts should override this
        let col = (series_index + 2) as u16; // Column C for bubble sizes
        let col_letter = column_letter(col);
        format!("{}!${col_letter}$2:${col_letter}$100", self.worksheet_name())
    }
    
    /// Get the Excel reference for bubble sizes with specific range (for bubble charts)
    fn bubble_sizes_ref_with_range(&self, series_index: usize, start_row: u32, end_row: u32) -> String {
        // Default implementation - bubble charts should override this
        let col = (series_index + 2) as u16; // Column C for bubble sizes
        let col_letter = column_letter(col);
        format!("{}!${col_letter}${}:${col_letter}${}", self.worksheet_name(), start_row as usize, end_row as usize)
    }
    
    /// Get the worksheet name for this chart (default: Sheet1)
    fn worksheet_name(&self) -> String {
        "Sheet1".to_string()
    }
}

/// Excel writer for category-based charts (bar, line, pie, area, etc.)
pub struct CategoryExcelWriter {
    pub chart: Option<&'static Chart>,
    pub worksheet_name: String,
}

impl CategoryExcelWriter {
    /// Create a new category Excel writer with default worksheet name
    pub fn new() -> Self {
        CategoryExcelWriter {
            chart: None,
            worksheet_name: "Sheet1".to_string(),
        }
    }
    
    /// Create a new category Excel writer with specific worksheet name
    pub fn with_worksheet_name(name: String) -> Self {
        CategoryExcelWriter {
            chart: None,
            worksheet_name: name,
        }
    }
}

impl ExcelWriter for CategoryExcelWriter {
    fn generate_excel_bytes(&self, chart: &Chart) -> Vec<u8> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        // Write categories
        if !chart.categories.is_empty() {
            for (i, category) in chart.categories.iter().enumerate() {
                worksheet.write_string((i + 1) as u32, 0, category).unwrap();
            }
        }
        
        // Write series data
        for (series_idx, series) in chart.series.iter().enumerate() {
            let col = (series_idx + 1) as u16;
            
            // Write series name
            worksheet.write_string(0, col, &series.name).unwrap();
            
            // Write series values
            for (i, value) in series.values.iter().enumerate() {
                worksheet.write_number((i + 1) as u32, col, *value).unwrap();
            }
        }
        
        // Get Excel bytes
        workbook.save_to_buffer().unwrap()
    }
    
    fn categories_ref(&self) -> String {
        // For category charts, we need to know the actual data size
        // This will be handled by the caller with proper context
        println!("DEBUG: categories_ref called (old method) for worksheet '{}'", self.worksheet_name());
        format!("{}!$A$2:$A$100", self.worksheet_name())
    }
    
    fn categories_ref_with_range(&self, start_row: u32, end_row: u32) -> String {
        println!("DEBUG: categories_ref_with_range called with start_row={}, end_row={}", start_row, end_row);
        format!("{}!$A${}:$A${}", self.worksheet_name(), start_row, end_row)
    }
    
    fn worksheet_name(&self) -> String {
        self.worksheet_name.clone()
    }
    
    fn values_ref(&self, series_index: usize) -> String {
        let col = (series_index + 2) as u16; // +2 because column A is categories, B is first series values
        let col_letter = column_letter(col);
        println!("DEBUG: values_ref called (old method) for series_index={}", series_index);
        format!("{}!${col_letter}$2:${col_letter}$100", self.worksheet_name())
    }
    
    fn values_ref_with_range(&self, series_index: usize, start_row: u32, end_row: u32) -> String {
        let col = (series_index + 2) as u16; // +2 because column A is categories, B is first series values
        let col_letter = column_letter(col);
        println!("DEBUG: values_ref_with_range called for series_index={}, start_row={}, end_row={}, col={}, col_letter='{}'", series_index, start_row, end_row, col, col_letter);
        format!("{}!${col_letter}${}:${col_letter}${}", self.worksheet_name(), start_row as usize, end_row as usize)
    }
    
    fn series_name_ref(&self, series_index: usize) -> String {
        let col = (series_index + 2) as u16; // +2 because column A is categories, B is first series values
        let col_letter = column_letter(col);
        format!("{}!${col_letter}$1", self.worksheet_name())
    }
}

/// Excel writer for XY charts (scatter, bubble)
pub struct XyExcelWriter {
    pub worksheet_name: String,
}

/// Excel writer for bubble charts
pub struct BubbleExcelWriter {
    pub worksheet_name: String,
}

impl XyExcelWriter {
    /// Create a new XY Excel writer with default worksheet name
    pub fn new() -> Self {
        XyExcelWriter {
            worksheet_name: "Sheet1".to_string(),
        }
    }
    
    /// Create a new XY Excel writer with specific worksheet name
    pub fn with_worksheet_name(name: String) -> Self {
        XyExcelWriter {
            worksheet_name: name,
        }
    }
}

impl ExcelWriter for XyExcelWriter {
    fn generate_excel_bytes(&self, chart: &Chart) -> Vec<u8> {
        let mut workbook = Workbook::new();
        let worksheet = workbook.add_worksheet();
        
        for (series_idx, series) in chart.series.iter().enumerate() {
            let offset = series_idx * 3; // 3 rows per series (header + data)
            
            // Write series name in column B header
            worksheet.write_string(offset as u32, 1, &series.name).unwrap();
            
            // Write X and Y values
            if let Some(x_values) = &series.x_values {
                for (i, (x, y)) in x_values.iter().zip(&series.values).enumerate() {
                    let row = (offset + i + 1) as u32;
                    worksheet.write_number(row, 0, *x).unwrap();
                    worksheet.write_number(row, 1, *y).unwrap();
                }
            }
        }
        
        workbook.save_to_buffer().unwrap()
    }
    
    fn categories_ref(&self) -> String {
        format!("{}!$A$2:$A$100", self.worksheet_name()) // Default range
    }
    
    fn categories_ref_with_range(&self, start_row: u32, end_row: u32) -> String {
        format!("{}!$A${}:$A${}", self.worksheet_name(), start_row, end_row)
    }
    
    fn worksheet_name(&self) -> String {
        self.worksheet_name.clone()
    }
    
    fn values_ref(&self, series_index: usize) -> String {
        let offset = series_index * 3;
        format!("{}!$B${}:$B${}", self.worksheet_name(), offset + 2, offset + 100)
    }
    
    fn values_ref_with_range(&self, series_index: usize, start_row: u32, end_row: u32) -> String {
        let offset = series_index * 3;
        format!("{}!$B${}:$B${}", self.worksheet_name(), offset + start_row as usize, offset + end_row as usize)
    }
    
    fn series_name_ref(&self, series_index: usize) -> String {
        let offset = series_index * 3;
        format!("{}!$B${}", self.worksheet_name(), offset + 1)
    }
}

impl BubbleExcelWriter {
    /// Create a new bubble Excel writer with default worksheet name
    pub fn new() -> Self {
        BubbleExcelWriter {
            worksheet_name: "Sheet1".to_string(),
        }
    }
    
    /// Create a new bubble Excel writer with specific worksheet name
    pub fn with_worksheet_name(name: String) -> Self {
        BubbleExcelWriter {
            worksheet_name: name,
        }
    }
}

impl ExcelWriter for BubbleExcelWriter {
    fn generate_excel_bytes(&self, chart: &Chart) -> Vec<u8> {
        generate_bubble_excel(chart)
    }
    
    fn categories_ref(&self) -> String {
        format!("{}!$A$2:$A$100", self.worksheet_name()) // Default range for X values
    }
    
    fn categories_ref_with_range(&self, start_row: u32, end_row: u32) -> String {
        format!("{}!$A${}:$A${}", self.worksheet_name(), start_row, end_row)
    }
    
    fn values_ref(&self, series_index: usize) -> String {
        let offset = series_index * 4;
        format!("{}!$B${}:$B${}", self.worksheet_name(), offset + 2, offset + 100)
    }
    
    fn values_ref_with_range(&self, series_index: usize, start_row: u32, end_row: u32) -> String {
        let offset = series_index * 4;
        format!("{}!$B${}:$B${}", self.worksheet_name(), offset + start_row as usize, offset + end_row as usize)
    }
    
    fn series_name_ref(&self, series_index: usize) -> String {
        let offset = series_index * 4;
        format!("{}!$B${}", self.worksheet_name(), offset + 1)
    }
    
    fn worksheet_name(&self) -> String {
        self.worksheet_name.clone()
    }
}

/// Convert column number to Excel letter (1 -> A, 2 -> B, etc.)
fn column_letter(col_num: u16) -> String {
    let mut result = String::new();
    let mut n = col_num;
    
    while n > 0 {
        n -= 1;
        result = format!("{}{}", ((n % 26) as u8 + b'A') as char, result);
        n /= 26;
    }
    
    result
}

/// Generate Excel workbook bytes based on chart type
pub fn generate_excel_for_chart(chart: &Chart) -> Vec<u8> {
    generate_excel_for_chart_with_name(chart, "Sheet1".to_string())
}

/// Generate Excel workbook bytes based on chart type with specific worksheet name
pub fn generate_excel_for_chart_with_name(chart: &Chart, worksheet_name: String) -> Vec<u8> {
    match chart.chart_type {
        ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => {
            let writer = XyExcelWriter::with_worksheet_name(worksheet_name);
            writer.generate_excel_bytes(chart)
        }
        ChartType::Bubble => {
            // For bubble charts, we need to include bubble sizes
            generate_bubble_excel_with_name(chart, worksheet_name)
        }
        _ => {
            // Category-based charts
            let writer = CategoryExcelWriter::with_worksheet_name(worksheet_name);
            writer.generate_excel_bytes(chart)
        }
    }
}

/// Get the appropriate Excel writer for the chart type
pub fn get_excel_writer(chart_type: &ChartType) -> Box<dyn ExcelWriter> {
    get_excel_writer_with_name(chart_type, "Sheet1".to_string())
}

/// Get the appropriate Excel writer for the chart type with specific worksheet name
pub fn get_excel_writer_with_name(chart_type: &ChartType, worksheet_name: String) -> Box<dyn ExcelWriter> {
    match chart_type {
        ChartType::Scatter | ChartType::ScatterLines | ChartType::ScatterSmooth => {
            Box::new(XyExcelWriter::with_worksheet_name(worksheet_name))
        }
        ChartType::Bubble => {
            Box::new(BubbleExcelWriter::with_worksheet_name(worksheet_name))
        }
        _ => {
            // Category-based charts
            Box::new(CategoryExcelWriter::with_worksheet_name(worksheet_name))
        }
    }
}

/// Generate Excel for bubble charts (includes bubble sizes)
fn generate_bubble_excel(chart: &Chart) -> Vec<u8> {
    generate_bubble_excel_with_name(chart, "Sheet1".to_string())
}

/// Generate Excel for bubble charts with specific worksheet name
fn generate_bubble_excel_with_name(chart: &Chart, worksheet_name: String) -> Vec<u8> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name(&worksheet_name).unwrap();
    
    for (series_idx, series) in chart.series.iter().enumerate() {
        let offset = series_idx * 4; // 4 rows per series (header + data)
        
        // Write series name in column B header
        worksheet.write_string(offset as u32, 1, &series.name).unwrap();
        worksheet.write_string((offset + 1) as u32, 2, "Size").unwrap();
        
        // Write X, Y, and bubble size values
        if let Some(x_values) = &series.x_values {
            for (i, (x, y)) in x_values.iter().zip(&series.values).enumerate() {
                let row = (offset + i + 2) as u32;
                worksheet.write_number(row, 0, *x).unwrap();
                worksheet.write_number(row, 1, *y).unwrap();
                
                // Add bubble size if available
                if let Some(bubble_sizes) = &series.bubble_sizes {
                    if let Some(size) = bubble_sizes.get(i) {
                        worksheet.write_number(row, 2, *size).unwrap();
                    }
                }
            }
        }
    }
    
    workbook.save_to_buffer().unwrap()
}

/// Generate worksheet name based on chart number
pub fn worksheet_name_for_chart(chart_number: usize) -> String {
    if chart_number == 1 {
        "Sheet1".to_string()
    } else {
        format!("Sheet{}", chart_number)
    }
}

/// Generate Excel bytes for a chart based on its type with specific chart number
pub fn generate_excel_bytes_for_chart(chart: &Chart, chart_number: usize) -> Vec<u8> {
    let worksheet_name = worksheet_name_for_chart(chart_number);
    let writer = get_excel_writer_with_name(&chart.chart_type, worksheet_name);
    writer.generate_excel_bytes(chart)
}

/// Generate Excel bytes for a chart based on its type (legacy function)
pub fn generate_excel_bytes(chart: &Chart) -> Vec<u8> {
    let writer = get_excel_writer(&chart.chart_type);
    writer.generate_excel_bytes(chart)
}