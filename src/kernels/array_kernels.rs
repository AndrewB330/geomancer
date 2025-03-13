use crate::traits::{DefaultKernel, FieldNumber, Point2D};

use super::GenericKernel2D;

impl<T> Point2D for [T; 2]
where
    T: FieldNumber,
{
    type Field = T;

    fn x(&self) -> Self::Field {
        self[0].clone()
    }

    fn y(&self) -> Self::Field {
        self[1].clone()
    }
}

pub type ArrayKernel2D<T> = GenericKernel2D<[T; 2]>;

impl<T> DefaultKernel for [T; 2]
where
    T: FieldNumber,
{
    type Kernel = ArrayKernel2D<T>;
}
