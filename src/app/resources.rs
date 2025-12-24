use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Resource)]
pub enum GameStates {
    MainMenu,
    Playing,
    Paused,
}
