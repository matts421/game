use crate::common::GameAction;
use crate::plugins::movement::Velocity;
use bevy::prelude::*;
use lightyear::prelude::input::{InputConfig, leafwing::InputPlugin};
use lightyear::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Debug, PartialEq, Reflect, Serialize, Deserialize)]
pub struct Player(pub PeerId);

#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerColor(pub Color);

#[derive(Clone)]
pub struct ProtocolPlugin;
impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin::<GameAction> {
            config: InputConfig {
                rebroadcast_inputs: true,
                ..default()
            },
        });

        app.register_component::<Player>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);

        app.register_component::<PlayerColor>()
            .add_prediction(PredictionMode::Once)
            .add_interpolation(InterpolationMode::Once);
    }
}
