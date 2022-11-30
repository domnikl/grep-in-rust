use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;
use clap::Parser;
use io::Result;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let lines = match read_lines(&args.path) {
        Ok(lines) => { lines },
        Err(error) => { return Err(error.into()) }
    };

    for line in lines {
        let l = match line {
            Ok(line) => { line },
            Err(error) => { return Err(error.into()); }
        };

        if l.contains(&args.pattern) {
            println!("{}", l)
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
