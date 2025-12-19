use bevy::prelude::*;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct Collidable {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Moveable;

#[derive(Component)]
pub struct Velocity(pub Vec3);
