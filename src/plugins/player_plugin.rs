use crate::prelude::*;

use crate::systems::player_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, player_controller::spawn);
        app.add_systems(Update,player_controller::handle_player_movement);
    }
}
