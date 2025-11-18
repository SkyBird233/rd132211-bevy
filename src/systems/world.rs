use crate::prelude::*;

pub fn setup_debug_camera(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 3.0, 8.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));
}

pub fn setup_world(mut commands: Commands) {
    for x in -5..=5 {
        for z in -5..=5 {
            commands
                .spawn(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(Transform::from_xyz(x as f32, 0.0, z as f32));
        }
    }
    commands
        .spawn(Collider::cuboid(0.5, 0.5, 0.5))
        .insert(Transform::from_xyz(1.0, 2.0, 3.0));
}
