use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::plugins::movement::Velocity;
//the character is a garbage can for now...

const INIT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const INIT_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 0.5;
pub const PLAYER_SCALE: f32 = 2.0;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character)
            .add_systems(Update, (mouse_look, player_movement));
    }
}

fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model_scene = asset_server.load("Trashcan Small.glb#Scene0");
    commands.spawn((
        Transform {
            translation: INIT_TRANSLATION,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, PLAYER_SCALE),
            ..default()
        },
        Player,
        SceneRoot(model_scene),
        Velocity {
            value: INIT_VELOCITY,
        },
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    let (transform, mut velocity) = query.single_mut().unwrap();

    let forward = transform.forward().as_vec3();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let right = transform.right().as_vec3();
    let right_flat = Vec3::new(right.x, 0.0, right.z).normalize_or_zero();

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= forward_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= right_flat;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += right_flat;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction += Vec3::Y;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        direction -= Vec3::Y;
    }
    velocity.value = direction.normalize_or_zero() * PLAYER_SPEED;
}

fn mouse_look(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut().unwrap();

    let delta = mouse_motion
        .read()
        .into_iter()
        .fold(Vec2::ZERO, |curr, ev| curr + ev.delta);

    if delta != Vec2::ZERO {
        let sensitivity = 0.002;
        transform.rotation = Quat::from_rotation_y(-delta.x * sensitivity) * transform.rotation;
        transform.rotation = transform.rotation * Quat::from_rotation_x(-delta.y * sensitivity);
    }
}
