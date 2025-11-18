use crate::prelude::*;

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
