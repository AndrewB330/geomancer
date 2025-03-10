use crate::math::Orientation2D;

use super::{Point2D, Number};

/// Defines basic 2D geometric operations using an abstract number and point type.
pub trait Kernel2D {
    type Field: Number;
    type Point: Point2D<Field=Self::Field>;

    fn point(x: Self::Field, y: Self::Field) -> Self::Point;

    fn length_sqr(p: &Self::Point) -> Self::Field;

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field;

    fn cross_from_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field;

    fn orientation(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Orientation2D;
}
