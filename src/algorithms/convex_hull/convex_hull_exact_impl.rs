use std::cmp::Ordering;

use crate::{
    common::{GeometryError, Orientation2D},
    traits::{ExactPredicates2D, FieldNumber, Kernel2D, Point2D},
};

fn farthest_point<K>(points: &[K::Point], from_point: usize) -> usize
where
    K: Kernel2D + ExactPredicates2D,
{
    let mut farthest_point = 0;
    for i in 1..points.len() {
        if K::compare_distance(&points[farthest_point], &points[i], &points[from_point])
            == Ordering::Less
        {
            farthest_point = i;
        }
    }
    farthest_point
}

fn extreme_point<K>(points: &[K::Point], from_point: usize, rightmost: bool) -> usize
where
    K: Kernel2D + ExactPredicates2D,
{
    let mut extreme_point = 0;
    for i in 1..points.len() {
        let orientation = K::orientation(&points[extreme_point], &points[i], &points[from_point]);
        if orientation == Orientation2D::Clockwise && rightmost
            || orientation == Orientation2D::CounterClockwise && !rightmost
        {
            extreme_point = i;
        }
    }
    extreme_point
}

pub(super) fn convex_hull_exact_impl<K>(
    points: &[K::Point],
    include_colinear: bool,
) -> Result<(Vec<usize>, bool), GeometryError>
where
    K: Kernel2D + ExactPredicates2D,
{
    if points.is_empty() {
        return Err(GeometryError::InputIsEmpty);
    }

    for p in points {
        if !p.x().is_valid() || !p.y().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }

    // todo: add short comment why this is here
    if points.len() == 1 {
        return Ok((vec![0], true));
    }

    let first_point = farthest_point::<K>(points, 0);
    let second_point = farthest_point::<K>(points, first_point);
    let rightmost_point = extreme_point::<K>(points, first_point, true);
    let leftmost_point = extreme_point::<K>(points, first_point, false);

    // todo: add short comment why this is here
    if K::is_same_point(&points[first_point], &points[second_point]) {
        return Ok((vec![first_point], true));
    }

    let mut candidates: Vec<usize> = vec![];

    for i in 0..points.len() {
        if !K::is_same_point(&points[first_point], &points[i]) {
            candidates.push(i);
        }
    }

    if candidates.is_empty() {
        return Ok((vec![first_point, second_point], true));
    }

    let mut sorting_error = Ok(());

    candidates.sort_by(|i, j| {
        let a = &points[*i];
        let b = &points[*j];

        match K::orientation(a, b, &points[first_point]) {
            crate::common::Orientation2D::CounterClockwise => Ordering::Less,
            crate::common::Orientation2D::Clockwise => Ordering::Greater,
            crate::common::Orientation2D::Collinear => {
                match K::orientation(a, &points[rightmost_point], &points[first_point]) {
                    Orientation2D::Collinear => K::compare_distance(a, b, &points[first_point]),
                    Orientation2D::CounterClockwise => {
                        K::compare_distance(b, a, &points[first_point])
                    }
                    Orientation2D::Clockwise => {
                        sorting_error = Err(GeometryError::ExactPredicateReturnedConflictingResult(
                            "Convex Hull: Graham Scan sorting: Clockwise compared to rightmost.".to_string(),
                        ));
                        K::compare_distance(b, a, &points[first_point])
                    }
                }
            }
        }
    });

    sorting_error?;

    let mut result = vec![first_point, candidates[0]];

    for candidate in candidates {
        if K::is_same_point(&points[candidate], &points[*result.last().unwrap()]) {
            continue;
        }

        loop {
            let last = result[result.len() - 1];
            let prev_last = result[result.len() - 2];
            match K::orientation(&points[last], &points[candidate], &points[prev_last]) {
                Orientation2D::CounterClockwise => {
                    break;
                }
                Orientation2D::Collinear => {
                    if include_colinear {
                        break;
                    }
                    result.pop();
                    if result.len() == 1 {
                        if K::orientation(
                            &points[rightmost_point],
                            &points[candidate],
                            &points[first_point],
                        ) != Orientation2D::Collinear
                        {
                            return Err(GeometryError::ExactPredicateReturnedConflictingResult(
                                "Convex Hull: Graham Scan loop: Collinear case.".to_string(),
                            ));
                        }
                        break;
                    }
                }
                Orientation2D::Clockwise => {
                    result.pop();
                    if result.len() == 1 {
                        return Err(GeometryError::ExactPredicateReturnedConflictingResult(
                            "Convex Hull: Graham Scan loop: Clockwise case.".to_string(),
                        ));
                    }
                }
            }
        }

        result.push(candidate);
    }

    let degenerate = K::orientation(
        &points[leftmost_point],
        &points[rightmost_point],
        &points[first_point],
    ) == Orientation2D::Collinear;

    Ok((result, degenerate))
}

