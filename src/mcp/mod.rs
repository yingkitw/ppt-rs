//! MCP (Model Context Protocol) server module
//!
//! Exposes ppt-rs capabilities as MCP tools via [rmcp](https://crates.io/crates/rmcp).

use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
    handler::server::wrapper::Parameters,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars,
    tool, tool_handler, tool_router,
};
use serde::Deserialize;
use serde_json::json;

// ---------------------------------------------------------------------------
// Tool argument types (JSON Schema via schemars)
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreatePresentationArgs {
    pub title: String,
    pub output_path: String,
    pub slides: Vec<SlideArg>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SlideArg {
    pub title: String,
    #[serde(default)]
    pub bullets: Vec<String>,
    #[serde(default)]
    pub layout: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MarkdownToPptxArgs {
    pub markdown: String,
    pub output_path: String,
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PathArg {
    pub path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportPptxArgs {
    pub input_path: String,
    pub output_path: String,
    pub format: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MergePptxArgs {
    pub input_paths: Vec<String>,
    pub output_path: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateWithTablesArgs {
    pub title: String,
    pub output_path: String,
    pub tables: Vec<TableSlideArg>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TableSlideArg {
    pub slide_title: String,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ChartSeriesArg {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ChartSlideArg {
    pub slide_title: String,
    pub chart_title: String,
    pub chart_type: String,
    pub categories: Vec<String>,
    pub series: Vec<ChartSeriesArg>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateWithChartsArgs {
    pub title: String,
    pub output_path: String,
    pub charts: Vec<ChartSlideArg>,
}

// ---------------------------------------------------------------------------
// MCP server
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Default)]
pub struct PptMcp;

fn tool_text(text: impl Into<String>) -> CallToolResult {
    CallToolResult::success(vec![Content::text(text)])
}

fn tool_error(text: impl Into<String>) -> CallToolResult {
    CallToolResult::error(vec![Content::text(format!("Error: {}", text.into()))])
}

#[tool_router]
impl PptMcp {
    #[tool(
        name = "create_presentation",
        description = "Create a new PowerPoint presentation from structured slide data. Each slide has a title and bullet points."
    )]
    fn create_presentation(
        &self,
        Parameters(args): Parameters<CreatePresentationArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(handle_create_presentation(args))
    }

    #[tool(
        name = "markdown_to_pptx",
        description = "Convert Markdown text to a PowerPoint (.pptx) file. Supports headings as slide titles, bullet lists, bold/italic, code blocks, tables, and more."
    )]
    fn markdown_to_pptx(
        &self,
        Parameters(args): Parameters<MarkdownToPptxArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(handle_markdown_to_pptx(args))
    }

    #[tool(
        name = "get_pptx_info",
        description = "Read a PowerPoint file and return its metadata: title, slide count, slide titles, and content summary."
    )]
    fn get_pptx_info(&self, Parameters(args): Parameters<PathArg>) -> Result<CallToolResult, McpError> {
        Ok(handle_get_pptx_info(&args.path))
    }

    #[tool(
        name = "export_pptx",
        description = "Export a PowerPoint file to another format: HTML, PDF, Markdown, or PNG images."
    )]
    fn export_pptx(&self, Parameters(args): Parameters<ExportPptxArgs>) -> Result<CallToolResult, McpError> {
        Ok(handle_export_pptx(args))
    }

    #[tool(
        name = "merge_pptx",
        description = "Merge multiple PowerPoint files into a single presentation."
    )]
    fn merge_pptx(&self, Parameters(args): Parameters<MergePptxArgs>) -> Result<CallToolResult, McpError> {
        Ok(handle_merge_pptx(args))
    }

    #[tool(
        name = "validate_pptx",
        description = "Validate a PowerPoint file for structural integrity and ECMA-376 compliance. Reports any issues found."
    )]
    fn validate_pptx(&self, Parameters(args): Parameters<PathArg>) -> Result<CallToolResult, McpError> {
        Ok(handle_validate_pptx(&args.path))
    }

    #[tool(
        name = "create_presentation_with_tables",
        description = "Create a PowerPoint presentation containing tables with styled data."
    )]
    fn create_presentation_with_tables(
        &self,
        Parameters(args): Parameters<CreateWithTablesArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(handle_create_with_tables(args))
    }

    #[tool(
        name = "create_presentation_with_charts",
        description = "Create a PowerPoint presentation with bar, line, or pie charts."
    )]
    fn create_presentation_with_charts(
        &self,
        Parameters(args): Parameters<CreateWithChartsArgs>,
    ) -> Result<CallToolResult, McpError> {
        Ok(handle_create_with_charts(args))
    }
}

