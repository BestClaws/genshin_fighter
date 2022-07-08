/* bootstraps the game */

mod battle;
mod domain;
mod game;
mod player;
mod anim;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::game::GamePlugin;

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
