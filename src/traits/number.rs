use num_traits::{Float, One, Zero};
use std::fmt::Debug;

pub trait FieldNumber: 'static + One + Zero + PartialEq + Debug {
    fn is_valid(&self) -> bool;

    fn is_exact() -> bool;
}

pub trait RealFieldNumber: FieldNumber + Float {}

impl<F: 'static + Float + Debug> FieldNumber for F {
    fn is_valid(&self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }

    fn is_exact() -> bool {
        false
    }
}

impl<F: FieldNumber + Float> RealFieldNumber for F {}
