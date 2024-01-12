use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Failed to parse server address")]
    AddressParsing,
}
