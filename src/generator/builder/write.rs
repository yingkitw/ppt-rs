//! Writes PPTX package parts into a ZIP archive (eager and lazy paths).

use super::super::layout_parts::{STANDARD_LAYOUT_COUNT, create_slide_layout_xml};
use super::super::media_registry::MediaRegistry;
use super::super::notes_xml::*;
use super::super::package_cache::{self, print_affects_theme_parts};
use super::super::package_xml::{
    append_digital_signature_content_type, append_embedded_font_content_type,
    create_content_types_xml_with_notes_and_charts, create_handout_master_rels_xml,
    create_pres_props_xml, create_presentation_rels_xml_full,
    create_presentation_rels_xml_full_with_fonts, create_presentation_xml,
    create_presentation_xml_with_fonts, create_rels_xml_with_signature, create_table_styles_xml,
    create_view_props_xml,
};
use super::super::props_xml::{create_app_props_xml, create_core_props_xml};
use super::super::slide_content::SlideContent;
use super::super::slide_content::presentation_settings::PresentationSettings;
use super::super::slide_xml::{
    create_slide_rels_xml, create_slide_xml, create_slide_xml_with_content,
};
use super::super::template::PptxTemplate;
use super::super::theme_xml::{
    create_layout_rels_xml, create_master_rels_xml, create_slide_master_xml, create_theme_xml,
};
use super::LazySlideSource;
use super::plan::{
    ChartInfo, build_media_registry, build_media_registry_lazy, collect_chart_info,
    collect_chart_info_lazy, collect_slide_titles, collect_slide_titles_lazy, embedded_fonts,
    has_comment_authors, has_digital_signature, has_embedded_fonts, load_template,
    prepare_settings, push_chart_rid, resolve_layout_number, set_notes_part_path,
    set_slide_rels_path, set_slide_xml_path, slide_content_rel_start,
    slide_hyperlink_relationships, slide_image_rel_targets, slide_media_plan, slide_media_rid_base,
    uses_handouts,
};
use crate::core::append_usize;
use crate::exc::Result;
use crate::generator::charts::{
    chart_embedding_filename, create_chart_rels_xml, generate_chart_part_xml,
    reference_workbook_bytes,
};
use crate::generator::presentation_theme::office_theme_xml;
use std::io::{Seek, Write};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

/// Write `[Content_Types].xml` (eager slice or lazy counts + shared registry).
#[allow(clippy::too_many_arguments)] // ponytail: one writer beats two copy-paste paths
fn write_content_types<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slide_count: usize,
    custom_slides: Option<&[SlideContent]>,
    notes_count: usize,
    ink_count: usize,
    comment_count: usize,
    chart_info: &ChartInfo,
    has_handout: bool,
    settings: Option<&PresentationSettings>,
    media_registry: &MediaRegistry,
) -> Result<()> {
    let mut content_types = create_content_types_xml_with_notes_and_charts(
        slide_count,
        custom_slides,
        notes_count,
        chart_info.total_charts,
        has_handout,
        &media_registry.extensions(),
    );

    if has_digital_signature(settings) {
        append_digital_signature_content_type(&mut content_types);
    }
    if has_embedded_fonts(settings) {
        append_embedded_font_content_type(&mut content_types);
    }

    super::super::package_xml::append_ink_content_types(&mut content_types, ink_count);
    super::super::package_xml::append_comment_content_types(
        &mut content_types,
        comment_count,
        has_comment_authors(settings),
    );
    super::super::package_xml::append_media_content_type_defaults(
        &mut content_types,
        &media_registry.media_extensions(),
    );

    zip.start_file("[Content_Types].xml", *options)?;
    zip.write_all(content_types.as_bytes())?;
    Ok(())
}

/// Write presentation relationships in PowerPoint order.
fn write_presentation_relationships<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slide_count: usize,
    has_notes: bool,
    has_handout: bool,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let has_authors = has_comment_authors(settings);
    let mut pres_rels = if let Some(fonts) = embedded_fonts(settings) {
        create_presentation_rels_xml_full_with_fonts(slide_count, has_notes, has_handout, fonts)
    } else {
        create_presentation_rels_xml_full(slide_count, has_notes, has_handout)
    };
    if has_authors {
        super::super::package_xml::append_comment_authors_relationship(
            &mut pres_rels,
            super::super::package_xml::comment_authors_rel_id(slide_count, has_notes, has_handout),
        );
    }

    zip.start_file("ppt/_rels/presentation.xml.rels", *options)?;
    zip.write_all(pres_rels.as_bytes())?;
    Ok(())
}

