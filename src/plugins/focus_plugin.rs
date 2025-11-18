use crate::prelude::*;
use crate::systems::focus_handler;

pub struct FocusPlugin;

impl Plugin for FocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<focus_handler::FocusStatus>();
        app.add_systems(
            Startup,
            |mut focus_status: ResMut<focus_handler::FocusStatus>| focus_status.0 = true,
        );
        app.add_systems(Update, focus_handler::handle_focus);
    }
}
