use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use declart_core::render::DEFAULT_THEME;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "declart", version, about = "Declare what to show. The engine decides how it looks.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a diagram declaration to SVG
    Render {
        /// Input TOML file
        input: PathBuf,
        /// Output SVG file [default: input with .svg extension]
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme name
        #[arg(long, default_value = "default")]
        theme: String,
        /// Write SVG to stdout instead of file
        #[arg(long)]
        stdout: bool,
    },
    /// Validate a diagram declaration without rendering
    Validate {
        /// Input TOML file
        input: PathBuf,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Render { input, output, theme: theme_name, stdout } => {
            let content = fs::read_to_string(&input)
                .with_context(|| format!("failed to read {}", input.display()))?;

            let model = declart_core::parse(&content)
                .with_context(|| format!("invalid declaration: {}", input.display()))?;

            let theme = resolve_theme(&theme_name)?;
            let svg = declart_core::render(&model, theme)
                .with_context(|| "rendering failed")?;

            if stdout {
                print!("{}", svg);
            } else {
                let out_path = output.unwrap_or_else(|| input.with_extension("svg"));
                fs::write(&out_path, &svg)
                    .with_context(|| format!("failed to write {}", out_path.display()))?;
            }
        }
        Commands::Validate { input } => {
            let content = fs::read_to_string(&input)
                .with_context(|| format!("failed to read {}", input.display()))?;
            declart_core::parse(&content)
                .with_context(|| format!("invalid declaration: {}", input.display()))?;
        }
    }

    Ok(())
}

fn resolve_theme(name: &str) -> Result<&'static declart_core::render::Theme> {
    match name {
        "default" => Ok(&DEFAULT_THEME),
        other => anyhow::bail!(
            "unknown theme `{}`\n  = hint: Available themes: default",
            other
        ),
    }
}
