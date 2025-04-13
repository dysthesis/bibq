mod cli;
mod filter;
mod output;
mod template;

use std::fs::read_to_string;

use crate::{cli::Args, template::Template};
use biblatex::{Bibliography, Entry};
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Entries = Vec<Entry>;

fn main() {
    let args = Args::parse();
    let src = read_to_string(args.file).unwrap();
    let biblio = Bibliography::parse(src.as_str()).unwrap();
    let entries: Entries = match args.citekey {
        Some(key) => {
            let key: String = key.split_ascii_whitespace().take(1).collect();
            match biblio.get(key.as_str()) {
                Some(res) => vec![res.to_owned()],
                None => {
                    return eprintln!("Failed to find entry with that citekey");
                }
            }
        }
        None => biblio.into_vec(),
    };
    let formatted: Vec<String> = match args.output_type {
        output::OutputType::Json => todo!(),
        output::OutputType::String => {
            let template = match args.output_format {
                Some(format) => Template::from(format),
                None => unreachable!(),
            };
            entries
                .par_iter()
                .map(|entry| template.render(entry))
                .collect()
        }
    };
    formatted.par_iter().for_each(|res| println!("{res}"));
}
