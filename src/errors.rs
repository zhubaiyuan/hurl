use std::fmt;

pub enum Error {
    ParameterMissingSeparator(String),
    MissingUrlAndCommand,
    NotFormButHasFormFile,
    ClientSerialization,
    ClientTimeout,
    ClientWithStatus(reqwest::StatusCode),
    ClientOther,
    SerdeJson(serde_json::error::Category),
    IO(std::io::ErrorKind),
    UrlParseError(reqwest::UrlError),
    SyntaxLoadError(&'static str),
}

pub type HurlResult<T> = Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParameterMissingSeparator(s) => {
                write!(f, "Missing separator when parsing parameter: {}", s)
            }
            Error::MissingUrlAndCommand => write!(f, "Must specify a url or a command!"),
            Error::NotFormButHasFormFile => write!(
                f,
                "Cannot have a form file 'key@filename' unless --form option is set"
            ),
            Error::ClientSerialization => write!(f, "Serializing the request/response failed"),
            Error::ClientTimeout => write!(f, "Timeout during request"),
            Error::ClientWithStatus(status) => write!(f, "Got status code: {}", status),
            Error::ClientOther => write!(f, "Unknown client error"),
            Error::SerdeJson(c) => write!(f, "JSON error: {:?}", c),
            Error::IO(k) => write!(f, "IO Error: {:?}", k),
            Error::UrlParseError(e) => write!(f, "URL Parsing Error: {}", e),
            Error::SyntaxLoadError(typ) => write!(f, "Error loading syntax for {}", typ),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
