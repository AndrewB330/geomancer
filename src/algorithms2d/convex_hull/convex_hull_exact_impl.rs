use crate::{
    common::GeometryError,
    traits::{ExactPredicates2D, Kernel2D},
};

pub(super) fn convex_hull_exact_impl<K>(
    _: &[K::Point],
    _: bool,
) -> Result<Vec<usize>, GeometryError>
where
    K: Kernel2D + ExactPredicates2D,
{
    todo!();
}
