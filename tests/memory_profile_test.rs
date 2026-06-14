//! Memory profiling integration tests for large presentations

use ppt_rs::generator::memory_profile::{
    profile_eager_generation, profile_lazy_generation, sample_slides,
};
use std::time::Duration;
use zip::ZipArchive;
use std::io::Cursor;

fn assert_valid_pptx(data: &[u8]) {
    let cursor = Cursor::new(data);
    let mut archive = ZipArchive::new(cursor).expect("valid pptx zip");
    archive.by_name("[Content_Types].xml").expect("content types");
    archive.by_name("ppt/presentation.xml").expect("presentation");
}

#[test]
fn test_memory_profile_100_slides_eager() {
    let metrics = profile_eager_generation("Large Deck", sample_slides(100));

    assert_eq!(metrics.slide_count, 100);
    assert!(metrics.output_bytes > 50_000);
    assert!(metrics.bytes_per_slide() > 400);
    assert!(metrics.duration < Duration::from_secs(10));
}

#[test]
fn test_memory_profile_500_slides_lazy() {
    let slides = sample_slides(500);
    let metrics = profile_lazy_generation("Lazy Deck", slides);

    assert_eq!(metrics.slide_count, 500);
    assert!(metrics.output_bytes > 200_000);
    assert!(metrics.duration < Duration::from_secs(30));
}

#[test]
fn test_eager_and_lazy_output_similar_size() {
    let slides = sample_slides(50);
    let eager = profile_eager_generation("Compare", slides.clone());
    let lazy = profile_lazy_generation("Compare", slides);

    let delta = eager.output_bytes.abs_diff(lazy.output_bytes);
    assert!(
        delta < 500,
        "eager={} lazy={} delta={delta}",
        eager.output_bytes,
        lazy.output_bytes
    );
}

#[test]
fn test_large_presentation_valid_zip() {
    let slides = sample_slides(150);
    let metrics = profile_eager_generation("Validation", slides.clone());
    assert_valid_pptx(
        &ppt_rs::generator::create_pptx_with_content("Validation", slides).unwrap(),
    );
    assert_eq!(metrics.slide_count, 150);
}

#[test]
fn test_slide_payload_estimate_scales_linearly() {
    let small = profile_eager_generation("Small", sample_slides(10));
    let large = profile_eager_generation("Large", sample_slides(100));

    assert!(large.slide_payload_bytes > small.slide_payload_bytes * 5);
    assert!(large.output_bytes > small.output_bytes);
}
