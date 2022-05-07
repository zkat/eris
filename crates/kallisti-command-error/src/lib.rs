use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize)]
pub enum KallistiCommandError {
    #[error("{0}")]
    GenericError(String)
}