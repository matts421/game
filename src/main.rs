use bevy::prelude::*;

pub const WINDOW_WIDTH: i32 = 1366;
pub const WINDOW_HEIGHT: i32 = 768;

fn main() {
    App::new() 
        .add_plugins(DefaultPlugins)
        .run();
    println!("Program finished running.");
}
