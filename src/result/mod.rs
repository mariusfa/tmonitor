use std::error;

pub type ResultWrap<T> = std::result::Result<T, Box<dyn error::Error>>;