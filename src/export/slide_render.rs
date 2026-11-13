//! Render slides as PDF vector pages using `pdfrs::pdf_generator::PdfGenerator`.
//!
//! Each slide becomes one PDF page with:
//! - White background
//! - Title text (bold, large)
//! - Bullet points / content text
//! - Slide number footer
//!
//! No external binaries required — all rendering is done with PDF
//! vector/text operators.

use crate::api::Presentation;
use crate::exc::{PptxError, Result};
use crate::generator::images::ImageSource;
use crate::generator::slide::formatting::parse_inline_formatting;
use crate::generator::slide_content::{BulletStyle, SlideContent, SlideLayout};
use crate::generator::Image;

use pdfrs::image::{self, ImageInfo, create_image_content_stream, create_image_object};
use pdfrs::pdf_generator::{PageLayout, PdfGenerator};

const FONT_HELVETICA: &str = "Helvetica";
const FONT_HELVETICA_BOLD: &str = "Helvetica-Bold";
const FONT_HELVETICA_OBLIQUE: &str = "Helvetica-Oblique";
const FONT_HELVETICA_BOLD_OBLIQUE: &str = "Helvetica-BoldOblique";
const FONT_COURIER: &str = "Courier";

struct FontIds {
    helvetica: u32,
    helvetica_bold: u32,
    helvetica_oblique: u32,
    helvetica_bold_oblique: u32,
    courier: u32,
}

fn add_fonts(generator: &mut PdfGenerator) -> FontIds {
    let helvetica = generator.add_object(format!(
        "<< /Type /Font\n/Subtype /Type1\n/BaseFont /{}\n>>\n",
        FONT_HELVETICA
    ));
    let helvetica_bold = generator.add_object(format!(
        "<< /Type /Font\n/Subtype /Type1\n/BaseFont /{}\n>>\n",
        FONT_HELVETICA_BOLD
    ));
    let helvetica_oblique = generator.add_object(format!(
        "<< /Type /Font\n/Subtype /Type1\n/BaseFont /{}\n>>\n",
        FONT_HELVETICA_OBLIQUE
    ));
    let helvetica_bold_oblique = generator.add_object(format!(
        "<< /Type /Font\n/Subtype /Type1\n/BaseFont /{}\n>>\n",
        FONT_HELVETICA_BOLD_OBLIQUE
    ));
    let courier = generator.add_object(format!(
        "<< /Type /Font\n/Subtype /Type1\n/BaseFont /{}\n>>\n",
        FONT_COURIER
    ));
    FontIds {
        helvetica,
        helvetica_bold,
        helvetica_oblique,
        helvetica_bold_oblique,
        courier,
    }
}

fn font_name(bold: bool, italic: bool) -> &'static str {
    match (bold, italic) {
        (true, true) => FONT_HELVETICA_BOLD_OBLIQUE,
        (true, false) => FONT_HELVETICA_BOLD,
        (false, true) => FONT_HELVETICA_OBLIQUE,
        (false, false) => FONT_HELVETICA,
    }
}

