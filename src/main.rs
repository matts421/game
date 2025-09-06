use bevy::prelude::*;

pub mod plugins; 

use plugins::scene::ScenePlugin; 
use plugins::movement::MovementPlugin;
use plugins::player::PlayerPlugin;
use plugins::print::PrintPlugin;
use plugins::camera::CameraPlugin; 

pub const WINDOW_WIDTH: i32 = 1366;
pub const WINDOW_HEIGHT: i32 = 768;



fn main() {
    App::new() 
        .add_plugins(ScenePlugin)
        // .insert_resource(
        //     ClearColor(Color::srgb(0.1, 0., 0.15)))
        // .insert_resource(AmbientLight { 
        //     color: Color::WHITE,
        //     brightness: 750.0,
        //     ..Default::default()
        // })
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PrintPlugin)
        .add_plugins(DefaultPlugins)
        .run();
    println!("Program finished running.");
}

