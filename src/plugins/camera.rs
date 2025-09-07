use bevy::prelude::*;

use crate::plugins::player::Player;

const CAMERA_DISTANCE: f32 = 80.0; 

pub struct CameraPlugin; 

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

fn spawn_camera(mut commands: Commands){
    // player_spawn = 
    commands.spawn((Camera3d::default(), 
    Transform::from_xyz(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE)
        .looking_at(Vec3::ZERO, Vec3::ZERO),
    ));
}

fn camera_follow(mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>, player_query: Query<&Transform, With<Player>>){
    let player_translation = player_query.single().unwrap().translation; // might have to change when multiplayer
    let mut camera_transform = camera_query.single_mut().unwrap();
    camera_transform.translation = player_translation + Vec3::new(0.0, CAMERA_DISTANCE, CAMERA_DISTANCE); 
    
}


