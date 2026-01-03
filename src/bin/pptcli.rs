//! PPTX CLI - Command-line tool for creating PowerPoint presentations

use clap::Parser;
use ppt_rs::cli::{Cli, Commands, CreateCommand, FromMarkdownCommand, InfoCommand, ValidateCommand, ExportFormat};
use ppt_rs::api::Presentation;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { output, title, slides, template } => {
            match CreateCommand::execute(
                &output,
                title.as_deref(),
                slides,
                template.as_deref(),
            ) {
                Ok(_) => {
                    println!("✓ Created presentation: {output}");
                    let title = title.as_deref().unwrap_or("Presentation");
                    println!("  Title: {title}");
                    println!("  Slides: {slides}");
                }
                Err(e) => {
                    eprintln!("✗ Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Md2Ppt { input, output, title } => {
            // Auto-generate output if not provided
            let output_path = output.unwrap_or_else(|| {
                use std::path::Path;
                let input_path = Path::new(&input);
                if let Some(stem) = input_path.file_stem() {
                    if let Some(parent) = input_path.parent() {
                        if parent.as_os_str().is_empty() {
                            format!("{}.pptx", stem.to_string_lossy())
                        } else {
                            format!("{}/{}.pptx", parent.display(), stem.to_string_lossy())
                        }
                    } else {
                        format!("{}.pptx", stem.to_string_lossy())
                    }
                } else {
                    format!("{}.pptx", input)
                }
            });
            
            match FromMarkdownCommand::execute(
                &input,
                &output_path,
                title.as_deref(),
            ) {
                Ok(_) => {
                    println!("✓ Created presentation: {output_path}");
                    println!("  Input: {input}");
                    let title = title.as_deref().unwrap_or("Presentation from Markdown");
                    println!("  Title: {title}");
                }
                Err(e) => {
                    eprintln!("✗ Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Info { file } => {
            match InfoCommand::execute(&file) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("✗ Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Validate { file } => {
            match ValidateCommand::execute(&file) {
                Ok(_) => {
                    println!("\n✓ Validation completed successfully");
                }
                Err(e) => {
                    eprintln!("✗ Error: {e}");
                    std::process::exit(1);
                }
            }
        }
        Commands::Export { input, output, format } => {
            println!("Exporting {}...", input);
            let pres = match Presentation::from_path(&input) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("✗ Error loading presentation: {}", e);
                    std::process::exit(1);
                }
            };

            let format = format.or_else(|| {
                let out_lower = output.to_lowercase();
                if out_lower.ends_with(".pdf") {
                    Some(ExportFormat::Pdf)
                } else if out_lower.ends_with(".html") {
                    Some(ExportFormat::Html)
                } else if out_lower.ends_with(".png") {
                    Some(ExportFormat::Png)
                } else {
                    None
                }
            }).unwrap_or(ExportFormat::Pdf);

            let result = match format {
                ExportFormat::Pdf => pres.save_as_pdf(&output),
                ExportFormat::Html => pres.save_as_html(&output),
                ExportFormat::Png => pres.save_as_png(&output),
            };
            
            match result {
                Ok(_) => println!("✓ Export completed: {}", output),
                Err(e) => {
                    eprintln!("✗ Error exporting: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Merge { output, inputs } => {
            println!("Merging {} files into {}...", inputs.len(), output);
            let mut final_pres = match Presentation::from_path(&inputs[0]) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("✗ Error loading {}: {}", inputs[0], e);
                    std::process::exit(1);
                }
            };
            
            for input in inputs.iter().skip(1) {
                match Presentation::from_path(input) {
                    Ok(p) => {
                        final_pres = final_pres.add_presentation(p);
                    }
                    Err(e) => {
                        eprintln!("✗ Error loading {}: {}", input, e);
                        std::process::exit(1);
                    }
                }
            }
            
            match final_pres.save(&output) {
                Ok(_) => println!("✓ Merge completed: {}", output),
                Err(e) => {
                    eprintln!("✗ Error saving merged file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Pdf2Ppt { input, output } => {
            let output = output.unwrap_or_else(|| {
                let path = std::path::Path::new(&input);
                path.with_extension("pptx").to_string_lossy().to_string()
            });
            
            println!("Converting {} to {}...", input, output);
            match Presentation::from_pdf(&input) {
                Ok(p) => {
                    match p.save(&output) {
                        Ok(_) => println!("✓ Conversion completed: {}", output),
                        Err(e) => {
                            eprintln!("✗ Error saving presentation: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("✗ Error converting PDF: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Web2Ppt { url, output, title, max_slides, max_bullets, no_images, no_tables, no_code, no_source_url, timeout, verbose } => {
            execute_web2ppt(url, output, title, max_slides, max_bullets, no_images, no_tables, no_code, no_source_url, timeout, verbose);
        }
    }
}

#[cfg(feature = "web2ppt")]
fn execute_web2ppt(
    url: String,
    output: String,
    title: Option<String>,
    max_slides: usize,
    max_bullets: usize,
    no_images: bool,
    no_tables: bool,
    no_code: bool,
    no_source_url: bool,
    timeout: u64,
    verbose: bool,
) {
    use ppt_rs::{Web2PptConfig, ConversionOptions, WebFetcher, WebParser, Web2Ppt};

    if verbose {
        println!("🌐 Web2PPT - Converting webpage to PowerPoint");
        println!("   URL: {}", url);
        println!("   Output: {}", output);
        println!("   Max slides: {}", max_slides);
        println!("   Max bullets: {}", max_bullets);
        println!("   Include images: {}", !no_images);
        println!("   Include tables: {}", !no_tables);
        println!("   Include code: {}", !no_code);
        println!();
        println!("📥 Fetching webpage...");
    }

    // Build config
    let config = Web2PptConfig::new()
        .max_slides(max_slides)
        .max_bullets(max_bullets)
        .with_images(!no_images)
        .with_tables(!no_tables)
        .with_code(!no_code)
        .timeout(timeout);

    // Build options
    let mut options = ConversionOptions::new()
        .with_source_url(!no_source_url);

    if let Some(t) = title.as_ref() {
        options = options.title(t);
    }

    // Fetch
    let fetcher = match WebFetcher::with_config(config.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("✗ Failed to create fetcher: {}", e);
            std::process::exit(1);
        }
    };

    let html = match fetcher.fetch(&url) {
        Ok(h) => {
            if verbose {
                println!("   Fetched {} bytes of HTML", h.len());
            }
            h
        }
        Err(e) => {
            eprintln!("✗ Failed to fetch: {}", e);
            std::process::exit(1);
        }
    };

    // Parse
    let parser = WebParser::with_config(config.clone());
    let content = match parser.parse(&html, &url) {
        Ok(c) => {
            if verbose {
                println!("📄 Parsed content:");
                println!("   Title: {}", c.title);
                println!("   Description: {}", c.description.as_deref().unwrap_or("(none)"));
                println!("   Content blocks: {}", c.blocks.len());
                println!("   Images: {}", c.images.len());
                
                // Show headings found
                let headings: Vec<_> = c.blocks.iter()
                    .filter(|b| b.is_heading())
                    .map(|b| b.text.as_str())
                    .take(10)
                    .collect();
                if !headings.is_empty() {
                    println!("   Headings found: {:?}", headings);
                }
            }
            c
        }
        Err(e) => {
            eprintln!("✗ Failed to parse: {}", e);
            std::process::exit(1);
        }
    };

    // Convert
    let converter = Web2Ppt::with_config(config);
    match converter.convert(&content, &options) {
        Ok(pptx_data) => {
            if verbose {
                println!("✅ Conversion successful!");
                println!("   Size: {} bytes", pptx_data.len());
                println!("💾 Saving to {}...", output);
            }

            match std::fs::write(&output, &pptx_data) {
                Ok(_) => {
                    println!("✓ Created: {} ({} bytes)", output, pptx_data.len());
                }
                Err(e) => {
                    eprintln!("✗ Failed to save file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Conversion failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "web2ppt"))]
fn execute_web2ppt(
    _url: String,
    _output: String,
    _title: Option<String>,
    _max_slides: usize,
    _max_bullets: usize,
    _no_images: bool,
    _no_tables: bool,
    _no_code: bool,
    _no_source_url: bool,
    _timeout: u64,
    _verbose: bool,
) {
    eprintln!("✗ web2ppt feature is not enabled.");
    eprintln!("  Rebuild with: cargo build --features web2ppt");
    std::process::exit(1);
}
