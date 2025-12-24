use crate::app::components::{Ball, Collidable, Paddle, Velocity};
use crate::app::constants::BALL_RADIUS;
use crate::app::resources::GameStates;
use bevy::prelude::*;

pub struct CollisionSystem;

impl Plugin for CollisionSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_collisions);
    }
}

fn detect_collision(
    pos_1: &Transform,
    pos_2: &Transform,
    rect1: &Collidable,
    rect2: &Collidable,
) -> bool {
    let x_collision = (pos_1.translation.x - pos_2.translation.x).abs() < (rect1.x + rect2.x) / 2.0;
    let y_collision = (pos_1.translation.y - pos_2.translation.y).abs() < (rect1.y + rect2.y) / 2.0;
    x_collision && y_collision
}

fn handle_collisions(
    windows: Query<&Window>,
    mut ball_query: Query<(&mut Velocity, &mut Transform, &Collidable), With<Ball>>,
    paddle_query: Query<(&Transform, &Collidable), (With<Paddle>, Without<Ball>)>,
    gamestate: ResMut<GameStates>,
) {
    let mut window_height = 0.0;
    let mut window_width = 0.0;
    let window = windows.single();
    match window {
        Ok(dim) => {
            window_width = dim.width();
            window_height = dim.height();
        }
        Err(_) => println!("Error while getting window dimensions"),
    }

    for (mut velocity, mut transform, ball_collidable) in ball_query.iter_mut() {
        // Left/Right walls - reflect horizontally
        if transform.translation.x >= window_width / 2.0 - BALL_RADIUS {
            velocity.0.x = -velocity.0.x.abs();
            transform.translation.x = window_width / 2.0 - BALL_RADIUS;
        }
        if transform.translation.x <= -window_width / 2.0 + BALL_RADIUS {
            velocity.0.x = velocity.0.x.abs();
            transform.translation.x = -window_width / 2.0 + BALL_RADIUS;
        }

        // Top wall only - reflect down
        if transform.translation.y >= window_height / 2.0 - BALL_RADIUS {
            velocity.0.y = -velocity.0.y.abs();
            transform.translation.y = window_height / 2.0 - BALL_RADIUS;
        }
        if transform.translation.y <= -window_height / 2.0 + BALL_RADIUS {}

        // Paddle collision
        for (paddle_transform, paddle_collidable) in paddle_query.iter() {
            if detect_collision(
                &transform,
                paddle_transform,
                ball_collidable,
                paddle_collidable,
            ) {
                velocity.0.y = velocity.0.y.abs();
            }
        }
    }
}
