use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Markdown processing error: {0}")]
    MarkdownError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Other error: {0}")]
    OtherError(String),
}

impl From<markdown::message::Message> for Error {
    fn from(message: markdown::message::Message) -> Self {
        Error::MarkdownError(message.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::OtherError(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::OtherError(err)
    }
}