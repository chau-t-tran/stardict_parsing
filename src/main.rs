#![allow(unused)]

use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

mod dictionary;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
