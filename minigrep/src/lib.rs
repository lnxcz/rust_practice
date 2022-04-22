use std::io::Error;
use std::io::ErrorKind;

pub struct Input {
    pub filename: String,
    pub query: String,
}

impl Input {
    pub fn new(args: Vec<String>) -> Result<Input, Error> {
        if args.len() < 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "Not enough arguments!"));
        } else {
            Ok(Input {
                filename: args[1].clone(),
                query: args[2].clone(),
            })
        }
    }
}

pub struct SearchResult {
    pub line_number: usize,
    pub line: String,
}

impl SearchResult {
    pub fn new(line_number: usize, line: String) -> SearchResult {
        SearchResult { line_number, line }
    }
}
