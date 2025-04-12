use clap::{Parser, ValueEnum};
use std::str::FromStr;

#[derive(Debug, Parser, ValueEnum, Clone)]
pub(crate) enum OutputType {
    Json,
    String,
}

pub(crate) enum Output {
    Json,
    String { format: String },
}

impl Output {
    pub fn from(output_type: OutputType, format: Option<String>) -> Result<Self, &'static str> {
        let format = match format {
            Some(format) => format,
            None => return Err("No format string provided!"),
        };
        match output_type {
            OutputType::Json => Ok(Output::Json),
            OutputType::String => Ok(Output::String { format }),
        }
    }

    pub fn format(self: &Self) -> String {
        // Probably use serde for json
        todo!()
    }
}
