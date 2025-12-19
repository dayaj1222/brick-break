use bevy::prelude::*;

use crate::app::components::{Ball, Velocity};

pub struct UpdateEntities;

impl Plugin for UpdateEntities {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

fn update(mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}
