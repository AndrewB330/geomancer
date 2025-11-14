use num_traits::Zero;

use crate::{
    primitives::Polygon,
    traits::{Cross2D, Kernel2D, Norm2D},
};

pub struct ConvexPolygon<K>
where
    K: Kernel2D,
{
    pub(super) polygon: Polygon<K>,
}

impl<K> ConvexPolygon<K>
where
    K: Kernel2D,
{
    pub(crate) fn from_points_unchecked(points: Vec<K::Point>) -> Self {
        Self {
            polygon: Polygon::from_points_unchecked(points),
        }
    }

    pub fn area(&self) -> K::Scalar
    where
        K: Cross2D,
        K::Scalar: Zero,
    {
        self.polygon.area()
    }

    pub fn perimeter(&self) -> K::Real
    where
        K: Norm2D,
    {
        self.polygon.perimeter()
    }
}

impl<K> From<ConvexPolygon<K>> for Polygon<K>
where
    K: Kernel2D,
{
    fn from(value: ConvexPolygon<K>) -> Self {
        value.polygon
    }
}
