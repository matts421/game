use bevy::prelude::*;

pub const WINDOW_WIDTH: i32 = 1366;
pub const WINDOW_HEIGHT: i32 = 768;

#[derive(Component, Debug)]
struct Position { 
    x: f32, 
    y: f32, 
    z: f32, 
}

#[derive(Component, Debug)]
struct Velocity { 
    x: f32, 
    y: f32, 
    z: f32,
}



fn main() {
    App::new() 
        .add_systems(Startup, spawn_character)
        .add_systems(Update, (update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
    println!("Program finished running.");
}

fn spawn_character(mut commands: Commands){ 
    commands.spawn((Position {x: 0.0, y: 0.0, z: 0.0 }, Velocity { x: 1.0, y: 1.0, z: 0.0}));
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>){
    for (velocity, mut position) in query.iter_mut(){
        position.x += velocity.x; 
        position.y += velocity.y;
        position.z += velocity.z;
    }
}

fn print_position(query: Query<(Entity, &Position)>){
    for (entity, position) in query.iter(){
        info!("Entity {:?} is at position (x, y, z) {:?}", entity, position);
    }
}