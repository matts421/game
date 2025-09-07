use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0; 

pub struct CameraPlugin; 

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands){
    player_spawn = 
    commands.spawn((Camera3d::default(), 
    Transform::from_xyz(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE)
        .looking_at(Vec3::ZERO, Vec3::ZERO),
    ));
}



