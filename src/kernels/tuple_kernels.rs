use num_traits::Float;

use crate::traits::{DefaultKernel, Point2D};

use super::GenericKernel2D;

impl<T> Point2D for (T, T)
where
    T: Float,
{
    type Scalar = T;

    fn x(&self) -> Self::Scalar {
        self.0.clone()
    }

    fn y(&self) -> Self::Scalar {
        self.1.clone()
    }
}

pub type TupleKernel2D<T> = GenericKernel2D<(T, T)>;

impl<T> DefaultKernel for (T, T)
where
    T: Float,
{
    type Kernel = TupleKernel2D<T>;
}
