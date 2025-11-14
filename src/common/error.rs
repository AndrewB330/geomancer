use std::fmt;

#[derive(Debug)]
pub enum GeometryError {
    InputIsEmpty,
    InputHasTooFewPoints,
    InputValueInvalidForField,
    ExactPredicateReturnedConflictingResult(String),
    DegenerateGeometry,
    NotConvexGeometry,
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
            GeometryError::InputIsEmpty => {
                write!(f, "Input is empty. Expected at least one element.")
            }
            GeometryError::DegenerateGeometry => {
                write!(f, "The geometry is degenerate and cannot be processed.")
            }
            GeometryError::NotConvexGeometry => write!(
                f,
                "The geometry is not convex, which is required for this operation."
            ),
            GeometryError::InputHasTooFewPoints => todo!(),
        }
    }
}

impl std::error::Error for GeometryError {}
