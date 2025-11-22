//mod convex_hull;
//mod farthest_points;
//pub use convex_hull::{convex_hull, convex_hull_exact, ConvexHull2D};
//pub use farthest_points::farthest_points;

mod self_intersections;
pub use self_intersections::*;

use crate::kernel::{Cross2D, ExactOrientation2D, Kernel2D};

pub trait AlgorithmBundle<K: Kernel2D> {
    type SelfIntersection: SelfIntersectionsAlgo<K>;
}

pub struct DefaultAlgorithmBundle;

impl<K: Kernel2D + Cross2D> AlgorithmBundle<K> for DefaultAlgorithmBundle {
    type SelfIntersection = FastIntersectionAlgo;
}

pub struct DefaultExactAlgorithmBundle;

impl<K: Kernel2D + ExactOrientation2D> AlgorithmBundle<K> for DefaultExactAlgorithmBundle {
    type SelfIntersection = ExactIntersectionAlgo;
}
