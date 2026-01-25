use bevy::prelude::*;

#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States, Resource)]
pub enum GameStates {
    #[default]
    MainMenu,
    Playing,
    Over,
    Won,
}
