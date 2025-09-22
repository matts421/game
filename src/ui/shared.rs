use crate::common::{AppState, MenuState};
use crate::ui::components::button::*;
use crate::ui::menu::*;
use bevy::prelude::*;
use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .init_resource::<InputFocus>()
            .add_systems(OnEnter(AppState::Menu), enter_menu)
            .add_systems(OnExit(AppState::Menu), leave_menu)
            .add_systems(Update, button_system.run_if(in_state(AppState::Menu)))
            .add_systems(Update, key_handler)
            .add_plugins(FpsCounterPlugin)
            .add_plugins(MainMenu)
            .add_plugins(MultiplayerMenu);
    }
}
fn enter_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main)
}
fn leave_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::None)
}

#[derive(Resource, Default)]
pub struct InputFocus(Option<Entity>);
impl InputFocus {
    pub fn set(&mut self, entity: Entity) {
        self.0 = Some(entity);
    }
    pub fn clear(&mut self) {
        self.0 = None;
    }
}

type ButtonElements = (
    Entity,
    &'static Interaction,
    &'static mut BackgroundColor,
    &'static mut BorderColor,
    &'static mut Button,
);
pub type ButtonInteraction = (With<Button>, Changed<Interaction>);

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<ButtonElements, Changed<Interaction>>,
) {
    for (entity, interaction, mut color, mut border_color, mut button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor(RED);
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor(Color::WHITE);
            }
        }
    }
}


fn key_handler(
    key_input: Res<ButtonInput<KeyCode>>,
    mut diags_state: ResMut<FpsCounter>,
) {
    if key_input.just_pressed(KeyCode::AltLeft) {
        if diags_state.is_enabled() {
            diags_state.disable();
        } else {
            diags_state.enable();
        }
    }
}
