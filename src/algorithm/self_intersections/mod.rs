use crate::kernel::{Cross2D, ExactOrientation2D, Kernel2D};

pub trait SelfIntersectionsAlgo<K: Kernel2D> {
    fn has_self_intersections(kernel: &K, points: &[K::Point]) -> bool;
}

// 2. Provide the "Fast" Implementation
// This logic lives in your library. The user never touches it.
pub struct FastIntersectionAlgo;

impl<K> SelfIntersectionsAlgo<K> for FastIntersectionAlgo
where
    K: Cross2D, // REQUIREMENT: Kernel must support Cross product
{
    fn has_self_intersections(kernel: &K, points: &[K::Point]) -> bool {
        // Your implementation using epsilons and simple math
        // e.g. Bentley-Ottmann or naive N^2 check using kernel.cross()
        false 
    }
}

// 3. Provide the "Exact" Implementation
// This logic also lives in your library.
pub struct ExactIntersectionAlgo;

impl<K> SelfIntersectionsAlgo<K> for ExactIntersectionAlgo
where
    K: ExactOrientation2D, // REQUIREMENT: Kernel must support Exact Orientation
{
    fn has_self_intersections(kernel: &K, points: &[K::Point]) -> bool {
        // Your robust implementation 
        // e.g. using exact predicates
        true
    }
}