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

pub(super) fn convex_hull_impl<K>(points: &[K::Point]) -> Result<Vec<usize>, GeometryError>
where
    K: Kernel2D + Operations2D + RealOperations2D,
{
    for p in points {
        if !p.x().is_valid() || !p.y().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }

    // todo: add short comment why this if is there
    if points.len() <= 1 {
        return Ok((0..points.len()).collect());
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
        return Ok(vec![first_point]);
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

    Ok(result)
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
    quickhull_recursive::<K>(points, farthest, right_point, left, epsilon, result);
}
