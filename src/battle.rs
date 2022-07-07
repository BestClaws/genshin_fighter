use crate::domain::{spawn_domain_sys, DomainPlugin};
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::domain::DomainAssets;
use crate::player::{spawn_player_sys, PlayerAssets};

/// represents the stage where the battle happens
pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        // don't load battle screen until game says so.
        app.add_state(BattleState::Unloaded);

        // setup asset loader
        AssetLoader::new(BattleState::Loading)
            .continue_to_state(BattleState::Prepare)
            .with_collection::<DomainAssets>()
            .with_collection::<PlayerAssets>()
            .build(app);

        // app.add_plugin(DomainPlugin).add_startup_system(setup);

        app.add_system_set(
            SystemSet::on_enter(BattleState::Loading).with_system(beg_load_screen_sys),
        );
        app.add_system_set(
            SystemSet::on_exit(BattleState::Loading).with_system(end_load_screen_sys),
        );
        app.add_system_set(SystemSet::on_enter(BattleState::Prepare).with_system(spawn_all));

        info!("plugin ready...");
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum BattleState {
    Unloaded,
    Loading,
    Prepare,
    Battle,
    End,
}

fn setup() {}

fn show_loading_system() {
    info!("now loading assets");
}

fn beg_load_screen_sys(mut commands: Commands) {
    info!("showing loading screen");

    // todo: this should be a ui camera instead, also despawn these components when done loading.
    // spawn the camera.
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(LoadingScreenItem);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(10., 10., 10.)),
            ..default()
        })
        .insert(LoadingScreenItem);
}

fn end_load_screen_sys(mut commands: Commands, query: Query<Entity, With<LoadingScreenItem>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }
}

fn spawn_all(
    mut commands: Commands,
    windows: Res<Windows>,
    domain_assets: Res<DomainAssets>,
    player_assets: Res<PlayerAssets>,
    image_assets: Res<Assets<Image>>,
) {
    info!("loading finished, spawing entities");

    // spawn the camera.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // spawn the domain
    spawn_domain_sys(&mut commands, windows, domain_assets, image_assets);
    // spawn the player
    spawn_player_sys(&mut commands, player_assets);
}

#[derive(Component)]
struct LoadingScreenItem;
