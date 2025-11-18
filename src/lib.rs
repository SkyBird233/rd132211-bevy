// mod components;
mod plugins;
pub mod prelude;
mod systems;

use bevy::asset::AssetMetaCheck;

use crate::plugins::focus_plugin::FocusPlugin;
use crate::plugins::player_plugin::PlayerPlugin;
use crate::plugins::world_plugin::WorldPlugin;
use crate::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                // Note: https://github.com/bevyengine/bevy/issues/10157
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb(
            0x66 as f32 / 0xFF as f32,
            0xCC as f32 / 0xFF as f32,
            1.0,
        )))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default().disabled())
        .add_plugins((WorldPlugin, PlayerPlugin, FocusPlugin));
    }
}
