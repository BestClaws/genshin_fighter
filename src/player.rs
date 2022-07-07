use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct PlayerPlugin;

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "xl.png")]
    sprite_sheet: Handle<Image>,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {

        info!("plugin read...");
    }
}

// spawn the domain bg, platforms etc.,
pub fn spawn_player_sys(
    commands: &mut Commands,
    loaded_images: Res<PlayerAssets>,
) {


    commands.spawn_bundle(SpriteBundle {
        texture: loaded_images.sprite_sheet.clone(),
        ..default()
    });
}


