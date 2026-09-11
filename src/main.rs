use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

//my own library
mod lib;
// use crate::lib;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    ///The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {    
    let args = Cli::parse();
    if args.pattern.is_empty(){
        return Err(anyhow::anyhow!("pattern can not be empty"));
    }
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content = BufReader::new(file) ;
    grrs::find_matches(content, &args.pattern, &mut std::io::stdout());
    Ok(())
}
