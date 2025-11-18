use bevy::{math::VectorSpace, platform::collections::HashMap};

use crate::prelude::*;

#[derive(Component)]
pub struct Block;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct BlockMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub terrain_texture: Handle<Image>,
    #[uniform(2)]
    pub highlight_pos: Vec3,
    #[uniform(3)]
    pub highlight_normal: Vec3,
}

impl Default for BlockMaterial {
    fn default() -> Self {
        Self {
            terrain_texture: Handle::default(),
            highlight_pos: Vec3::ZERO,
            highlight_normal: Vec3::ZERO,
        }
    }
}

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

#[derive(Resource, Default, Debug)]
pub struct BlockManager {
    blocks: HashMap<IVec3, Entity>,
    target: Option<BlockTarget>,
    mesh: Handle<Mesh>,
    material: Handle<BlockMaterial>,
}

pub enum BlockType {
    Air,
    Block,
}

#[derive(Resource, Default, Debug)]
pub struct BlockTarget {
    pub position: IVec3,
    pub normal: IVec3,
}

impl BlockManager {
    pub fn set_block(&mut self, commands: &mut Commands, position: IVec3, block_type: BlockType) {
        match block_type {
            BlockType::Air => {
                self.remove_block(commands, position);
            }
            BlockType::Block => {
                self.spawn_block(commands, position);
            }
        }
    }

    pub fn set_target(&mut self, position: IVec3, normal: IVec3) {
        self.target = Some(BlockTarget { position, normal });
    }

    fn spawn_block(&mut self, commands: &mut Commands, position: IVec3) {
        let entity = commands
            .spawn(BlockBundle {
                mesh: Mesh3d(self.mesh.clone()),
                material: MeshMaterial3d(self.material.clone()),
                transform: Transform::from_translation(position.as_vec3()),
                ..default()
            })
            .id();
        self.blocks.insert(position, entity);
    }

    fn remove_block(&mut self, commands: &mut Commands, position: IVec3) {
        let entity = self.blocks.get(&position);
        match entity {
            Some(entity) => {
                commands.entity(*entity).despawn();
                self.blocks.remove(&position);
            }
            None => {
                error!("Failed to remove block {}", position);
            }
        }
    }
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BlockMaterial>>,
    asset_server: Res<AssetServer>,
    mut block_manager: ResMut<BlockManager>,
) {
    block_manager.mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let terrain_texture = asset_server.load("textures/terrain.png");
    block_manager.material = materials.add(BlockMaterial {
        terrain_texture,
        highlight_pos: Vec3::ZERO,
        highlight_normal: Vec3::ZERO,
    });
    block_manager.target = None;

    for x in -10..=10 {
        for z in -10..=10 {
            for y in -5..=0 {
                block_manager.set_block(&mut commands, ivec3(x, y, z), BlockType::Block);
            }
        }
    }
    block_manager.set_block(&mut commands, ivec3(1, 2, 3), BlockType::Block);
}

pub fn update_highlight_block(
    block_manager: Res<BlockManager>,
    mut materials: ResMut<Assets<BlockMaterial>>,
) {
    let Some(material) = materials.get_mut(&block_manager.material) else {
        return;
    };
    let Some(target) = &block_manager.target else {
        return;
    };

    material.highlight_pos = target.position.as_vec3();
    material.highlight_normal = target.normal.as_vec3();
}
