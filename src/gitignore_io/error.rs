use std::fmt;

#[derive(Debug)]
pub struct Error {
    kind: String,
    message: String
}

impl Error {
    pub fn new(msg: String) -> Error {
        Error {
            kind: String::from("gigen"),
            message: msg
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.kind)
    }
}

impl From<hyper::Error> for Error {
    fn from(hyper_error: hyper::Error) -> Self {
        Error {
            kind: String::from("hyper"),
            message: hyper_error.to_string()
        }
    }
}
