use clap::Parser;
use std::path::PathBuf;

/// A fast, AST-based Markdown outline CLI that extracts clean hierarchical heading trees with exact line numbers.
#[derive(Debug, Parser)]
#[command(name = "md-outline", version, about, long_about = None)]
pub struct CliArgs {
    /// Target Markdown file to inspect (reads from standard input if omitted or set to '-')
    #[arg(value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Maximum heading depth to display (1-6)
    #[arg(short = 'd', long = "depth", value_name = "DEPTH", default_value_t = 6)]
    pub depth: u8,
}
