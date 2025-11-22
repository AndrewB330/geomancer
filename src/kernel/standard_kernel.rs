use crate::algorithm::DefaultAlgorithmBundle;

use super::{Kernel2D, Norm2D};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StandardKernel2D;

impl Kernel2D for StandardKernel2D {
    type Point = Vec2D;
    type Scalar = f64;
    type Algorithms = DefaultAlgorithmBundle;
}

impl Norm2D for StandardKernel2D {
    type Real = f64;

    fn distance(&self, a: &Self::Point, b: &Self::Point) -> Self::Real {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }

    fn distance_to_zero(&self, a: &Self::Point) -> Self::Real {
        (a.x.powi(2) + a.y.powi(2)).sqrt()
    }
}
