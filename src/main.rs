mod cli;
mod output;

use crate::cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("file: {:?}", args.file);
    println!("output: {:?}", args.output_type);
    println!("format: {:?}", args.output_format);
}
