use bevy::prelude::*;

use crate::app::systems::gameplay::{
    collision::CollisionSystem, constraints::Constraints, input::Input, setup::Setup,
    update::UpdateEntities,
};

pub struct System;

impl Plugin for System {
    fn build(&self, app: &mut App) {
        app.add_plugins(Setup);
        app.add_plugins(Constraints);
        app.add_plugins(Input);
        app.add_plugins(UpdateEntities);
        app.add_plugins(CollisionSystem);
    }
}
