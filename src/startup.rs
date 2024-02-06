use bevy::{core::FrameCount, prelude::*, window::{PresentMode, WindowTheme}};

fn startup(mut commands: Commands){

    commands.spawn(Camera2dBundle{
        ..default()
    });

}

fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}

pub struct StartupPlugin;

pub const CLEAR_COLOUR: Color = Color::rgb(154./255., 110./255., 61./255.);

const RESOLUTION: (f32, f32)  = (1024., 720.);

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App){

        app.add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "do something".into(),
                    resolution: RESOLUTION.into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    visible: false,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        );

        app.add_systems(Startup, play_music);

        app.insert_resource(ClearColor(CLEAR_COLOUR));

        app.add_systems(Startup, startup);

        app.add_systems(Update, make_visible);
    }
}

fn play_music (mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn(AudioBundle {
        source: asset_server.load("music.mp3"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        }
    });
}