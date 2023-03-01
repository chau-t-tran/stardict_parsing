#![allow(unused)]
use std::env;

use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

extern crate pest;
#[macro_use]
extern crate pest_derive; 

mod dictionary;
mod iterator;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(args.path)?;

    Ok(())
}
