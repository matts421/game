use bevy::prelude::*; 

#[derive(Component)]
pub struct GroundPlugin;
impl Plugin for GroundPlugin{
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, spawn_ground);
    }
} 


fn spawn_ground(mut commands: Commands, asset_server: Res<AssetServer>){
    let ground_handle = asset_server.load("theappxr_Floor_2.glb#Scene0");
    info!("Ground handle: {:?}", ground_handle);
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(100.0, 0.2, 100.0),
            ..default()
        },
        GroundPlugin,
        SceneRoot(ground_handle),
    ));
    info!("Spawned");
}