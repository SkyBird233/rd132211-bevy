use crate::prelude::*;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Bundle, Debug)]
struct PlayerBundle {
    marker: Player,
    rigid_body: RigidBody,
    collider: Collider,
    transform: Transform,
    kinematic_character_controller: KinematicCharacterController,
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
        }
    }
}

pub fn spawn(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

pub fn gogogo(
    mut player_controllers: Query<&mut KinematicCharacterController, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // Should be only 1 player now
    let mut controller = player_controllers.single_mut().unwrap();

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

    controller.translation = Some(direction * 10.0 * time.delta_secs());
}

pub fn read_result_system(controllers: Query<(&Player, &KinematicCharacterControllerOutput)>) {
    for (player, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            player, output.effective_translation, output.grounded
        );
    }
}
