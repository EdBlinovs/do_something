use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod app_state;
mod main_menu;
mod startup;
pub mod game;
mod game_over;
pub mod generic_ui;
pub mod util;

fn main() {
    let mut app = App::new();

    app.add_plugins(startup::StartupPlugin);

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_plugins(app_state::AppStatePlugin);
    
    app.add_plugins(main_menu::MainMenuPlugin);

    app.add_plugins(game::GamePlugin);

    app.add_plugins(game_over::GameOverPlugin);

    app.add_systems(Update, generic_ui::button_interaction_system);

    app.run();
}
