/// A trait for 2D points with x and y coordinates
pub trait Point2D: Clone {
    type Field;
    fn x(&self) -> Self::Field;
    fn y(&self) -> Self::Field;
}
