use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt) => {
        impl $trait for Vector {
            type Output = Self;

            fn $func(self, other: Self) -> Self {
                Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                }
            }
        }

        impl $trait<f32> for Vector {
            type Output = Self;

            fn $func(self, other: f32) -> Self {
                Self {
                    x: self.x $op other,
                    y: self.y $op other,
                }
            }
        }
    };
}

macro_rules! impl_op_assign {
    ($trait:ident, $func:ident, $op:tt) => {
        impl $trait for Vector {
            fn $func(&mut self, other: Self) {
                *self = Self {
                    x: self.x $op other.x,
                    y: self.y $op other.y,
                };
            }
        }

        impl $trait<f32> for Vector {
            fn $func(&mut self, other: f32) {
                *self = Self {
                    x: self.x $op other,
                    y: self.y $op other,
                };
            }
        }
    };
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector { x, y }
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn as_tuple_i32(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    pub fn zero() -> Vector {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Vector {
        Vector { x: 1.0, y: 1.0 }
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y
    }

    pub fn dot(&self, other: Vector) -> f32 {
        (*self * other).sum()
    }

    pub fn magnitude(&self) -> f32 {
        self.dot(*self).sqrt()
    }
    pub fn normalize(&self) -> Vector {
        let magnitude = self.magnitude();
        if magnitude == 0.0 {
            return Vector::zero();
        } else {
            return Vector::new(self.x / magnitude, self.y / magnitude);
        }
    }
}

impl_op!(Add, add, +);
impl_op_assign!(AddAssign, add_assign, +);
impl_op!(Sub, sub, -);
impl_op_assign!(SubAssign, sub_assign, -);
impl_op!(Mul, mul, *);
impl_op_assign!(MulAssign, mul_assign, *);
impl_op!(Div, div , /);
impl_op_assign!(DivAssign, div_assign, /);
