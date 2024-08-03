use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct FrPmError {
    pub message: String
}

impl Display for FrPmError {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for FrPmError {}