/// Write presProps, viewProps, and tableStyles (always emitted).
fn write_standard_package_parts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let pres_props = create_pres_props_xml(settings);
    zip.start_file("ppt/presProps.xml", *options)?;
    zip.write_all(pres_props.as_bytes())?;

    let view_props = create_view_props_xml();
    zip.start_file("ppt/viewProps.xml", *options)?;
    zip.write_all(view_props.as_bytes())?;

    let table_styles = create_table_styles_xml();
    zip.start_file("ppt/tableStyles.xml", *options)?;
    zip.write_all(table_styles.as_bytes())?;
    Ok(())
}

/// Write handout master when print settings use handouts.
fn write_handout_master<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    use crate::generator::slide_content::print_settings::PrintSettings;

    let handout_xml = settings
        .and_then(|s| s.print.as_ref())
        .map(|p| p.to_handout_master_xml())
        .unwrap_or_else(|| PrintSettings::default().to_handout_master_xml());

    zip.start_file("ppt/handoutMasters/handoutMaster1.xml", *options)?;
    zip.write_all(handout_xml.as_bytes())?;

    zip.start_file("ppt/theme/theme3.xml", *options)?;
    zip.write_all(office_theme_xml().as_bytes())?;

    let rels = create_handout_master_rels_xml();
    zip.start_file("ppt/handoutMasters/_rels/handoutMaster1.xml.rels", *options)?;
    zip.write_all(rels.as_bytes())?;
    Ok(())
}

/// Write notes master files if needed
fn write_notes_master<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
) -> Result<()> {
    let notes_master = create_notes_master_xml();
    zip.start_file("ppt/notesMasters/notesMaster1.xml", *options)?;
    zip.write_all(notes_master.as_bytes())?;

    zip.start_file("ppt/theme/theme2.xml", *options)?;
    zip.write_all(office_theme_xml().as_bytes())?;

    let notes_master_rels = create_notes_master_rels_xml();
    zip.start_file("ppt/notesMasters/_rels/notesMaster1.xml.rels", *options)?;
    zip.write_all(notes_master_rels.as_bytes())?;
    Ok(())
}

/// Write theme and layout files
fn write_theme_and_layouts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    settings: Option<&PresentationSettings>,
    template: Option<&PptxTemplate>,
) -> Result<()> {
    let print = settings.and_then(|s| s.print.as_ref());

    if let Some(tmpl) = template {
        let replaces_theme = settings.is_some_and(|s| s.theme.is_some());
        for (path, data) in tmpl.parts() {
            // Standard package parts are emitted elsewhere; never write a
            // duplicate entry when cloning the template into the ZIP archive.
            if path == "ppt/tableStyles.xml" || (replaces_theme && path == "ppt/theme/theme1.xml") {
                continue;
            }
            zip.start_file(path, *options)?;
            zip.write_all(data)?;
        }
        if let Some(theme) = settings.and_then(|s| s.theme.as_ref()) {
            let theme_xml = create_theme_xml(Some(theme));
            zip.start_file("ppt/theme/theme1.xml", *options)?;
            zip.write_all(theme_xml.as_bytes())?;
        }
        return Ok(());
    }

    let use_cached_layouts = !print_affects_theme_parts(print);

    for n in 1..=STANDARD_LAYOUT_COUNT {
        zip.start_file(format!("ppt/slideLayouts/slideLayout{n}.xml"), *options)?;
        if use_cached_layouts {
            zip.write_all(package_cache::default_layout_xml(n).as_bytes())?;
        } else {
            let layout_xml = create_slide_layout_xml(n, print);
            zip.write_all(layout_xml.as_bytes())?;
        }

        zip.start_file(
            format!("ppt/slideLayouts/_rels/slideLayout{n}.xml.rels"),
            *options,
        )?;
        zip.write_all(create_layout_rels_xml().as_bytes())?;
    }

    zip.start_file("ppt/slideMasters/slideMaster1.xml", *options)?;
    if use_cached_layouts {
        zip.write_all(package_cache::default_slide_master_xml().as_bytes())?;
    } else {
        let slide_master = create_slide_master_xml(print);
        zip.write_all(slide_master.as_bytes())?;
    }

    zip.start_file("ppt/slideMasters/_rels/slideMaster1.xml.rels", *options)?;
    if use_cached_layouts {
        zip.write_all(package_cache::master_rels_xml().as_bytes())?;
    } else {
        let master_rels = create_master_rels_xml();
        zip.write_all(master_rels.as_bytes())?;
    }

    if let Some(theme) = settings.and_then(|s| s.theme.as_ref()) {
        let theme_xml = create_theme_xml(Some(theme));
        zip.start_file("ppt/theme/theme1.xml", *options)?;
        zip.write_all(theme_xml.as_bytes())?;
    } else {
        zip.start_file("ppt/theme/theme1.xml", *options)?;
        zip.write_all(office_theme_xml().as_bytes())?;
    }

    Ok(())
}

