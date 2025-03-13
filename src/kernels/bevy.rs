use bevy_math::prelude::*;

use crate::traits::{DefaultKernel, Kernel2D, Operations2D, Point2D, RealOperations2D};

pub struct BevyVec2Kernel;

impl Point2D for Vec2 {
    type Field = f32;

    fn x(&self) -> Self::Field {
        self.x
    }

    fn y(&self) -> Self::Field {
        self.y
    }
}

impl Kernel2D for BevyVec2Kernel {
    type Point = Vec2;

    type Field = f32;
}

impl Operations2D for BevyVec2Kernel {
    fn length_sqr(a: &Self::Point) -> Self::Field {
        a.length_squared()
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.distance_squared(b.clone())
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.dot(b.clone())
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        (a - origin).dot(b - origin)
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.perp_dot(b.clone())
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        (a - origin).perp_dot(b - origin)
    }
}

impl RealOperations2D for BevyVec2Kernel {
    type RealField = f32;

    fn length(a: &Self::Point) -> Self::RealField {
        a.length()
    }

    fn distance(a: &Self::Point, b: &Self::Point) -> Self::RealField {
        a.distance(b.clone())
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
