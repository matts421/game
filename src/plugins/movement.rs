use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::common::GameState;

pub struct MovementPlugin;

#[derive(Component, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub value: Vec3,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position.run_if(in_state(GameState::Playing)));
    }
}

pub fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += time.delta_secs() * velocity.value;
    }
}
