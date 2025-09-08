use bevy::prelude::*;

use crate::plugins::movement::Velocity;
//the character is a garbage can for now... 


const INIT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const INIT_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const PLAYER_SPEED: f32 = 2.5; 
pub const PLAYER_SCALE: f32 = 50.0;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin; 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_character)
            .add_systems(Update, player_movement); 
    }
}


fn spawn_character(mut commands: Commands, asset_server : Res<AssetServer>){ 
    let model_scene = asset_server.load("Trashcan Small.glb#Scene0");
    commands.spawn((
        Transform {
            translation: INIT_TRANSLATION,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, PLAYER_SCALE ),
            ..default()
        },
        Player,
        SceneRoot(model_scene),
        Velocity { value: INIT_VELOCITY },
    ));
}

fn player_movement(keyboard_input : Res<ButtonInput<KeyCode>>, mut query: Query<(&Player, &mut Velocity)>){
    
    for (_player, mut velocity) in query.iter_mut(){
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW){
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS){
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA){
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD){
            direction.x += 1.0;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        velocity.value = direction * PLAYER_SPEED;
    }
}