use app::systems::{gameplay::system::System, menu::menu::Menu};
use bevy::prelude::*;

use crate::app::{
    resources::{GameStates, Score},
    systems::{over::game_over::GameOver, won::winning_screen::GameWon},
};

pub mod app;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_state(GameStates::MainMenu)
        .insert_resource(Score(0))
        .add_plugins(System)
        .add_plugins(GameOver)
        .add_plugins(GameWon)
        .add_plugins(Menu)
        .run();
}

fn setup(mut commands: Commands) {
    // Cameras
    commands.spawn((Camera2d, IsDefaultUiCamera));
}
