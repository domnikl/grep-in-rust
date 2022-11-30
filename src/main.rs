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

fn main() {
    let args = Cli::parse();
    let lines = read_lines(&args.path).expect("Could not read file");

    for line in lines {
        let l = line.expect("Could not read line");

        if l.contains(&args.pattern) {
            println!("{}", l)
        }
    }
}

fn read_lines<P>(filename: P) -> Result<io::Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
