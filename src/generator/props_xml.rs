//! Document properties XML generation

/// Get current timestamp in ISO 8601 format (UTC)
fn current_timestamp() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0));

    let secs = duration.as_secs();
    let days = secs / 86400;
    let seconds = secs % 86400;

    let year = 1970 + days / 365;
    let remaining_days = days % 365;
    let month = 1 + remaining_days / 30;
    let day = 1 + remaining_days % 30;

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, secs
    )
}

/// Create core properties XML (docProps/core.xml)
pub fn create_core_props_xml(title: &str) -> String {
    let now = current_timestamp();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:dcterms="http://purl.org/dc/terms/" xmlns:dcmitype="http://purl.org/dc/dcmitype/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<dc:title>{title}</dc:title>
<dc:creator>pptx-rs</dc:creator>
<cp:lastModifiedBy>pptx-rs</cp:lastModifiedBy>
<cp:revision>1</cp:revision>
<dcterms:created xsi:type="dcterms:W3CDTF">{now}</dcterms:created>
<dcterms:modified xsi:type="dcterms:W3CDTF">{now}</dcterms:modified>
</cp:coreProperties>"#
    )
}

/// Escape a string for use in XML text content / attribute values.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Build the `<HeadingPairs>` and `<TitlesOfParts>` elements.
///
/// PowerPoint refuses to open a deck (offering to repair it) when these two
/// elements are missing from `docProps/app.xml`. The two must stay consistent:
/// the sum of the counts in `HeadingPairs` must equal the size of the
/// `TitlesOfParts` vector.
fn heading_pairs_and_titles(slide_titles: &[String]) -> String {
    // Use a single "Slide Titles" heading pair so the counts always match the
    // provided titles without having to enumerate fonts or theme names.
    let count = slide_titles.len();

    let mut titles_xml = String::with_capacity(count * 32);
    for title in slide_titles {
        titles_xml.push_str("<vt:lpstr>");
        titles_xml.push_str(&xml_escape(title));
        titles_xml.push_str("</vt:lpstr>");
    }

    format!(
        r#"<HeadingPairs><vt:vector size="2" baseType="variant"><vt:variant><vt:lpstr>Slide Titles</vt:lpstr></vt:variant><vt:variant><vt:i4>{count}</vt:i4></vt:variant></vt:vector></HeadingPairs><TitlesOfParts><vt:vector size="{count}" baseType="lpstr">{titles_xml}</vt:vector></TitlesOfParts>"#
    )
}

/// Create app properties XML (docProps/app.xml)
///
/// `slide_titles` provides the per-slide titles used to populate the
/// `TitlesOfParts` vector that PowerPoint requires. When fewer titles are
/// supplied than `slides`, placeholder titles are generated so the counts stay
/// consistent.
pub fn create_app_props_xml(slides: usize, notes_count: usize, slide_titles: &[String]) -> String {
    let titles: Vec<String> = if slide_titles.len() >= slides {
        slide_titles.iter().take(slides).cloned().collect()
    } else {
        let mut v: Vec<String> = slide_titles.to_vec();
        for i in v.len()..slides {
            v.push(format!("Slide {}", i + 1));
        }
        v
    };

    let heading_and_titles = heading_pairs_and_titles(&titles);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
<TotalTime>0</TotalTime>
<Words>0</Words>
<Application>pptx-rs</Application>
<PresentationFormat>On-screen Show (4:3)</PresentationFormat>
<Paragraphs>0</Paragraphs>
<Slides>{slides}</Slides>
<Notes>{notes_count}</Notes>
<HiddenSlides>0</HiddenSlides>
<MMClips>0</MMClips>
<ScaleCrop>false</ScaleCrop>
{heading_and_titles}
<LinksUpToDate>false</LinksUpToDate>
<SharedDoc>false</SharedDoc>
<HyperlinksChanged>false</HyperlinksChanged>
<AppVersion>1.0000</AppVersion>
</Properties>"#
    )
}
