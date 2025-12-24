use app::systems::system::System;
use bevy::prelude::*;

use crate::app::{resources::GameStates, resources::Score};

pub mod app;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Score(0))
        .insert_state(GameStates::MainMenu)
        .add_plugins(System)
        .run();
}
