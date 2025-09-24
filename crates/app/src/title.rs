use crate::{
    AppState,
    ui_button::{ButtonClicked, create_button},
};
use bevy::{prelude::*, window::WindowCloseRequested};

pub fn setup_title_ui(mut commands: Commands) {
    use bevy::color::palettes::tailwind::*;
    log::trace!("Setting up title UI...");
    commands
        .spawn((
            DespawnOnExit(AppState::Title),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10.0),
                ..Default::default()
            },
            BackgroundColor(Color::Srgba(GRAY_800)),
        ))
        .with_children(|p| {
            p.spawn((create_button("Game Start"),))
                .observe(on_game_start);
            p.spawn((create_button("Settings"),)).observe(on_settings);
            p.spawn((create_button("Exit"),)).observe(on_exit);
        });
}

fn on_game_start(_event: On<ButtonClicked>, mut state: ResMut<NextState<AppState>>) {
    log::trace!("Starting the game...");
    state.set(AppState::InGame);
}

fn on_settings(_event: On<ButtonClicked>, mut state: ResMut<NextState<AppState>>) {
    log::trace!("Opening settings...");
    state.set(AppState::Settings);
}

fn on_exit(
    _event: On<ButtonClicked>,
    mut window_close_event: MessageWriter<WindowCloseRequested>,
    query: Query<Entity, With<Window>>,
) {
    log::info!("Exiting the game...");
    for window in query.iter() {
        window_close_event.write(WindowCloseRequested { window });
    }
}
