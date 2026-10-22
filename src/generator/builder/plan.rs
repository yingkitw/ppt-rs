//! Media registry, relationship planning, and chart metadata for PPTX assembly.

use super::super::media_registry::MediaRegistry;
use super::super::package_xml::table_styles_rel_id;
use super::super::slide_content::SlideContent;
use super::super::template::PptxTemplate;
use super::LazySlideSource;
use crate::core::append_usize;
use crate::exc::{PptxError, Result};
use crate::generator::slide_content::presentation_settings::PresentationSettings;
use crate::generator::slide_content::print_settings::PrintWhat;
use zip::write::SimpleFileOptions;

pub(crate) fn zip_options() -> SimpleFileOptions {
    SimpleFileOptions::default()
}

/// First relationship id after layout (rId1) and optional notes slide.
pub(crate) fn slide_content_rel_start(has_notes: bool, image_count: usize) -> usize {
    2 + usize::from(has_notes) + image_count
}

pub(crate) fn build_media_registry(slides: &[SlideContent]) -> MediaRegistry {
    let mut registry = MediaRegistry::default();
    for slide in slides {
        for image in &slide.images {
            if let Some(bytes) = image.get_bytes() {
                registry.image_number(&bytes, &image.extension());
            }
        }
        register_slide_media(&mut registry, slide);
    }
    registry
}

/// Register video/audio bytes and their poster/placeholder frames so the
/// parts get written and rel targets resolve. Failures surface at plan time.
fn register_slide_media(registry: &mut MediaRegistry, slide: &SlideContent) {
    for video in &slide.videos {
        if let Ok((bytes, ext)) = read_media_source(&video.source) {
            registry.media_number(&bytes, &ext);
        }
        if let Ok((poster, ext)) = media_poster_bytes(video.poster.as_ref()) {
            registry.image_number(&poster, &ext);
        }
    }
    for audio in &slide.audios {
        if let Ok((bytes, ext)) = read_media_source(&audio.source) {
            registry.media_number(&bytes, &ext);
        }
        let (poster, ext) = media_poster_bytes(None).expect("placeholder poster is constant");
        registry.image_number(&poster, &ext);
    }
}

pub(crate) fn build_media_registry_lazy(slides: &dyn LazySlideSource) -> MediaRegistry {
    let mut registry = MediaRegistry::default();
    for i in 0..slides.slide_count() {
        if let Some(slide) = slides.generate_slide(i) {
            for image in &slide.images {
                if let Some(bytes) = image.get_bytes() {
                    registry.image_number(&bytes, &image.extension());
                }
            }
            register_slide_media(&mut registry, &slide);
        }
    }
    registry
}

pub(crate) fn slide_image_rel_targets(
    slide: &SlideContent,
    registry: &MediaRegistry,
) -> Vec<(usize, String)> {
    let mut images = Vec::with_capacity(slide.images.len());
    for image in &slide.images {
        if let Some(bytes) = image.get_bytes()
            && let Some(num) = registry.lookup_number(&bytes, &image.extension())
        {
            images.push((num, image.extension()));
        }
    }
    images
}

/// 1×1 dark-gray PNG used as the poster frame when a media element has no
/// poster image. `p:pic` requires a `p:blipFill`, so audio elements and
/// poster-less videos embed this placeholder.
pub const DEFAULT_MEDIA_POSTER_PNG: &[u8] = &[
    137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0,
    0, 0, 144, 119, 83, 222, 0, 0, 0, 12, 73, 68, 65, 84, 120, 156, 99, 112, 112, 112, 0, 0, 1,
    132, 0, 193, 131, 95, 105, 148, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
];
const DEFAULT_MEDIA_POSTER_EXT: &str = "png";

/// Read a media source file and return its bytes plus lowercased extension.
fn read_media_source(source: &str) -> Result<(Vec<u8>, String)> {
    let ext = std::path::Path::new(source)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    let bytes = std::fs::read(source)
        .map_err(|e| PptxError::Generic(format!("Failed to read media file \"{source}\": {e}")))?;
    Ok((bytes, ext))
}

/// Poster bytes for a media element: the configured poster image, or the
/// built-in 1×1 placeholder.
fn media_poster_bytes(poster: Option<&String>) -> Result<(Vec<u8>, String)> {
    match poster {
        Some(path) => read_media_source(path),
        None => Ok((
            DEFAULT_MEDIA_POSTER_PNG.to_vec(),
            DEFAULT_MEDIA_POSTER_EXT.to_string(),
        )),
    }
}

/// Per-slide media plan: relationship ids for the slide XML, prebuilt
/// relationship entries for the slide rels, and the poster image-rel
/// entries (appended after the slide's own images).
pub(crate) struct SlideMediaPlan {
    pub(crate) rel_ids: Vec<super::super::media::MediaRelIds>,
    pub(crate) rels_xml: Vec<String>,
    pub(crate) poster_entries: Vec<(usize, String)>,
}

impl SlideMediaPlan {
    fn empty() -> Self {
        Self {
            rel_ids: Vec::new(),
            rels_xml: Vec::new(),
            poster_entries: Vec::new(),
        }
    }
}

