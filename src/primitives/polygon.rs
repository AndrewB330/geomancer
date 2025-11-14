use std::ops::Index;

use crate::traits::{Cross2D, Kernel2D, Norm2D};

use num_traits::Zero;

pub struct Polygon<K>
where
    K: Kernel2D,
{
    pub(super) points: Vec<K::Point>,
}

impl<K> Polygon<K>
where
    K: Kernel2D,
{
    pub(crate) fn from_points_unchecked(points: Vec<K::Point>) -> Self {
        Self { points }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn area(&self) -> K::Scalar
    where
        K: Cross2D,
        K::Scalar: Zero,
    {
        let mut area = K::Scalar::zero();
        let n = self.len();
        for i in 1..(n - 1) {
            area = area + K::cross_with_origin(&self[i], &self[(i + 1) % n], &self.points[0]);
        }
        area
    }

    pub fn perimeter(&self) -> K::Real
    where
        K: Norm2D,
    {
        let mut perimeter = K::Real::zero();
        let n = self.len();
        for i in 0..n {
            perimeter = perimeter + K::distance(&self[i], &self[(i + 1) % n]);
        }
        perimeter
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
