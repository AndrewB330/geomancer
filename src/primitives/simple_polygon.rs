use crate::kernel::StandardKernel2D;
use crate::primitives::Polygon;
use crate::kernel::{Cross2D, Kernel2D};

pub struct SimplePolygon<K = StandardKernel2D>
where
    K: Kernel2D,
{
    inner: Polygon<K>,
}

impl<K> SimplePolygon<K>
where
    K: Kernel2D,
{
    fn from_polygon_unchecked(polygon: Polygon<K>) -> Self {
        Self { inner: polygon }
    }
}

impl<K: Kernel2D + Cross2D> TryFrom<Polygon<K>> for SimplePolygon<K> {
    type Error = crate::common::GeometryError;

    fn try_from(polygon: Polygon<K>) -> Result<Self, Self::Error> {
        if polygon.has_self_intersections() {
            return Err(crate::common::GeometryError::Unknown);
        }
        Ok(SimplePolygon::from_polygon_unchecked(polygon))
    }
}
