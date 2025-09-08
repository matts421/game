use bevy::prelude::*;

use crate::plugins::player::Player;
use crate::plugins::player::PLAYER_SCALE;
use crate::plugins::movement::update_position;


const CAMERA_DISTANCE_Z: f32 = 80.0; 
const CAMERA_DISTANCE_Y: f32 = 60.0;

pub struct CameraPlugin; 

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow.after(update_position))
            .add_systems(Update, camera_mouse_move);
    }
}

fn spawn_camera(mut commands: Commands){
    let observing: Vec3 = Vec3::new(0.0, PLAYER_SCALE / 2., 0.0); 
    commands.spawn((Camera3d::default(), 
    Transform::from_xyz(0.0, CAMERA_DISTANCE_Y, CAMERA_DISTANCE_Z)
        .looking_at(observing, Vec3::Y),
    ));
}

fn camera_follow(mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>, player_query: Query<&Transform, With<Player>>){
       let player_translation = player_query.single().unwrap().translation; 
       let mut camera_transform = camera_query.single_mut().unwrap();
       camera_transform.translation = player_translation + Vec3::new(0.0, CAMERA_DISTANCE_Y, CAMERA_DISTANCE_Z); 
    
}

fn camera_mouse_move(){ 

}

