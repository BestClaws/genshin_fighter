mod game;
mod domain;
mod asset_library;

use crate::game::GamePlugin;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Genshin Fighter!".to_string(), // TODO: don't hard code values.
            width: 1280.,
            height: 720.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .run();
}



