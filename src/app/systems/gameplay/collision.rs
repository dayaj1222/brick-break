use crate::app::components::{Ball, Collidable, Paddle, Velocity};
use crate::app::constants::BALL_RADIUS;
use crate::app::resources::{GameStates, Score};
use bevy::prelude::*;
pub struct CollisionSystem;
impl Plugin for CollisionSystem {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_collisions.run_if(in_state(GameStates::Playing)),
        );
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
    mut commands: Commands,
    windows: Query<&Window>,
    mut ball_query: Query<(&mut Velocity, &mut Transform, &Collidable), With<Ball>>,
    paddle_query: Query<(&Transform, &Collidable), (With<Paddle>, Without<Ball>)>,
    block_query: Query<(Entity, &Transform, &Collidable), (Without<Ball>, Without<Paddle>)>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut score: ResMut<Score>,
    asset_server: Res<AssetServer>,
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
        // Left/Right walls
        if transform.translation.x >= window_width / 2.0 - BALL_RADIUS {
            velocity.0.x = -velocity.0.x.abs();
            play_sound(&mut commands, &asset_server);
            transform.translation.x = window_width / 2.0 - BALL_RADIUS;
        }
        if transform.translation.x <= -window_width / 2.0 + BALL_RADIUS {
            velocity.0.x = velocity.0.x.abs();
            play_sound(&mut commands, &asset_server);
            transform.translation.x = -window_width / 2.0 + BALL_RADIUS;
        }
        // Top wall only
        if transform.translation.y >= window_height / 2.0 - BALL_RADIUS {
            velocity.0.y = -velocity.0.y.abs();
            play_sound(&mut commands, &asset_server);
            transform.translation.y = window_height / 2.0 - BALL_RADIUS;
        }
        //Bottom wall
        if transform.translation.y <= -window_height / 2.0 - BALL_RADIUS {
            next_state.set(GameStates::Over);
        }
        // Paddle collision
        for (paddle_transform, paddle_collidable) in paddle_query.iter() {
            if detect_collision(
                &transform,
                paddle_transform,
                ball_collidable,
                paddle_collidable,
            ) {
                velocity.0.y = velocity.0.y.abs();
                play_sound(&mut commands, &asset_server);
            }
        }
        // Block collisions
        for (block_entity, block_transform, block_collidable) in block_query.iter() {
            if detect_collision(
                &transform,
                block_transform,
                ball_collidable,
                block_collidable,
            ) {
                velocity.0.y = -velocity.0.y;
                score.0 += 1;
                play_sound(&mut commands, &asset_server);
                commands.entity(block_entity).despawn();

                // Check if this was the last block
                if block_query.iter().count() == 1 {
                    next_state.set(GameStates::Won);
                }
            }
        }
    }
}
fn play_sound(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(AudioPlayer::new(asset_server.load("audio.ogg")));
}
