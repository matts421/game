use bevy::prelude::States;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Menu,
    Playing,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum MenuState {
    #[default]
    None,
    Main,
    Multiplayer,
}