#[tool_handler]
impl ServerHandler for PptMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .build(),
        )
        .with_server_info(Implementation::new(
            "ppt-rs",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_protocol_version(ProtocolVersion::V_2024_11_05)
    }
}

// ---------------------------------------------------------------------------
// Tool handlers
// ---------------------------------------------------------------------------

fn handle_create_presentation(args: CreatePresentationArgs) -> CallToolResult {
    let CreatePresentationArgs {
        title,
        output_path,
        slides: slides_val,
    } = args;

    let mut slides = Vec::new();
    for slide_val in slides_val {
        let _layout = slide_val.layout;
        let slide_title = slide_val.title;
        let mut slide = crate::generator::SlideContent::new(&slide_title);
        for b in slide_val.bullets {
            slide = slide.add_bullet(&b);
        }
        slides.push(slide);
    }

    if slides.is_empty() {
        return tool_error("No slides provided");
    }

    let pres = crate::api::Presentation::with_title(&title);
    let mut pres = pres;
    for s in slides {
        pres = pres.add_slide(s);
    }

    match pres.save(&output_path) {
        Ok(()) => tool_text(format!(
            "Created presentation '{}' with {} slide(s) at: {}",
            title,
            pres.slide_count(),
            output_path
        )),
        Err(e) => tool_error(format!("Failed to save: {}", e)),
    }
}

fn handle_markdown_to_pptx(args: MarkdownToPptxArgs) -> CallToolResult {
    let MarkdownToPptxArgs {
        markdown,
        output_path,
        title,
    } = args;
    let title = title.unwrap_or_else(|| "Presentation from Markdown".into());

    let mut pres = crate::api::Presentation::with_title(&title);

    let parsed_slides = match crate::cli::markdown::parse_markdown(&markdown) {
        Ok(s) => s,
        Err(e) => return tool_error(format!("Failed to parse markdown: {}", e)),
    };
    for slide in &parsed_slides {
        pres = pres.add_slide(slide.clone());
    }

    match pres.save(&output_path) {
        Ok(()) => tool_text(format!(
            "Converted markdown to presentation with {} slide(s) at: {}",
            parsed_slides.len(),
            output_path
        )),
        Err(e) => tool_error(format!("Failed to save: {}", e)),
    }
}

fn handle_get_pptx_info(path: &str) -> CallToolResult {
    match crate::api::Presentation::from_path(path) {
        Ok(pres) => {
            let mut slide_summaries = Vec::new();
            for (i, slide) in pres.slides().iter().enumerate() {
                let bullet_count = slide.bullets.len();
                let has_table = slide.table.is_some();
                let has_images = !slide.images.is_empty();
                let has_charts = !slide.charts.is_empty();
                let has_shapes = !slide.shapes.is_empty();

                let mut features = Vec::new();
                if has_table {
                    features.push("table");
                }
                if has_images {
                    features.push("images");
                }
                if has_charts {
                    features.push("charts");
                }
                if has_shapes {
                    features.push("shapes");
                }

                slide_summaries.push(json!({
                    "slide_number": i + 1,
                    "title": slide.title,
                    "bullet_count": bullet_count,
                    "features": features,
                }));
            }

            let info = json!({
                "title": pres.get_title(),
                "slide_count": pres.slide_count(),
                "slides": slide_summaries,
            });

            tool_text(serde_json::to_string_pretty(&info).unwrap_or_default())
        }
        Err(e) => tool_error(format!("Failed to read presentation: {}", e)),
    }
}

fn handle_export_pptx(args: ExportPptxArgs) -> CallToolResult {
    let ExportPptxArgs {
        input_path,
        output_path,
        format: fmt,
    } = args;

    match fmt.as_str() {
        "html" | "pdf" | "markdown" | "png" => {}
        _ => {
            return tool_error(format!(
                "Unsupported format: {}. Use html, pdf, markdown, or png.",
                fmt
            ));
        }
    }

    let pres = match crate::api::Presentation::from_path(&input_path) {
        Ok(p) => p,
        Err(e) => return tool_error(format!("Failed to load presentation: {}", e)),
    };

    let result = match fmt.as_str() {
        "html" => pres.save_as_html(&output_path),
        "pdf" => pres.save_as_pdf(&output_path),
        "markdown" => pres.save_as_markdown(&output_path),
        "png" => pres.save_as_png(&output_path),
        _ => unreachable!(),
    };

    match result {
        Ok(()) => tool_text(format!("Exported to {} at: {}", fmt, output_path)),
        Err(e) => tool_error(format!("Export failed: {}", e)),
    }
}

