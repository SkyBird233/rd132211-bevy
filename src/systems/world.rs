use crate::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlockMaterial {}

impl Material for BlockMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/blocks.wgsl".into()
    }
}

#[derive(Bundle)]
struct BlockBundle {
    mark: Block,
    collider: Collider,
    mesh: Mesh3d,
    material: MeshMaterial3d<BlockMaterial>,
    transform: Transform,
}

impl Default for BlockBundle {
    fn default() -> Self {
        Self {
            mark: Block,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            mesh: Mesh3d(Handle::default()),
            material: MeshMaterial3d(Handle::default()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        }
    }
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BlockMaterial>>,
) {
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let block_material = materials.add(BlockMaterial {});

    for x in -5..=5 {
        for z in -5..=5 {
            commands.spawn(BlockBundle {
                mesh: Mesh3d(cube_mesh.clone()),
                material: MeshMaterial3d(block_material.clone()),
                transform: Transform::from_xyz(x as f32, 0.0, z as f32),
                ..default()
            });
        }
    }

    commands.spawn(BlockBundle {
        mesh: Mesh3d(cube_mesh.clone()),
        material: MeshMaterial3d(block_material.clone()),
        transform: Transform::from_xyz(1.0, 2.0, 3.0),
        ..default()
    });
}
