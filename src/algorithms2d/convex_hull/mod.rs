mod convex_hull_exact_impl;
mod convex_hull_impl;

use std::borrow::Cow;

use crate::{
    common::GeometryError,
    traits::{DefaultKernel, ExactPredicates2D, Kernel2D, Operations2D, RealOperations2D},
};
use convex_hull_exact_impl::convex_hull_exact_impl;
use convex_hull_impl::convex_hull_impl;

pub struct ConvexHull2D<'a, K: Kernel2D>
where
    K::Point: Clone,
{
    hull_indices: Vec<usize>,
    points: Cow<'a, [K::Point]>,
}

pub fn convex_hull<'a, V>(points: &'a [V]) -> Result<ConvexHull2D<'a, V::Kernel>, GeometryError>
where
    V: DefaultKernel + Clone,
    V::Kernel: Operations2D + RealOperations2D,
{
    ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points(points)
}

pub fn convex_hull_exact<'a, V>(
    points: &'a [V],
    include_collinear: bool,
) -> Result<ConvexHull2D<'a, V::Kernel>, GeometryError>
where
    V: DefaultKernel + Clone,
    V::Kernel: ExactPredicates2D,
{
    ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points_exact(points, include_collinear)
}

impl<'a, K> ConvexHull2D<'a, K>
where
    K: Kernel2D + Operations2D + RealOperations2D,
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

impl<'a, K> ConvexHull2D<'a, K>
where
    K: Kernel2D + ExactPredicates2D,
    K::Point: Clone,
{
    pub fn from_points_owned_exact(
        points: impl IntoIterator<Item = K::Point>,
        include_collinear: bool,
    ) -> Result<Self, GeometryError> {
        let points_owned = points.into_iter().collect::<Vec<_>>();
        Ok(Self {
            hull_indices: convex_hull_exact_impl::<K>(&points_owned, include_collinear)?,
            points: Cow::Owned(points_owned),
        })
    }

    pub fn from_points_exact(
        points: &'a [K::Point],
        include_collinear: bool,
    ) -> Result<Self, GeometryError> {
        Ok(Self {
            hull_indices: convex_hull_exact_impl::<K>(points.as_ref(), include_collinear)?,
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
