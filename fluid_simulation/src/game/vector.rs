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

    pub fn zero() -> Vector {
        Self::new(0.0, 0.0)
    }

    pub fn one() -> Vector {
        Self::new(1.0, 1.0)
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

    pub fn ceil(&self) -> Vector {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    pub fn floor(&self) -> Vector {
        Self::new(self.x.floor(), self.y.floor())
    }

    pub fn max(&self, value: f32) -> Vector {
        Self::new(self.x.max(value), self.y.max(value))
    }

    pub fn min(&self, value: f32) -> Vector {
        Self::new(self.x.min(value), self.y.min(value))
    }

    pub fn clamp(&self, min: Vector, max: Vector) -> Vector {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }
}

impl From<(u32, u32)> for Vector {
    fn from(size: (u32, u32)) -> Self {
        Vector::new(size.0 as f32, size.1 as f32)
    }
}

impl TryInto<(usize, usize)> for Vector {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> Result<(usize, usize), Self::Error> {
        let x = self.x as usize;
        let y = self.y as usize;
        Ok((x, y))
    }
}

impl TryInto<(u32, u32)> for Vector {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> Result<(u32, u32), Self::Error> {
        let x = self.x as u32;
        let y = self.y as u32;
        Ok((x, y))
    }
}

impl TryInto<(i32, i32)> for Vector {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> Result<(i32, i32), Self::Error> {
        let x = self.x as i32;
        let y = self.y as i32;
        Ok((x, y))
    }
}

impl TryInto<(f32, f32)> for Vector {
    type Error = std::num::TryFromIntError;

    fn try_into(self) -> Result<(f32, f32), Self::Error> {
        Ok((self.x, self.y))
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
