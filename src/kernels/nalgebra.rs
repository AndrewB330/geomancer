use std::{fmt::Debug, marker::PhantomData};

use nalgebra::{Point2, Vector2};
use num_traits::Zero;

use crate::{
    kernels::GenericKernel2D,
    traits::{DefaultKernel, Point2D},
};

pub struct NalgebraVector2Kernel<T>(PhantomData<T>);
pub struct NalgebraPoint2Kernel<T>(PhantomData<T>);

impl<T: PartialOrd + Zero + Clone> Point2D for Vector2<T> {
    type Scalar = T;

    fn x(&self) -> Self::Scalar {
        // TODO: maybe Point2D should return references?
        self[0].clone()
    }

    fn y(&self) -> Self::Scalar {
        self[1].clone()
    }
}

impl<T: 'static + PartialOrd + Zero + Clone + std::fmt::Debug> Point2D for Point2<T> {
    type Scalar = T;

    fn x(&self) -> Self::Scalar {
        self[0].clone()
    }

    fn y(&self) -> Self::Scalar {
        self.y.clone()
    }
}

impl<T: PartialOrd + Zero + Clone> DefaultKernel for Vector2<T> {
    type Kernel = GenericKernel2D<Self>;
}

impl<T: 'static + PartialOrd + Zero + Clone + Debug> DefaultKernel for Point2<T> {
    type Kernel = GenericKernel2D<Self>;
}

#[cfg(test)]
mod test {
    use nalgebra::{Point2, Vector2};

    use crate::algorithms::convex_hull;
    use crate::common::assert_eq_cycle;

    #[test]
    fn nalgbra_point2_kernel_simple_test() {
        let points = [
            Point2::new(0.1, 0.2),
            Point2::new(10.0, -1.0),
            Point2::new(5.0, 2.0),
            Point2::new(7.0, 7.0),
        ];
        let result = convex_hull(&points);
        assert_eq_cycle(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
    }

    #[test]
    fn nalgbra_vector2_kernel_simple_test() {
        let points = [
            Vector2::new(0.1, 0.2),
            Vector2::new(10.0, -1.0),
            Vector2::new(5.0, 2.0),
            Vector2::new(7.0, 7.0),
        ];
        let result = convex_hull(&points);
        assert_eq_cycle(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
    }
}
