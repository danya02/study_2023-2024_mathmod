use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use crate::Num;

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, Default)]
pub struct Vec2 {
    pub x: Num,
    pub y: Num,
}

impl Vec2 {
    pub fn new(x: Num, y: Num) -> Self {
        Vec2 { x, y }
    }

    pub fn magnitude_squared(&self) -> Num {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn magnitude(&self) -> Num {
        Num::sqrt(self.magnitude_squared())
    }
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        debug_assert_ne!(mag, 0.0);
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Num> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Num) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div<Num> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Num) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
