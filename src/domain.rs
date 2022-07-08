use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};





pub struct DomainPlugin;

impl Plugin for DomainPlugin {
    fn build(&self, app: &mut App) {
        info!("plugin ready...");
    }
}



#[derive(AssetCollection)]
pub struct DomainAssets {
    #[asset(path = "zhou.png")]
    bg: Handle<Image>,
}


// spawn the domain bg, platforms etc.,
pub fn spawn_domain_sys(
    commands: &mut Commands,
    windows: Res<Windows>,
    loaded_images: Res<DomainAssets>,
    image_assets: Res<Assets<Image>>,
) {

    // load bg image
    // only image for now. (future: image > animated image > dynamic sfx maybe?)

    // get the first(only) game window (for dimensions).
    // todo: dont use unwrap
    let win = windows.get_primary().unwrap();

    // get the bg_size
    let bg_size = image_assets.get(&loaded_images.bg).unwrap().size();
    info!("Got bg size: {:?}", bg_size);

    // find bg scale factor
    // todo: (surely the logic isn't right. fix this later)
    let bg_scale_factor = if win.width() > win.height() {
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



