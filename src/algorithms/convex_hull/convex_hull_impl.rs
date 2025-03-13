use crate::{
    common::GeometryError,
    traits::{FieldNumber, Kernel2D, Operations2D, Point2D, RealOperations2D},
};
use num_traits::{Float, Zero};

fn farthest_point<K>(points: &[K::Point], from_point: Option<&K::Point>) -> usize
where
    K: Kernel2D + Operations2D + RealOperations2D,
{
    let mut farthest_point = 0;
    if let Some(from_point) = from_point {
        for i in 1..points.len() {
            if K::RealField::from(K::distance_sqr(from_point, &points[farthest_point]))
                < K::RealField::from(K::distance_sqr(from_point, &points[i]))
            {
                farthest_point = i;
            }
        }
    } else {
        for i in 1..points.len() {
            if K::RealField::from(K::length_sqr(&points[farthest_point]))
                < K::RealField::from(K::length_sqr(&points[i]))
            {
                farthest_point = i;
            }
        }
    }
    farthest_point
}

pub(super) fn convex_hull_impl<K>(points: &[K::Point]) -> Result<(Vec<usize>, bool), GeometryError>
where
    K: Kernel2D + Operations2D + RealOperations2D,
{
    if points.is_empty() {
        return Err(GeometryError::InputIsEmpty);
    }

    for p in points {
        if !p.x().is_valid() || !p.y().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }

    // todo: add short comment why this if is there
    if points.len() == 1 {
        return Ok((vec![0], true));
    }

    let first_point = farthest_point::<K>(points, None);
    let second_point = farthest_point::<K>(points, Some(&points[first_point]));

    let distance = K::distance(&points[first_point], &points[second_point]);

    // todo: explain this
    let max_magnitude = K::RealField::max(
        distance * K::RealField::from(1.732051),
        K::length(&points[first_point]),
    );

    let epsilon = max_magnitude * K::RealField::epsilon() * K::RealField::from(2.0);

    // Distance between two farthest points is less than (2 * epsilon) - all points are too close
    // to each other to build a convex hull from more than one point.
    // Return single point, because it is the best approximation of a convex hull in this case.
    if distance < epsilon * K::RealField::from(2.0) {
        return Ok((vec![first_point], true));
    }

    let mut candidates_left: Vec<usize> = vec![];
    let mut candidates_right: Vec<usize> = vec![];

    for i in 0..points.len() {
        if i == first_point || i == second_point {
            continue;
        }

        let signed_area: K::RealField =
            K::cross_with_origin(&points[second_point], &points[i], &points[first_point]).into();

        // We want to use only points that are at least at (2 * epsilon) distance from
        // the the segment that connects first_point and second_point. This is to ensure
        // that all candidates are far enough from collinearity with first and second points.
        if signed_area.abs() / distance > epsilon * K::RealField::from(2.0) {
            if signed_area > K::RealField::zero() {
                candidates_left.push(i);
            } else {
                candidates_right.push(i);
            }
        }
    }

    let mut result = vec![];
    let degenerate = candidates_left.is_empty() && candidates_right.is_empty();

    quickhull_recursive::<K>(
        points,
        first_point,
        second_point,
        candidates_right,
        epsilon,
        &mut result,
    );
    quickhull_recursive::<K>(
        points,
        second_point,
        first_point,
        candidates_left,
        epsilon,
        &mut result,
    );

    Ok((result, degenerate))
}

