use crate::prelude::*;

use crate::systems::world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, world::setup_world);
        // app.add_systems(Startup, world::setup_debug_camera);
    }
}
