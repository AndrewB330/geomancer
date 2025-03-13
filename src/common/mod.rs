mod error;
mod orientation;

pub use error::GeometryError;
pub use orientation::Orientation2D;

#[cfg(test)]
mod test_utils;

#[cfg(test)]
pub(crate) use test_utils::*;