#[cfg(test)]
mod test {
    use crate::{
        common::Orientation2D,
        traits::{DefaultKernel, ExactPredicates2D, Kernel2D, Operations2D, Point2D},
    };

    use super::convex_hull_exact_impl;

    impl Point2D for (f64, f64) {
        type Field = f64;

        fn x(&self) -> Self::Field {
            self.0
        }

        fn y(&self) -> Self::Field {
            self.1
        }
    }

    pub struct F64TupleKernel;

    impl Kernel2D for F64TupleKernel {
        type Point = (f64, f64);

        type Field = f64;
    }

    impl Operations2D for F64TupleKernel {
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

    // This implementation of ExactPredicates IS NOT exact, it is here for tests only
    unsafe impl ExactPredicates2D for F64TupleKernel {
        fn is_same_point(a: &Self::Point, b: &Self::Point) -> bool {
            a == b
        }

        fn compare_distance(
            a: &Self::Point,
            b: &Self::Point,
            to: &Self::Point,
        ) -> std::cmp::Ordering {
            Self::distance_sqr(a, to)
                .partial_cmp(&Self::distance_sqr(b, to))
                .unwrap()
        }

        fn compare_length(a: &Self::Point, b: &Self::Point) -> std::cmp::Ordering {
            Self::length_sqr(a)
                .partial_cmp(&Self::length_sqr(b))
                .unwrap()
        }

        fn orientation(
            a: &Self::Point,
            b: &Self::Point,
            c: &Self::Point,
        ) -> crate::prelude::Orientation2D {
            match Self::cross_with_origin(a, b, c).partial_cmp(&0.0).unwrap() {
                std::cmp::Ordering::Less => Orientation2D::Clockwise,
                std::cmp::Ordering::Equal => Orientation2D::Collinear,
                std::cmp::Ordering::Greater => Orientation2D::CounterClockwise,
            }
        }
    }

    impl DefaultKernel for (f64, f64) {
        type Kernel = F64TupleKernel;
    }

    fn assert_convex_hull_f64<'a>(
        points: &'a [(f64, f64)],
        expected: &[usize],
        include_collinear: bool,
    ) {
        assert_convex_hull(points, expected, include_collinear);
    }

    fn assert_convex_hull<'a, V>(points: &'a [V], expected: &[usize], include_collinear: bool)
    where
        V: DefaultKernel + Point2D + Clone,
        V::Kernel: ExactPredicates2D,
    {
        let hull = convex_hull_exact_impl::<V::Kernel>(points, include_collinear);
        assert!(hull.is_ok());

        let hull = hull.unwrap().0;

        let mut offset = None;

        for i in 0..hull.len() {
            if hull[i] == expected[0] {
                offset = Some(i);
                break;
            }
        }

        assert!(
            offset.is_some(),
            "First point of expected convex hull not found."
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
        assert_convex_hull_f64(
            &[(0.1, 0.2), (10.0, -1.0), (5.0, 2.0), (7.0, 7.0)],
            &[0, 1, 3],
            true,
        );
    }

    #[test]
    fn collinear() {
        assert_convex_hull_f64(
            &[(0.125, 0.25), (0.5, 1.0), (1., 2.), (0.25, 0.5)],
            &[0, 2],
            false,
        );
        assert_convex_hull_f64(
            &[(0.125, 0.25), (0.5, 1.0), (1., 2.), (0.25, 0.5)],
            &[2, 1, 3, 0],
            true,
        );
    }

    #[test]
    fn relatively_close_together() {
        // Points are too close together to meaningfully compute convex hull.
        // The best approximation is single point, since we are alowing approximation to
        // be slightly smaller than the exact convex hull. See ConvexHull guarantees for more details.
        assert_convex_hull_f64(
            &[(1000000.1, 0.0), (1000000.2, 0.0), (1000000.1, 0.1)],
            &[0, 1, 2],
            true,
        );
    }

    #[test]
    fn square() {
        // Points are too close together to meaningfully compute convex hull.
        // The best approximation is single point, since we are alowing approximation to
        // be slightly smaller than the exact convex hull. See ConvexHull guarantees for more details.
        assert_convex_hull_f64(
            &[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)],
            &[0, 2, 1, 3],
            true,
        );
    }
}