/// Write document properties
fn write_document_properties<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    title: &str,
    slide_count: usize,
    notes_count: usize,
    slide_titles: &[String],
) -> Result<()> {
    // Core properties
    let core_props = create_core_props_xml(title);
    zip.start_file("docProps/core.xml", *options)?;
    zip.write_all(core_props.as_bytes())?;

    // App properties
    let app_props = create_app_props_xml(slide_count, notes_count, slide_titles);
    zip.start_file("docProps/app.xml", *options)?;
    zip.write_all(app_props.as_bytes())?;

    Ok(())
}

struct PackageCoreParams<'a> {
    title: &'a str,
    slide_count: usize,
    has_notes: bool,
    has_handout: bool,
    has_signature: bool,
    settings: Option<&'a PresentationSettings>,
}

/// Steps 2–4: `_rels/.rels`, presentation relationships, and `presentation.xml`.
fn write_package_core<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    params: PackageCoreParams<'_>,
) -> Result<()> {
    let rels = create_rels_xml_with_signature(params.has_signature);
    zip.start_file("_rels/.rels", *options)?;
    zip.write_all(rels.as_bytes())?;

    write_presentation_relationships(
        zip,
        options,
        params.slide_count,
        params.has_notes,
        params.has_handout,
        params.settings,
    )?;

    let mut presentation = if let Some(fonts) = embedded_fonts(params.settings) {
        create_presentation_xml_with_fonts(
            params.title,
            params.slide_count,
            params.has_notes,
            params.has_handout,
            fonts,
        )
    } else {
        create_presentation_xml(
            params.title,
            params.slide_count,
            params.has_notes,
            params.has_handout,
        )
    };
    if let Some(s) = params.settings {
        super::super::package_xml::append_section_lst_ext(
            &mut presentation,
            &s.sections,
            params.slide_count,
        );
    }
    zip.start_file("ppt/presentation.xml", *options)?;
    zip.write_all(presentation.as_bytes())?;
    Ok(())
}

/// Steps 5–6: presProps/viewProps/tableStyles and optional handout master.
fn write_package_masters<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    has_handout: bool,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    write_standard_package_parts(zip, options, settings)?;
    if has_handout {
        write_handout_master(zip, options, settings)?;
    }
    Ok(())
}

/// Steps 14–16: embedded fonts, digital signature, comment authors.
fn write_package_optional_parts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    has_signature: bool,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    if let Some(fonts) = embedded_fonts(settings) {
        write_embedded_font_parts(zip, options, fonts)?;
    }
    if has_signature {
        write_digital_signature_parts(zip, options, settings)?;
    }
    if let Some(s) = settings.filter(|s| !s.comment_authors.is_empty()) {
        zip.start_file("ppt/commentAuthors.xml", *options)?;
        zip.write_all(s.comment_authors.to_xml().as_bytes())?;
    }
    Ok(())
}

