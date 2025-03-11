use std::{cmp::Ordering, marker::PhantomData};

use num_traits::Float;

use crate::common::Orientation2D;

use super::{FieldNumber, Point2D, RealNumber};

/// Defines basic 2D geometric operations using an abstract number and point type.
pub trait Kernel2D {
    type Field: FieldNumber;
    type Real: RealNumber + From<Self::Field>;
    type Point: Point2D<Field = Self::Field> + Clone;

    fn point(x: Self::Field, y: Self::Field) -> Self::Point;

    // ===== Inexact operations that return Self::Real instead of Self::Field ======

    /// Computes the distance between two points.
    fn distance(a: &Self::Point, b: &Self::Point) -> Self::Real {
        let dx = a.x() - b.x();
        let dy = a.y() - b.y();
        Self::Real::sqrt((dx.clone() * dx + dy.clone() * dy).into())
    }

    /// Computes the signed area of the triangle defined by three points.
    /// A positive value indicates counterclockwise order.
    fn signed_area(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Self::Real {
        let (adx, ady) = (a.x() - c.x(), a.y() - c.y());
        let (bdx, bdy) = (b.x() - c.x(), b.y() - c.y());
        Self::Real::from(adx * bdy - ady * bdx)
    }

    // ===== Exact operations that return Self::Field =====

    /// Computes which point "a" or "b" is closer to the given point "to".
    /// Ordering::Less - point a closer
    /// Ordering::Greater - point b closer
    /// Ordering::Equal - points are at the same distance
    fn is_closer(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> Ordering {
        Self::distance(a, to)
            .partial_cmp(&Self::distance(b, to))
            .unwrap()
    }

    fn length_sqr(p: &Self::Point) -> Self::Field {
        p.x() * p.x() + p.y() * p.y()
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let dx = a.x() - b.x();
        let dy = a.y() - b.y();
        dx.clone() * dx + dy.clone() * dy
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.x() * b.x() + a.y() * b.y()
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.x() * b.y() - a.y() * b.x()
    }

    fn cross_from_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.x() - origin.x();
        let ady = a.y() - origin.y();
        let bdx = b.x() - origin.x();
        let bdy = b.y() - origin.y();
        adx * bdy - ady * bdx
    }

    fn orientation(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> Orientation2D;
}

pub trait DefaultKernel {
    type Kernel;
}
