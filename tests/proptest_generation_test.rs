//! Property-based checks that randomly built decks always pass structural validation.

use ppt_rs::core::validate_package_bytes;
use ppt_rs::generator::{
    SlideContent, SlideLayout, TransitionType, create_pptx_with_content,
};
use proptest::prelude::*;

fn arb_text(min_len: usize, max_len: usize) -> impl Strategy<Value = String> {
    prop::string::string_regex(&format!(r"[\PC]{{{min_len},{max_len}}}"))
        .unwrap()
        .prop_filter("no control chars", |s| {
            !s.is_empty() && !s.chars().any(|c| c.is_control())
        })
}

fn arb_layout() -> impl Strategy<Value = SlideLayout> {
    prop_oneof![
        Just(SlideLayout::CenteredTitle),
        Just(SlideLayout::TitleAndContent),
        Just(SlideLayout::TwoColumn),
        Just(SlideLayout::SectionHeader),
        Just(SlideLayout::Blank),
        Just(SlideLayout::TitleOnly),
        Just(SlideLayout::TitleAndBigContent),
    ]
}

fn arb_transition() -> impl Strategy<Value = TransitionType> {
    prop_oneof![
        Just(TransitionType::None),
        Just(TransitionType::Fade),
        Just(TransitionType::Cut),
        Just(TransitionType::Push),
        Just(TransitionType::Wipe),
        Just(TransitionType::Split),
        Just(TransitionType::Reveal),
        Just(TransitionType::Cover),
        Just(TransitionType::Zoom),
    ]
}

fn arb_slide() -> impl Strategy<Value = SlideContent> {
    (
        arb_text(1, 80),
        0usize..16usize,
        arb_layout(),
        arb_transition(),
    )
        .prop_map(|(title, bullet_count, layout, transition)| {
            let mut slide = SlideContent::new(&title)
                .layout(layout)
                .with_transition(transition);
            for i in 0..bullet_count {
                slide = slide.add_bullet(&format!("Point {i}"));
            }
            slide
        })
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: std::env::var("PROPTEST_CASES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000),
        .. ProptestConfig::default()
    })]

    #[test]
    fn random_slide_decks_pass_package_validation(
        slides in prop::collection::vec(arb_slide(), 1..=20usize),
    ) {
        let bytes = create_pptx_with_content("Proptest Deck", slides)
            .expect("create_pptx_with_content");
        let report = validate_package_bytes(&bytes);
        prop_assert!(
            report.is_valid(),
            "package validation failed ({} errors): {:?}",
            report.error_count(),
            report.issues,
        );
    }
}
