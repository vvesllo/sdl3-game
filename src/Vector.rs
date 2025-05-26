use std::ops;

#[derive(Clone, Copy)]
pub struct Vector2<T>
{
    pub x: T,
    pub y: T
}

impl<T: ops::Add<Output = T>> ops::Add<Vector2<T>> for Vector2<T>
{
    type Output = Vector2<T>;

    fn add(self, other: Vector2<T>) -> Self::Output {
        Vector2::<T>::new(
            self.x + other.x,
            self.y + other.y
        )
    }
}

impl<T> ops::AddAssign for Vector2<T>
where
    T: ops::AddAssign + Copy,
{
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}



impl<T> Vector2<T>
{
    pub fn new(x: T, y: T) -> Vector2<T>
    {
        Vector2 {
            x: x,
            y: y
        }
    }
}