use std::cmp::Ordering;

use crate::common::Orientation2D;

use super::{FieldNumber, Point2D, RealFieldNumber};

/// Defines basic 2D geometric operations using an abstract number and point type.
pub trait Kernel2D {
    type Point: Point2D<Field = Self::Field>;
    type Field: FieldNumber;
}

pub trait ExactPredicates2D: Kernel2D {
    fn compare_distance(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> Ordering;

    fn compare_length(a: &Self::Point, b: &Self::Point) -> Ordering;

    fn orientation(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Orientation2D;
}

pub trait Operations2D: Kernel2D {
    fn length_sqr(a: &Self::Point) -> Self::Field;

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field;

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field;
}

pub trait RealOperations2D: Kernel2D + Operations2D {
    type RealField: RealFieldNumber + From<Self::Field> + From<f32>;
    fn length(a: &Self::Point) -> Self::RealField;

    fn distance(a: &Self::Point, b: &Self::Point) -> Self::RealField;
}

pub trait DefaultKernel {
    type Kernel: Kernel2D<Point = Self>;
}

pub trait DefaultExactKernel {
    type Kernel: Kernel2D<Point = Self>;
}
