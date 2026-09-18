//! main.rs — Boneyard CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, --format, -o/--output, -q/--quiet, -v/--verbose.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use boneyard::{enrich, load_hall_file, parse_hall_json, parse_policy_toml, report, Policy};

const VERSION: &str = "0.2.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug)]
struct CliConfig {
    subcommand: String,
    input_file: Option<PathBuf>,
    policy_file: Option<PathBuf>,
    format: OutputFormat,
    output_file: Option<PathBuf>,
    quiet: bool,
    verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: "enrich".to_string(),
            input_file: None,
            policy_file: None,
            format: OutputFormat::Text,
            output_file: None,
            quiet: false,
            verbose: false,
        }
    }
}

fn print_help() {
    println!(
        "boneyard {} — Org-wide tech-debt radar\n\
        \n\
        USAGE:\n\
          boneyard [SUBCOMMAND] [OPTIONS] [FILE]\n\
        \n\
        SUBCOMMANDS:\n\
          enrich         Score repository tech-debt and dormancy (default)\n\
          report         Generate executive Markdown/JSON remediation report\n\
          policy check   Assert compliance against organization policy\n\
        \n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          --format <fmt>          Output format: text, json, markdown [default: text]\n\
          -o, --output <file>     Write report to file instead of stdout\n\
          -i, --input <file>      Input JSON hall file\n\
          --policy <file>         Custom policy TOML configuration\n\
          -q, --quiet             Quiet mode; exit code only\n\
          -v, --verbose           Verbose diagnostic logging to stderr\n\
        \n\
        EXAMPLES:\n\
          boneyard enrich repos.json\n\
          boneyard report -i repos.json --format markdown -o REPORT.md\n\
          boneyard policy check -i repos.json --policy policy.toml\n",
        VERSION
    );
}

fn parse_args(args: &[String]) -> Result<Option<CliConfig>, String> {
    let mut config = CliConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(None);
            }
            "-V" | "--version" => {
                println!("boneyard {}", VERSION);
                return Ok(None);
            }
            "-q" | "--quiet" => config.quiet = true,
            "-v" | "--verbose" => config.verbose = true,
            "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for --format".to_string());
                }
                config.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json,
                    "markdown" | "md" => OutputFormat::Markdown,
                    "text" => OutputFormat::Text,
                    other => return Err(format!("Unknown format: {}", other)),
                };
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for -o/--output".to_string());
                }
                config.output_file = Some(PathBuf::from(&args[i]));
            }
            "-i" | "--input" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for -i/--input".to_string());
                }
                config.input_file = Some(PathBuf::from(&args[i]));
            }
            "--policy" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for --policy".to_string());
                }
                config.policy_file = Some(PathBuf::from(&args[i]));
            }
            "policy" => {
                if i + 1 < args.len() && args[i + 1] == "check" {
                    i += 1;
                    config.subcommand = "policy_check".to_string();
                }
            }
            "enrich" | "report" => {
                config.subcommand = args[i].clone();
            }
            arg if !arg.starts_with('-') => {
                config.input_file = Some(PathBuf::from(arg));
            }
            other => return Err(format!("Unknown option: {}", other)),
        }
        i += 1;
    }
    Ok(Some(config))
}

fn read_hall(path: Option<&PathBuf>) -> Result<boneyard::Hall, String> {
    if let Some(p) = path {
        load_hall_file(p).map_err(|e| format!("Failed to load hall: {}", e))
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read stdin: {}", e))?;
        parse_hall_json(&buffer).map_err(|e| format!("Failed to parse JSON: {}", e))
    }
}

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target {
        fs::write(path, content)
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args)? {
        Some(c) => c,
        None => return Ok(0),
    };

    let hall = read_hall(config.input_file.as_ref())?;
    if config.verbose {
        eprintln!("boneyard: ingested {} repositories for org '{}'", hall.repos.len(), hall.org_name);
    }

    let policy = if let Some(ref p_path) = config.policy_file {
        let content = fs::read_to_string(p_path)
            .map_err(|e| format!("Failed to read policy {}: {}", p_path.display(), e))?;
        parse_policy_toml(&content)
    } else {
        Policy::default()
    };

    let enriched = enrich(&hall, &policy).map_err(|e| format!("Enrichment failed: {}", e))?;

    match config.subcommand.as_str() {
        "policy_check" => {
            let verdict = boneyard::policy::evaluate(&enriched, &policy);
            if !config.quiet {
                if verdict.passed {
                    println!("boneyard policy check: PASSED (org tech-debt within thresholds)");
                } else {
                    eprintln!("boneyard policy check: FAILED");
                    for v in &verdict.violations {
                        eprintln!("  - {}", v);
                    }
                }
            }
            Ok(if verdict.passed { 0 } else { 1 })
        }
        _ => {
            let formatted = match config.format {
                OutputFormat::Json => report::emit_json(&enriched),
                OutputFormat::Markdown => report::emit_markdown(&enriched),
                OutputFormat::Text => report::emit_text(&enriched),
            };
            write_output(&formatted, config.output_file.as_ref())
                .map_err(|e| format!("Write failed: {}", e))?;
            Ok(if enriched.critical_count > 0 { 1 } else { 0 })
        }
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
    }
}
