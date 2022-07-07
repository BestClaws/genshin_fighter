use bevy::prelude::*;
use crate::battle::BattlePlugin;


/// represents the entire game as a whole
/// manages different screens in the game like menu, battle etc.,
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BattlePlugin).add_startup_system(setup);
        info!("plugin ready...");

    }
}


fn setup() {

}


