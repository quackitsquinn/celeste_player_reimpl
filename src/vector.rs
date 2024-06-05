use raylib::math::Vector2;

use crate::aabb::AABB;

#[derive(Debug, Clone, Copy, PartialEq, Eq)] // i32s are Eq
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    /// The zero vector.
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };
    /// The unit vector in the x direction. (1, 0)
    pub const UNIT_X: Vec2 = Vec2 { x: 1, y: 0 };
    /// The unit vector in the y direction. (0, 1)
    pub const UNIT_Y: Vec2 = Vec2 { x: 0, y: 1 };

    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn is_in(&self, bounds: AABB) -> bool {
        *self & bounds
    }
}
// convert Vec2 to Vector2. If the 2 types had the same number type, I would of just did a `mem::transmute` but they don't.
// Interpreting a float as an int is.. not good.
impl Into<Vector2> for Vec2 {
    fn into(self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::vector::Vec2::new($x, $y)
    };
}
#[macro_export]
macro_rules! x {
    ($x:expr) => {
        $crate::vector::Vec2::new($x, 0)
    };
}

#[macro_export]
macro_rules! y {
    ($y:expr) => {
        $crate::vector::Vec2::new(0, $y)
    };
}

// unrelated, but there should be a derive macro for the standard ops that would only work for types that implement the ops

// region: Operation implementations

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl std::ops::Div for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}

impl std::ops::MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Vec2) {
        *self = *self * rhs;
    }
}

impl std::ops::DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Vec2) {
        *self = *self / rhs;
    }
}

// endregion: std::ops
