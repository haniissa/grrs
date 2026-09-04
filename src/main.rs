

use clap::Parser;

/// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    ///The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let file = File::open(&args.path).expect("file not found");
    // let file = File::open(&args.path)
    //     .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let content = BufReader::new(file);

    for line in content.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    // println!("pattern: {:?}, path:{:?}", args.pattern, args.path);
    // Ok(())
}
