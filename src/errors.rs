use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    #[error("[EpubError]: {0}")]
    EpubError(String),
    #[error("[HTTPError]: {0}")]
    HTTPError(String),
    #[error("[IOError]: {0}")]
    IOError(String),
    #[error("[UTF8Error]: {0}")]
    UTF8Error(String),
    #[error("[ReadabilityError]: {0}")]
    ReadabilityError(String),
}

#[derive(Error, Debug)]
#[error("{kind}")]
/// Used to represent errors from downloading images. Errors from here are used solely for debugging
/// as they are considered recoverable.
pub struct ImgError {
    kind: ErrorKind,
    url: Option<String>,
}

impl ImgError {
    pub fn with_kind(kind: ErrorKind) -> Self {
        ImgError { url: None, kind }
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = Some(url.to_string());
    }

    pub fn url(&self) -> &Option<String> {
        &self.url
    }
}

impl From<ErrorKind> for ImgError {
    fn from(kind: ErrorKind) -> Self {
        ImgError::with_kind(kind)
    }
}

impl From<surf::Error> for ImgError {
    fn from(err: surf::Error) -> Self {
        ImgError::with_kind(ErrorKind::HTTPError(err.to_string()))
    }
}

impl From<url::ParseError> for ImgError {
    fn from(err: url::ParseError) -> Self {
        ImgError::with_kind(ErrorKind::HTTPError(err.to_string()))
    }
}

impl From<std::io::Error> for ImgError {
    fn from(err: std::io::Error) -> Self {
        ImgError::with_kind(ErrorKind::IOError(err.to_string()))
    }
}

#[derive(Error, Debug)]
#[error("{kind}")]
pub struct PaperoniError {
    article_source: Option<String>,
    kind: ErrorKind,
}

impl PaperoniError {
    pub fn with_kind(kind: ErrorKind) -> Self {
        PaperoniError {
            article_source: None,
            kind,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn article_source(&self) -> &Option<String> {
        &self.article_source
    }

    pub fn set_article_source(&mut self, article_source: &str) {
        self.article_source = Some(article_source.to_owned());
    }
}

impl From<ErrorKind> for PaperoniError {
    fn from(kind: ErrorKind) -> Self {
        PaperoniError::with_kind(kind)
    }
}

impl From<epub_builder::Error> for PaperoniError {
    fn from(err: epub_builder::Error) -> Self {
        PaperoniError::with_kind(ErrorKind::EpubError(err.description().to_owned()))
    }
}

impl From<surf::Error> for PaperoniError {
    fn from(err: surf::Error) -> Self {
        PaperoniError::with_kind(ErrorKind::HTTPError(err.to_string()))
    }
}

impl From<url::ParseError> for PaperoniError {
    fn from(err: url::ParseError) -> Self {
        PaperoniError::with_kind(ErrorKind::HTTPError(err.to_string()))
    }
}

impl From<std::io::Error> for PaperoniError {
    fn from(err: std::io::Error) -> Self {
        PaperoniError::with_kind(ErrorKind::IOError(err.to_string()))
    }
}

impl From<std::str::Utf8Error> for PaperoniError {
    fn from(err: std::str::Utf8Error) -> Self {
        PaperoniError::with_kind(ErrorKind::UTF8Error(err.to_string()))
    }
}
