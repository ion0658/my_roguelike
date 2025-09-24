mod in_game;
mod pause;
mod setting;
mod setup;
mod title;
mod ui_button;

use bevy::{prelude::*, window::WindowResized};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
#[states(scoped_entities)]
enum AppState {
    #[default]
    Title,
    InGame,
    Settings,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(AppState = AppState::InGame)]
#[states(scoped_entities)]
enum GameState {
    #[default]
    Running,
    GameOver,
    Paused,
}

#[derive(Resource, Debug, Clone, Default)]
struct GoGameResource {
    game: igo_core::Game,
}

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(setup::generate_window_settings())
            .set(setup::generate_rendere_settings()),
    )
    .init_state::<AppState>()
    .add_sub_state::<GameState>()
    .insert_resource(ClearColor(Color::BLACK))
    .init_resource::<GoGameResource>()
    .add_plugins(fps_counter::FpsCounterPlugin::default())
    .add_systems(
        Startup,
        (
            setup::setup_camera,
            title::setup_title_ui.after(setup::setup_camera),
        ),
    )
    .add_systems(
        Update,
        setup::track_window_size.run_if(on_message::<WindowResized>),
    )
    .add_systems(
        Update,
        ui_button::button_interaction_event.after(bevy::ui::ui_focus_system),
    )
    .add_systems(OnEnter(AppState::Title), title::setup_title_ui)
    .add_systems(OnEnter(AppState::InGame), in_game::setup_in_game_ui)
    .add_systems(
        Update,
        pause::toggle_pause
            .run_if(in_state(AppState::InGame).and(not(in_state(GameState::GameOver)))),
    )
    .add_systems(
        Update,
        (in_game::update_in_game, in_game::tick_game)
            .run_if(in_state(AppState::InGame).and(in_state(GameState::Running))),
    )
    .add_systems(OnEnter(GameState::GameOver), in_game::setup_game_over_ui)
    .add_systems(
        OnEnter(GameState::Paused),
        pause::setup_pause.run_if(in_state(AppState::InGame)),
    )
    .add_systems(
        Update,
        (setting::update_setting, setting::next_state).run_if(in_state(AppState::Settings)),
    );
    app
}
