//! Batch conversion pipeline: directory of Markdown/HTML → one `.pptx` per file.
//!
//! Applies shared theme/template settings across every job. Markdown input
//! requires the `cli` feature (pulldown-cmark parser lives in `cli::markdown`).

use std::path::{Path, PathBuf};

use crate::exc::{PptxError, Result};
use crate::generator::{PresentationSettings, PresentationTheme, SlideContent, create_pptx_with_settings};
use crate::import::{HtmlParseOptions, parse_html_with_options};

/// Supported batch input formats (detected from file extension when `Auto`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    Markdown,
    Html,
    Auto,
}

/// One file-to-deck conversion job.
#[derive(Debug, Clone)]
pub struct BatchJob {
    pub input: PathBuf,
    /// When `None`, output is `{output_dir}/{input_stem}.pptx`.
    pub output: Option<PathBuf>,
    /// Override deck title; defaults to filename stem when unset.
    pub title: Option<String>,
}

impl BatchJob {
    pub fn new(input: impl Into<PathBuf>) -> Self {
        Self {
            input: input.into(),
            output: None,
            title: None,
        }
    }

    pub fn with_output(mut self, output: impl Into<PathBuf>) -> Self {
        self.output = Some(output.into());
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Shared options applied to every job in a batch run.
#[derive(Debug, Clone, Default)]
pub struct BatchOptions {
    pub output_dir: PathBuf,
    pub theme: Option<PresentationTheme>,
    pub template: Option<String>,
    pub html: HtmlParseOptions,
}

impl BatchOptions {
    pub fn new(output_dir: impl Into<PathBuf>) -> Self {
        Self {
            output_dir: output_dir.into(),
            ..Self::default()
        }
    }

    pub fn with_theme(mut self, theme: PresentationTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn with_template(mut self, template: impl Into<String>) -> Self {
        self.template = Some(template.into());
        self
    }

    pub fn with_html_options(mut self, html: HtmlParseOptions) -> Self {
        self.html = html;
        self
    }
}

/// Outcome of one successful batch conversion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchResult {
    pub input: PathBuf,
    pub output: PathBuf,
    pub title: String,
    pub slide_count: usize,
    pub bytes: usize,
}

/// Detect input format from a path's extension.
pub fn detect_format(path: &Path) -> Result<InputFormat> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "md" | "markdown" => Ok(InputFormat::Markdown),
        "html" | "htm" => Ok(InputFormat::Html),
        _ => Err(PptxError::Generic(format!(
            "Unsupported batch input extension \".{ext}\" for {}",
            path.display()
        ))),
    }
}

/// Resolve the output path for a job.
pub fn resolve_output_path(job: &BatchJob, options: &BatchOptions) -> PathBuf {
    job.output.clone().unwrap_or_else(|| {
        let stem = job
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("deck");
        options.output_dir.join(format!("{stem}.pptx"))
    })
}

fn default_title(job: &BatchJob) -> String {
    job.title.clone().unwrap_or_else(|| {
        job.input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Presentation")
            .to_string()
    })
}

fn build_settings(options: &BatchOptions) -> PresentationSettings {
    let mut settings = PresentationSettings::new();
    if let Some(theme) = &options.theme {
        settings = settings.theme(theme.clone());
    }
    if let Some(template) = &options.template {
        settings = settings.template(template.clone());
    }
    settings
}

fn parse_slides(
    path: &Path,
    format: InputFormat,
    html: &HtmlParseOptions,
) -> Result<Vec<SlideContent>> {
    let content = std::fs::read_to_string(path).map_err(|e| {
        PptxError::Generic(format!("Failed to read {}: {e}", path.display()))
    })?;

    let slides = match format {
        InputFormat::Html => parse_html_with_options(&content, html.clone())
            .map_err(PptxError::Generic)?,
        InputFormat::Markdown => parse_markdown_slides(&content, path)?,
        InputFormat::Auto => unreachable!("callers must resolve Auto before parse_slides"),
    };

    if slides.is_empty() {
        return Err(PptxError::Generic(format!(
            "No slides found in {}",
            path.display()
        )));
    }
    Ok(slides)
}

