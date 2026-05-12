use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use declart_core::render::{Theme, ACCESSIBLE_THEME, DEFAULT_THEME, MONOCHROME_THEME, WARM_THEME};
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
        /// Input TOML or JSON file
        input: PathBuf,
        /// Output file [default: input with format extension]
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme to use for rendering [default, monochrome, accessible, warm]
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
        /// Input TOML or JSON file
        input: PathBuf,
    },
    /// Print a starter TOML declaration for a diagram kind
    Init {
        /// Diagram kind: sequence, hierarchy, timeline, matrix, hub_spoke, venn, comparison
        kind: String,
    },
    /// Watch a file and re-render on every change
    Watch {
        /// Input TOML or JSON file
        input: PathBuf,
        /// Output file [default: input with format extension]
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Theme to use for rendering [default, monochrome, accessible, warm]
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
            let bytes = render_bytes(&input, &theme, width, &format)?;
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
            declart_core::parse_auto(&content)
                .with_context(|| format!("invalid declaration: {}", input.display()))?;
        }
        Commands::Init { kind } => {
            let template = init_template(&kind).with_context(|| {
                format!(
                    "unknown kind `{}`\n  = hint: Available kinds: sequence, hierarchy, timeline, matrix, hub_spoke, venn, comparison",
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
            render_bytes(&input, &theme, width, &format)
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
                        match render_bytes(&input, &theme, width, &format).and_then(|bytes| {
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

fn render_bytes(input: &Path, theme: &Theme, width: Option<u32>, format: &str) -> Result<Vec<u8>> {
    let content = fs::read_to_string(input)
        .with_context(|| format!("failed to read {}", input.display()))?;
    let model = declart_core::parse_auto(&content)
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
    let mut opts = usvg::Options::default();
    opts.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg_str, &opts)?;
    let size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height())
        .ok_or_else(|| anyhow::anyhow!("zero-size canvas"))?;
    resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());
    Ok(pixmap.encode_png()?)
}

fn resolve_theme(name: &str) -> Result<Theme> {
    // If the value looks like a file path, load it as a custom theme TOML
    if name.contains('/') || name.contains('\\') || name.ends_with(".toml") {
        let toml_str = fs::read_to_string(name)
            .with_context(|| format!("failed to read theme file `{name}`"))?;
        return Theme::from_toml(&toml_str)
            .with_context(|| format!("invalid theme file `{name}`"));
    }
    match name {
        "default" => Ok(DEFAULT_THEME.clone()),
        "monochrome" => Ok(MONOCHROME_THEME.clone()),
        "accessible" => Ok(ACCESSIBLE_THEME.clone()),
        "warm" => Ok(WARM_THEME.clone()),
        other => anyhow::bail!(
            "unknown theme `{}`\n  = hint: Available themes: default, monochrome, accessible, warm, or a path to a .toml theme file",
            other
        ),
    }
}

fn init_template(kind: &str) -> Option<&'static str> {
    match kind {
        "sequence" => Some(
            r#"kind = "sequence"
title = "My Process"

[[items]]
label = "Step 1"

[[items]]
label = "Step 2"

[[items]]
label = "Step 3"
"#,
        ),
        "hierarchy" => Some(
            r#"kind = "hierarchy"
title = "My Org Chart"

[[nodes]]
label = "CEO"

[[nodes]]
label = "CTO"
parent = "CEO"

[[nodes]]
label = "CFO"
parent = "CEO"
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
        "comparison" => Some(
            r#"kind = "comparison"
title = "My Comparison"

[[rows]]
label = "Option A"

[[rows]]
label = "Option B"

[[columns]]
label = "Criterion 1"

[[columns]]
label = "Criterion 2"

[[cells]]
row = "Option A"
column = "Criterion 1"
value = "★★★"

[[cells]]
row = "Option A"
column = "Criterion 2"
value = "★★"

[[cells]]
row = "Option B"
column = "Criterion 1"
value = "★★"

[[cells]]
row = "Option B"
column = "Criterion 2"
value = "★★★★"
"#,
        ),
        _ => None,
    }
}
