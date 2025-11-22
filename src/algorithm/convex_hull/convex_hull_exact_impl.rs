use std::cmp::Ordering;

use crate::{
    common::{GeometryError, Orientation2D},
    traits::{ExactCompareNorm2D, ExactOrientation2D, Kernel2D},
};

fn farthest_point<K>(points: &[K::Point], from_point: usize) -> usize
where
    K: Kernel2D + ExactCompareNorm2D,
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
    K: Kernel2D + ExactOrientation2D,
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
    K: ExactCompareNorm2D + ExactOrientation2D,
    K::Point: PartialEq,
{
    if points.is_empty() {
        return Err(GeometryError::InputIsEmpty);
    }

    /*for p in points {
        if !p.x().is_valid() || !p.y().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }*/

    // todo: add short comment why this is here
    if points.len() == 1 {
        return Ok((vec![0], true));
    }

    let first_point = farthest_point::<K>(points, 0);
    let second_point = farthest_point::<K>(points, first_point);
    let rightmost_point = extreme_point::<K>(points, first_point, true);
    let leftmost_point = extreme_point::<K>(points, first_point, false);

    // todo: add short comment why this is here
    if points[first_point] == points[second_point] {
        return Ok((vec![first_point], true));
    }

    let mut candidates: Vec<usize> = vec![];

    for i in 0..points.len() {
        if points[first_point] != points[i] {
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
        if points[candidate] == points[*result.last().unwrap()] {
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
        common::assert_eq_cycle,
        traits::{DefaultKernel, ExactCompareNorm2D, ExactOrientation2D, Kernel2D},
    };

    use super::convex_hull_exact_impl;

    fn assert_convex_hull_f64<'a>(
        points: &'a [(f64, f64)],
        expected: &[usize],
        include_collinear: bool,
    ) {
        //todo: assert_convex_hull(points, expected, include_collinear);
    }

    fn assert_convex_hull<'a, V>(points: &'a [V], expected: &[usize], include_collinear: bool)
    where
        V: DefaultKernel,
        V::Kernel: ExactCompareNorm2D + ExactOrientation2D,
        <V::Kernel as Kernel2D>::Point: Eq,
    {
        let hull = convex_hull_exact_impl::<V::Kernel>(points, include_collinear);
        assert!(hull.is_ok());

        let (hull, degenrate) = hull.unwrap();

        if expected.len() <= 2 {
            assert!(degenrate);
            assert_eq!(hull.len(), expected.len());
            // TODO: add perimeter comparison?
            return;
        }

        assert_eq_cycle(hull, expected.to_vec());
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
