use std::ops::Index;

use crate::traits::{Kernel2D, Operations2D, RealOperations2D};

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

    pub fn area(&self) -> K::Field
    where
        K: Operations2D,
    {
        let mut area = K::Field::zero();
        let n = self.len();
        for i in 1..(n - 1) {
            area = area + K::cross_with_origin(&self[i], &self[(i + 1) % n], &self.points[0]);
        }
        area
    }

    pub fn perimeter(&self) -> K::RealField
    where
        K: RealOperations2D,
    {
        let mut perimeter = K::RealField::zero();
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
