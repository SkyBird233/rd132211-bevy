use crate::{
    prelude::*,
    systems::world::{BlockManager, BlockType},
};
use bevy::input::mouse::AccumulatedMouseMotion;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerCamera;

#[derive(Component, Debug, Default)]
pub struct PlayerPhysics {
    vertical_velocity: f32,
}

const MOVE_SPEED: f32 = 8.0;
const GRAVITY: f32 = -30.0;
const JUMP_SPEED: f32 = 10.0;
const MOUSE_SENSITIVITY: Vec2 = Vec2::new(0.003, 0.002);

#[derive(Bundle, Debug)]
struct PlayerBundle {
    marker: Player,
    rigid_body: RigidBody,
    collider: Collider,
    transform: Transform,
    kinematic_character_controller: KinematicCharacterController,
    physics: PlayerPhysics,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            rigid_body: RigidBody::KinematicPositionBased,
            collider: Collider::cuboid(0.5, 1.0, 0.5),
            transform: Transform::from_xyz(0.0, 1.5, 0.0),
            kinematic_character_controller: KinematicCharacterController {
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Absolute(0.5),
                    min_width: CharacterLength::Absolute(0.1),
                    include_dynamic_bodies: true,
                }),
                ..default()
            },
            physics: default(),
        }
    }
}

pub fn spawn(mut commands: Commands) {
    let player_id = commands.spawn(PlayerBundle::default()).id();

    // Spawn camera as child of player
    commands.spawn((
        PlayerCamera,
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.8, 0.0),
        Projection::from(PerspectiveProjection {
            fov: 60.0_f32.to_radians(),
            ..default()
        }),
        ChildOf(player_id),
    ));
}

pub fn handle_player_movement(
    mut players: Query<(&mut KinematicCharacterController, &mut PlayerPhysics, &mut Transform), With<Player>>,
    camera_transform: Query<&Transform, (With<PlayerCamera>, Without<Player>)>, // https://bevy.org/learn/errors/b0001
    outputs: Query<&KinematicCharacterControllerOutput, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    // Should be only 1 player now
    let (mut controller, mut physics, mut transform) = players.single_mut().unwrap();
    let camera_transform = camera_transform.single().unwrap();

    if keys.pressed(KeyCode::KeyR) {
        transform.translation = Vec3 { x: 0.0, y: 10.0, z: 0.0 };
        return;
    }

    // WASD
    let forward_input = keys.pressed(KeyCode::KeyW) as i32 - keys.pressed(KeyCode::KeyS) as i32;
    let right_input = keys.pressed(KeyCode::KeyD) as i32 - keys.pressed(KeyCode::KeyA) as i32;

    let forward_flat = camera_transform.forward().with_y(0.0).normalize();
    let right_flat = camera_transform.right().with_y(0.0).normalize();

    let direction = forward_flat * forward_input as f32 + right_flat * right_input as f32;

    // Jump and gravity
    // The output is only added when `KinematicCharacterController::translation` set to a value other than `None`.
    if let Ok(output) = outputs.single() {
        if output.grounded && physics.vertical_velocity < 0.0 {
            physics.vertical_velocity = 0.0;
        }
        if keys.just_pressed(KeyCode::Space) && output.grounded {
            physics.vertical_velocity = JUMP_SPEED;
        }
    }
    physics.vertical_velocity += GRAVITY * dt;

    let mut translation = direction * MOVE_SPEED * dt;
    translation.y = physics.vertical_velocity * dt;
    controller.translation = Some(translation);
}

pub fn handle_player_camera(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    player_camera_transform: Single<&mut Transform, With<PlayerCamera>>,
) {
    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    let mut transform = player_camera_transform.into_inner();

    let delta_yaw = -delta.x * MOUSE_SENSITIVITY.x;
    let delta_pitch = -delta.y * MOUSE_SENSITIVITY.y;

    let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
    let yaw = yaw + delta_yaw;

    const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;
    let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}

pub fn handle_player_interaction(
    rapier_context: ReadRapierContext,
    player_entity: Single<Entity, With<Player>>,
    camera_transform: Single<&GlobalTransform, With<PlayerCamera>>,
    transforms: Query<&GlobalTransform>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut block_manager: ResMut<BlockManager>,
    mut commands: Commands,
) {
    let rapier_context = rapier_context.single().unwrap();
    let player_entity = player_entity.into_inner();
    let camera_transform = camera_transform.into_inner();

    let filter = QueryFilter::exclude_dynamic().exclude_collider(player_entity);

    if let Some((entity, ray_intersection)) = rapier_context.cast_ray_and_get_normal(
        camera_transform.translation(),
        camera_transform.forward().into(),
        4.0,
        false,
        filter,
    ) {
        if let Ok(entity_transform) = transforms.get(entity) {
            let position = entity_transform.translation().as_ivec3();
            let normal = ray_intersection.normal.as_ivec3();
            block_manager.set_target(position, normal);
            // TODO: highlight target surface

            // Place or remove blocks
            if mouse.just_pressed(MouseButton::Left) {
                block_manager.set_block(&mut commands, position + normal, BlockType::Block);
            }
            if mouse.just_pressed(MouseButton::Right) {
                block_manager.set_block(&mut commands, position, BlockType::Air);
            }
        }
    } else {
        block_manager.set_target(IVec3::ZERO, IVec3::ZERO);
    }
}
