use ppt_rs::api::Presentation;
use ppt_rs::generator::{
    SlideContent, Shape, ShapeType, TableBuilder, TableCell, TableRow, TransitionType
};
use ppt_rs::generator::hyperlinks::{Hyperlink, HyperlinkAction};

#[test]
fn test_visual_polish_features() {
    let mut pres = Presentation::new();
    
    // Test 1: Slide Transition
    let mut slide1 = SlideContent::new("Transition Slide");
    slide1 = slide1.with_transition(TransitionType::Fade);
    pres = pres.add_slide(slide1);
    
    // Test 2: Shape Rotation & Hyperlink
    let mut slide2 = SlideContent::new("Rotation Slide");
    let shape = Shape::new(ShapeType::Rectangle, 1000000, 1000000, 1000000, 1000000)
        .with_rotation(45)
        .with_hyperlink(Hyperlink::new(HyperlinkAction::url("https://example.com")).with_r_id("rId2"));
    slide2.shapes.push(shape);
    pres = pres.add_slide(slide2);
    
    // Test 3: Table Merging
    let mut slide3 = SlideContent::new("Table Merge Slide");
    let table = TableBuilder::new(vec![2000000, 2000000])
        .position(1000000, 1000000)
        .add_simple_row(vec!["A1", "B1"])
        // Row 2: Merged cells (horizontal merge)
        .add_row(TableRow::new(vec![
            TableCell::new("Merged Row").with_col_span(2).with_h_merge()
        ]))
        // Row 3: Vertical merge start
        .add_row(TableRow::new(vec![
            TableCell::new("Row Span").with_row_span(2),
            TableCell::new("Normal")
        ]))
        // Row 4: Vertical merge continuation (v_merge=true)
        .add_row(TableRow::new(vec![
            TableCell::new("").with_v_merge(),
            TableCell::new("Normal 2")
        ]))
        .build();
    
    slide3.table = Some(table);
    pres = pres.add_slide(slide3);
    
    // Save
    let output = "test_visual_polish.pptx";
    let res = pres.save(output);
    assert!(res.is_ok());
    
    // Cleanup
    std::fs::remove_file(output).unwrap_or(());
}
