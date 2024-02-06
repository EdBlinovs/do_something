use bevy::prelude::*;
use crate::app_state::AppState;
use crate::game as game;
use crate::util as util;
use crate::generic_ui::{ ButtonActionType, spawn_button };

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::GameOver), setup_gameover)
        .add_systems(OnExit(AppState::GameOver), util::cleanup_system::<GameOverEntity>);

    }
}

#[derive(Component)]
struct GameOverEntity;

fn setup_gameover(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<game::Score>, mut background_colour: ResMut<ClearColor>) {

    background_colour.0 = Color::rgb(47./255., 31./255., 13./255.);

    let node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let score_text: String = format!("score: {}", score.0.to_string());

    commands
        .spawn((node, GameOverEntity))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "you are a failure",
                    TextStyle {
                        font: asset_server.load("PoorStory-Regular.ttf"),
                        font_size: 60.0,
                        color: Color::rgb(0.9, 0.3, 0.3)
                    }
                )
            );

            parent.spawn(
                TextBundle::from_section(
                    score.1.to_string(),
                    TextStyle {
                        font: asset_server.load("PoorStory-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
            );

            parent.spawn(
                TextBundle::from_section(
                    score_text,
                    TextStyle {
                        font: asset_server.load("PoorStory-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                )
            );

            spawn_button(parent, &asset_server, "try again", ButtonActionType::ChangeAppState(AppState::Game));
        });

    commands.spawn((util::image(Vec2::new(0., 220.), "logo.png".into(), &asset_server), GameOverEntity));
}