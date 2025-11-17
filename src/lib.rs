// mod components;
mod plugins;
pub mod prelude;
mod systems;

use crate::plugins::player_plugin::PlayerPlugin;
use crate::plugins::world_plugin::WorldPlugin;
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins((WorldPlugin, PlayerPlugin));
    }
}
