use std::fmt::Debug;

use num_traits::{Float, Num};

pub trait Number: Sized + Clone + Debug + Num {}

pub trait FloatNumber: Number + Float {}

pub trait ExactNumber: Number {}
