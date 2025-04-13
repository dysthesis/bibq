use clap::{Parser, ValueEnum};

#[derive(Debug, Parser, ValueEnum, Clone)]
pub(crate) enum OutputType {
    Json,
    String,
}

impl Default for OutputType {
    fn default() -> Self {
        Self::Json
    }
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
        match self {
            Output::Json => todo!(),
            Output::String { format } => todo!(),
        }
    }
}
