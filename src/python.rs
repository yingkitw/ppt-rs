//! Python bindings (PyPI / `pip install ppt-rs`) via PyO3.
#![allow(unsafe_op_in_unsafe_fn)]

use crate::core::{
    PackageValidationIssue, PackageValidationReport, ValidationCategory, ValidationSeverity,
    validate_package_bytes,
};
use crate::exc::PptxError;
use crate::generator::{SlideContent, create_pptx_with_content};
use crate::import::parse_html;
use crate::oxml::extract_slide_texts_from_bytes;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn map_err(err: PptxError) -> PyErr {
    PyValueError::new_err(err.to_string())
}

fn map_parse_err(err: String) -> PyErr {
    PyValueError::new_err(err)
}

fn validation_category_name(category: ValidationCategory) -> &'static str {
    match category {
        ValidationCategory::MissingPart => "missing_part",
        ValidationCategory::Relationship => "relationship",
        ValidationCategory::ContentType => "content_type",
        ValidationCategory::Presentation => "presentation",
        ValidationCategory::SlideMaster => "slide_master",
        ValidationCategory::Slide => "slide",
        ValidationCategory::Chart => "chart",
        ValidationCategory::Xml => "xml",
        ValidationCategory::Theme => "theme",
    }
}

fn validation_severity_name(severity: ValidationSeverity) -> &'static str {
    match severity {
        ValidationSeverity::Warning => "warning",
        ValidationSeverity::Error => "error",
    }
}

fn report_to_dict<'py>(
    py: Python<'py>,
    report: &PackageValidationReport,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new_bound(py);
    dict.set_item("valid", report.is_valid())?;
    dict.set_item("error_count", report.error_count())?;
    dict.set_item("warning_count", report.warning_count())?;
    let issues = PyList::empty_bound(py);
    for issue in &report.issues {
        issues.append(issue_to_dict(py, issue)?)?;
    }
    dict.set_item("issues", issues)?;
    Ok(dict)
}

fn issue_to_dict<'py>(
    py: Python<'py>,
    issue: &PackageValidationIssue,
) -> PyResult<Bound<'py, PyDict>> {
    let d = PyDict::new_bound(py);
    d.set_item("category", validation_category_name(issue.category))?;
    d.set_item("severity", validation_severity_name(issue.severity))?;
    d.set_item("message", &issue.message)?;
    d.set_item("path", &issue.path)?;
    Ok(d)
}

fn slides_from_py(slides: &Bound<'_, PyList>) -> PyResult<Vec<SlideContent>> {
    let mut out = Vec::with_capacity(slides.len());
    for (i, item) in slides.iter().enumerate() {
        let slide = if let Ok(dict) = item.downcast::<PyDict>() {
            let title: String = dict
                .get_item("title")?
                .and_then(|v| v.extract().ok())
                .ok_or_else(|| {
                    PyValueError::new_err(format!("slides[{i}]: missing or invalid 'title'"))
                })?;
            let mut content = SlideContent::new(&title);
            if let Some(bullets) = dict.get_item("bullets")? {
                let list = bullets.downcast::<PyList>().map_err(|_| {
                    PyTypeError::new_err(format!("slides[{i}].bullets must be a list of strings"))
                })?;
                for (j, b) in list.iter().enumerate() {
                    let text: String = b.extract().map_err(|_| {
                        PyTypeError::new_err(format!(
                            "slides[{i}].bullets[{j}] must be a string"
                        ))
                    })?;
                    content = content.add_bullet(&text);
                }
            }
            content
        } else if let Ok(pair) = item.downcast::<PyList>() {
            if pair.len() < 1 {
                return Err(PyValueError::new_err(format!(
                    "slides[{i}]: expected [title, bullets?] with at least a title"
                )));
            }
            let title: String = pair.get_item(0)?.extract()?;
            let mut content = SlideContent::new(&title);
            if pair.len() >= 2 {
                let bullet_item = pair.get_item(1)?;
                let bullets = bullet_item.downcast::<PyList>().map_err(|_| {
                    PyTypeError::new_err(format!("slides[{i}][1] must be a list of strings"))
                })?;
                for b in bullets.iter() {
                    let text: String = b.extract()?;
                    content = content.add_bullet(&text);
                }
            }
            content
        } else {
            return Err(PyTypeError::new_err(format!(
                "slides[{i}]: expected dict with title/bullets or [title, bullets] list"
            )));
        };
        out.push(slide);
    }
    Ok(out)
}

#[pyfunction]
fn version() -> &'static str {
    crate::VERSION
}

#[pyfunction]
#[pyo3(signature = (markdown, title=None))]
fn markdown_to_pptx(markdown: &str, title: Option<String>) -> PyResult<Vec<u8>> {
    let slides =
        crate::cli::parse_markdown(markdown).map_err(map_parse_err)?;
    if slides.is_empty() {
        return Err(PyValueError::new_err("Markdown produced no slides"));
    }
    let title = title.unwrap_or_else(|| "Presentation from Markdown".into());
    create_pptx_with_content(&title, slides).map_err(map_err)
}

#[pyfunction]
#[pyo3(signature = (html, title=None))]
fn html_to_pptx(html: &str, title: Option<String>) -> PyResult<Vec<u8>> {
    let slides = parse_html(html).map_err(map_parse_err)?;
    if slides.is_empty() {
        return Err(PyValueError::new_err("HTML produced no slides"));
    }
    let title = title.unwrap_or_else(|| "Presentation from HTML".into());
    create_pptx_with_content(&title, slides).map_err(map_err)
}

#[pyfunction]
#[pyo3(signature = (title, slides))]
fn create_presentation(title: &str, slides: &Bound<'_, PyList>) -> PyResult<Vec<u8>> {
    let slide_models = slides_from_py(slides)?;
    if slide_models.is_empty() {
        return Err(PyValueError::new_err("slides must not be empty"));
    }
    create_pptx_with_content(title, slide_models).map_err(map_err)
}

#[pyfunction]
fn validate_pptx<'py>(py: Python<'py>, data: &[u8]) -> PyResult<Bound<'py, PyDict>> {
    let report = validate_package_bytes(data);
    report_to_dict(py, &report)
}

#[pyfunction]
fn extract_text(data: &[u8]) -> PyResult<Vec<Vec<String>>> {
    extract_slide_texts_from_bytes(data).map_err(map_err)
}

#[pymodule]
fn _native(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(markdown_to_pptx, m)?)?;
    m.add_function(wrap_pyfunction!(html_to_pptx, m)?)?;
    m.add_function(wrap_pyfunction!(create_presentation, m)?)?;
    m.add_function(wrap_pyfunction!(validate_pptx, m)?)?;
    m.add_function(wrap_pyfunction!(extract_text, m)?)?;
    m.add("__version__", crate::VERSION)?;
    Ok(())
}
