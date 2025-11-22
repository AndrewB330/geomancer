use std::cmp::Ordering;

use num_traits::{Float, Zero};

use crate::{algorithm::{AlgorithmBundle, SelfIntersectionsAlgo}, common::{GeometryError, Orientation2D}};

pub trait Kernel2D: Sized {
    type Point;
    type Scalar: PartialOrd + Zero;
    type Algorithms: AlgorithmBundle<Self>;
}

pub trait Norm2D: Kernel2D {
    type Real: Float + From<Self::Scalar> + From<f32>;
    fn distance(&self, a: &Self::Point, b: &Self::Point) -> Self::Real;

    fn distance_to_zero(&self, a: &Self::Point) -> Self::Real;
}

pub trait NormSqr2D: Kernel2D {
    fn distance_sqr(&self, a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn distance_sqr_to_zero(&self, a: &Self::Point) -> Self::Scalar;
}

pub trait Dot2D: Kernel2D {
    fn dot(&self, a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn dot_with_origin(
        &self,
        a: &Self::Point,
        b: &Self::Point,
        origin: &Self::Point,
    ) -> Self::Scalar;
}

pub trait Cross2D: Kernel2D {
    fn cross(&self, a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn cross_with_origin(
        &self,
        a: &Self::Point,
        b: &Self::Point,
        origin: &Self::Point,
    ) -> Self::Scalar;
}

pub unsafe trait ExactOrientation2D: Kernel2D {
    fn orientation(&self, a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Orientation2D;
}

pub unsafe trait ExactCompareNorm2D: Kernel2D {
    fn compare_distance(&self, a: &Self::Point, b: &Self::Point, to: &Self::Point) -> Ordering;
    fn compare_length(&self, a: &Self::Point, b: &Self::Point) -> Ordering;
}

pub trait DefaultKernel {
    type Kernel: Kernel2D<Point = Self>;
}

pub trait DefaultExactKernel {
    type Kernel: Kernel2D<Point = Self>;
}

/// A trait for 2D points with x and y coordinates
pub trait Point2D: Clone {
    type Scalar: PartialOrd + Zero;
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
}
