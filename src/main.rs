use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use md_outline::args::CliArgs;
use md_outline::formatter::format_tree;
use md_outline::parser::parse_headings;
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::Path;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    let content = match &args.file {
        None => {
            if io::stdin().is_terminal() {
                CliArgs::command().print_help()?;
                println!();
                return Ok(());
            }
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from standard input")?;
            buffer
        }
        Some(path) if path == Path::new("-") => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from standard input")?;
            buffer
        }
        Some(path) => fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?,
    };

    let headings = parse_headings(&content, args.depth);
    let output = format_tree(&headings);

    if !output.is_empty() {
        println!("{output}");
    }

    Ok(())
}
