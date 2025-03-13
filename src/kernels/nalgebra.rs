use std::marker::PhantomData;

use nalgebra::{ClosedAddAssign, ClosedMulAssign, ClosedSubAssign, Point2, Vector2};

use crate::traits::{DefaultKernel, FieldNumber, Kernel2D, Operations2D, Point2D, RealFieldNumber, RealOperations2D};

pub struct NalgebraVector2Kernel<T>(PhantomData<T>);
pub struct NalgebraPoint2Kernel<T>(PhantomData<T>);

impl<T: Clone> Point2D for Vector2<T> {
    type Field = T;

    fn x(&self) -> Self::Field {
        // TODO: maybe Point2D should return references?
        self[0].clone()
    }

    fn y(&self) -> Self::Field {
        self[1].clone()
    }
}

impl<T: FieldNumber + Clone> Point2D for Point2<T> {
    type Field = T;

    fn x(&self) -> Self::Field {
        self[0].clone()
    }

    fn y(&self) -> Self::Field {
        self.y.clone()
    }
}

impl<T: Clone + FieldNumber> Kernel2D for NalgebraVector2Kernel<T> {
    type Point = Vector2<T>;

    type Field = T;
}

impl<T: Clone + FieldNumber> Kernel2D for NalgebraPoint2Kernel<T> {
    type Point = Point2<T>;

    type Field = T;
}

impl<T: Clone + FieldNumber + ClosedAddAssign + ClosedMulAssign + ClosedSubAssign> Operations2D
    for NalgebraVector2Kernel<T>
{
    fn length_sqr(a: &Self::Point) -> Self::Field {
        a.dot(a)
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let diff = a - b;
        diff.dot(&diff)
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.dot(b)
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        (a - origin).dot(&(b - origin))
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a[0].clone() * b[1].clone() - a[1].clone() * b[0].clone()
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let a_rel = a - origin;
        let b_rel = b - origin;
        a_rel[0].clone() * b_rel[1].clone() - a_rel[1].clone() * b_rel[0].clone()
    }
}

impl<T: FieldNumber + Clone + ClosedAddAssign + ClosedMulAssign + ClosedSubAssign> Operations2D for NalgebraPoint2Kernel<T>
{
    fn length_sqr(a: &Self::Point) -> Self::Field {
        a.coords.dot(&a.coords)
    }

    fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
        let diff = a - b;
        diff.dot(&diff)
    }

    fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.coords.dot(&b.coords)
    }

    fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        (a - origin).dot(&(b - origin))
    }

    fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
        a.coords[0].clone() * b.coords[1].clone() - a.coords[1].clone() * b.coords[0].clone()
    }

    fn cross_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
        let a_rel = a - origin;
        let b_rel = b - origin;
        a_rel[0].clone() * b_rel[1].clone() - a_rel[1].clone() * b_rel[0].clone()
    }
}

impl<T: RealFieldNumber + ClosedAddAssign + ClosedMulAssign + ClosedSubAssign + From<f32>> RealOperations2D for NalgebraVector2Kernel<T> {
    type RealField = T;
}

impl<T: RealFieldNumber + ClosedAddAssign + ClosedMulAssign + ClosedSubAssign + From<f32>> RealOperations2D for NalgebraPoint2Kernel<T> {
    type RealField = T;
}

impl<T: Clone + FieldNumber> DefaultKernel for Vector2<T> {
    type Kernel = NalgebraVector2Kernel<T>;
}

impl<T: Clone + FieldNumber> DefaultKernel for Point2<T> {
    type Kernel = NalgebraPoint2Kernel<T>;
}

#[cfg(test)]
mod test {
    use nalgebra::{Point2, Vector2};

    use crate::algorithms2d::convex_hull;

    #[test]
    fn nalgbra_point2_kernel_simple_test() {
        let points = [
            Point2::new(0.1, 0.2),
            Point2::new(10.0, -1.0),
            Point2::new(5.0, 2.0),
            Point2::new(7.0, 7.0),
        ];
        let result = convex_hull(&points);
        assert_eq!(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
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
        assert_eq!(result.unwrap().hull_indices().clone(), vec![0, 1, 3]);
    }
}
