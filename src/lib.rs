
extern crate rustc_serialize;

pub mod parser;
mod property;
mod value;
mod param;

use std::fmt;
use std::io;
use std::error::Error;

// pub const MULTIVALUE_DELIMITER: char = ',';
pub const VALUE_DELIMITER: char = ':';
pub const PARAM_DELIMITER: char = ';';
pub const PARAM_NAME_DELIMITER: char = '=';

#[derive(Debug)]
pub enum ErrorKind {
    File(io::Error),
    InvalidLineFormat,
    InvalidParamFormat,
    InvalidProperty,
    InvalidVersion,
    InvalidValueType,
    InvalidProtocol,
    NotImplemented,
    UnacceptedType,
}

#[derive(Debug)]
pub struct ParseError {
    kind: ErrorKind,
}

impl ParseError {
    pub fn new(kind: ErrorKind) -> ParseError {
        ParseError { kind: kind }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::File(ref err) => err.fmt(f),
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::File(ref err) => err.description(),
            ErrorKind::InvalidLineFormat => "Invalid line format.",
            ErrorKind::InvalidParamFormat => "Invalid parameter format.",
            ErrorKind::InvalidProperty => "Invalid property.",
            ErrorKind::InvalidVersion => "Invalid version.",
            ErrorKind::InvalidValueType => "Invalid value type.",
            ErrorKind::InvalidProtocol => "Invalid protocol.",
            ErrorKind::NotImplemented => "Element not implemented.",
            ErrorKind::UnacceptedType => "Unaccepted type.",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self.kind {
            ErrorKind::File(ref err) => Some(err),
            _ => None,
        }
    }
}
