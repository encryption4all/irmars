use thiserror::Error as ThisError;

/// Errors resulting from IrmaClient operations
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Irma session cancelled")]
    SessionCancelled,
    #[error("Irma session timed out")]
    SessionTimedOut,
    #[error("Irma session not finished")]
    SessionNotFinished(super::sessionresult::SessionStatus),
    #[error("Irma proof not valid: {0:?}")]
    ProofNotValid(Option<super::sessionresult::ProofStatus>),
    #[error("Invalid session token: only [A-Za-z0-9_-] characters are allowed")]
    InvalidToken,
}
