use anyhow::{Context, Ok, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::Parser;

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
    let file = File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content = BufReader::new(file);

    for line in content.lines() {
        //Unpack the result to ge the actual String
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{:?}", line);
        }
    }
    Ok(())
}
