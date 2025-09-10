use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;

pub mod plugins;

use plugins::movement::MovementPlugin;
use plugins::player::PlayerPlugin;

use crate::plugins::camera::CameraPlugin;

pub const WINDOW_WIDTH: i32 = 1366;
pub const WINDOW_HEIGHT: i32 = 768;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
    println!("Program finished running.");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube mesh
    let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // Grid of cubes
    let mut count = 0;

    for x in -2..=2 {
        for y in -2..=2 {
            for z in 0..=2 {
                if (x + y + z) % 3 == 0 {
                    count += 1;
                    continue;
                }

                let hue = (count as f32 / 125.0) * 360.0;
                let color = Color::hsl(hue, 0.8, 0.5);
                let cube_material = materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                });

                commands.spawn((
                    Mesh3d(cube_mesh.clone()),
                    MeshMaterial3d(cube_material),
                    Transform::from_xyz(x as f32, y as f32, z as f32),
                    Wireframe,
                ));

                count += 1;
            }
        }
    }
}
