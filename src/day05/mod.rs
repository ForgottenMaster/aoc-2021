mod line;
mod parse_line_error;
mod parse_point_error;
mod point;

use std::{error::Error, fmt::Display};

pub fn run() -> Result<(Box<dyn Display>, Box<dyn Display>), Box<dyn Error>> {
    Ok((Box::new(0), Box::new(0)))
}
