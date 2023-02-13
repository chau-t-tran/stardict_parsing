#![allow(unused)]

use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let contents = fs::read_to_string(args.path)
        .expect("Error reading file");
    println!("{}", contents);
}