fn quickhull_recursive<K>(
    points: &[K::Point],
    right_point: usize,
    left_point: usize,
    candidates: Vec<usize>,
    epsilon: K::RealField,
    result: &mut Vec<usize>,
) where
    K: Kernel2D + Operations2D + RealOperations2D,
{
    if candidates.len() == 0 {
        result.push(right_point);
        return;
    }

    let cross_real = |a: usize, b: usize, c: usize| -> K::RealField {
        K::cross_with_origin(&points[a], &points[b], &points[c]).into()
    };
    let dot_real = |a: usize, b: usize, c: usize| -> K::RealField {
        K::dot_with_origin(&points[a], &points[b], &points[c]).into()
    };

    let mut cross_max = K::RealField::zero();

    let mut farthest = candidates[0];
    for i in &candidates {
        let cross = cross_real(right_point, *i, left_point);
        if cross > cross_max {
            cross_max = cross;
            farthest = *i;
        }
    }

    let cross_epsilon = epsilon * K::distance(&points[right_point], &points[left_point]);

    let mut dot_max = dot_real(right_point, farthest, left_point);

    for i in &candidates {
        if cross_real(right_point, *i, left_point) > cross_max - cross_epsilon {
            let dot = dot_real(right_point, *i, left_point);
            if dot > dot_max {
                dot_max = dot;
                farthest = *i;
            }
        }
    }

    let mut right = vec![];
    let mut left = vec![];

    let cross_right_epsilon = K::distance(&points[right_point], &points[farthest]) * epsilon;
    let cross_left_epsilon = K::distance(&points[right_point], &points[farthest]) * epsilon;

    for i in candidates {
        if cross_real(i, farthest, right_point) > cross_right_epsilon {
            right.push(i);
        } else if cross_real(i, left_point, farthest) > cross_left_epsilon {
            left.push(i);
        }
    }

    quickhull_recursive::<K>(points, right_point, farthest, right, epsilon, result);
    quickhull_recursive::<K>(points, farthest, left_point, left, epsilon, result);
}

#[cfg(test)]
mod test {
    use core::f32;

    use crate::{
        common::Orientation2D,
        traits::{
            DefaultKernel, ExactPredicates2D, Kernel2D, Operations2D, Point2D, RealOperations2D,
        },
    };

    use super::convex_hull_impl;

    impl Point2D for (f32, f32) {
        type Field = f32;

        fn x(&self) -> Self::Field {
            self.0
        }

        fn y(&self) -> Self::Field {
            self.1
        }
    }

    pub struct F32TupleKernel;

    impl Kernel2D for F32TupleKernel {
        type Point = (f32, f32);

        type Field = f32;
    }

    impl Operations2D for F32TupleKernel {
        fn length_sqr(a: &Self::Point) -> Self::Field {
            a.0 * a.0 + a.1 * a.1
        }

        fn distance_sqr(a: &Self::Point, b: &Self::Point) -> Self::Field {
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            dx * dx + dy * dy
        }

        fn dot(a: &Self::Point, b: &Self::Point) -> Self::Field {
            a.0 * b.0 + a.1 * b.1
        }

        fn dot_with_origin(a: &Self::Point, b: &Self::Point, origin: &Self::Point) -> Self::Field {
            let adx = a.0 - origin.0;
            let ady = a.1 - origin.1;
            let bdx = b.0 - origin.0;
            let bdy = b.1 - origin.1;
            adx * bdx + ady * bdy
        }

        fn cross(a: &Self::Point, b: &Self::Point) -> Self::Field {
            a.0 * b.1 - a.1 * b.0
        }

        fn cross_with_origin(
            a: &Self::Point,
            b: &Self::Point,
            origin: &Self::Point,
        ) -> Self::Field {
            let adx = a.0 - origin.0;
            let ady = a.1 - origin.1;
            let bdx = b.0 - origin.0;
            let bdy = b.1 - origin.1;
            adx * bdy - ady * bdx
        }
    }

    impl RealOperations2D for F32TupleKernel {
        type RealField = f32;

        fn length(a: &Self::Point) -> Self::RealField {
            Self::length_sqr(a).sqrt()
        }

        fn distance(a: &Self::Point, b: &Self::Point) -> Self::RealField {
            Self::distance_sqr(a, b).sqrt()
        }
    }

    // Bad implementation, using just cast to f64, not precise, just for now for tests.
    unsafe impl ExactPredicates2D for F32TupleKernel {
        fn is_same_point(a: &Self::Point, b: &Self::Point) -> bool {
            a.x() == b.x() && a.y() == b.y()
        }

