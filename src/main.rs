use bevy::prelude::*;
use game::common::{GameState, InputPlugin};
use game::plugins::*;
use game::ui::UiPlugin;
use game::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputPlugin)
        .init_state::<GameState>()
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .run();
    println!("Program finished running.");
}
