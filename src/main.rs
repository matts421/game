use bevy::prelude::*;
use game::common::AppState;
use game::plugins::*;
use game::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(WorldPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .run();
    println!("Program finished running.");
}
