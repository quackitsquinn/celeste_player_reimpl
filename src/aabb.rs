use crate::vector::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

impl AABB {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn from_wh(origin: Vec2, width: u16, height: u16) -> Self {
        Self {
            min: origin,
            max: origin + Vec2::new(width as i32, height as i32),
        }
    }
    /// Checks if the AABB collides with another AABB. Probably use the `&` operator instead.
    pub fn collides_with(&self, other: Self) -> bool {
        *self & other
    }
    /// Checks if a point is inside the AABB. Probably use the `&` operator instead.
    pub fn contains(&self, point: Vec2) -> bool {
        point & *self
    }
}

impl std::ops::Add<Vec2> for AABB {
    type Output = AABB;

    fn add(self, rhs: Vec2) -> AABB {
        AABB::new(self.min + rhs, self.max + rhs)
    }
}

impl std::ops::Sub<Vec2> for AABB {
    type Output = AABB;

    fn sub(self, rhs: Vec2) -> AABB {
        AABB::new(self.min - rhs, self.max - rhs)
    }
}

impl std::ops::AddAssign<Vec2> for AABB {
    fn add_assign(&mut self, rhs: Vec2) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Vec2> for AABB {
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = *self - rhs;
    }
}
// Using and operator to check if two AABBs are colliding. bit weird, but could be cool.
impl std::ops::BitAnd for AABB {
    type Output = bool;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.min.x < rhs.max.x
            && self.max.x > rhs.min.x
            && self.min.y < rhs.max.y
            && self.max.y > rhs.min.y
    }
}

impl std::ops::BitAnd<AABB> for Vec2 {
    type Output = bool;

    fn bitand(self, rhs: AABB) -> Self::Output {
        rhs.min.x < self.x && rhs.max.x > self.x && rhs.min.y < self.y && rhs.max.y > self.y
    }
}
