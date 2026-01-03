use crate::api::Presentation;
use crate::oxml::presentation::PresentationReader;
use crate::generator::{SlideContent, Shape, ShapeType, TableBuilder, TableRow, TableCell};
use crate::exc::Result;

/// Import a presentation from a file path
pub fn import_pptx(path: &str) -> Result<Presentation> {
    let reader = PresentationReader::open(path)?;
    let mut presentation = Presentation::new();
    
    if let Some(title) = &reader.info().title {
        presentation = presentation.title(title);
    }
    
    for parsed_slide in reader.get_all_slides()? {
        let mut content = SlideContent::new(parsed_slide.title.as_deref().unwrap_or(""));
        
        // Add body text as bullets
        for text in parsed_slide.body_text {
            content = content.add_bullet(&text);
        }
        
        // Add shapes (skip title and body)
        for parsed_shape in parsed_slide.shapes {
            if !parsed_shape.is_title && !parsed_shape.is_body {
                let mut shape = Shape::new(
                    map_shape_type(&parsed_shape.shape_type),
                    parsed_shape.x.max(0) as u32,
                    parsed_shape.y.max(0) as u32,
                    parsed_shape.width.max(0) as u32,
                    parsed_shape.height.max(0) as u32
                );
                
                // Set text
                let text = parsed_shape.text();
                if !text.is_empty() {
                    shape = shape.with_text(&text);
                }
                
                content.shapes.push(shape);
            }
        }
        
        // Add tables
        for parsed_table in parsed_slide.tables {
             // Determine column count from first row
             let col_count = parsed_table.rows.first().map(|r| r.len()).unwrap_or(0);
             if col_count == 0 { continue; }
             
             // Default column width (approx 2 inches)
             let col_widths = vec![1828800; col_count];
             
             let mut table_builder = TableBuilder::new(col_widths);
             for row in parsed_table.rows {
                 let cells: Vec<TableCell> = row.into_iter()
                     .map(|cell| TableCell::new(&cell.text))
                     .collect();
                 let table_row = TableRow::new(cells);
                 table_builder = table_builder.add_row(table_row);
             }
             
             // SlideContent currently supports only one table via 'table' field
             if content.table.is_none() {
                 content.table = Some(table_builder.build());
                 content.has_table = true;
             }
        }
        
        presentation = presentation.add_slide(content);
    }
    
    Ok(presentation)
}

fn map_shape_type(type_name: &Option<String>) -> ShapeType {
    if let Some(name) = type_name {
        match name.as_str() {
            "rect" => ShapeType::Rectangle,
            "roundRect" => ShapeType::RoundedRectangle,
            "ellipse" => ShapeType::Ellipse,
            "triangle" => ShapeType::Triangle,
            "rtTriangle" => ShapeType::RightTriangle,
            "diamond" => ShapeType::Diamond,
            "pentagon" => ShapeType::Pentagon,
            "hexagon" => ShapeType::Hexagon,
            "octagon" => ShapeType::Octagon,
            "star5" => ShapeType::Star5,
            "rightArrow" => ShapeType::RightArrow,
            _ => ShapeType::Rectangle, // Default
        }
    } else {
        ShapeType::Rectangle
    }
}
