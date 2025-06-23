use std::{error::Error, fmt::Display , fmt::Formatter};

use crate::err;

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Error),
}

impl Display for ParseErr {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

impl Error for ParseErr {
}

#[derive(Debug)]
pub struct ReadErr {
    // expected public fields
}

impl Display for ReadErr {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}

impl Error for ReadErr {
}