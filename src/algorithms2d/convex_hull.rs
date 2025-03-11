use std::{borrow::Cow, cmp::Ordering};

use crate::{
    common::GeometryError,
    traits::{DefaultKernel, FieldNumber, Kernel2D, Point2D},
};
use num_traits::Zero;

pub struct ConvexHull2D<'a, K: Kernel2D> {
    hull_indices: Vec<usize>,
    points: Cow<'a, [K::Point]>,
}

pub fn convex_hull<V>(points: &[V]) -> Result<Vec<V>, GeometryError>
where
    V: DefaultKernel + Clone,
    <V as DefaultKernel>::Kernel: Kernel2D<Point = V>,
{
    Ok(
        ConvexHull2D::<<V as DefaultKernel>::Kernel>::from_points(points)?
            .hull_points()
            .cloned()
            .collect(),
    )
}

impl<'a, K: Kernel2D> ConvexHull2D<'a, K> {
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

    pub fn perimeter(&self) -> K::Real {
        let mut result = K::Real::zero();
        for i in 0..self.hull_indices.len() {
            let ai = self.hull_indices[i];
            let bi = self.hull_indices[(i + 1) % self.hull_indices.len()];
            result = result + K::distance(&self.points[ai], &self.points[bi]);
        }
        result
    }

    pub fn area(&self) -> K::Real {
        let mut result = K::Real::zero();
        let start = self.hull_indices[0];
        for i in 1..self.hull_indices.len() - 1 {
            let ai = self.hull_indices[i];
            let bi = self.hull_indices[(i + 1) % self.hull_indices.len()];
            result =
                result + K::signed_area(&self.points[start], &self.points[ai], &self.points[bi]);
        }
        result
    }
}

fn farthest_point<K: Kernel2D>(
    points: &[K::Point],
    from_point: &K::Point,
) -> usize {
    let mut farthest_point = 0;
    for i in 1..points.len() {
        if K::is_closer(&points[farthest_point], &points[i], from_point) == Ordering::Less {
            farthest_point = i;
        }
    }
    farthest_point
}

fn convex_hull_impl<K: Kernel2D>(points: &[K::Point]) -> Result<Vec<usize>, GeometryError> {
    for p in points {
        if !p.x().is_valid() {
            return Err(GeometryError::InputValueInvalidForField);
        }
    }

    // todo: add short comment why this if is there
    if points.len() <= 1 {
        return Ok((0..points.len()).collect());
    }

    let first_point = farthest_point::<K>(points, &K::point(K::Field::zero(), K::Field::zero()));
    let _ = farthest_point::<K>(points, &points[first_point]);

    todo!()
}

#[cfg(test)]
mod test {
    use crate::traits::{Kernel2D, Point2D, DefaultKernel};

    use super::convex_hull;

    impl Point2D for (f32, f32) {
        type Field=f32;
    
        fn x(&self) -> Self::Field {
            self.0
        }
    
        fn y(&self) -> Self::Field {
            self.1
        }
    }

    pub struct DefaultKernelF32;

    impl Kernel2D for DefaultKernelF32 {
        type Field=f32;
    
        type Real=f32;
    
        type Point=(f32, f32);
    
        fn point(x: Self::Field, y: Self::Field) -> Self::Point {
            (x, y)
        }
        
        fn orientation(a: &Self::Point, b: &Self::Point, c: &Self::Point) -> crate::prelude::Orientation2D {
            todo!()
        }
    }

    impl DefaultKernel for (f32, f32) {
        type Kernel=DefaultKernelF32;
    }

    #[test]
    fn simple_triangle() {
        let triangle = [
            (0.1f32, 0.5f32),
            (4.3, -1.6),
            (10.5, 1.1),
            (6.0, -5.2),
        ];
        let convex_hull = convex_hull(&triangle).unwrap();
        assert_eq!(convex_hull.len(), 3);
    }
}