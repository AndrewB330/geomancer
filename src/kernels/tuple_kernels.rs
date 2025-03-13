use crate::traits::{DefaultKernel, FieldNumber, Point2D};

use super::GenericKernel2D;

impl<T> Point2D for (T, T)
where
    T: FieldNumber,
{
    type Field = T;

    fn x(&self) -> Self::Field {
        self.0.clone()
    }

    fn y(&self) -> Self::Field {
        self.1.clone()
    }
}

pub type TupleKernel2D<T> = GenericKernel2D<(T, T)>;

impl<T> DefaultKernel for (T, T)
where
    T: FieldNumber,
{
    type Kernel = TupleKernel2D<T>;
}
