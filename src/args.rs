use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Output formats supported by md-outline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum OutputFormat {
    /// Token-saving 2-space indentation (default)
    #[default]
    Indent,
    /// Classic Unicode tree hierarchy
    Tree,
    /// JSON array representation
    Json,
}

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

    /// Output formatting mode
    #[arg(short = 'f', long = "format", value_enum, default_value_t = OutputFormat::Indent)]
    pub format: OutputFormat,
}