/// Plan the media relationships for one slide.
///
/// * `image_rel_start` — first image-band rId (`2 + has_notes`)
/// * `resolved_image_count` — images with resolvable bytes (rel order)
/// * `media_rid_base` — first free rId after layout/notes/images/charts/ink/comments
pub(crate) fn slide_media_plan(
    slide: &SlideContent,
    registry: &mut MediaRegistry,
    image_rel_start: usize,
    resolved_image_count: usize,
    media_rid_base: usize,
) -> Result<SlideMediaPlan> {
    if slide.videos.is_empty() && slide.audios.is_empty() {
        return Ok(SlideMediaPlan::empty());
    }

    let mut plan = SlideMediaPlan::empty();
    let mut rid = media_rid_base;

    let elements: Vec<(&String, Option<&String>, bool)> = slide
        .videos
        .iter()
        .map(|v| (&v.source, v.poster.as_ref(), true))
        .chain(slide.audios.iter().map(|a| (&a.source, None, false)))
        .collect();

    for (poster_slot, (source, poster, is_video)) in elements.into_iter().enumerate() {
        let (bytes, ext) = read_media_source(source)?;
        if is_video && super::super::media::VideoFormat::from_extension(&ext).is_none()
            || !is_video && super::super::media::AudioFormat::from_extension(&ext).is_none()
        {
            return Err(PptxError::Generic(format!(
                "Unsupported media extension \".{ext}\" for \"{source}\""
            )));
        }
        let media_num = registry.media_number(&bytes, &ext);

        let (poster_bytes, poster_ext) = media_poster_bytes(poster)?;
        let poster_num = registry.image_number(&poster_bytes, &poster_ext);
        plan.poster_entries.push((poster_num, poster_ext));

        let link_rid = format!("rId{rid}");
        let embed_rid = format!("rId{}", rid + 1);
        rid += 2;
        let poster_rid = format!(
            "rId{}",
            image_rel_start + resolved_image_count + poster_slot
        );

        let rel_type = if is_video {
            super::super::package_xml::VIDEO_REL_TYPE
        } else {
            super::super::package_xml::AUDIO_REL_TYPE
        };
        plan.rels_xml
            .push(super::super::package_xml::media_relationship_xml(
                &link_rid, rel_type, media_num, &ext,
            ));
        plan.rels_xml
            .push(super::super::package_xml::media_relationship_xml(
                &embed_rid,
                super::super::package_xml::MEDIA_REL_TYPE,
                media_num,
                &ext,
            ));

        plan.rel_ids.push(super::super::media::MediaRelIds {
            link_rid,
            embed_rid,
            poster_rid,
            is_video,
        });
    }
    Ok(plan)
}

/// Media rId base for a slide: first free id after layout/notes/images/
/// charts/ink/comments.
pub(crate) fn slide_media_rid_base(
    start_rid: usize,
    chart_count: usize,
    has_ink: bool,
    has_comments: bool,
) -> usize {
    start_rid + chart_count + usize::from(has_ink) + usize::from(has_comments)
}

/// Collect hyperlink relationship XML (`<Relationship .../>`) for every shape on
/// the slide whose hyperlink has an assigned relationship id. The slide XML
/// references these ids via `<a:hlinkClick r:id="..."/>`, so the matching
/// relationship must be present in the slide's `.rels` part.
pub(crate) fn slide_hyperlink_relationships(slide: &SlideContent) -> Vec<String> {
    slide
        .shapes
        .iter()
        .filter_map(|s| s.hyperlink.as_ref())
        .filter_map(|h| {
            h.r_id
                .as_ref()
                .map(|rid| crate::generator::generate_hyperlink_relationship_xml(h, rid))
        })
        .collect()
}

pub(crate) fn set_notes_part_path(path: &mut String, notes_part_num: usize) {
    path.clear();
    path.push_str("ppt/notesSlides/notesSlide");
    append_usize(path, notes_part_num);
    path.push_str(".xml");
}

#[cfg(debug_assertions)]
pub(crate) fn debug_assert_package_valid(bytes: &[u8]) {
    use crate::core::validate_package_bytes;
    let report = validate_package_bytes(bytes);
    debug_assert!(
        report.is_valid(),
        "generated PPTX failed package validation: {:?}",
        report.error_messages()
    );
}

#[cfg(not(debug_assertions))]
pub(crate) fn debug_assert_package_valid(_bytes: &[u8]) {}
pub(crate) fn resolve_layout_number(
    slide: &SlideContent,
    template: Option<&PptxTemplate>,
) -> usize {
    let requested = slide.layout.layout_number();
    template
        .map(|t| t.resolve_layout_number(requested))
        .unwrap_or(requested)
}

pub(crate) fn load_template(
    settings: Option<&PresentationSettings>,
) -> Result<Option<PptxTemplate>> {
    if let Some(path) = settings.and_then(|s| s.template_path.as_deref()) {
        Ok(Some(PptxTemplate::load(path)?))
    } else {
        Ok(None)
    }
}
pub(crate) fn set_slide_xml_path(path: &mut String, slide_num: usize) {
    path.clear();
    path.push_str("ppt/slides/slide");
    append_usize(path, slide_num);
    path.push_str(".xml");
}

