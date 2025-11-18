use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerCamera;

#[derive(Component, Debug, Default)]
pub struct PlayerPhysics {
    vertical_velocity: f32,
}

const MOVE_SPEED: f32 = 10.0;
const GRAVITY: f32 = -30.0;
const JUMP_SPEED: f32 = 12.0;

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
        Transform::from_xyz(0.0, 0.5, 0.0),
        ChildOf(player_id),
    ));
}

pub fn handle_player_movement(
    mut players: Query<(&mut KinematicCharacterController, &mut PlayerPhysics), With<Player>>,
    camera_transform: Query<&Transform, With<PlayerCamera>>,
    outputs: Query<&KinematicCharacterControllerOutput, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    // Should be only 1 player now
    let (mut controller, mut physics) = players.single_mut().unwrap();
    let camera_transform = camera_transform.single().unwrap();

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
