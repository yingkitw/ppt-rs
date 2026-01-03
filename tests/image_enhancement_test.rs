use ppt_rs::generator::{Image, ImageSource, generate_image_xml};
use ppt_rs::generator::images::{Crop, ImageEffect};

#[test]
fn test_image_crop_xml() {
    let mut img = Image::new("test.png", 1000, 1000, "PNG");
    img = img.with_crop(0.1, 0.2, 0.3, 0.4);
    
    let xml = generate_image_xml(&img, 1, 1);
    
    // Check for srcRect with correct percentage values (0.1 -> 10000)
    assert!(xml.contains("<a:srcRect l=\"10000\" t=\"20000\" r=\"30000\" b=\"40000\"/>"));
    
    // Check structure
    assert!(xml.contains("<p:blipFill>"));
    assert!(xml.contains("<a:blip r:embed=\"rId1\"/>"));
}

#[test]
fn test_image_effect_shadow_xml() {
    let mut img = Image::new("test.png", 1000, 1000, "PNG");
    img = img.with_effect(ImageEffect::Shadow);
    
    let xml = generate_image_xml(&img, 1, 1);
    
    assert!(xml.contains("<p:spPr>"));
    assert!(xml.contains("<a:effectLst>"));
    assert!(xml.contains("<a:outerShdw"));
}

#[test]
fn test_image_effect_reflection_xml() {
    let mut img = Image::new("test.png", 1000, 1000, "PNG");
    img = img.with_effect(ImageEffect::Reflection);
    
    let xml = generate_image_xml(&img, 1, 1);
    
    assert!(xml.contains("<p:spPr>"));
    assert!(xml.contains("<a:effectLst>"));
    assert!(xml.contains("<a:ref"));
}

#[test]
fn test_image_multiple_effects() {
    let mut img = Image::new("test.png", 1000, 1000, "PNG");
    img = img.with_effect(ImageEffect::Shadow)
             .with_effect(ImageEffect::Reflection);
             
    let xml = generate_image_xml(&img, 1, 1);
    
    assert!(xml.contains("<a:outerShdw"));
    assert!(xml.contains("<a:ref"));
}
