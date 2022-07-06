use bevy::prelude::*;
use crate::domain::DomainPlugin;

/// represents the stage where the battle happens
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        info!("GamePlugin build...");
        app.add_plugin(DomainPlugin);

    }
}


