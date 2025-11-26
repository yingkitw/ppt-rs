//! Command-line argument parser


#[derive(Debug, Clone)]
pub enum Command {
    Create(CreateArgs),
    Info(InfoArgs),
    Help,
}

#[derive(Debug, Clone)]
pub struct CreateArgs {
    pub output: String,
    pub title: Option<String>,
    pub slides: usize,
    pub template: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InfoArgs {
    pub file: String,
}

pub struct Parser;

impl Parser {
    pub fn parse(args: &[String]) -> Result<Command, String> {
        if args.is_empty() {
            return Ok(Command::Help);
        }

        match args[0].as_str() {
            "create" => Self::parse_create(&args[1..]),
            "info" => Self::parse_info(&args[1..]),
            "help" | "-h" | "--help" => Ok(Command::Help),
            cmd => Err(format!("Unknown command: {}", cmd)),
        }
    }

    fn parse_create(args: &[String]) -> Result<Command, String> {
        if args.is_empty() {
            return Err("create requires an output file".to_string());
        }

        let output = args[0].clone();
        let mut title = None;
        let mut slides = 1;
        let mut template = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--title" => {
                    if i + 1 < args.len() {
                        title = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err("--title requires an argument".to_string());
                    }
                }
                "--slides" => {
                    if i + 1 < args.len() {
                        slides = args[i + 1].parse()
                            .map_err(|_| "Invalid slide count".to_string())?;
                        i += 2;
                    } else {
                        return Err("--slides requires an argument".to_string());
                    }
                }
                "--template" => {
                    if i + 1 < args.len() {
                        template = Some(args[i + 1].clone());
                        i += 2;
                    } else {
                        return Err("--template requires an argument".to_string());
                    }
                }
                _ => {
                    return Err(format!("Unknown option: {}", args[i]));
                }
            }
        }

        Ok(Command::Create(CreateArgs {
            output,
            title,
            slides,
            template,
        }))
    }

    fn parse_info(args: &[String]) -> Result<Command, String> {
        if args.is_empty() {
            return Err("info requires a file path".to_string());
        }

        Ok(Command::Info(InfoArgs {
            file: args[0].clone(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_create() {
        let args = vec![
            "create".to_string(),
            "test.pptx".to_string(),
            "--title".to_string(),
            "My Presentation".to_string(),
        ];
        let cmd = Parser::parse(&args).unwrap();
        match cmd {
            Command::Create(create_args) => {
                assert_eq!(create_args.output, "test.pptx");
                assert_eq!(create_args.title, Some("My Presentation".to_string()));
            }
            _ => panic!("Expected Create command"),
        }
    }

    #[test]
    fn test_parse_info() {
        let args = vec!["info".to_string(), "test.pptx".to_string()];
        let cmd = Parser::parse(&args).unwrap();
        match cmd {
            Command::Info(info_args) => {
                assert_eq!(info_args.file, "test.pptx");
            }
            _ => panic!("Expected Info command"),
        }
    }
}
