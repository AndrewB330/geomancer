use std::fmt;

#[derive(Debug)]
pub enum GeometryError {
    InputValueInvalidForField,
    ExactPredicateReturnedConflictingResult(String),
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::InputValueInvalidForField => write!(
                f,
                "Input contains an invalid value (Most likely NaN or infinite value)"
            ),
            GeometryError::ExactPredicateReturnedConflictingResult(details) => write!(
                f,
                "Exact predicate evaluation returned conflicting results. Details: {}",
                details
            ),
        }
    }
}

impl std::error::Error for GeometryError {}
