use crate::traits::{
    DefaultKernel, ExactPredicates2D, Kernel2D, Operations2D, Point2D, RealOperations2D,
};

use super::Orientation2D;

pub(crate) fn assert_eq_cycle(a: Vec<usize>, b: Vec<usize>) {
    assert!(!a.is_empty());
    assert!(!b.is_empty());
    let mut offset = None;

    for i in 0..a.len() {
        if a[i] == b[0] {
            offset = Some(i);
            break;
        }
    }
    let offset = offset.unwrap();

    let a: Vec<usize> = a[offset..]
        .iter()
        .chain(a[..offset].iter())
        .cloned()
        .collect();

    assert_eq!(a, b);
}

pub struct F32TupleKernel;

impl Kernel2D for F32TupleKernel {
    type Point = (f32, f32);

    type Field = f32;
}

impl Operations2D for F32TupleKernel {
    fn length_sqr(a: &Self::Point) -> Self::Field {
        a.0 * a.0 + a.1 * a.1
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        dx * dx + dy * dy
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.0 * b.0 + a.1 * b.1
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.0 - origin.0;
        let ady = a.1 - origin.1;
        let bdx = b.0 - origin.0;
        let bdy = b.1 - origin.1;
        adx * bdx + ady * bdy
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.0 * b.1 - a.1 * b.0
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.0 - origin.0;
        let ady = a.1 - origin.1;
        let bdx = b.0 - origin.0;
        let bdy = b.1 - origin.1;
        adx * bdy - ady * bdx
    }
}

impl RealOperations2D for F32TupleKernel {
    type RealField = f32;

    fn length(a: &Self::Point) -> Self::RealField {
        Self::length_sqr(a).sqrt()
    }

    fn distance(a: &Self::Point, b: &Self::Point) -> Self::RealField {
        Self::distance_sqr(a, b).sqrt()
    }
}

// Bad implementation, using just cast to f64, not precise, just for now for tests.
unsafe impl ExactPredicates2D for F32TupleKernel {
    fn is_same_point(a: &Self::Point, b: &Self::Point) -> bool {
        a.x() == b.x() && a.y() == b.y()
    }

    fn compare_distance(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> std::cmp::Ordering {
        let adx = a.x() as f64 - to.x() as f64;
        let ady = a.y() as f64 - to.y() as f64;
        let bdx = b.x() as f64 - to.x() as f64;
        let bdy = b.y() as f64 - to.y() as f64;
        (adx * adx + ady * ady)
            .partial_cmp(&(bdx * bdx + bdy * bdy))
            .unwrap()
    }

    fn compare_length(a: &Self::Point, b: &Self::Point) -> std::cmp::Ordering {
        Self::compare_distance(a, b, &(0.0, 0.0))
    }

    fn orientation(
        a: &Self::Point,
        b: &Self::Point,
        c: &Self::Point,
    ) -> crate::prelude::Orientation2D {
        let adx = a.x() as f64 - c.x() as f64;
        let ady = a.y() as f64 - c.y() as f64;
        let bdx = b.x() as f64 - c.x() as f64;
        let bdy = b.y() as f64 - c.y() as f64;
        let cross = adx * bdy - ady * bdx;
        match cross.partial_cmp(&0.0).unwrap() {
            std::cmp::Ordering::Less => Orientation2D::Clockwise,
            std::cmp::Ordering::Equal => Orientation2D::Collinear,
            std::cmp::Ordering::Greater => Orientation2D::CounterClockwise,
        }
    }
}

pub struct F64TupleKernel;

impl Kernel2D for F64TupleKernel {
    type Point = (f64, f64);

    type Field = f64;
}

impl Operations2D for F64TupleKernel {
    fn length_sqr(a: &Self::Point) -> Self::Field {
        a.0 * a.0 + a.1 * a.1
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let dx = a.0 - b.0;
        let dy = a.1 - b.1;
        dx * dx + dy * dy
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.0 * b.0 + a.1 * b.1
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.0 - origin.0;
        let ady = a.1 - origin.1;
        let bdx = b.0 - origin.0;
        let bdy = b.1 - origin.1;
        adx * bdx + ady * bdy
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.0 * b.1 - a.1 * b.0
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.0 - origin.0;
        let ady = a.1 - origin.1;
        let bdx = b.0 - origin.0;
        let bdy = b.1 - origin.1;
        adx * bdy - ady * bdx
    }
}

// This implementation of ExactPredicates IS NOT exact, it is here for tests only
unsafe impl ExactPredicates2D for F64TupleKernel {
    fn is_same_point(a: &Self::Point, b: &Self::Point) -> bool {
        a == b
    }

    fn compare_distance(a: &Self::Point, b: &Self::Point, to: &Self::Point) -> std::cmp::Ordering {
        Self::distance_sqr(a, to)
            .partial_cmp(&Self::distance_sqr(b, to))
            .unwrap()
    }

    fn compare_length(a: &Self::Point, b: &Self::Point) -> std::cmp::Ordering {
        Self::length_sqr(a)
            .partial_cmp(&Self::length_sqr(b))
            .unwrap()
    }

    fn orientation(
        a: &Self::Point,
        b: &Self::Point,
        c: &Self::Point,
    ) -> crate::prelude::Orientation2D {
        match Self::cross_with_origin(a, b, c).partial_cmp(&0.0).unwrap() {
            std::cmp::Ordering::Less => Orientation2D::Clockwise,
            std::cmp::Ordering::Equal => Orientation2D::Collinear,
            std::cmp::Ordering::Greater => Orientation2D::CounterClockwise,
        }
    }
}
