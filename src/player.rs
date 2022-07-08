use crate::battle::BattleState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("plugin read...");

        app.add_system_set(
            SystemSet::on_update(BattleState::Prepare).with_system(player_movement_sys),
        );
    }
}

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "xl.png")]
    sprite_sheet: Handle<Image>,
}

// spawn the domain bg, platforms etc.,
pub fn spawn_player_sys(commands: &mut Commands, loaded_images: Res<PlayerAssets>) {
    info!("spawning player");
    commands
        .spawn_bundle(SpriteBundle {
            texture: loaded_images.sprite_sheet.clone(),
            ..default()
        })
        .insert(Player);
}



#[derive(Component)]
pub struct Animations {}

fn player_movement_sys(
    key_input: Res<Input<KeyCode>>,
    mut player_transforms: Query<&mut Transform, With<Player>>,
) {
    for mut transform in player_transforms.iter_mut() {
        if key_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if key_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if key_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if key_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}
