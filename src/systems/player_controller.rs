use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

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
    commands.spawn(PlayerBundle::default());
}

pub fn handle_player_movement(
    mut players: Query<(&mut KinematicCharacterController, &mut PlayerPhysics), With<Player>>,
    outputs: Query<&KinematicCharacterControllerOutput, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    // Should be only 1 player now
    let (mut controller, mut physics) = players.single_mut().unwrap();

    let mut direction = Vec3::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        direction.x += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.z += 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.z -= 1.0;
    }

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
