use geo::{Coord, CoordNum};

use crate::traits::{DefaultKernel, Point2D};

use super::GenericKernel2D;

impl<T> Point2D for Coord<T>
where
    T: CoordNum,
{
    type Scalar = T;

    fn x(&self) -> Self::Scalar {
        self.x
    }

    fn y(&self) -> Self::Scalar {
        self.y
    }
}

pub type GeoCoordKernel<T> = GenericKernel2D<Coord<T>>;

impl<T> DefaultKernel for Coord<T>
where
    T: CoordNum,
{
    type Kernel = GeoCoordKernel<T>;
}

#[cfg(test)]
mod test {
    use geo::Coord;

    use crate::algorithms::convex_hull;
    use crate::common::assert_eq_cycle;

    #[test]
    fn geo_kernel_simple_test() {
        let points = [
            Coord { x: 0.1, y: 0.2 },
            Coord { x: 10.0, y: -1.0 },
            Coord { x: 5.0, y: 2.0 },
            Coord { x: 7.0, y: 7.0 },
        ];
        let result = convex_hull(&points);
        assert_eq_cycle(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
    }
}
