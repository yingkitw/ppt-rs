//! PPTX CLI - Command-line tool for creating PowerPoint presentations

use pptx::cli::{Parser, Command, CreateCommand, InfoCommand};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = &args[1..]; // Skip program name

    match Parser::parse(args) {
        Ok(Command::Create(create_args)) => {
            match CreateCommand::execute(
                &create_args.output,
                create_args.title.as_deref(),
                create_args.slides,
                create_args.template.as_deref(),
            ) {
                Ok(_) => {
                    println!("✓ Created presentation: {}", create_args.output);
                    println!("  Title: {}", create_args.title.as_deref().unwrap_or("Presentation"));
                    println!("  Slides: {}", create_args.slides);
                }
                Err(e) => eprintln!("✗ Error: {}", e),
            }
        }
        Ok(Command::Info(info_args)) => {
            match InfoCommand::execute(&info_args.file) {
                Ok(_) => {}
                Err(e) => eprintln!("✗ Error: {}", e),
            }
        }
        Ok(Command::Help) => print_help(),
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            print_help();
        }
    }
}

fn print_help() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║           PPTX CLI - PowerPoint Generator                 ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    println!("USAGE:");
    println!("  pptx-cli <command> [options]");
    println!();
    println!("COMMANDS:");
    println!("  create <file.pptx>    Create a new presentation");
    println!("  info <file.pptx>      Show presentation information");
    println!("  help                  Show this help message");
    println!();
    println!("CREATE OPTIONS:");
    println!("  --title <text>        Set presentation title");
    println!("  --slides <count>      Number of slides to create (default: 1)");
    println!("  --template <file>     Use template file");
    println!();
    println!("EXAMPLES:");
    println!("  pptx-cli create my.pptx");
    println!("  pptx-cli create my.pptx --title 'My Presentation' --slides 5");
    println!("  pptx-cli create output/demo.pptx --title 'Demo' --slides 10");
    println!("  pptx-cli info my.pptx");
    println!();
}