pub(crate) fn set_slide_rels_path(path: &mut String, slide_num: usize) {
    path.clear();
    path.push_str("ppt/slides/_rels/slide");
    append_usize(path, slide_num);
    path.push_str(".xml.rels");
}

pub(crate) fn push_chart_rid(rids: &mut Vec<String>, rel_num: usize) {
    let mut rid = String::with_capacity(8);
    rid.push_str("rId");
    append_usize(&mut rid, rel_num);
    rids.push(rid);
}

/// Chart metadata for slides
pub(crate) struct ChartInfo {
    pub total_charts: usize,
    pub slide_start_indices: Vec<usize>,
}

/// Collect chart metadata from slides (eager version)
pub(crate) fn collect_chart_info(slides: Option<&[SlideContent]>) -> ChartInfo {
    let mut total_charts = 0;
    let mut slide_start_indices = Vec::new();

    if let Some(slides) = slides {
        slide_start_indices.reserve(slides.len());
        for slide in slides {
            slide_start_indices.push(total_charts + 1);
            total_charts += slide.charts.len();
        }
    }

    ChartInfo {
        total_charts,
        slide_start_indices,
    }
}

/// Collect chart metadata from lazy slide source
pub(crate) fn collect_chart_info_lazy(slides: &dyn LazySlideSource) -> ChartInfo {
    let mut total_charts = 0;
    let mut slide_start_indices = Vec::with_capacity(slides.slide_count());

    for i in 0..slides.slide_count() {
        slide_start_indices.push(total_charts + 1);
        total_charts += slides
            .slide_features(i)
            .map(|(_, chart_count)| chart_count)
            .unwrap_or(0);
    }

    ChartInfo {
        total_charts,
        slide_start_indices,
    }
}

/// Whether print settings request handout master packaging.
pub(crate) fn uses_handouts(settings: Option<&PresentationSettings>) -> bool {
    settings
        .and_then(|s| s.print.as_ref())
        .map(|p| p.print_what == PrintWhat::Handouts)
        .unwrap_or(false)
}

/// Whether settings configure a digital signature package.
pub(crate) fn has_digital_signature(settings: Option<&PresentationSettings>) -> bool {
    settings
        .and_then(|s| s.digital_signature.as_ref())
        .is_some()
}

/// Whether settings configure embedded fonts.
pub(crate) fn has_embedded_fonts(settings: Option<&PresentationSettings>) -> bool {
    settings
        .and_then(|s| s.embedded_fonts.as_ref())
        .map(|f| !f.is_empty())
        .unwrap_or(false)
}

/// Borrow embedded fonts from settings, if any.
pub(crate) fn embedded_fonts(
    settings: Option<&PresentationSettings>,
) -> Option<&super::super::slide_content::embedded_fonts::EmbeddedFontList> {
    settings.and_then(|s| s.embedded_fonts.as_ref())
}

/// Prepare settings by assigning relationship IDs to embedded fonts.
/// Must be called after `has_notes` and `has_handout` are known.
pub(crate) fn prepare_settings(
    settings: &mut Option<PresentationSettings>,
    slide_count: usize,
    has_notes: bool,
    has_handout: bool,
) {
    if let Some(s) = settings
        && let Some(fonts) = s.embedded_fonts.as_mut()
    {
        let first_rid = table_styles_rel_id(slide_count, has_notes, has_handout) + 1;
        fonts.assign_relationship_ids(first_rid);
    }
}

/// Collect slide titles for `docProps/app.xml` (eager version).
///
/// Returns one title per slide, falling back to "Slide N" placeholders when no
/// custom slide content is available or a slide has an empty title.
pub(crate) fn collect_slide_titles(
    custom_slides: Option<&[SlideContent]>,
    slide_count: usize,
) -> Vec<String> {
    match custom_slides {
        Some(slides) => slides
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if s.title.trim().is_empty() {
                    format!("Slide {}", i + 1)
                } else {
                    s.title.clone()
                }
            })
            .collect(),
        None => (0..slide_count)
            .map(|i| format!("Slide {}", i + 1))
            .collect(),
    }
}

/// Collect slide titles for `docProps/app.xml` (lazy version).
pub(crate) fn collect_slide_titles_lazy(
    slides: &dyn LazySlideSource,
    slide_count: usize,
) -> Vec<String> {
    let mut titles = Vec::with_capacity(slide_count);
    for i in 0..slide_count {
        let title = slides
            .generate_slide(i)
            .map(|s| {
                if s.title.trim().is_empty() {
                    format!("Slide {}", i + 1)
                } else {
                    s.title
                }
            })
            .unwrap_or_else(|| format!("Slide {}", i + 1));
        titles.push(title);
    }
    titles
}
pub(crate) fn has_comment_authors(settings: Option<&PresentationSettings>) -> bool {
    settings.is_some_and(|s| !s.comment_authors.is_empty())
}
