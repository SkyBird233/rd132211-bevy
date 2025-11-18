use crate::prelude::*;
use bevy::window::CursorGrabMode;

#[derive(Resource, Default)]
pub struct FocusStatus(pub bool);

// TODO: update when bevy_rapier support bevy 0.17
pub fn handle_focus(
    window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut focus_status: ResMut<FocusStatus>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        focus_status.0 = true;
    }
    if key.just_pressed(KeyCode::Escape) {
        focus_status.0 = false;
    }

    match focus_status.0 {
        true => set_focus(window),
        false => set_unfocus(window),
    }
}

fn set_focus(mut window: Single<&mut Window>) {
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn set_unfocus(mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}
