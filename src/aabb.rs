use ggez::{
    graphics::{
        Canvas, Color, DrawMode, DrawParam, Drawable, GraphicsContext, Mesh, MeshBuilder, Rect,
    },
    Context,
};

use crate::vector::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Collision {
    None,
    Vertical,
    Horizontal,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AABB {
    min: Vec2,
    max: Vec2,
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
    pub fn collides_with(&self, other: Self) -> Collision {
        *self & other
    }
    /// Checks if a point is inside the AABB. Probably use the `&` operator instead.
    pub fn contains(&self, point: Vec2) -> bool {
        point & *self
    }

    pub fn set_positon(&mut self, pos: Vec2) {
        let size = self.max - self.min;
        self.min = pos;
        self.max = pos + size;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, color: Color) {
        let mut rect = MeshBuilder::new();
        rect.rectangle(
            DrawMode::stroke(1.0),
            Rect::new(
                self.min.x as f32,
                self.min.y as f32,
                (self.max.x - self.min.x) as f32,
                (self.max.y - self.min.y) as f32,
            ),
            color,
        )
        .expect("Failed to draw AABB");

        let mesh = Mesh::from_data(ctx, rect.build());

        mesh.draw(canvas, DrawParam::default());
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
    type Output = Collision;

    fn bitand(self, rhs: Self) -> Self::Output {
        let x_overlap = self.min.x < rhs.max.x && self.max.x > rhs.min.x;
        let y_overlap = self.min.y < rhs.max.y && self.max.y > rhs.min.y;

        if x_overlap && y_overlap {
            let x_overlap = (self.max.x - rhs.min.x)
                .abs()
                .min((rhs.max.x - self.min.x).abs());
            let y_overlap = (self.max.y - rhs.min.y)
                .abs()
                .min((rhs.max.y - self.min.y).abs());

            if x_overlap < y_overlap {
                Collision::Horizontal
            } else {
                Collision::Vertical
            }
        } else {
            Collision::None
        }
    }
}

impl std::ops::BitAnd<AABB> for Vec2 {
    type Output = bool;

    fn bitand(self, rhs: AABB) -> Self::Output {
        rhs.min.x < self.x && rhs.max.x > self.x && rhs.min.y < self.y && rhs.max.y > self.y
    }
}
