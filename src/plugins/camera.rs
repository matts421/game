use std::{f32::consts::FRAC_PI_2, ops::Range};
use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::plugins::movement::update_position;
use crate::plugins::player::{Player};

// Change for camera feel. 
const CAMERA_RADIUS : f32 = 150.0;
const PITCH_SPEED : f32 = 0.003;
const YAW_SPEED : f32 = 0.004;


#[derive(Debug, Resource)]
struct CameraSettings {
    pub orbit_distance: f32,
    pub pitch_speed: f32,
    // Clamp pitch to this range
    pub pitch_range: Range<f32>,
    pub yaw_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        let pitch_limit = FRAC_PI_2 - 0.01;
        Self {
            orbit_distance: CAMERA_RADIUS,
            pitch_speed: PITCH_SPEED,
            pitch_range: -pitch_limit..pitch_limit,
            yaw_speed: YAW_SPEED,
        }
    }
}

#[derive(Component)]
pub struct CameraPlugin;
impl Plugin for CameraPlugin{
fn build(&self, app: &mut App) {
    app
        .init_resource::<CameraSettings>()
        .add_systems(Startup, setup)
        .add_systems(Update, orbit.after(update_position));     
}
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn orbit(
    mut camera: Single<&mut Transform, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    player_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let delta = mouse_motion.delta;

    let delta_pitch = delta.y * camera_settings.pitch_speed;
    let delta_yaw = delta.x * camera_settings.yaw_speed;

    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

    let pitch = (pitch + delta_pitch).clamp(
        camera_settings.pitch_range.start,
        camera_settings.pitch_range.end,
    );
    let yaw = yaw + delta_yaw;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    let target = player_transform.single().unwrap().translation;
    camera.translation = target - camera.forward() * camera_settings.orbit_distance;
}