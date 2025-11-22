use std::{alloc::System, ops::Index};

use crate::{
    algorithm::AlgorithmBundle, common::GeometryError, kernel::{Cross2D, Kernel2D, Norm2D, StandardKernel2D, Vec2D}
};
use crate::algorithm::SelfIntersectionsAlgo;
use num_traits::Zero;

pub struct Polygon<K = StandardKernel2D>
where
    K: Kernel2D,
{
    pub(super) points: Vec<K::Point>,
    pub(super) kernel: K,
}

impl Polygon {
    pub fn new(points: Vec<Vec2D>) -> Result<Self, GeometryError> {
        Self::with_kernel(points, StandardKernel2D)
    }
}

impl<K> Polygon<K>
where
    K: Kernel2D,
{
    pub fn with_kernel(mut points: Vec<K::Point>, kernel: K) -> Result<Self, GeometryError>
    where
        K::Point: PartialEq,
    {
        if points.is_empty() {
            return Err(GeometryError::InputIsEmpty);
        }
        points.dedup();
        Ok(Self { points, kernel })
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> K::Real
    where
        K: Norm2D,
    {
        let mut perimeter = K::Real::zero();
        let n = self.len();
        for i in 0..n {
            perimeter = perimeter + self.kernel.distance(&self[i], &self[(i + 1) % n]);
        }
        perimeter
    }

    pub fn has_self_intersections(&self) -> bool
    {
        <K::Algorithms as AlgorithmBundle<K>>::SelfIntersection::has_self_intersections(&self.kernel, &self.points)
    }
}

impl<K> Index<usize> for Polygon<K>
where
    K: Kernel2D,
{
    type Output = K::Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.points[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn empty_input() {
        let res = Polygon::new(vec![]);
        assert!(res.is_err());
    }

    #[rstest]
    fn dupe_input() {
        let res = Polygon::new(vec![Vec2D::new(0.0, 0.0), Vec2D::new(0.0, 0.0)]);
        assert!(res.is_ok());
        assert!(res.unwrap().len() == 1);
    }

    #[rstest]
    fn perimeter() {
        let res = Polygon::new(vec![
            Vec2D::new(0.0, 0.0),
            Vec2D::new(0.0, 1.0),
            Vec2D::new(1.0, 1.0),
            Vec2D::new(1.0, 0.0),
        ]);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().perimeter(), 4.0);
    }
}