/// Write all package files to the ZIP archive (eager version with Vec<SlideContent>)
pub(crate) fn write_package_files<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    title: &str,
    slide_count: usize,
    custom_slides: Option<&[SlideContent]>,
    mut settings: Option<PresentationSettings>,
) -> Result<()> {
    let has_notes = custom_slides
        .map(|slides| slides.iter().any(|s| s.notes.is_some()))
        .unwrap_or(false);
    let has_handout = uses_handouts(settings.as_ref());
    prepare_settings(&mut settings, slide_count, has_notes, has_handout);
    let has_signature = has_digital_signature(settings.as_ref());
    let template = load_template(settings.as_ref())?;

    let chart_info = collect_chart_info(custom_slides);
    let media_registry = custom_slides.map(build_media_registry).unwrap_or_default();
    let ink_count = custom_slides
        .map(|slides| {
            slides
                .iter()
                .filter(|s| s.ink_annotations.is_some())
                .count()
        })
        .unwrap_or(0);
    let comment_count = custom_slides
        .map(|slides| slides.iter().filter(|s| !s.comments.is_empty()).count())
        .unwrap_or(0);

    write_content_types(
        zip,
        options,
        slide_count,
        custom_slides,
        0,
        ink_count,
        comment_count,
        &chart_info,
        has_handout,
        settings.as_ref(),
        &media_registry,
    )?;

    write_package_core(
        zip,
        options,
        PackageCoreParams {
            title,
            slide_count,
            has_notes,
            has_handout,
            has_signature,
            settings: settings.as_ref(),
        },
    )?;
    write_package_masters(zip, options, has_handout, settings.as_ref())?;

    // 7. Slides
    write_slides(zip, options, slide_count, custom_slides)?;

    // 8. Slide relationships
    write_slide_relationships_extended(
        zip,
        options,
        custom_slides,
        &chart_info.slide_start_indices,
        slide_count,
        template.as_ref(),
    )?;

    // 9. Notes relationships and master
    if has_notes {
        write_notes_relationships(zip, options, custom_slides)?;
        write_notes_master(zip, options)?;
    }

    // 10. Theme and layouts
    write_theme_and_layouts(zip, options, settings.as_ref(), template.as_ref())?;

    // 11. Document properties
    let notes_count = custom_slides
        .map(|slides| slides.iter().filter(|s| s.notes.is_some()).count())
        .unwrap_or(0);
    let slide_titles = collect_slide_titles(custom_slides, slide_count);
    write_document_properties(zip, options, title, slide_count, notes_count, &slide_titles)?;

    // 12. Charts (with embedded workbooks)
    if chart_info.total_charts > 0 {
        write_charts(zip, options, custom_slides, &chart_info.slide_start_indices)?;
    }

    // 13. Images
    write_images(zip, options, custom_slides)?;

    write_package_optional_parts(zip, options, has_signature, settings.as_ref())?;

    Ok(())
}

/// Write all package files to the ZIP archive (lazy version with LazySlideSource)
pub(crate) fn write_package_files_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    title: &str,
    slides: &dyn LazySlideSource,
    mut settings: Option<PresentationSettings>,
) -> Result<()> {
    let slide_count = slides.slide_count();
    let has_notes = (0..slide_count).any(|i| {
        slides
            .slide_features(i)
            .map(|(notes, _)| notes)
            .unwrap_or(false)
    });
    let has_handout = uses_handouts(settings.as_ref());
    prepare_settings(&mut settings, slide_count, has_notes, has_handout);
    let has_signature = has_digital_signature(settings.as_ref());
    let template = load_template(settings.as_ref())?;

    let chart_info = collect_chart_info_lazy(slides);
    let media_registry = build_media_registry_lazy(slides);
    let notes_count = (0..slide_count)
        .filter(|i| {
            slides
                .slide_features(*i)
                .map(|(notes, _)| notes)
                .unwrap_or(false)
        })
        .count();
    let ink_count = (0..slide_count)
        .filter(|i| {
            slides
                .generate_slide(*i)
                .map(|s| s.ink_annotations.is_some())
                .unwrap_or(false)
        })
        .count();
    let comment_count = (0..slide_count)
        .filter(|i| {
            slides
                .generate_slide(*i)
                .map(|s| !s.comments.is_empty())
                .unwrap_or(false)
        })
        .count();

    write_content_types(
        zip,
        options,
        slide_count,
        None,
        notes_count,
        ink_count,
        comment_count,
        &chart_info,
        has_handout,
        settings.as_ref(),
        &media_registry,
    )?;

    write_package_core(
        zip,
        options,
        PackageCoreParams {
            title,
            slide_count,
            has_notes,
            has_handout,
            has_signature,
            settings: settings.as_ref(),
        },
    )?;
    write_package_masters(zip, options, has_handout, settings.as_ref())?;

    // 7–8–12. Slides, relationships, and charts (single pass per slide)
    write_slide_packages_lazy(
        zip,
        options,
        slides,
        &chart_info.slide_start_indices,
        template.as_ref(),
    )?;

    // 9. Notes relationships and master (lazy version)
    if has_notes {
        write_notes_relationships_lazy(zip, options, slides)?;
        write_notes_master(zip, options)?;
    }

    // 10. Theme and layouts
    write_theme_and_layouts(zip, options, settings.as_ref(), template.as_ref())?;

    // 11. Document properties
    let notes_count = (0..slide_count)
        .filter(|i| {
            slides
                .slide_features(*i)
                .map(|(notes, _)| notes)
                .unwrap_or(false)
        })
        .count();
    let slide_titles = collect_slide_titles_lazy(slides, slide_count);
    write_document_properties(zip, options, title, slide_count, notes_count, &slide_titles)?;

    // 12. Images
    write_images_lazy(zip, options, slides)?;

    write_package_optional_parts(zip, options, has_signature, settings.as_ref())?;

    Ok(())
}

