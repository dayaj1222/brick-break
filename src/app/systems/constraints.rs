use bevy::prelude::*;

use crate::app::components::{Moveable, Paddle, Velocity};
use crate::app::constants::{PADDLE_HEIGHT, PADDLE_WIDTH};

pub struct Constraints;

impl Plugin for Constraints {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_boundaries);
    }
}

fn check_boundaries(
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Paddle>>,
) {
    let mut window_height = 0.0;
    let mut window_width = 0.0;

    let window = windows.single();
    match window {
        Ok(dim) => {
            window_width = dim.width();
            window_height = dim.height();
        }
        Err(_) => println!("Error while getting window dimentions"),
    }

    for (mut transform, mut velocity) in query.iter_mut() {
        if transform.translation.x >= window_width / 2.0 - PADDLE_WIDTH / 2.0 {
            velocity.0 = Vec3::ZERO;
            transform.translation.x = window_width / 2.0 - PADDLE_WIDTH / 2.0;
        }
        if transform.translation.x <= -window_width / 2.0 + PADDLE_WIDTH / 2.0 {
            velocity.0 = Vec3::ZERO;
            transform.translation.x = -window_width / 2.0 + PADDLE_WIDTH / 2.0;
        }
        if transform.translation.y >= window_height / 2.0 - PADDLE_HEIGHT / 2.0 {
            velocity.0 = Vec3::ZERO;
            transform.translation.y = window_height / 2.0 - PADDLE_HEIGHT / 2.0;
        }
        if transform.translation.y <= -window_height / 2.0 + PADDLE_HEIGHT / 2.0 {
            velocity.0 = Vec3::ZERO;
            transform.translation.y = -window_height / 2.0 + PADDLE_HEIGHT / 2.0;
        }
    }
}
