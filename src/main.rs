use anyhow::{Context, Result};
use clap::Parser;
use md_outline::args::CliArgs;
use md_outline::formatter::format_tree;
use md_outline::parser::parse_headings;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let content = if args.file == Path::new("-") {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from standard input")?;
        buffer
    } else {
        fs::read_to_string(&args.file)
            .with_context(|| format!("Failed to read file: {}", args.file.display()))?
    };

    let headings = parse_headings(&content, args.depth);
    let output = format_tree(&headings);

    if !output.is_empty() {
        println!("{output}");
    }

    Ok(())
}