/// Write a chart part with rels and embedded Excel workbook.
fn write_chart_package<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    chart_idx: usize,
    chart: &crate::generator::charts::Chart,
) -> Result<()> {
    let chart_xml = generate_chart_part_xml(chart);
    zip.start_file(format!("ppt/charts/chart{chart_idx}.xml"), *options)?;
    zip.write_all(chart_xml.as_bytes())?;

    let embedding_name = chart_embedding_filename(chart_idx);
    let rels_xml = create_chart_rels_xml(&embedding_name);
    zip.start_file(
        format!("ppt/charts/_rels/chart{chart_idx}.xml.rels"),
        *options,
    )?;
    zip.write_all(rels_xml.as_bytes())?;

    zip.start_file(format!("ppt/embeddings/{embedding_name}"), *options)?;
    zip.write_all(reference_workbook_bytes())?;
    Ok(())
}

/// Write slide XML, relationships, and chart parts in one pass (lazy version).
fn write_slide_packages_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slides: &dyn LazySlideSource,
    slide_chart_start_indices: &[usize],
    template: Option<&PptxTemplate>,
) -> Result<()> {
    let mut media_registry = build_media_registry_lazy(slides);
    let mut slide_path = String::with_capacity(48);
    let mut rels_path = String::with_capacity(56);
    let mut notes_part_num = 0usize;
    let mut ink_part_num = 0usize;
    let mut comment_part_num = 0usize;

    for (i, &start_chart_idx) in slide_chart_start_indices.iter().enumerate() {
        let Some(slide) = slides.generate_slide(i) else {
            continue;
        };
        let slide_num = i + 1;
        let layout_number = resolve_layout_number(&slide, template);
        let resolved_images = slide_image_rel_targets(&slide, &media_registry);
        let poster_count = slide.videos.len() + slide.audios.len();
        let image_band_len = resolved_images.len() + poster_count;

        let start_rid = slide_content_rel_start(slide.notes.is_some(), image_band_len);
        let mut chart_rids = Vec::with_capacity(slide.charts.len());
        for j in 0..slide.charts.len() {
            push_chart_rid(&mut chart_rids, start_rid + j);
        }

        let ink_rel_id = if let Some(ref ink) = slide.ink_annotations {
            ink_part_num += 1;
            let ink_xml = ink.part_xml();
            zip.start_file(format!("ppt/ink/ink{ink_part_num}.xml"), *options)?;
            zip.write_all(ink_xml.as_bytes())?;
            Some(format!("rId{}", start_rid + slide.charts.len()))
        } else {
            None
        };

        let has_comments = !slide.comments.is_empty();
        if has_comments {
            comment_part_num += 1;
            let comments_xml = slide.comments.to_xml();
            zip.start_file(
                format!("ppt/comments/comment{comment_part_num}.xml"),
                *options,
            )?;
            zip.write_all(comments_xml.as_bytes())?;
        }
        let comment_part = has_comments.then_some(comment_part_num);

        let media_plan = slide_media_plan(
            &slide,
            &mut media_registry,
            2 + usize::from(slide.notes.is_some()),
            resolved_images.len(),
            slide_media_rid_base(
                start_rid,
                slide.charts.len(),
                slide.ink_annotations.is_some(),
                has_comments,
            ),
        )?;

        let slide_xml = create_slide_xml_with_content(
            slide_num,
            &slide,
            &chart_rids,
            ink_rel_id.as_deref(),
            &media_plan.rel_ids,
        );
        set_slide_xml_path(&mut slide_path, slide_num);
        zip.start_file(&slide_path, *options)?;
        zip.write_all(slide_xml.as_bytes())?;

        let notes_part = if slide.notes.is_some() {
            notes_part_num += 1;
            Some(notes_part_num)
        } else {
            None
        };

        if let Some(ref notes) = slide.notes {
            let notes_xml = create_notes_xml(slide_num, notes);
            set_notes_part_path(&mut slide_path, notes_part_num);
            zip.start_file(&slide_path, *options)?;
            zip.write_all(notes_xml.as_bytes())?;
        }

        let mut chart_rels = Vec::with_capacity(slide.charts.len());
        for j in 0..slide.charts.len() {
            let mut rid = String::with_capacity(8);
            rid.push_str("rId");
            append_usize(&mut rid, start_rid + j);
            let mut target = String::with_capacity(24);
            target.push_str("../charts/chart");
            append_usize(&mut target, start_chart_idx + j);
            target.push_str(".xml");
            chart_rels.push((rid, target));
        }

        let ink_rel_tuple = ink_rel_id.map(|_| (start_rid + slide.charts.len(), ink_part_num));

        let comment_rel = comment_part.map(|num| {
            (
                start_rid + slide.charts.len() + usize::from(slide.ink_annotations.is_some()),
                num,
            )
        });

        let mut images = resolved_images;
        images.extend(media_plan.poster_entries.iter().cloned());

        let slide_rels = super::super::package_xml::create_slide_rels_xml_with_images(
            layout_number,
            slide.notes.is_some(),
            notes_part.unwrap_or(1),
            &chart_rels,
            &images,
            &slide_hyperlink_relationships(&slide),
            ink_rel_tuple,
            comment_rel,
            &media_plan.rels_xml,
        );
        set_slide_rels_path(&mut rels_path, slide_num);
        zip.start_file(&rels_path, *options)?;
        zip.write_all(slide_rels.as_bytes())?;

        for (j, chart) in slide.charts.iter().enumerate() {
            let chart_idx = start_chart_idx + j;
            write_chart_package(zip, options, chart_idx, chart)?;
        }
    }

    Ok(())
}

