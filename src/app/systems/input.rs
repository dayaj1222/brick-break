use bevy::prelude::*;

use crate::app::components::Paddle;
use crate::app::constants::PADDLE_SPEED;

pub struct Input;

impl Plugin for Input {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, input);
    }
}

fn input(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle: Single<&mut Transform, With<Paddle>>,
) {
    // Speed of the Paddle

    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        paddle.translation.x += PADDLE_SPEED * time.delta_secs();
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        paddle.translation.x -= PADDLE_SPEED * time.delta_secs();
    }
}
