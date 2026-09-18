//! main.rs — Boneyard CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, -f/--format, -o/--output, -q/--quiet, -v/--verbose.

mod budget;
mod cli;
mod doctor;
mod update;
mod xdg;

use boneyard::{enrich, load_hall_file, parse_hall_json, parse_policy_toml, report, Policy};
use cli::{parse_args, print_help, CliConfig, CliError, OutputFormat, Subcommand, VERSION};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

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
        fs::write(path, content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o644));
        }
        Ok(())
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn get_enriched(config: &CliConfig) -> Result<(boneyard::enrich::EnrichedHall, Policy), CliError> {
    let hall = read_hall(config.input_file.as_ref())?;
    if config.verbose {
        eprintln!(
            "boneyard: ingested {} repositories for org '{}'",
            hall.repos.len(),
            hall.org_name
        );
    }
    let policy = if let Some(ref p_path) = config.policy_file {
        let content = fs::read_to_string(p_path)
            .map_err(|e| format!("Failed to read policy {}: {}", p_path.display(), e))?;
        parse_policy_toml(&content)
    } else {
        Policy::default()
    };
    let enriched = enrich(&hall, &policy).map_err(|e| format!("Enrichment failed: {}", e))?;
    Ok((enriched, policy))
}

fn run_app(config: &CliConfig) -> Result<i32, CliError> {
    let (enriched, policy) = get_enriched(config)?;

    match config.subcommand {
        Subcommand::Budget => {
            let out = budget::emit_budget(&enriched, config.format);
            write_output(&out, config.output_file.as_ref())?;
            Ok(0)
        }
        Subcommand::PolicyCheck => {
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
            write_output(&formatted, config.output_file.as_ref())?;
            Ok(if enriched.critical_count > 0 { 1 } else { 0 })
        }
    }
}

fn run() -> Result<i32, CliError> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    match config.subcommand {
        Subcommand::Help => {
            print_help();
            Ok(0)
        }
        Subcommand::Version => {
            println!("boneyard {}", VERSION);
            Ok(0)
        }
        Subcommand::Doctor => {
            let (code, output) = doctor::run_doctor("boneyard", VERSION, config.format);
            if !config.quiet || config.output_file.is_some() {
                write_output(&output, config.output_file.as_ref())?;
            }
            Ok(code)
        }
        Subcommand::Update => {
            let (code, output) =
                update::run_update("boneyard", VERSION, config.format)
                    .map_err(CliError::Runtime)?;
            if !config.quiet || config.output_file.is_some() {
                write_output(&output, config.output_file.as_ref())?;
            }
            Ok(code)
        }
        _ => run_app(&config),
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(CliError::Parse(err)) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
        Err(CliError::Runtime(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
