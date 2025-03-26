use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when trying to read from clipboard")]
    ClipboardReadError,
    #[error("Error when trying to write to clipboard")]
    ClipboardWriteError,
}