        fn compare_distance(
            a: &Self::Point,
            b: &Self::Point,
            to: &Self::Point,
        ) -> std::cmp::Ordering {
            let adx = a.x() as f64 - to.x() as f64;
            let ady = a.y() as f64 - to.y() as f64;
            let bdx = b.x() as f64 - to.x() as f64;
            let bdy = b.y() as f64 - to.y() as f64;
            (adx * adx + ady * ady)
                .partial_cmp(&(bdx * bdx + bdy * bdy))
                .unwrap()
        }

        fn compare_length(a: &Self::Point, b: &Self::Point) -> std::cmp::Ordering {
            Self::compare_distance(a, b, &(0.0, 0.0))
        }

        fn orientation(
            a: &Self::Point,
            b: &Self::Point,
            c: &Self::Point,
        ) -> crate::prelude::Orientation2D {
            let adx = a.x() as f64 - c.x() as f64;
            let ady = a.y() as f64 - c.y() as f64;
            let bdx = b.x() as f64 - c.x() as f64;
            let bdy = b.y() as f64 - c.y() as f64;
            let cross = adx * bdy - ady * bdx;
            match cross.partial_cmp(&0.0).unwrap() {
                std::cmp::Ordering::Less => Orientation2D::Clockwise,
                std::cmp::Ordering::Equal => Orientation2D::Collinear,
                std::cmp::Ordering::Greater => Orientation2D::CounterClockwise,
            }
        }
    }

    impl DefaultKernel for (f32, f32) {
        type Kernel = F32TupleKernel;
    }

    fn assert_convex_hull_f32<'a>(points: &'a [(f32, f32)], expected: &[usize]) {
        assert_convex_hull(points, expected);
    }

    fn assert_convex_hull<'a, V>(points: &'a [V], expected: &[usize])
    where
        V: DefaultKernel + Point2D + Clone,
        V::Kernel: Operations2D + RealOperations2D,
    {
        let hull = convex_hull_impl::<V::Kernel>(points);
        assert!(hull.is_ok());

        let hull = hull.unwrap().0;

        if expected.len() == 1 {
            return;
        }

        let mut offset = None;

        for i in 0..hull.len() {
            if hull[i] == expected[0] {
                offset = Some(i);
                break;
            }
        }

        assert!(
            offset.is_some(),
            "First point of expected convex hull not found. {:?} {:?}",
            hull,
            expected
        );
        let offset = offset.unwrap();

        let offseted: Vec<usize> = hull[offset..]
            .iter()
            .chain(hull[..offset].iter())
            .cloned()
            .collect();

        assert_eq!(offseted, expected);
    }

    #[test]
    fn simple_triangle() {
        assert_convex_hull_f32(
            &[(0.1, 0.2), (10.0, -1.0), (5.0, 2.0), (7.0, 7.0)],
            &[0, 1, 3],
        );
    }

    #[test]
    fn collinear() {
        assert_convex_hull_f32(&[(0.1, 0.3), (0.01, 0.03), (1., 3.), (0.3, 0.9)], &[1, 2]);
    }

    #[test]
    fn relatively_close_together() {
        // Points are too close together to meaningfully compute convex hull.
        // The best approximation is single point, since we are alowing approximation to
        // be slightly smaller than the exact convex hull. See ConvexHull guarantees for more details.
        assert_convex_hull_f32(
            &[(1000000.1, 0.0), (1000000.2, 0.0), (1000000.1, 0.1)],
            &[0],
        );
    }

    #[test]
    fn square() {
        // Points are too close together to meaningfully compute convex hull.
        // The best approximation is single point, since we are alowing approximation to
        // be slightly smaller than the exact convex hull. See ConvexHull guarantees for more details.
        assert_convex_hull_f32(
            &[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)],
            &[0, 2, 1, 3],
        );
    }
}
