use bevy::prelude::*;

use crate::app::resources::GameStates;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::MainMenu), setup_menu)
            .add_systems(Update, menu.run_if(in_state(GameStates::MainMenu)))
            .add_systems(OnExit(GameStates::MainMenu), cleanup_menu);
    }
}

#[derive(Resource)] // The button entitiy is stored to deconstruct the menu later
struct MenuData {
    button_entity: Entity,
}

//button colors
const SCREEN_COLOR: Color = Color::srgb(0.08, 0.08, 0.12);
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_menu(mut commands: Commands) {
    let button_entity = commands
        .spawn((
            Node {
                //Screen
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),

                ..default()
            },
            BackgroundColor(SCREEN_COLOR),
            BorderColor::all(Color::srgb(0.05, 0.85, 0.95)),
            children![(
                Button,
                Node {
                    width: px(150),
                    height: px(65),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                children![(
                    Text::new("Play"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            )],
        ))
        .id();
    commands.insert_resource(MenuData { button_entity });
}

fn menu(
    mut next_state: ResMut<NextState<GameStates>>,
    mut interactive_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interactive_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameStates::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}
