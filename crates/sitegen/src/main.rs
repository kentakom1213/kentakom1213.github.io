use anyhow::Context;
use clap::{Args, Parser, Subcommand};
use content::{LoadOptions, load_all};
use std::path::PathBuf;
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
}

#[derive(Args, Clone)]
struct BuildArgs {
    /// Content directory
    #[arg(long, default_value = "content")]
    content_dir: PathBuf,

    /// Disable item sorting
    #[arg(long)]
    no_sort: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Build(args) => {
            build_site(&args)?;
        }
    }
    Ok(())
}

fn build_site(args: &BuildArgs) -> anyhow::Result<()> {
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

    Ok(())
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
