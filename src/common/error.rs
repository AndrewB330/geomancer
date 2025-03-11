use std::fmt;

#[derive(Debug)]
pub enum GeometryError {
    InputValueInvalidForField,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InputValueInvalidForField => write!(f, "Input contains an invalid value (Most likely NaN or infinite value)"),
        }
    }
}

impl std::error::Error for GeometryError {}
