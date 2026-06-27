//! Document properties XML generation

use crate::generator::slide_content::SlideSize;

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

/// Create app properties XML (docProps/app.xml)
pub fn create_app_props_xml(slides: usize, slide_size: SlideSize) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
<TotalTime>0</TotalTime>
<Words>0</Words>
<Application>pptx-rs</Application>
<PresentationFormat>{}</PresentationFormat>
<Paragraphs>0</Paragraphs>
<Slides>{slides}</Slides>
<Notes>0</Notes>
<HiddenSlides>0</HiddenSlides>
<MMClips>0</MMClips>
<ScaleCrop>false</ScaleCrop>
<LinksUpToDate>false</LinksUpToDate>
<SharedDoc>false</SharedDoc>
<HyperlinksChanged>false</HyperlinksChanged>
<AppVersion>1.0000</AppVersion>
</Properties>"#,
        slide_size.app_presentation_format()
    )
}
