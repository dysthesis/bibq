mod cli;
mod filter;
mod output;

use std::fs::read_to_string;

use crate::cli::Args;
use biblatex::Bibliography;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let src = read_to_string(args.file).unwrap();
    let biblio = Bibliography::parse(src.as_str()).unwrap();
    biblio.iter().for_each(|entry| println!("{:?}\n", entry));
}
