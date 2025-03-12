use std::{borrow::Cow, cmp::Ordering};

use crate::{
    common::GeometryError,
    traits::{
        ApproxPredicates2D, DefaultKernel, FieldNumber, Kernel2D, Point2D,
        RealFieldOperations2D,
    },
};
use num_traits::{Float, Zero};

pub struct ConvexHull2D<'a, K: Kernel2D>
where
    K::Point: Clone,
{
    hull_indices: Vec<usize>,
    points: Cow<'a, [K::Point]>,
}

pub fn convex_hull<V>(points: &[V]) -> Result<Vec<V>, GeometryError>
where
    V: DefaultKernel + Clone,
    <V as DefaultKernel>::Kernel: Kernel2D<Point = V> + ApproxPredicates2D + RealFieldOperations2D,
{
    Ok(
        ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points(points)?
            .hull_points()
            .cloned()
            .collect(),
    )
}

impl<'a, K: Kernel2D + ApproxPredicates2D + RealFieldOperations2D> ConvexHull2D<'a, K>
where
    K::Point: Clone,
{
    pub fn from_points_owned(
        points: impl IntoIterator<Item = K::Point>,
    ) -> Result<Self, GeometryError> {
        let points_owned = points.into_iter().collect::<Vec<_>>();
        Ok(Self {
            hull_indices: convex_hull_impl::<K>(&points_owned)?,
            points: Cow::Owned(points_owned),
        })
    }

    pub fn from_points(points: &'a [K::Point]) -> Result<Self, GeometryError> {
        Ok(Self {
            hull_indices: convex_hull_impl::<K>(points.as_ref())?,
            points: Cow::Borrowed(points.as_ref()),
        })
    }
}

impl<'a, K: Kernel2D> ConvexHull2D<'a, K>
where
    K::Point: Clone,
{
    pub fn to_owned(self) -> ConvexHull2D<'static, K> {
        ConvexHull2D::<'static, K> {
            hull_indices: self.hull_indices,
            points: Cow::Owned(self.points.into_owned()),
        }
    }

    pub fn is_hull_only(&self) -> bool {
        self.hull_indices.len() == self.points.len()
    }

    pub fn hull_indices(&self) -> &Vec<usize> {
        &self.hull_indices
    }

    pub fn inside_indices(&self) -> Vec<usize> {
        // We are not storing indices of points that are inside of the convex hull
        // so we need to compute them every time the method is called.
        if self.is_hull_only() {
            vec![]
        } else {
            let mut all_indices = (0..self.points.len()).collect::<Vec<_>>();
            for index in &self.hull_indices {
                all_indices[*index] = usize::MAX;
            }
            all_indices.retain(|index| *index != usize::MAX);
            all_indices
        }
    }

    pub fn hull_points(&self) -> impl Iterator<Item = &K::Point> {
        self.hull_indices().iter().map(|i| &self.points[*i])
    }

    pub fn inside_points(&self) -> Vec<&K::Point> {
        self.inside_indices()
            .iter()
            .map(|i| &self.points[*i])
            .collect()
    }
}

fn farthest_point<K: Kernel2D + ApproxPredicates2D>(
    points: &[K::Point],
    from_point: Option<&K::Point>,
) -> usize {
    let mut farthest_point = 0;
    if let Some(from_point) = from_point {
        for i in 1..points.len() {
            if K::compare_distance(
                &points[farthest_point],
                &points[i],
                from_point,
                K::RealField::zero(),
            ) == Ordering::Less
            {
                farthest_point = i;
            }
        }
        for i in 1..points.len() {
            if K::compare_length(&points[farthest_point], &points[i], K::RealField::zero())
                == Ordering::Less
            {
                farthest_point = i;
            }
        }
    }
    farthest_point
}

fn convex_hull_impl<K: Kernel2D + ApproxPredicates2D + RealFieldOperations2D>(
    points: &[K::Point],
) -> Result<Vec<usize>, GeometryError> {
    for p in points {
        if !p.x().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }

    // todo: add short comment why this if is there
    if points.len() <= 1 {
        return Ok((0..points.len()).collect());
    }

    let first_point = farthest_point::<K>(points, None);
    let second_point = farthest_point::<K>(points, Some(&points[first_point]));

    let mut candidates: Vec<usize> = vec![];

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

    for i in 0..points.len() {
        if i == first_point || i == second_point {
            continue;
        }

        let signed_area: K::RealField =
            K::cross_with_origin(&points[first_point], &points[second_point], &points[i]).into();

        // We want to use only points that are at least at (2 * epsilon) distance from
        // the the segment that connects first_point and second_point. This is to ensure
        // that all candidates are far enough from colinearity with first and second points.
        if signed_area.abs() / distance > epsilon * K::RealField::from(2.0) {
            candidates.push(i);
        }
    }

    // There are no points that are not colinear to first and second point.
    if candidates.len() == 0 {
        return Ok(vec![first_point, second_point]);
    }

    // Add first point and second point as they are definitely part of the convex hull.
    candidates.push(first_point);
    candidates.push(second_point);

    todo!("Implement quickhull")
}
