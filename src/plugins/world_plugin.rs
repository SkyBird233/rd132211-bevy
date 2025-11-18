use crate::prelude::*;

use crate::systems::world::{self, BlockManager, BlockMaterial};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BlockManager>()
            .add_plugins(MaterialPlugin::<BlockMaterial>::default())
            .add_systems(Startup, world::setup_world)
            .add_systems(Update, world::update_highlight_block);
    }
}
