use bevy::prelude::*;
use crate::domain::DomainPlugin;

/// represents the stage where the battle happens
pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DomainPlugin).add_startup_system(setup);
        info!("plugin ready...");
    }
}

enum BattleState {
    Loading,
    BattlePrepare,
    Battle,
    BattleEnd,


}

fn setup() {

}


