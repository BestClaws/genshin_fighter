use crate::anim::Animation;
use crate::battle::BattleState;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use std::borrow::Borrow;
use std::collections::HashMap;
use crate::player::Action::{Attack, Idle};

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        info!("plugin read...");

        app.add_system_set(
            SystemSet::on_update(BattleState::Prepare)
                .with_system(player_action_sys)
                .with_system(animate_player_sys),
        );
    }
}

#[derive(AssetCollection)]
pub struct PlayerAssets {
    #[asset(path = "xl.png")]
    sprite_sheet: Handle<Image>,
}

pub fn spawn_player(
    commands: &mut Commands,
    player_assets: Res<PlayerAssets>,
    mut atlas_assets: ResMut<Assets<TextureAtlas>>,
) {
    info!("spawning player");
    let texture_atlas = TextureAtlas::from_grid(
        player_assets.sprite_sheet.clone(),
        Vec2::new(514.0, 663.0),
        5,
        1,
    );
    let texture_atlas_handle = atlas_assets.add(texture_atlas);


    let mut anims = HashMap::new();
    anims.insert(Attack, Animation::from(2, vec![50, 50, 50]));
    anims.insert(Idle, Animation::from(0, vec![200, 200]));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(Player)
        .insert(Action::Idle)
        .insert(ActionAnimations(anims))
        .insert(LastAction(Action::Idle));
}

#[derive(Component)]
struct ActionAnimations(HashMap<Action, Animation>);

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum Action {
    Attack,
    Idle,
}

#[derive(Component)]
struct LastAction(Action);





// move as in action, not movement.
fn player_action_sys(
    key_input: Res<Input<KeyCode>>,
    mut actions: Query<&mut Action, With<Player>>,
) {



    let mut action = actions.iter_mut().next().unwrap();

    if key_input.pressed(KeyCode::A) {

        *action = Action::Attack;
    }

    // idle
    if key_input.pressed(KeyCode::I) {

        *action = Action::Idle;
    }
}

fn animate_player_sys(
    mut query: Query<(&mut Action, &mut LastAction, &mut ActionAnimations, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
) {

    let delta = time.delta();
    let (mut action, mut last_action, mut action_anims, mut sprite) = query.iter_mut().next().unwrap();


    if last_action.0 != *action {
        let anim = action_anims.0.get_mut(&last_action.0).unwrap();
        anim.reset();
    }

    let anim = action_anims.0.get_mut(&action).unwrap();
    info!("advance {} before for action {:?}: {:?}",delta.as_millis() as u32, *action, anim.current_frame);
    anim.advance(delta.as_millis() as u32);
    sprite.index = anim.current_frame;


    last_action.0 = *action;


}
