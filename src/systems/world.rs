use crate::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Bundle)]
struct BlockBundle {
    mark: Block,
    collider: Collider,
    transform: Transform,
}

impl Default for BlockBundle {
    fn default() -> Self {
        Self {
            mark: Block,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}

pub fn setup_world(mut commands: Commands) {
    for x in -5..=5 {
        for z in -5..=5 {
            commands.spawn(BlockBundle {
                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                ..default()
            });
        }
    }

    commands.spawn(BlockBundle {
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
        ..default()
    });
}
