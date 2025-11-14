use std::cmp::Ordering;

use num_traits::Float;

use crate::common::Orientation2D;

pub trait Kernel2D {
    type Point;
    type Scalar;
}

pub trait Norm2D: Kernel2D {
    type Real: Float + From<Self::Scalar> + From<f32>;
    fn distance(a: &Self::Point, b: &Self::Point) -> Self::Real;

    fn distance_to_zero(a: &Self::Point) -> Self::Real;
}

pub trait NormSqr2D: Kernel2D {
    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn distance_sqr_to_zero(a: &Self::Point) -> Self::Scalar;
}

pub trait Dot2D: Kernel2D {
    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar;
}

pub trait Cross2D: Kernel2D {
    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Scalar;

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar;
}

pub unsafe trait ExactOrientation2D: Kernel2D {
    fn orientation(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Orientation2D;
}

pub unsafe trait ExactCompareNorm2D: Kernel2D {
    fn compare_distance(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> Ordering;

    fn compare_length(a: &Self::Point, b: &Self::Point) -> Ordering;
}

pub trait DefaultKernel {
    type Kernel: Kernel2D<Point = Self>;
}

pub trait DefaultExactKernel {
    type Kernel: Kernel2D<Point = Self>;
}
