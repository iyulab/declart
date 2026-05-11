use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use declart_core::render::{DEFAULT_THEME, MONOCHROME_THEME};
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
        /// Theme to use for rendering [default, monochrome]
        #[arg(long, default_value = "default")]
        theme: String,
        /// Override canvas width in pixels (height scales proportionally)
        #[arg(long)]
        width: Option<u32>,
        /// Write SVG to stdout instead of file
        #[arg(long)]
        stdout: bool,
    },
    /// Validate a diagram declaration without rendering
    Validate {
        /// Input TOML file
        input: PathBuf,
    },
    /// Print a starter TOML declaration for a diagram kind
    Init {
        /// Diagram kind: pyramid, process, cycle, matrix, hub_spoke, venn, timeline, fishbone
        kind: String,
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
        Commands::Render { input, output, theme: theme_name, width, stdout } => {
            let content = fs::read_to_string(&input)
                .with_context(|| format!("failed to read {}", input.display()))?;

            let model = declart_core::parse(&content)
                .with_context(|| format!("invalid declaration: {}", input.display()))?;

            let theme = resolve_theme(&theme_name)?;
            let svg = declart_core::render_opts(&model, theme, width)
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
        Commands::Init { kind } => {
            let template = init_template(&kind).with_context(|| {
                format!(
                    "unknown kind `{}`\n  = hint: Available kinds: pyramid, process, cycle, matrix, hub_spoke, venn, timeline, fishbone",
                    kind
                )
            })?;
            print!("{}", template);
        }
    }

    Ok(())
}

fn resolve_theme(name: &str) -> Result<&'static declart_core::render::Theme> {
    match name {
        "default" => Ok(&DEFAULT_THEME),
        "monochrome" => Ok(&MONOCHROME_THEME),
        other => anyhow::bail!(
            "unknown theme `{}`\n  = hint: Available themes: default, monochrome",
            other
        ),
    }
}

fn init_template(kind: &str) -> Option<&'static str> {
    match kind {
        "pyramid" => Some(
            r#"kind = "pyramid"
title = "My Pyramid"

[[items]]
label = "Top"

[[items]]
label = "Middle"

[[items]]
label = "Bottom"
"#,
        ),
        "process" => Some(
            r#"kind = "process"
title = "My Process"

[[items]]
label = "Step 1"

[[items]]
label = "Step 2"

[[items]]
label = "Step 3"
"#,
        ),
        "cycle" => Some(
            r#"kind = "cycle"
title = "My Cycle"

[[items]]
label = "Plan"

[[items]]
label = "Do"

[[items]]
label = "Check"

[[items]]
label = "Act"
"#,
        ),
        "matrix" => Some(
            r#"kind = "matrix"
title = "My Matrix"
x_axis = "Importance"
y_axis = "Urgency"

[[quadrants]]
label = "Do First"

[[quadrants]]
label = "Schedule"

[[quadrants]]
label = "Delegate"

[[quadrants]]
label = "Eliminate"
"#,
        ),
        "hub_spoke" => Some(
            r#"kind = "hub_spoke"
title = "My Hub"

center = "Central Concept"

[[spokes]]
label = "Topic A"

[[spokes]]
label = "Topic B"

[[spokes]]
label = "Topic C"
"#,
        ),
        "venn" => Some(
            r#"kind = "venn"
title = "My Venn"

[[sets]]
label = "Group A"

[[sets]]
label = "Group B"

[[intersections]]
sets = ["Group A", "Group B"]
label = "Both"
"#,
        ),
        "timeline" => Some(
            r#"kind = "timeline"
title = "My Timeline"

[[events]]
date = "2024-01-01"
label = "Start"

[[events]]
date = "2024-06-01"
label = "Milestone"

[[events]]
date = "2024-12-31"
label = "End"
"#,
        ),
        "fishbone" => Some(
            r#"kind = "fishbone"
title = "My Fishbone"

effect = "Effect"

[[causes]]
label = "Cause A"

[[causes]]
label = "Cause B"

[[causes]]
label = "Cause C"
"#,
        ),
        _ => None,
    }
}
