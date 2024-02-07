use bevy::prelude::*;
use crate::{app_state::{self, AppState}, game::{Carousel, GoodThing, Score, Situation}};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);


#[derive(Component)]
pub struct ButtonAction {
    action_type: ButtonActionType
}

pub enum ButtonActionType {
    ChangeAppState(AppState),
    EndGame(String),
    ProlongInevitable(Situation)
}

pub fn spawn_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, text: &str, action_type: ButtonActionType){
    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(125.),
            border: UiRect::all(Val::Px(5.)),
            margin: UiRect::all(Val::Px(5.)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    }, ButtonAction { action_type }))
    .with_children(|button| {
        button.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font: asset_server.load("PoorStory-Regular.ttf"),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ));
    });
}

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_game_state: ResMut<NextState<app_state::AppState>>,
    mut score: ResMut<Score>,
    mut good_things: Query<
    (&GoodThing, &mut Transform)
    >,
    mut carousel: Query<
    (&mut Carousel)
    >,
) {
    for (interaction, mut color, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                match &button_action.action_type {
                    ButtonActionType::ChangeAppState(game_state) => current_game_state.set(*game_state),
                    ButtonActionType::EndGame(message) => {
                        current_game_state.set(AppState::GameOver);
                        score.1 = message.into();
                    },
                    ButtonActionType::ProlongInevitable(situation) => {
                        match situation {
                            Situation::Baby => {
                                for (good_thing, mut transform) in &mut good_things {
                                    match good_thing.situation {
                                        Situation::Baby => {
                                            transform.translation.x = -450.;
                                        }
                                        _ => {},
                                    }
                                }
                            },
                            Situation::Kitten => {
                                for mut c in &mut carousel {
                                    c.0 = -c.0;
                                }
                            },
                            Situation::Sloth => {
                                for (good_thing, mut transform) in &mut good_things {
                                    match good_thing.situation {
                                        Situation::Sloth => {
                                            transform.translation.y -= 200.;
                                        }
                                        _ => {},
                                    }
                                }
                            }
                        }
                    },
                }

            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            },
        }
    }
}