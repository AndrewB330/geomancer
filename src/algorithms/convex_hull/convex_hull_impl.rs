use crate::{
    common::GeometryError,
    traits::{Cross2D, Dot2D, Kernel2D, Norm2D, NormSqr2D},
};
use num_traits::{Float, Zero};

fn farthest_point<K>(points: &[K::Point], from_point: Option<&K::Point>) -> usize
where
    K: Kernel2D + NormSqr2D,
    K::Scalar: PartialOrd,
{
    let mut farthest_point = 0;
    if let Some(from_point) = from_point {
        for i in 1..points.len() {
            if K::distance_sqr(from_point, &points[farthest_point])
                < K::distance_sqr(from_point, &points[i])
            {
                farthest_point = i;
            }
        }
    } else {
        for i in 1..points.len() {
            if K::distance_sqr_to_zero(&points[farthest_point])
                < K::distance_sqr_to_zero(&points[i])
            {
                farthest_point = i;
            }
        }
    }
    farthest_point
}

pub(super) fn convex_hull_impl<K>(points: &[K::Point]) -> Result<(Vec<usize>, bool), GeometryError>
where
    K: Kernel2D + NormSqr2D + Norm2D + Cross2D + Dot2D,
    K::Scalar: PartialOrd,
{
    if points.is_empty() {
        return Err(GeometryError::InputIsEmpty);
    }

    /*for p in points {
        if !p.x().is_valid() || !p.y().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }*/

    // todo: add short comment why this if is there
    if points.len() == 1 {
        return Ok((vec![0], true));
    }

    let first_point = farthest_point::<K>(points, None);
    let second_point = farthest_point::<K>(points, Some(&points[first_point]));

    let distance = K::distance(&points[first_point], &points[second_point]);

    // todo: explain this
    let max_magnitude = K::Real::max(
        distance * K::Real::from(1.732051),
        K::distance_to_zero(&points[first_point]),
    );

    let epsilon = max_magnitude * K::Real::epsilon() * K::Real::from(2.0);

    // Distance between two farthest points is less than (2 * epsilon) - all points are too close
    // to each other to build a convex hull from more than one point.
    // Return single point, because it is the best approximation of a convex hull in this case.
    if distance < epsilon * K::Real::from(2.0) {
        return Ok((vec![first_point], true));
    }

    let mut candidates_left: Vec<usize> = vec![];
    let mut candidates_right: Vec<usize> = vec![];

    for i in 0..points.len() {
        if i == first_point || i == second_point {
            continue;
        }

        let signed_area: K::Real =
            K::cross_with_origin(&points[second_point], &points[i], &points[first_point]).into();

        // We want to use only points that are at least at (2 * epsilon) distance from
        // the the segment that connects first_point and second_point. This is to ensure
        // that all candidates are far enough from collinearity with first and second points.
        if signed_area.abs() / distance > epsilon * K::Real::from(2.0) {
            if signed_area > K::Real::zero() {
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
    epsilon: K::Real,
    result: &mut Vec<usize>,
) where
    K: Kernel2D + Dot2D + Cross2D + Norm2D,
    K::Real: Float + From<f32>,
{
    if candidates.len() == 0 {
        result.push(right_point);
        return;
    }

    let cross_real = |a: usize, b: usize, c: usize| -> K::Real {
        K::cross_with_origin(&points[a], &points[b], &points[c]).into()
    };
    let dot_real = |a: usize, b: usize, c: usize| -> K::Real {
        K::dot_with_origin(&points[a], &points[b], &points[c]).into()
    };

    let mut cross_max = K::Real::zero();

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

    use num_traits::Float;

    use crate::{
        common::assert_eq_cycle,
        traits::{Cross2D, DefaultKernel, Dot2D, Kernel2D, Norm2D, NormSqr2D},
    };

    use super::convex_hull_impl;

    fn assert_convex_hull_f32<'a>(points: &'a [(f32, f32)], expected: &[usize]) {
        assert_convex_hull(points, expected);
    }

    fn assert_convex_hull<'a, V>(points: &'a [V], expected: &[usize])
    where
        V: DefaultKernel,
        V::Kernel: NormSqr2D + Norm2D + Cross2D + Dot2D,
        <V::Kernel as Kernel2D>::Scalar: PartialOrd,
        <V::Kernel as Norm2D>::Real: Float + From<f32>,
    {
        let hull = convex_hull_impl::<V::Kernel>(points);
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
        assert_convex_hull_f32(
            &[(0.0, 0.0), (1.0, 1.0), (1.0, 0.0), (0.0, 1.0)],
            &[0, 2, 1, 3],
        );
    }
}
