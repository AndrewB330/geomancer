use crate::{
    primitives::Polygon,
    traits::{Kernel2D, Operations2D, RealOperations2D},
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

    pub fn area(&self) -> K::Field
    where
        K: Operations2D,
    {
        self.polygon.area()
    }

    pub fn perimeter(&self) -> K::RealField
    where
        K: RealOperations2D,
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
