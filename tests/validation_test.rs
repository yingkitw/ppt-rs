//! Tests for core validation and placement modules.

use ppt_rs::core::{
    check_required_parts, validate_index, validate_non_empty, validate_non_empty_str,
    validate_well_formed_xml, ElementPlacement, REQUIRED_PARTS_MINIMAL,
};
use std::collections::HashSet;

#[test]
fn test_required_parts_minimal_constant() {
    assert!(REQUIRED_PARTS_MINIMAL.contains(&"[Content_Types].xml"));
    assert!(REQUIRED_PARTS_MINIMAL.contains(&"ppt/presentation.xml"));
}

#[test]
fn test_check_required_parts_reports_missing() {
    let found: HashSet<_> = ["_rels/.rels"].into_iter().map(str::to_string).collect();
    let issues = check_required_parts(&found, REQUIRED_PARTS_MINIMAL);
    assert!(!issues.is_empty());
    assert!(issues.iter().all(|i| i.message().contains("Missing required part")));
}

#[test]
fn test_validate_presentation_inputs() {
    assert!(validate_non_empty_str("Title", "title").is_ok());
    assert!(validate_non_empty(&[1, 2], "slides").is_ok());
    assert!(validate_non_empty::<i32>(&[], "slides").is_err());
    assert!(validate_index(1, 3, "slide").is_ok());
    assert!(validate_index(3, 3, "slide").is_err());
}

#[test]
fn test_element_placement_chart_and_image_defaults() {
    let chart = ElementPlacement::chart_defaults();
    assert_eq!(chart.width, 5_000_000);

    let image = ElementPlacement::image_defaults();
    assert_eq!(image.width, image.height);
}

#[test]
fn test_validate_well_formed_xml_rejects_blank() {
    assert!(validate_well_formed_xml("<a/>").is_ok());
    assert!(validate_well_formed_xml("").is_err());
}

#[test]
fn test_error_messages_are_consistent() {
    use ppt_rs::exc::messages;
    assert_eq!(messages::slide_not_found(2), "Slide 2 not found");
    assert_eq!(
        messages::unsupported_media_format("xyz"),
        "Unsupported media format: xyz"
    );
}
