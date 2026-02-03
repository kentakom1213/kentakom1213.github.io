use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use content::{LoadOptions, load_all};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use ui::render_index_string;

#[derive(Parser)]
#[command(name = "sitegen")]
#[command(about = "Static site generator for this repository", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Build(BuildArgs),
    Serve(ServeArgs),
}

#[derive(Args, Clone)]
struct BuildArgs {
    /// Content directory
    content_dir: PathBuf,

    /// Disable item sorting
    #[arg(long)]
    no_sort: bool,
}

#[derive(Args, Clone)]
struct ServeArgs {
    /// Content directory
    content_dir: PathBuf,

    /// Disable item sorting
    #[arg(long)]
    no_sort: bool,

    /// Server port
    #[arg(long, default_value_t = 8080)]
    port: u16,

    /// Open browser automatically
    #[arg(long)]
    open: bool,
}

struct BuildOutput {
    output_dir: PathBuf,
    output_file: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build(args) => {
            build_site(&args)?;
        }
        Commands::Serve(args) => {
            serve_site(args)?;
        }
    }

    Ok(())
}

fn build_site(args: &BuildArgs) -> anyhow::Result<BuildOutput> {
    let data = load_all(LoadOptions {
        content_dir: args.content_dir.clone(),
        sort_items: !args.no_sort,
    })?;

    let (output_dir, output_file) = resolve_build_output(&data);
    let output_path = output_dir.join(&output_file);

    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create output dir: {}", parent.display()))?;
    }

    let html = render_index_string(&data);
    std::fs::write(&output_path, html)
        .with_context(|| format!("failed to write {}", output_path.display()))?;

    Ok(BuildOutput {
        output_dir,
        output_file,
    })
}

fn serve_site(args: ServeArgs) -> anyhow::Result<()> {
    let initial_build = build_site(&BuildArgs {
        content_dir: args.content_dir.clone(),
        no_sort: args.no_sort,
    })?;

    let mut miniserve = Command::new("miniserve");
    miniserve.arg(&initial_build.output_dir);
    miniserve.arg("--port").arg(args.port.to_string());
    miniserve.arg("--index").arg(&initial_build.output_file);
    if args.open {
        miniserve.arg("--open");
    }

    let mut child = miniserve
        .spawn()
        .context("failed to start miniserve; ensure it is installed and in PATH")?;

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(&args.content_dir, RecursiveMode::Recursive)?;

    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return Err(anyhow::anyhow!("miniserve exited: {status}"));
        }

        let event = match rx.recv() {
            Ok(Ok(event)) => event,
            Ok(Err(err)) => {
                eprintln!("watch error: {err}");
                continue;
            }
            Err(err) => return Err(anyhow::anyhow!("watch error: {err}")),
        };

        if !should_rebuild(&event) {
            continue;
        }

        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(200) {
            match rx.try_recv() {
                Ok(Ok(next)) => {
                    if should_rebuild(&next) {
                        continue;
                    }
                }
                Ok(Err(err)) => {
                    eprintln!("watch error: {err}");
                }
                Err(mpsc::TryRecvError::Empty) => {
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(mpsc::TryRecvError::Disconnected) => break,
            }
        }

        match build_site(&BuildArgs {
            content_dir: args.content_dir.clone(),
            no_sort: args.no_sort,
        }) {
            Ok(next_build) => {
                if next_build.output_dir != initial_build.output_dir {
                    eprintln!(
                        "warning: output_dir changed to {}, but server is still serving {}",
                        next_build.output_dir.display(),
                        initial_build.output_dir.display()
                    );
                }
            }
            Err(err) => {
                eprintln!("build failed: {err}");
            }
        }
    }
}

fn resolve_build_output(data: &content::model::IndexData) -> (PathBuf, String) {
    let mut output_dir = PathBuf::from("docs");
    let mut output_file = "index.html".to_string();

    if let Some(build) = &data.config.build {
        if let Some(dir) = &build.output_dir
            && !dir.trim().is_empty()
        {
            output_dir = PathBuf::from(dir);
        }
        if let Some(file) = &build.output_file
            && !file.trim().is_empty()
        {
            output_file = file.clone();
        }
    }

    (output_dir, output_file)
}

fn should_rebuild(event: &Event) -> bool {
    event
        .paths
        .iter()
        .any(|path| path.extension().and_then(|s| s.to_str()) == Some("toml"))
}