fn hex_to_rgb(hex: &str) -> (f32, f32, f32) {
    let h = hex.trim_start_matches('#');
    if h.len() == 6 {
        let r = u8::from_str_radix(&h[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&h[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&h[4..6], 16).unwrap_or(0);
        (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    } else {
        (0.0, 0.0, 0.0)
    }
}

fn escape_pdf_text(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

fn set_color(out: &mut Vec<u8>, r: f32, g: f32, b: f32) {
    out.extend_from_slice(format!("{:.3} {:.3} {:.3} rg\n", r, g, b).as_bytes());
}

fn set_font(out: &mut Vec<u8>, name: &str, size: f32) {
    out.extend_from_slice(format!("/{} {} Tf\n", name, size).as_bytes());
}

fn draw_rect(out: &mut Vec<u8>, x: f32, y: f32, w: f32, h: f32) {
    out.extend_from_slice(format!("{:.1} {:.1} {:.1} {:.1} re\n", x, y, w, h).as_bytes());
    out.extend_from_slice(b"f\n");
}

struct TextLine<'a> {
    text: &'a str,
    x: f32,
    y: f32,
    font_name: &'a str,
    font_size: f32,
    color: Rgb,
}

#[derive(Clone, Copy)]
struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

fn draw_text_line(out: &mut Vec<u8>, line: TextLine<'_>) {
    let TextLine { text, x, y, font_name, font_size, color: Rgb { r, g, b } } = line;

    set_color(out, r, g, b);
    set_font(out, font_name, font_size);
    out.extend_from_slice(b"BT\n");
    out.extend_from_slice(format!("{} {:.1} Td\n", x, y).as_bytes());
    out.extend_from_slice(format!("({}) Tj\n", escape_pdf_text(text)).as_bytes());
    out.extend_from_slice(b"ET\n");
}

fn approx_text_width(text: &str, font_size: f32, monospace: bool) -> f32 {
    let char_count = text.chars().count() as f32;
    let factor = if monospace { 0.6 } else { 0.52 };
    char_count * font_size * factor
}

fn draw_plain_text_line(
    cs: &mut Vec<u8>,
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    bold: bool,
    italic: bool,
    color: Rgb,
) {
    if text.is_empty() {
        return;
    }
    draw_text_line(
        cs,
        TextLine {
            text,
            x,
            y,
            font_name: font_name(bold, italic),
            font_size,
            color,
        },
    );
}

fn draw_markdown_line(
    cs: &mut Vec<u8>,
    text: &str,
    mut x: f32,
    y: f32,
    font_size: f32,
    base_bold: bool,
    base_italic: bool,
    color: Rgb,
) {
    for segment in parse_inline_formatting(text) {
        let bold = base_bold || segment.bold;
        let italic = base_italic || segment.italic;
        let font = if segment.code {
            FONT_COURIER
        } else {
            font_name(bold, italic)
        };
        draw_text_line(
            cs,
            TextLine {
                text: &segment.text,
                x,
                y,
                font_name: font,
                font_size: if segment.code {
                    font_size * 0.95
                } else {
                    font_size
                },
                color,
            },
        );
        x += approx_text_width(&segment.text, font_size, segment.code);
    }
}

fn bullet_prefix(style: BulletStyle, index: u32) -> String {
    match style {
        BulletStyle::Bullet => "\u{2022}".to_string(),
        BulletStyle::Number => format!("{}.", index + 1),
        BulletStyle::LetterLower => {
            format!("{}.", char::from_u32(b'a' as u32 + index).unwrap_or('a'))
        }
        BulletStyle::LetterUpper => {
            format!("{}.", char::from_u32(b'A' as u32 + index).unwrap_or('A'))
        }
        BulletStyle::RomanLower => to_roman(index + 1).to_lowercase(),
        BulletStyle::RomanUpper => to_roman(index + 1),
        BulletStyle::Custom(ch) => ch.to_string(),
        BulletStyle::None => String::new(),
    }
}

struct SlideRenderOutput {
    content: Vec<u8>,
    images: Vec<(String, ImageInfo)>,
}

fn resolve_image_bytes(image: &Image) -> Option<Vec<u8>> {
    match image.source.as_ref()? {
        ImageSource::Bytes(b) => Some(b.clone()),
        ImageSource::File(p) => std::fs::read(p).ok(),
        ImageSource::Base64(_) => None,
        #[cfg(feature = "web2ppt")]
        ImageSource::Url(_) => None,
    }
}

fn image_info_from_bytes(bytes: &[u8], alt: Option<String>) -> Option<ImageInfo> {
    let suffix = if image::detect_image_format(bytes).ok()? == pdfrs::image::ImageFormat::Png {
        "png"
    } else {
        "jpg"
    };
    let tmp = std::env::temp_dir().join(format!(
        "ppt_rs_img_{}.{}",
        uuid::Uuid::new_v4(),
        suffix
    ));
    std::fs::write(&tmp, bytes).ok()?;
    let path = tmp.to_string_lossy();
    let info = image::load_image_with_alt_text(&path, alt).ok();
    let _ = std::fs::remove_file(&tmp);
    info
}

fn to_roman(mut num: u32) -> String {
    let pairs = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    let mut result = String::new();
    for (val, sym) in pairs {
        while num >= val {
            result.push_str(sym);
            num -= val;
        }
    }
    result
}

fn render_slide(
    slide: &SlideContent,
    slide_num: usize,
    total: usize,
    layout: &PageLayout,
) -> SlideRenderOutput {
    let mut cs = Vec::with_capacity(2048);
    let mut page_images: Vec<(String, ImageInfo)> = Vec::new();
    let page_w = layout.width;
    let page_h = layout.height;
    let margin = 54.0;

    // White background
    set_color(&mut cs, 1.0, 1.0, 1.0);
    draw_rect(&mut cs, 0.0, 0.0, page_w, page_h);

    // Title
    let title_size = slide.title_size.unwrap_or(36) as f32 * 0.75;
    let title_bold = slide.title_bold;
    let title_italic = slide.title_italic;
    let (tr, tg, tb) = slide
        .title_color
        .as_deref()
        .map(hex_to_rgb)
        .unwrap_or((0.1, 0.1, 0.1));

    let title_y = page_h - margin - title_size;

    match slide.layout {
        SlideLayout::CenteredTitle | SlideLayout::SectionHeader => {
            let tw = approx_text_width(&slide.title, title_size, false);
            let tx = (page_w - tw) / 2.0;
                        draw_text_line(&mut cs, TextLine {
                        text: &slide.title,
                        x: tx,
                        y: title_y,
                        font_name: font_name(title_bold, title_italic),
                        font_size: title_size,
                        color: Rgb { r: tr, g: tg, b: tb },
                });
        }
        _ => {
                        draw_text_line(&mut cs, TextLine {
                        text: &slide.title,
                        x: margin,
                        y: title_y,
                        font_name: font_name(title_bold, title_italic),
                        font_size: title_size,
                        color: Rgb { r: tr, g: tg, b: tb },
                });
            // Underline
            if slide.title_underline {
                set_color(&mut cs, tr, tg, tb);
                cs.extend_from_slice(
                    format!(
                        "{:.1} {:.1} m\n{:.1} {:.1} l\n1 w\nS\n",
                        margin,
                        title_y - 4.0,
                        page_w - margin,
                        title_y - 4.0
                    )
                    .as_bytes(),
                );
            }
        }
    }

    // Content
    let content_size = slide.content_size.unwrap_or(24) as f32 * 0.75;
    let content_bold = slide.content_bold;
    let content_italic = slide.content_italic;
    let (cr, cg, cb) = slide
        .content_color
        .as_deref()
        .map(hex_to_rgb)
        .unwrap_or((0.2, 0.2, 0.2));

    let line_height = content_size * 1.5;
    let mut y = title_y - line_height * 1.5;

    let is_centered = matches!(
        slide.layout,
        SlideLayout::CenteredTitle | SlideLayout::SectionHeader
    );

  let body_color = Rgb { r: cr, g: cg, b: cb };

    // Tables first (match PPTX / markdown order)
    if let Some(table) = &slide.table {
        let cols = table.column_count().max(1);
        let table_width = page_w - 2.0 * margin;
        let col_w = table_width / cols as f32;
        let row_h = line_height * 1.35;
        let max_chars = |w: f32| -> usize { ((w / (content_size * 0.45)).floor() as usize).max(4) };

        for (ri, row) in table.rows.iter().enumerate() {
            if y < margin + row_h {
                break;
            }
            for (ci, cell) in row.cells.iter().enumerate() {
                let x = margin + ci as f32 * col_w;
                if ri == 0 {
                    set_color(&mut cs, 0.86, 0.89, 0.94);
                } else {
                    set_color(&mut cs, 0.98, 0.98, 0.98);
                }
                draw_rect(&mut cs, x + 1.0, y - row_h + 2.0, col_w - 2.0, row_h - 2.0);
                let limit = max_chars(col_w - 8.0);
                let text: String = cell.text.chars().take(limit).collect();
                let display = if cell.text.chars().count() > limit {
                    format!("{text}…")
                } else {
                    text
                };
                draw_text_line(
                    &mut cs,
                    TextLine {
                        text: &display,
                        x: x + 6.0,
                        y: y - row_h + 8.0,
                        font_name: font_name(ri == 0, false),
                        font_size: content_size * if ri == 0 { 0.95 } else { 0.9 },
                        color: Rgb { r: cr, g: cg, b: cb },
                    },
                );
            }
            y -= row_h;
        }
        y -= line_height * 0.4;
    }

    for code_block in &slide.code_blocks {
        if y < margin + line_height {
            break;
        }
        let code_size = content_size * 0.85;
        let code_line_h = code_size * 1.4;
        let line_cap = if slide.table.is_some() && !slide.bullets.is_empty() {
            5usize
        } else {
            usize::MAX
        };
        let code_lines: Vec<&str> = code_block
            .code
            .lines()
            .take(line_cap)
            .collect();
        let code_bg_h = (code_lines.len() as f32 + 1.0) * code_line_h;

        // Background
        set_color(&mut cs, 0.95, 0.95, 0.95);
        draw_rect(
            &mut cs,
            margin,
            y - code_bg_h,
            page_w - 2.0 * margin,
            code_bg_h,
        );

        // Code text
        y -= code_line_h;
        for line in &code_lines {
            if y < margin {
                break;
            }
            draw_plain_text_line(
                &mut cs,
                line,
                margin + 8.0,
                y,
                code_size,
                false,
                false,
                Rgb { r: 0.1, g: 0.1, b: 0.1 },
            );
            y -= code_line_h;
        }
        y -= line_height * 0.5;
    }

    if !slide.bullets.is_empty() {
        let mut numbered = 0u32;
        for bp in &slide.bullets {
            if y < margin + line_height {
                break;
            }

            let prefix = bullet_prefix(bp.style, numbered);
            let indent = (bp.level as f32) * 20.0;
            let bx = margin + indent;

            let fmt = bp.format.as_ref();
            let b_bold = content_bold || fmt.is_some_and(|f| f.bold);
            let b_italic = content_italic || fmt.is_some_and(|f| f.italic);
            let (btr, btg, btb) = fmt
                .and_then(|f| f.color.as_deref())
                .map(hex_to_rgb)
                .unwrap_or((cr, cg, cb));
            let b_size = fmt
                .and_then(|f| f.font_size)
                .map(|s| s as f32 * 0.75)
                .unwrap_or(content_size);
            let bullet_color = Rgb { r: btr, g: btg, b: btb };

            if !prefix.is_empty() {
                draw_plain_text_line(
                    &mut cs,
                    &prefix,
                    bx,
                    y,
                    b_size,
                    b_bold,
                    b_italic,
                    bullet_color,
                );
            }

            let text_x = bx + if prefix.is_empty() { 0.0 } else { 20.0 };
            draw_markdown_line(
                &mut cs,
                &bp.text,
                text_x,
                y,
                b_size,
                b_bold,
                b_italic,
                bullet_color,
            );

            y -= line_height;
            if matches!(
                bp.style,
                BulletStyle::Number
                    | BulletStyle::LetterLower
                    | BulletStyle::LetterUpper
                    | BulletStyle::RomanLower
                    | BulletStyle::RomanUpper
            ) {
                numbered += 1;
            }
        }
    } else if !slide.content.is_empty() {
        for line in &slide.content {
            if y < margin + line_height {
                break;
            }
            if is_centered {
                let tw = approx_text_width(line, content_size, false);
                let tx = (page_w - tw) / 2.0;
                draw_markdown_line(
                    &mut cs,
                    line,
                    tx,
                    y,
                    content_size,
                    content_bold,
                    content_italic,
                    body_color,
                );
            } else {
                draw_markdown_line(
                    &mut cs,
                    line,
                    margin,
                    y,
                    content_size,
                    content_bold,
                    content_italic,
                    body_color,
                );
            }
            y -= line_height;
        }
    }

    for image in &slide.images {
        if y < margin + 60.0 {
            break;
        }
        let max_w = page_w - 2.0 * margin;
        let max_h = (y - margin - 20.0).max(60.0);
        if let Some(bytes) = resolve_image_bytes(image)
            && let Some(info) =
                image_info_from_bytes(&bytes, image.alt_text.clone())
        {
            let (dw, dh) = image::scale_to_fit(info.width, info.height, max_w, max_h);
            let name = format!("Im{}", page_images.len() + 1);
            let x = margin + (max_w - dw) / 2.0;
            let y_bottom = y - dh;
            cs.extend_from_slice(&create_image_content_stream(x, y_bottom, dw, dh, &name));
            page_images.push((name, info));
            y = y_bottom - line_height * 0.4;
            continue;
        }
        let img_h = 120.0;
        set_color(&mut cs, 0.92, 0.92, 0.92);
        draw_rect(&mut cs, margin, y - img_h, max_w, img_h);
        let label = image.alt_text.as_deref().unwrap_or(&image.filename);
        draw_plain_text_line(
            &mut cs,
            label,
            margin + 12.0,
            y - img_h / 2.0 - content_size * 0.35,
            content_size * 0.9,
            false,
            false,
            Rgb { r: 0.35, g: 0.35, b: 0.35 },
        );
        y -= img_h + line_height * 0.5;
    }

    for shape in &slide.shapes {
        if y < margin + 40.0 {
            break;
        }
        if let Some(ref text) = shape.text {
            let sh = 80.0;
            set_color(&mut cs, 0.9, 0.9, 0.9);
            draw_rect(&mut cs, margin, y - sh, page_w - 2.0 * margin, sh);
            draw_plain_text_line(
                &mut cs,
                text,
                margin + 8.0,
                y - sh + 24.0,
                content_size * 0.85,
                false,
                false,
                body_color,
            );
            y -= sh + line_height * 0.3;
        }
    }

    // Slide number footer
    let footer = format!("{} / {}", slide_num, total);
    let fw = approx_text_width(&footer, 10.0, false);
        draw_text_line(&mut cs, TextLine {
                text: &footer,
                x: (page_w - fw) / 2.0,
                y: margin / 2.0,
                font_name: FONT_HELVETICA,
                font_size: 10.0,
                color: Rgb { r: 0.5, g: 0.5, b: 0.5 },
        });

    SlideRenderOutput {
        content: cs,
        images: page_images,
    }
}

fn add_page(
    generator: &mut PdfGenerator,
    output: SlideRenderOutput,
    layout: &PageLayout,
    font_ids: &FontIds,
    pages_obj_id: u32,
) -> Result<u32> {
    let dict = format!("<< /Length {} >>\n", output.content.len());
    let content_id = generator.add_stream_object(dict, output.content);

    let mut xobject_parts = Vec::new();
    for (name, info) in &output.images {
        let id = create_image_object(generator, info.clone())
            .map_err(|e| PptxError::Generic(format!("PDF image embed failed: {e}")))?;
        xobject_parts.push(format!("/{} {} 0 R", name, id));
    }
    let xobject_resource = if xobject_parts.is_empty() {
        String::new()
    } else {
        format!(" /XObject << {} >>", xobject_parts.join(" "))
    };

    let page_dict = format!(
        "<< /Type /Page\n\
         /Parent {} 0 R\n\
         /MediaBox [0 0 {} {}]\n\
         /Contents {} 0 R\n\
         /Resources << /Font << \
             /{} {} 0 R \
             /{} {} 0 R \
             /{} {} 0 R \
             /{} {} 0 R \
             /{} {} 0 R \
         >>{}>>\n\
         >>\n",
        pages_obj_id,
        layout.width,
        layout.height,
        content_id,
        FONT_HELVETICA,
        font_ids.helvetica,
        FONT_HELVETICA_BOLD,
        font_ids.helvetica_bold,
        FONT_HELVETICA_OBLIQUE,
        font_ids.helvetica_oblique,
        FONT_HELVETICA_BOLD_OBLIQUE,
        font_ids.helvetica_bold_oblique,
        FONT_COURIER,
        font_ids.courier,
        xobject_resource,
    );
    Ok(generator.add_object(page_dict))
}

/// Render a presentation to PDF bytes using vector PDF operators.
///
/// Each slide becomes one PDF page with title, bullets, and content
/// rendered as native PDF text and vector graphics.
pub fn render_to_pdf_bytes(presentation: &Presentation) -> Result<Vec<u8>> {
    let slides = presentation.slides();
    if slides.is_empty() {
        return Err(PptxError::InvalidState(
            crate::exc::messages::must_not_be_empty("presentation slides for PDF export"),
        ));
    }

    let layout = PageLayout::landscape();
    let mut generator = PdfGenerator::new().with_version(layout.version);
    let font_ids = add_fonts(&mut generator);

    let total = slides.len();
    let outputs: Vec<SlideRenderOutput> = slides
        .iter()
        .enumerate()
        .map(|(i, slide)| render_slide(slide, i + 1, total, &layout))
        .collect();
    let image_xobjects: u32 = outputs.iter().map(|o| o.images.len() as u32).sum();
    let pages_obj_id = generator.next_id + total as u32 * 2 + image_xobjects;

    let mut page_ids = Vec::with_capacity(total);
    for output in outputs {
        let page_id = add_page(&mut generator, output, &layout, &font_ids, pages_obj_id)?;
        page_ids.push(page_id);
    }

    let kids: Vec<String> = page_ids.iter().map(|id| format!("{} 0 R", id)).collect();
    let pages_dict = format!(
        "<< /Type /Pages\n\
         /Kids [{}]\n\
         /Count {}\n\
         >>\n",
        kids.join(" "),
        page_ids.len()
    );
    let actual_pages_id = generator.add_object(pages_dict);
    assert_eq!(actual_pages_id, pages_obj_id);

    let catalog = format!(
        "<< /Type /Catalog\n\
         /Pages {} 0 R\n\
         >>\n",
        pages_obj_id
    );
    generator.add_object(catalog);

    Ok(generator.generate())
}

/// Render a presentation to a PDF file using vector PDF operators.
pub fn render_to_pdf<P: AsRef<std::path::Path>>(
    presentation: &Presentation,
    output_path: P,
) -> Result<()> {
    let bytes = render_to_pdf_bytes(presentation)?;
    std::fs::write(output_path.as_ref(), &bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::SlideContent;

    #[test]
    fn test_render_simple_presentation() {
        let pres = Presentation::with_title("Test")
            .add_slide(SlideContent::new("Slide 1").add_bullet("Hello"))
            .add_slide(SlideContent::new("Slide 2").add_bullet("World"));

        let bytes = render_to_pdf_bytes(&pres).unwrap();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_render_empty_presentation_errors() {
        let pres = Presentation::with_title("Empty");
        let result = render_to_pdf_bytes(&pres);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_with_styled_bullets() {
        let pres = Presentation::with_title("Styled").add_slide(
            SlideContent::new("Styled Slide")
                .add_styled_bullet("Bold item", BulletStyle::Number)
                .add_styled_bullet("Another", BulletStyle::Bullet),
        );

        let bytes = render_to_pdf_bytes(&pres).unwrap();
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_render_centered_title_layout() {
        let pres = Presentation::with_title("Centered").add_slide(
            SlideContent::new("Centered Title")
                .with_layout(SlideLayout::CenteredTitle)
                .add_bullet("Subtitle text"),
        );

        let bytes = render_to_pdf_bytes(&pres).unwrap();
        assert_eq!(&bytes[..5], b"%PDF-");
    }

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("FF0000"), (1.0, 0.0, 0.0));
        assert_eq!(hex_to_rgb("#00FF00"), (0.0, 1.0, 0.0));
        assert_eq!(hex_to_rgb("0000FF"), (0.0, 0.0, 1.0));
        assert_eq!(hex_to_rgb("invalid"), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_bullet_prefix() {
        assert_eq!(bullet_prefix(BulletStyle::Bullet, 0), "\u{2022}");
        assert_eq!(bullet_prefix(BulletStyle::Number, 0), "1.");
        assert_eq!(bullet_prefix(BulletStyle::Number, 2), "3.");
        assert_eq!(bullet_prefix(BulletStyle::LetterLower, 0), "a.");
        assert_eq!(bullet_prefix(BulletStyle::None, 0), "");
    }

    #[test]
    fn test_to_roman() {
        assert_eq!(to_roman(1), "I");
        assert_eq!(to_roman(4), "IV");
        assert_eq!(to_roman(9), "IX");
        assert_eq!(to_roman(2024), "MMXXIV");
    }

    #[test]
    fn test_escape_pdf_text() {
        assert_eq!(escape_pdf_text("hello"), "hello");
        assert_eq!(escape_pdf_text("a(b)c"), "a\\(b\\)c");
        assert_eq!(escape_pdf_text("a\\b"), "a\\\\b");
    }

    #[test]
    fn test_render_to_pdf_file() {
        let pres = Presentation::with_title("File Test")
            .add_slide(SlideContent::new("Slide 1").add_bullet("Content"));

        let path =
            std::env::temp_dir().join(format!("ppt_rs_vector_pdf_{}.pdf", uuid::Uuid::new_v4()));
        render_to_pdf(&pres, &path).unwrap();
        let read_back = std::fs::read(&path).unwrap();
        assert_eq!(&read_back[..5], b"%PDF-");
        let _ = std::fs::remove_file(&path);
    }
}
