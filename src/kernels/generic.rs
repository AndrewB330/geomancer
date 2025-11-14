use std::marker::PhantomData;

use num_traits::{Float, Num};

use crate::traits::{Cross2D, Dot2D, Kernel2D, Norm2D, NormSqr2D, Point2D};

pub struct GenericKernel2D<V>(PhantomData<V>);

impl<V> Kernel2D for GenericKernel2D<V>
where
    V: Point2D,
{
    type Point = V;
    type Scalar = V::Scalar;
}

impl<V> NormSqr2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Scalar: Num + Clone,
{
    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        let dx = a.x() - b.x();
        let dy = a.y() - b.y();
        dx.clone() * dx + dy.clone() * dy
    }

    fn distance_sqr_to_zero(a: &Self::Point) -> Self::Scalar {
        let dx = a.x();
        let dy = a.y();
        dx.clone() * dx + dy.clone() * dy
    }
}

impl<V> Norm2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Scalar: Float + From<f32>,
{
    type Real = V::Scalar;

    fn distance(a: &Self::Point, b: &Self::Point) -> Self::Real {
        (Self::distance_sqr(a, b)).sqrt()
    }

    fn distance_to_zero(a: &Self::Point) -> Self::Real {
        (Self::distance_sqr_to_zero(a)).sqrt()
    }
}

impl<V> Cross2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Scalar: Num,
{
    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        a.x() * b.y() - a.y() * b.x()
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar {
        let adx = a.x() - origin.x();
        let ady = a.y() - origin.y();
        let bdx = b.x() - origin.x();
        let bdy = b.y() - origin.y();
        adx * bdy - ady * bdx
    }
}

impl<V> Dot2D for GenericKernel2D<V>
where
    V: Point2D,
    V::Scalar: Num,
{
    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        a.x() * b.x() + a.y() * b.y()
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar {
        let adx = a.x() - origin.x();
        let ady = a.y() - origin.y();
        let bdx = b.x() - origin.x();
        let bdy = b.y() - origin.y();
        adx * bdx + ady * bdy
    }
}
