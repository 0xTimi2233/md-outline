use clap::Parser;
use std::path::PathBuf;

/// A fast, AST-based Markdown outline CLI that extracts clean hierarchical heading trees with exact line numbers.
#[derive(Debug, Parser)]
#[command(name = "md-outline", version, about, long_about = None)]
pub struct CliArgs {
    /// Target Markdown file to inspect (use '-' for stdin)
    #[arg(value_name = "FILE")]
    pub file: PathBuf,

    /// Maximum heading depth to display (1-6)
    #[arg(short = 'd', long = "depth", value_name = "DEPTH", default_value_t = 6)]
    pub depth: u8,
}
