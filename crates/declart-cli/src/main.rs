use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use declart_core::render::{Theme, DEFAULT_THEME, MONOCHROME_THEME};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "declart", version, about = "Declare what to show. The engine decides how it looks.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a diagram declaration to SVG or PNG
    Render {
        /// Input TOML file
        input: PathBuf,
        /// Output file [default: input with format extension]
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme to use for rendering [default, monochrome]
        #[arg(long, default_value = "default")]
        theme: String,
        /// Override canvas width in pixels (height scales proportionally)
        #[arg(long)]
        width: Option<u32>,
        /// Output format [svg, png]
        #[arg(long, default_value = "svg")]
        format: String,
        /// Write output to stdout instead of file
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
    /// Watch a file and re-render on every change
    Watch {
        /// Input TOML file
        input: PathBuf,
        /// Output file [default: input with format extension]
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme to use for rendering [default, monochrome]
        #[arg(long, default_value = "default")]
        theme: String,
        /// Override canvas width in pixels (height scales proportionally)
        #[arg(long)]
        width: Option<u32>,
        /// Output format [svg, png]
        #[arg(long, default_value = "svg")]
        format: String,
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
        Commands::Render { input, output, theme: theme_name, width, format, stdout } => {
            let theme = resolve_theme(&theme_name)?;
            let bytes = render_bytes(&input, theme, width, &format)?;
            if stdout {
                std::io::stdout().write_all(&bytes)?;
            } else {
                let ext = if format == "png" { "png" } else { "svg" };
                let out_path = output.unwrap_or_else(|| input.with_extension(ext));
                fs::write(&out_path, &bytes)
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
        Commands::Watch { input, output, theme: theme_name, width, format } => {
            let theme = resolve_theme(&theme_name)?;
            let ext = if format == "png" { "png" } else { "svg" };
            let out_path = output.unwrap_or_else(|| input.with_extension(ext));

            // Initial render
            render_bytes(&input, theme, width, &format)
                .and_then(|bytes| {
                    fs::write(&out_path, &bytes)
                        .with_context(|| format!("failed to write {}", out_path.display()))
                })?;

            eprintln!("[declart] watching {} ...", input.display());
            eprintln!("[declart] output  → {}", out_path.display());
            eprintln!("[declart] (press Ctrl+C to stop)");

            use notify::{EventKind, RecursiveMode, Watcher};
            use std::sync::mpsc;

            let (tx, rx) = mpsc::channel();
            let mut watcher = notify::recommended_watcher(move |res| {
                let _ = tx.send(res);
            })?;
            watcher.watch(&input, RecursiveMode::NonRecursive)?;

            loop {
                match rx.recv() {
                    Ok(Ok(event)) => {
                        if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                            continue;
                        }
                        // Drain rapid follow-up events (debounce)
                        std::thread::sleep(std::time::Duration::from_millis(50));
                        while rx.try_recv().is_ok() {}

                        eprint!("[declart] rebuilding ...");
                        match render_bytes(&input, theme, width, &format).and_then(|bytes| {
                            fs::write(&out_path, &bytes).with_context(|| {
                                format!("failed to write {}", out_path.display())
                            })
                        }) {
                            Ok(()) => eprintln!(" ok"),
                            Err(e) => eprintln!("\n[declart] error: {e:#}"),
                        }
                    }
                    Ok(Err(e)) => eprintln!("[declart] watcher error: {e}"),
                    Err(_) => break,
                }
            }
        }
    }

    Ok(())
}

fn render_bytes(input: &Path, theme: &'static Theme, width: Option<u32>, format: &str) -> Result<Vec<u8>> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("failed to read {}", input.display()))?;
    let model = declart_core::parse(&content)
        .with_context(|| format!("invalid declaration: {}", input.display()))?;
    let svg = declart_core::render_opts(&model, theme, width)?;
    match format {
        "svg" => Ok(svg.into_bytes()),
        "png" => svg_to_png(&svg),
        other => anyhow::bail!("unknown format `{}`\n  = hint: Available formats: svg, png", other),
    }
}

fn svg_to_png(svg_str: &str) -> Result<Vec<u8>> {
    use resvg::{tiny_skia, usvg};
    let opts = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &opts)?;
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| anyhow::anyhow!("zero-size canvas"))?;
    resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());
    Ok(pixmap.encode_png()?)
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
