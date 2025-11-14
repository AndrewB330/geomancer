mod farthest_points_impl;
use crate::{
    common::GeometryError,
    traits::{Cross2D, Dot2D, Norm2D, NormSqr2D},
};

pub fn farthest_points<K>(points: &[K::Point]) -> Result<(usize, usize), GeometryError>
where
    K: Norm2D + NormSqr2D + Dot2D + Cross2D,
    K::Point: Clone,
    K::Scalar: PartialOrd,
{
    return farthest_points_impl::farthest_points_impl::<K>(points);
}
