#![allow(unused)]
use std::env;

use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod dictionary;
mod iterator;

#[derive(Debug, Parser)]
struct Cli {
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let file = File::open(args.path)?;

    Ok(())
}
