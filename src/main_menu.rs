use bevy::prelude::*;
use crate::app_state::AppState;
use crate::util as util;
use crate::generic_ui::{ ButtonActionType, spawn_button };

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(OnExit(AppState::MainMenu), util::cleanup_system::<MainMenuEntity>);

    }
}

#[derive(Component)]
struct MainMenuEntity;


fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {

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

    commands
        .spawn((node, MainMenuEntity))
        .with_children(|parent| {
            spawn_button(parent, &asset_server, "Play", ButtonActionType::ChangeAppState(AppState::Game));
        });

    commands.spawn((util::image(Vec2::new(0., 220.), "logo.png".into(), &asset_server), MainMenuEntity));
}