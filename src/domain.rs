use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

/// represents the stage where the battle happens
pub struct DomainPlugin;

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "zhou.png")]
    bg: Handle<Image>,
}

struct AssetsLoading(Vec<HandleUntyped>);

impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {
        info!("DomainPlugin Build...");

        // setup asset loader
        AssetLoader::new(DomainState::Loading)
            .continue_to_state(DomainState::Loaded)
            .with_collection::<ImageAssets>()
            .build(app);
        // todo: stay unloaded, game should set this state to loading instead.
        app.add_state(DomainState::Loading)
            .add_system_set(SystemSet::on_enter(DomainState::Loaded).with_system(setup_system));
    }
}

fn setup_system(
    mut commands: Commands,
    windows: Res<Windows>,
    loaded_images: Res<ImageAssets>,
    image_assets: Res<Assets<Image>>,
) {
    // spawn the camera.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // load bg image
    // only image for now. (future: image > animated image > dynamic sfx maybe?)
    let mut bg_scale_factor = 1.;

    // get the first(only) game window (for dimensions).
    // todo: dont use unwrap
    let win = windows.get_primary().unwrap();

    // get the bg_size
    let bg_size = image_assets.get(&loaded_images.bg).unwrap().size();
    info!("Got bg size: {:?}", bg_size);


    // find bg scale factor
    // todo: (surely the logic isnt right. fix this later)
    bg_scale_factor = if win.width() > win.height() {
        info!("chose to scale height");
        win.height() / bg_size[1]
    } else {
        info!("chose to scale width");
        win.width() / bg_size[0]
    };

    info!("bg scale: {:#?}", bg_scale_factor);

    info!("paining the domain (bg).");
    commands.spawn_bundle(SpriteBundle {
        texture: loaded_images.bg.clone(),
        transform: Transform::from_scale(Vec3::splat(bg_scale_factor)),
        ..default()
    });
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
enum DomainState {
    Unloaded,
    Loading,
    Loaded,
}

