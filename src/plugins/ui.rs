use crate::state::AppState;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::BLACK;
const HOVERED_BUTTON: Color = Color::srgb(0.0, 0.0, 1.0);
const PRESSED_BUTTON: Color = Color::srgb(0.5, 0.5, 0.5);
const RED: Color = Color::srgb(1.0, 0.0, 0.0);

const START: &str = "Start";
const QUIT: &str = "Quit";

#[derive(Component)]
struct MenuUi;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputFocus>()
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnExit(AppState::Menu), cleanup_ui::<MenuUi>)
            .add_systems(Update, button_system.run_if(in_state(AppState::Menu)));
    }
}

fn cleanup_ui<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}

#[derive(Resource, Default)]
struct InputFocus(Option<Entity>);
impl InputFocus {
    fn set(&mut self, entity: Entity) {
        self.0 = Some(entity);
    }
    fn clear(&mut self) {
        self.0 = None;
    }
}

fn setup_menu(mut commands: Commands) {
    // Menu root
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Buttons
            parent.spawn(button(START));
            parent.spawn(button(QUIT));
        })
        .insert(MenuUi);
}

fn button(label: &str) -> impl Bundle {
    (
        Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        Button,
        BackgroundColor(NORMAL_BUTTON),
        BorderColor(Color::WHITE),
        BorderRadius::MAX,
        children![(
            Text::new(label),
            TextFont {
                font_size: 30.0,
                ..default()
            },
            TextColor(Color::WHITE),
            TextShadow::default(),
        )],
    )
}

fn button_system(
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<AppState>>,
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        Changed<Interaction>,
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        &mut interaction_query
    {
        let text = text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor(RED);
                button.set_changed();

                match text.0.as_str() {
                    START => {
                        next_state.set(AppState::Playing);
                    }
                    QUIT => {
                        exit.write(AppExit::Success);
                    }
                    _ => {}
                }
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
