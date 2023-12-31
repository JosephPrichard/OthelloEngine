/*
 * Copyright (c) Joseph Prichard 2022.
 */

use std::error::Error;
use std::fmt;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    details: String,
}

impl ParseError {
    pub(crate) fn new(msg: &str) -> ParseError {
        ParseError {
            details: msg.to_string()
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}