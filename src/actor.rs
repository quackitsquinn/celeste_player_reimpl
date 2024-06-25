use ggez::{
    graphics::{Canvas, Color},
    Context,
};

use crate::{
    aabb::{Collision, AABB},
    vector::Vec2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Actor {
    pos: Vec2,
    hitbox: AABB,
}

impl Actor {
    pub fn new(pos: Vec2, hitbox: AABB) -> Self {
        Self { pos, hitbox }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) {
        self.hitbox.draw(ctx, canvas, Color::WHITE);
    }

    pub fn set_position(&mut self, pos: Vec2) {
        self.pos = pos;
        self.hitbox.set_positon(pos);
    }

    pub fn get_position(&self) -> Vec2 {
        self.pos
    }

    pub fn get_hitbox(&self) -> AABB {
        self.hitbox
    }

    pub fn set_hitbox(&mut self, hitbox: AABB) {
        self.hitbox = hitbox;
    }

    pub fn movex(&mut self, x: i32) {

    }
}