fn parse_markdown_slides(content: &str, path: &Path) -> Result<Vec<SlideContent>> {
    #[cfg(feature = "cli")]
    {
        crate::cli::markdown::parse_markdown_file(content, path).map_err(PptxError::Generic)
    }
    #[cfg(not(feature = "cli"))]
    {
        let _ = content;
        Err(PptxError::Generic(format!(
            "Markdown batch input requires the \"cli\" feature: {}",
            path.display()
        )))
    }
}

/// Convert a single Markdown or HTML file to `.pptx`.
pub fn convert_file(
    job: &BatchJob,
    options: &BatchOptions,
    format: InputFormat,
) -> Result<BatchResult> {
    let resolved_format = match format {
        InputFormat::Auto => detect_format(&job.input)?,
        other => other,
    };

    let slides = parse_slides(&job.input, resolved_format, &options.html)?;
    let title = default_title(job);
    let output = resolve_output_path(job, options);

    if let Some(parent) = output.parent()
        && !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }

    let settings = build_settings(options);
    let bytes = create_pptx_with_settings(&title, &slides, Some(settings))?;
    std::fs::write(&output, &bytes)?;

    Ok(BatchResult {
        input: job.input.clone(),
        output,
        title,
        slide_count: slides.len(),
        bytes: bytes.len(),
    })
}

/// Run a list of batch jobs sequentially.
pub fn run_batch(jobs: &[BatchJob], options: &BatchOptions) -> Result<Vec<BatchResult>> {
    std::fs::create_dir_all(&options.output_dir)?;
    jobs.iter()
        .map(|job| convert_file(job, options, InputFormat::Auto))
        .collect()
}

/// Discover `*.md`, `*.markdown`, `*.html`, and `*.htm` files in a directory (non-recursive).
pub fn discover_inputs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && detect_format(&path).is_ok() {
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

/// Convert every Markdown/HTML file in a directory to `{output_dir}/{stem}.pptx`.
pub fn run_directory(input_dir: &Path, options: &BatchOptions) -> Result<Vec<BatchResult>> {
    let inputs = discover_inputs(input_dir)?;
    if inputs.is_empty() {
        return Err(PptxError::Generic(format!(
            "No .md/.html files found in {}",
            input_dir.display()
        )));
    }
    let jobs: Vec<BatchJob> = inputs.into_iter().map(BatchJob::new).collect();
    run_batch(&jobs, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_md(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::File::create(&path)
            .unwrap()
            .write_all(body.as_bytes())
            .unwrap();
        path
    }

    #[test]
    fn detect_format_by_extension() {
        assert_eq!(
            detect_format(Path::new("deck.md")).unwrap(),
            InputFormat::Markdown
        );
        assert_eq!(
            detect_format(Path::new("page.HTML")).unwrap(),
            InputFormat::Html
        );
        assert!(detect_format(Path::new("notes.txt")).is_err());
    }

    #[test]
    fn discover_inputs_filters_supported_extensions() {
        let dir = std::env::temp_dir().join(format!("ppt_rs_batch_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        write_temp_md(&dir, "a.md", "# A\n\n- one");
        write_temp_md(&dir, "skip.txt", "nope");
        write_temp_md(&dir, "b.html", "<h1>B</h1><p>Hi</p>");

        let found = discover_inputs(&dir).unwrap();
        assert_eq!(found.len(), 2);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(feature = "cli")]
    #[test]
    fn run_directory_converts_markdown_files() {
        let root = std::env::temp_dir().join(format!("ppt_rs_batch_run_{}", std::process::id()));
        let input_dir = root.join("in");
        let output_dir = root.join("out");
        std::fs::create_dir_all(&input_dir).unwrap();

        write_temp_md(
            &input_dir,
            "report.md",
            "# Slide 1\n\n- Point A\n\n# Slide 2\n\n- Point B",
        );

        let results = run_directory(
            &input_dir,
            &BatchOptions::new(&output_dir),
        )
        .unwrap();

        assert_eq!(results.len(), 1);
        assert!(results[0].output.exists());
        assert!(results[0].slide_count >= 2);
        assert!(results[0].bytes > 1000);

        let _ = std::fs::remove_dir_all(&root);
    }
}
