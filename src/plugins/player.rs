use bevy::prelude::*;

use crate::plugins::movement::Velocity;
//the character is a garbage can for now... 


const INIT_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const INIT_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);



pub struct PlayerPlugin; 
impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_character);
    }
}


fn spawn_character(mut commands: Commands, asset_server : Res<AssetServer>){ 
    let model_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Trashcan Small.glb"));
    commands.spawn((
        Transform {
            translation: INIT_TRANSLATION,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(50., 50., 50.),
            ..default()
        },
        SceneRoot(model_scene),
        Velocity { value: INIT_VELOCITY },
    ));
}