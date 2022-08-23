use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use tokio::io::AsyncReadExt;

use crate::error::error_handler::{ErrorHandler, ErrorType};

pub fn read_file_as_str(path: &str) -> Result<String, ErrorHandler> {
    let mut file = read_file(path)?;
    let mut str = String::new();
    let _ = file.read_to_string(&mut str)
        .map_err(|err| ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::FILE,
        })?;
    Ok(str)
}
pub fn read_file_as_vec(path: &str) -> Result<Vec<String>, ErrorHandler> {
    let mut file = read_file(path)?;
    Ok(BufReader::new(file)
        .lines()
        .filter(|line| line.is_ok())
        .map(|line| line.expect("It's never happen"))
        .collect())
}

fn read_file(path: &str) -> Result<File, ErrorHandler> {
    Ok(File::open(path)
        .map_err(|err| ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::FILE,
        })?)
}