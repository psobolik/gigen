use std::fmt;

#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(msg: &String) -> Error {
        Error { message: msg.to_string()}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<hyper::Error> for Error {
    fn from(hyper_error: hyper::Error) -> Self {
        Error::new(&hyper_error.to_string())
    }
}
