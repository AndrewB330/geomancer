use std::marker::PhantomData;

use crate::traits::{
    FieldNumber, Kernel2D, Operations2D, Point2D, RealFieldNumber, RealOperations2D,
};

pub struct GenericKernel2D<V>(PhantomData<V>);

impl<V> Kernel2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Field: FieldNumber,
{
    type Point = V;

    type Field = V::Field;
}

impl<V> Operations2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Field: RealFieldNumber,
{
    fn length_sqr(a: &Self::Point) -> Self::Field {
        Self::dot(a, a)
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let dx = a.x() - b.x();
        let dy = a.y() - b.y();
        dx * dx + dy * dy
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.x() * b.x() + a.y() * b.y()
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.x() - origin.x();
        let ady = a.y() - origin.y();
        let bdx = b.x() - origin.x();
        let bdy = b.y() - origin.y();
        adx * bdx + ady * bdy
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.x() * b.y() - a.y() * b.x()
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let adx = a.x() - origin.x();
        let ady = a.y() - origin.y();
        let bdx = b.x() - origin.x();
        let bdy = b.y() - origin.y();
        adx * bdy - ady * bdx
    }
}

impl<V> RealOperations2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Field: RealFieldNumber + From<f32>,
{
    type RealField = V::Field;
}
