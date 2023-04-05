use num::Num;
use std::ops::Add;

mod tests;

pub struct Triangle<T>([T; 3]);

impl<T> Triangle<T>
where
    T: Num + PartialOrd + Add + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        return if sides.iter().all(|&s| s > T::zero())
            && sides[0] + sides[1] >= sides[2]
            && sides[0] + sides[2] >= sides[1]
            && sides[1] + sides[2] >= sides[0]
        {
            Some(Triangle(sides))
        } else {
            None
        };
    }

    pub fn is_equilateral(&self) -> bool {
        self.0.iter().all(|s| s == self.0.first().unwrap())
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.is_equilateral()
            || self.0[0] == self.0[1]
            || self.0[0] == self.0[2]
            || self.0[1] == self.0[2]
    }
}
