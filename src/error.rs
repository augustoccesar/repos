#[derive(Debug)]
pub enum Error {
    Aborted,
    Clone(String),
    Init(String),
    Format(String),
    NotFound,
    Other(Box<dyn std::error::Error>),
}

impl<T> From<T> for Error
where
    T: std::error::Error + 'static,
{
    fn from(value: T) -> Self {
        Self::Other(Box::new(value))
    }
}

pub type Result<T> = std::result::Result<T, Error>;
