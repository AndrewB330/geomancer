/// A trait for 2D points with x and y coordinates
pub trait Point2D: Clone {
    type Scalar;
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
}