fn handle_merge_pptx(args: MergePptxArgs) -> CallToolResult {
    let MergePptxArgs {
        input_paths,
        output_path,
    } = args;

    if input_paths.len() < 2 {
        return tool_error("At least 2 input files required");
    }

    let mut merged = match crate::api::Presentation::from_path(&input_paths[0]) {
        Ok(p) => p,
        Err(e) => return tool_error(format!("Failed to load {}: {}", input_paths[0], e)),
    };

    for path in &input_paths[1..] {
        match crate::api::Presentation::from_path(path) {
            Ok(p) => merged = merged.add_presentation(p),
            Err(e) => return tool_error(format!("Failed to load {}: {}", path, e)),
        }
    }

    let total_slides = merged.slide_count();
    match merged.save(&output_path) {
        Ok(()) => tool_text(format!(
            "Merged {} files ({} total slides) to: {}",
            input_paths.len(),
            total_slides,
            output_path
        )),
        Err(e) => tool_error(format!("Failed to save merged file: {}", e)),
    }
}

fn handle_validate_pptx(path: &str) -> CallToolResult {
    match crate::oxml::repair::PptxRepair::open(path) {
        Ok(mut repair) => {
            let issues = repair.validate();
            if issues.is_empty() {
                tool_text("Validation passed. No issues found.")
            } else {
                let mut msgs = vec![format!("Found {} issue(s):", issues.len())];
                for issue in &issues {
                    let sev = match issue.severity() {
                        3 => "critical",
                        2 => "warning",
                        _ => "info",
                    };
                    msgs.push(format!("  [{}] {}", sev, issue.description(),));
                }
                tool_text(msgs.join("\n"))
            }
        }
        Err(e) => tool_error(format!("Failed to open file: {}", e)),
    }
}

fn handle_create_with_tables(args: CreateWithTablesArgs) -> CallToolResult {
    let CreateWithTablesArgs {
        title,
        output_path,
        tables: tables_val,
    } = args;

    let mut pres = crate::api::Presentation::with_title(&title);

    for tbl_val in &tables_val {
        let slide_title = tbl_val.slide_title.as_str();
        let rows_val = &tbl_val.rows;

        if rows_val.is_empty() {
            continue;
        }

        let col_count = rows_val[0].len();
        if col_count == 0 {
            continue;
        }

        let col_widths = vec![2800000u32; col_count];
        let mut builder = crate::generator::TableBuilder::new(col_widths);

        for (ri, cells_val) in rows_val.iter().enumerate() {
            let mut cells = Vec::new();
            for text in cells_val {
                let mut cell = crate::generator::TableCell::new(text.as_str());
                if ri == 0 {
                    cell = cell.bold();
                }
                cells.push(cell);
            }
            builder = builder.add_row(crate::generator::TableRow::new(cells));
        }

        let table = builder.build();
        let slide = crate::generator::SlideContent::new(slide_title).table(table);
        pres = pres.add_slide(slide);
    }

    match pres.save(&output_path) {
        Ok(()) => tool_text(format!(
            "Created presentation with {} table slide(s) at: {}",
            tables_val.len(),
            output_path
        )),
        Err(e) => tool_error(format!("Failed to save: {}", e)),
    }
}

fn handle_create_with_charts(args: CreateWithChartsArgs) -> CallToolResult {
    let CreateWithChartsArgs {
        title,
        output_path,
        charts: charts_val,
    } = args;

    let mut pres = crate::api::Presentation::with_title(&title);

    for chart_val in &charts_val {
        let slide_title = chart_val.slide_title.as_str();
        let chart_title = chart_val.chart_title.as_str();
        let chart_type_str = chart_val.chart_type.as_str();
        let categories_refs: Vec<&str> = chart_val.categories.iter().map(|s| s.as_str()).collect();

        let chart_type = match chart_type_str {
            "line" => crate::generator::ChartType::Line,
            "pie" => crate::generator::ChartType::Pie,
            "area" => crate::generator::ChartType::Area,
            _ => crate::generator::ChartType::Bar,
        };

        let mut chart_builder = crate::generator::ChartBuilder::new(chart_title, chart_type)
            .categories(categories_refs)
            .position(1000000u32, 1000000u32)
            .size(4000000u32, 3000000u32);

        for sv in &chart_val.series {
            chart_builder = chart_builder.add_series(crate::generator::ChartSeries::new(
                sv.name.as_str(),
                sv.values.clone(),
            ));
        }

        let chart = chart_builder.build();
        let slide = crate::generator::SlideContent::new(slide_title).add_chart(chart);
        pres = pres.add_slide(slide);
    }

    match pres.save(&output_path) {
        Ok(()) => tool_text(format!(
            "Created presentation with {} chart slide(s) at: {}",
            charts_val.len(),
            output_path
        )),
        Err(e) => tool_error(format!("Failed to save: {}", e)),
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service = PptMcp.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
