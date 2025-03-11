use std::fmt::Debug;

use num_traits::{Float, Num};

pub trait Number: Sized + Clone + Debug + Num {}

pub trait RealNumber: Number + Float {}

pub trait FieldNumber: Number {
    fn is_valid(&self) -> bool;

    fn is_exact() -> bool;
}

impl Number for f32 {}
impl Number for f64 {}

impl RealNumber for f32 {}
impl RealNumber for f64 {}

impl<F: RealNumber> FieldNumber for F {
    fn is_valid(&self) -> bool {
        !self.is_nan() && !self.is_infinite()
    }
    
    fn is_exact() -> bool {
        false
    }
}
