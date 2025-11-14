use num_traits::Float;

use crate::traits::{DefaultKernel, Point2D};

use super::GenericKernel2D;

impl<T> Point2D for [T; 2]
where
    T: Float,
{
    type Scalar = T;

    fn x(&self) -> Self::Scalar {
        self[0].clone()
    }

    fn y(&self) -> Self::Scalar {
        self[1].clone()
    }
}

pub type ArrayKernel2D<T> = GenericKernel2D<[T; 2]>;

impl<T> DefaultKernel for [T; 2]
where
    T: Float,
{
    type Kernel = ArrayKernel2D<T>;
}
