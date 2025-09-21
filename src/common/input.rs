use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub const MOUSE_SENSITIVITY: f32 = 0.002;

pub fn default_input_map() -> InputMap<Action> {
    InputMap::default()
        .with_dual_axis(Action::MoveHorizontal, VirtualDPad::wasd())
        .with_dual_axis(Action::Look, MouseMove::default())
        .with_axis(
            Action::MoveVertical,
            VirtualAxis::new(KeyCode::ShiftLeft, KeyCode::Space),
        )
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[actionlike(DualAxis)]
pub enum Action {
    MoveHorizontal,
    Look,
    #[actionlike(Axis)]
    MoveVertical,
}
