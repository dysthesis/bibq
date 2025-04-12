use clap::Parser;

use crate::output::OutputType;
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub(crate) file: String,

    // TODO: Arguments for filtering by attributes
    #[arg(short, long, value_enum, default_value = "json")]
    pub(crate) output_type: OutputType,

    #[arg(short = 'F', long, required_if_eq("output_type", "string"))]
    pub(crate) output_format: Option<String>,
}
