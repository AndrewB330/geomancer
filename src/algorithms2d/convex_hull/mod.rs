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

#[cfg(test)]
mod test {
    use core::f32;

    use crate::traits::{DefaultKernel, Kernel2D, Operations2D, Point2D, RealOperations2D};

    use super::convex_hull;

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
        let hull = convex_hull(points);
        assert!(hull.is_ok());

        let hull = hull.unwrap();
        let hull_indices = hull.hull_indices();

        assert_eq!(hull_indices.len(), expected.len());

        if expected.len() == 1 {
            return;
        }

        let mut offset = None;

        for i in 0..hull_indices.len() {
            if hull_indices[i] == expected[0] {
                offset = Some(i);
                break;
            }
        }

        assert!(
            offset.is_some(),
            "First point of expected convex hull not found."
        );
        let offset = offset.unwrap();

        let offseted: Vec<usize> = hull_indices[offset..]
            .iter()
            .chain(hull_indices[..offset].iter())
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
        assert_convex_hull_f32(&[(0.1, 0.3), (0.01, 0.03), (1., 3.), (0.3, 0.9)], &[0, 2]);
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
}
