use std::process;

use bevy::prelude::*;

use crate::app::constants::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON, SCREEN_COLOR};
use crate::app::resources::{GameStates, Score};

pub struct GameOver;

impl Plugin for GameOver {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameStates::Over), setup_game_over)
            .add_systems(
                Update,
                handle_gameover_buttons.run_if(in_state(GameStates::Over)),
            )
            .add_systems(OnExit(GameStates::Over), cleanup);
    }
}

#[derive(Resource)]
struct MenuData {
    button_entity: Entity,
}

fn setup_game_over(mut commands: Commands, score: Res<Score>) {
    let button_entity = commands
        .spawn((
            Node {
                //Screen
                width: percent(100),
                height: percent(100),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(5.0)),
                row_gap: px(20.0),
                ..default()
            },
            BackgroundColor(SCREEN_COLOR),
            BorderColor::all(Color::srgb(0.05, 0.85, 0.95)),
            children![
                (
                    Text::new("GAME OVER"),
                    TextFont {
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.95, 0.25, 0.65)),
                ),
                (
                    Text::new(format!("Score: {}", score.0)),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9))
                ),
                (
                    Button,
                    Node {
                        width: px(200),
                        height: px(65),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    RestartButton,
                    BackgroundColor(NORMAL_BUTTON),
                    children![(
                        Text::new("Restart"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )],
                ),
                (
                    Button,
                    Node {
                        width: px(200),
                        height: px(65),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    QuitButton,
                    children![(
                        Text::new("Quit"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )],
                )
            ],
        ))
        .id();
    commands.insert_resource(MenuData { button_entity });
}

#[derive(Component)]
struct QuitButton;

#[derive(Component)]
struct RestartButton;

fn handle_gameover_buttons(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&RestartButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameStates>>,
    mut score: ResMut<Score>,
) {
    for (interaction, mut color, restart, quit) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if restart.is_some() {
                    next_state.set(GameStates::Playing);
                    score.0 = 0;
                } else if quit.is_some() {
                    process::exit(0);
                }
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

fn cleanup(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.button_entity).despawn();
}
