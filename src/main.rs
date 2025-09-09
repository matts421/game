use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

pub mod plugins;

use plugins::camera::CameraPlugin;
use plugins::ground_sample::GroundPlugin;
use plugins::movement::MovementPlugin;
use plugins::player::PlayerPlugin;
use plugins::print::PrintPlugin;
use plugins::scene::ScenePlugin;

pub const WINDOW_WIDTH: i32 = 1366;
pub const WINDOW_HEIGHT: i32 = 768;

fn main() {
    App::new()
        .add_plugins(ScenePlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PrintPlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(DefaultPlugins)
        .run();
    println!("Program finished running.");
}
