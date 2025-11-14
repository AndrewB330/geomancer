use bevy_math::prelude::*;

use crate::traits::{Cross2D, DefaultKernel, Dot2D, Kernel2D, Norm2D, NormSqr2D, Point2D};

pub struct BevyVec2Kernel;

impl Point2D for Vec2 {
    type Scalar = f32;

    fn x(&self) -> Self::Scalar {
        self.x
    }

    fn y(&self) -> Self::Scalar {
        self.y
    }
}

impl Kernel2D for BevyVec2Kernel {
    type Point = Vec2;

    type Scalar = f32;
}

impl Dot2D for BevyVec2Kernel {
    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        a.dot(b.clone())
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar {
        (a - origin).dot(b - origin)
    }
}

impl Cross2D for BevyVec2Kernel {
    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        a.perp_dot(b.clone())
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Scalar {
        (a - origin).perp_dot(b - origin)
    }
}

impl NormSqr2D for BevyVec2Kernel {
    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Scalar {
        a.distance_squared(b.clone())
    }

    fn distance_sqr_to_zero(a: &Self::Point) -> Self::Scalar {
        a.length_squared()
    }
}

impl Norm2D for BevyVec2Kernel {
    type Real = f32;

    fn distance(a: &Self::Point, b: &Self::Point) -> Self::Real {
        a.distance(b.clone())
    }

    fn distance_to_zero(a: &Self::Point) -> Self::Real {
        a.length()
    }
}

impl DefaultKernel for Vec2 {
    type Kernel = BevyVec2Kernel;
}

#[cfg(test)]
mod test {
    use bevy_math::Vec2;

    use crate::algorithms::convex_hull;
    use crate::common::assert_eq_cycle;

    #[test]
    fn bevy_kernel_simple_test() {
        let points = [
            Vec2::new(0.1, 0.2),
            Vec2::new(10.0, -1.0),
            Vec2::new(5.0, 2.0),
            Vec2::new(7.0, 7.0),
        ];
        let result = convex_hull(&points);
        assert_eq_cycle(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
    }
}
