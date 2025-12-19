use app::systems::system::System;
use bevy::prelude::*;

use crate::app::{resources::Score, states::GameStates};

pub mod app;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_resource(State::<GameStates>::new(GameStates::MainMenu))
        .add_plugins(System)
        .run();
}