/// Write notes relationship files (lazy version)
fn write_notes_relationships_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slides: &dyn LazySlideSource,
) -> Result<()> {
    let mut notes_part_num = 0usize;
    for i in 0..slides.slide_count() {
        if slides
            .slide_features(i)
            .map(|(notes, _)| notes)
            .unwrap_or(false)
        {
            notes_part_num += 1;
            let slide_num = i + 1;
            let notes_rels = create_notes_rels_xml(slide_num);
            zip.start_file(
                format!("ppt/notesSlides/_rels/notesSlide{notes_part_num}.xml.rels"),
                *options,
            )?;
            zip.write_all(notes_rels.as_bytes())?;
        }
    }
    Ok(())
}

/// Write slide XML files (eager version)
fn write_slides<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slide_count: usize,
    custom_slides: Option<&[SlideContent]>,
) -> Result<()> {
    let mut zip_path = String::with_capacity(48);

    match custom_slides {
        Some(slides) => {
            let mut media_registry = build_media_registry(slides);
            let mut notes_part_num = 0usize;
            let mut ink_part_num = 0usize;
            let mut comment_part_num = 0usize;
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;

                let resolved_images = slide_image_rel_targets(slide, &media_registry);
                let poster_count = slide.videos.len() + slide.audios.len();

                let mut chart_rids = Vec::with_capacity(slide.charts.len());
                let start_rid = slide_content_rel_start(
                    slide.notes.is_some(),
                    resolved_images.len() + poster_count,
                );
                for j in 0..slide.charts.len() {
                    push_chart_rid(&mut chart_rids, start_rid + j);
                }

                let ink_rel_id = if let Some(ref ink) = slide.ink_annotations {
                    ink_part_num += 1;
                    let ink_xml = ink.part_xml();
                    zip.start_file(format!("ppt/ink/ink{ink_part_num}.xml"), *options)?;
                    zip.write_all(ink_xml.as_bytes())?;
                    Some(format!("rId{}", start_rid + slide.charts.len()))
                } else {
                    None
                };

                let has_comments = !slide.comments.is_empty();
                if has_comments {
                    comment_part_num += 1;
                    let comments_xml = slide.comments.to_xml();
                    zip.start_file(
                        format!("ppt/comments/comment{comment_part_num}.xml"),
                        *options,
                    )?;
                    zip.write_all(comments_xml.as_bytes())?;
                }

                let media_plan = slide_media_plan(
                    slide,
                    &mut media_registry,
                    2 + usize::from(slide.notes.is_some()),
                    resolved_images.len(),
                    slide_media_rid_base(
                        start_rid,
                        slide.charts.len(),
                        slide.ink_annotations.is_some(),
                        has_comments,
                    ),
                )?;

                let slide_xml = create_slide_xml_with_content(
                    slide_num,
                    slide,
                    &chart_rids,
                    ink_rel_id.as_deref(),
                    &media_plan.rel_ids,
                );
                set_slide_xml_path(&mut zip_path, slide_num);
                zip.start_file(&zip_path, *options)?;
                zip.write_all(slide_xml.as_bytes())?;

                if let Some(notes) = &slide.notes {
                    notes_part_num += 1;
                    let notes_xml = create_notes_xml(slide_num, notes);
                    set_notes_part_path(&mut zip_path, notes_part_num);
                    zip.start_file(&zip_path, *options)?;
                    zip.write_all(notes_xml.as_bytes())?;
                }
            }
        }
        None => {
            for i in 1..=slide_count {
                let slide_xml = create_slide_xml(i, "Presentation");
                set_slide_xml_path(&mut zip_path, i);
                zip.start_file(&zip_path, *options)?;
                zip.write_all(slide_xml.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write slide relationship files with notes and charts (eager version)
fn write_slide_relationships_extended<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    custom_slides: Option<&[SlideContent]>,
    slide_chart_start_indices: &[usize],
    slide_count: usize,
    template: Option<&PptxTemplate>,
) -> Result<()> {
    let mut media_registry = custom_slides.map(build_media_registry).unwrap_or_default();
    let mut notes_part_num = 0usize;

    match custom_slides {
        Some(slides) => {
            let mut zip_path = String::with_capacity(56);
            let mut ink_part_num = 0usize;
            let mut comment_part_num = 0usize;
            for (i, slide) in slides.iter().enumerate() {
                let slide_num = i + 1;
                let layout_number = resolve_layout_number(slide, template);
                let notes_part = if slide.notes.is_some() {
                    notes_part_num += 1;
                    Some(notes_part_num)
                } else {
                    None
                };

                let mut chart_rels = Vec::with_capacity(slide.charts.len());
                let start_chart_idx = slide_chart_start_indices[i];
                let resolved_images = slide_image_rel_targets(slide, &media_registry);
                let poster_count = slide.videos.len() + slide.audios.len();
                let start_rid = slide_content_rel_start(
                    slide.notes.is_some(),
                    resolved_images.len() + poster_count,
                );

                for j in 0..slide.charts.len() {
                    let mut rid = String::with_capacity(8);
                    rid.push_str("rId");
                    append_usize(&mut rid, start_rid + j);
                    let mut target = String::with_capacity(24);
                    target.push_str("../charts/chart");
                    append_usize(&mut target, start_chart_idx + j);
                    target.push_str(".xml");
                    chart_rels.push((rid, target));
                }

                let ink_rel_tuple = if slide.ink_annotations.is_some() {
                    ink_part_num += 1;
                    Some((start_rid + slide.charts.len(), ink_part_num))
                } else {
                    None
                };

                let has_comments = !slide.comments.is_empty();
                let comment_rel = if has_comments {
                    comment_part_num += 1;
                    Some((
                        start_rid
                            + slide.charts.len()
                            + usize::from(slide.ink_annotations.is_some()),
                        comment_part_num,
                    ))
                } else {
                    None
                };

                let media_plan = slide_media_plan(
                    slide,
                    &mut media_registry,
                    2 + usize::from(slide.notes.is_some()),
                    resolved_images.len(),
                    slide_media_rid_base(
                        start_rid,
                        slide.charts.len(),
                        slide.ink_annotations.is_some(),
                        has_comments,
                    ),
                )?;

                let mut images = resolved_images;
                images.extend(media_plan.poster_entries.iter().cloned());

                let slide_rels = super::super::package_xml::create_slide_rels_xml_with_images(
                    layout_number,
                    slide.notes.is_some(),
                    notes_part.unwrap_or(1),
                    &chart_rels,
                    &images,
                    &slide_hyperlink_relationships(slide),
                    ink_rel_tuple,
                    comment_rel,
                    &media_plan.rels_xml,
                );
                set_slide_rels_path(&mut zip_path, slide_num);
                zip.start_file(&zip_path, *options)?;
                zip.write_all(slide_rels.as_bytes())?;
            }
        }
        None => {
            let mut zip_path = String::with_capacity(56);
            for i in 1..=slide_count {
                let slide_rels = create_slide_rels_xml();
                set_slide_rels_path(&mut zip_path, i);
                zip.start_file(&zip_path, *options)?;
                zip.write_all(slide_rels.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write chart files (eager version)
fn write_charts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    custom_slides: Option<&[SlideContent]>,
    slide_chart_start_indices: &[usize],
) -> Result<()> {
    if let Some(slides) = custom_slides {
        for (i, slide) in slides.iter().enumerate() {
            let start_chart_idx = slide_chart_start_indices[i];
            for (j, chart) in slide.charts.iter().enumerate() {
                let chart_idx = start_chart_idx + j;
                write_chart_package(zip, options, chart_idx, chart)?;
            }
        }
    }
    Ok(())
}

/// Write notes relationship files (eager version)
fn write_notes_relationships<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    custom_slides: Option<&[SlideContent]>,
) -> Result<()> {
    if let Some(slides) = custom_slides {
        let mut notes_part_num = 0usize;
        for (i, slide) in slides.iter().enumerate() {
            if slide.notes.is_some() {
                notes_part_num += 1;
                let slide_num = i + 1;
                let notes_rels = create_notes_rels_xml(slide_num);
                zip.start_file(
                    format!("ppt/notesSlides/_rels/notesSlide{notes_part_num}.xml.rels"),
                    *options,
                )?;
                zip.write_all(notes_rels.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Write embedded font data parts to `ppt/fonts/`.
fn write_embedded_font_parts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    fonts: &super::super::slide_content::embedded_fonts::EmbeddedFontList,
) -> Result<()> {
    for font in fonts.fonts() {
        zip.start_file(font.part_name(), *options)?;
        zip.write_all(&font.data)?;
    }
    Ok(())
}

/// Write image files to ppt/media/
fn write_images<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    custom_slides: Option<&[SlideContent]>,
) -> Result<()> {
    if let Some(slides) = custom_slides {
        let registry = build_media_registry(slides);
        for (i, (bytes, ext)) in registry.files().iter().enumerate() {
            let filename = format!("ppt/media/image{}.{}", i + 1, ext);
            zip.start_file(filename, *options)?;
            zip.write_all(bytes)?;
        }
        write_media_files(zip, options, &registry)?;
    }
    Ok(())
}

/// Write digital signature parts (`_xmlsignatures/`).
fn write_digital_signature_parts<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    settings: Option<&PresentationSettings>,
) -> Result<()> {
    let signature = settings
        .and_then(|s| s.digital_signature.as_ref())
        .expect("digital signature parts requested but none configured");

    // `origin.sigs` is the signature origin relationships part.
    zip.start_file("_xmlsignatures/origin.sigs", *options)?;
    zip.write_all(signature.to_origin_xml().as_bytes())?;

    zip.start_file("_xmlsignatures/sig1.xml", *options)?;
    zip.write_all(signature.to_signature_xml().as_bytes())?;

    Ok(())
}

/// Write image files from a lazy slide source.
fn write_images_lazy<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    slides: &dyn LazySlideSource,
) -> Result<()> {
    let registry = build_media_registry_lazy(slides);
    for (i, (bytes, ext)) in registry.files().iter().enumerate() {
        let filename = format!("ppt/media/image{}.{}", i + 1, ext);
        zip.start_file(filename, *options)?;
        zip.write_all(bytes)?;
    }
    write_media_files(zip, options, &registry)?;
    Ok(())
}

/// Write video/audio parts (`ppt/media/mediaN.{ext}`).
fn write_media_files<W: Write + Seek>(
    zip: &mut ZipWriter<W>,
    options: &SimpleFileOptions,
    registry: &MediaRegistry,
) -> Result<()> {
    for (i, (bytes, ext)) in registry.media_files().iter().enumerate() {
        let filename = format!("ppt/media/media{}.{}", i + 1, ext);
        zip.start_file(filename, *options)?;
        zip.write_all(bytes)?;
    }
    Ok(())
}
