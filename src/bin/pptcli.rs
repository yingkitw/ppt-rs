//! PPTX CLI - Command-line tool for creating PowerPoint presentations

use clap::Parser;
use ppt_rs::cli::{Cli, Commands, CreateCommand, FromMarkdownCommand, InfoCommand};

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
    }
}
