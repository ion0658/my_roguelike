use crate::{
    AppState, GameState,
    ui_button::{ButtonClicked, create_button},
};
use bevy::prelude::*;

pub fn setup_pause(mut commands: Commands) {
    log::trace!("Setting up pause menu...");
    commands
        .spawn((
            DespawnOnExit(GameState::Paused),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            ZIndex(i32::MAX),
        ))
        .with_children(|p| {
            p.spawn((create_button("Resume"),)).observe(on_resume);
            p.spawn((create_button("Back To Title"),))
                .observe(on_back_to_title);
        });
}

pub fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut state: ResMut<NextState<GameState>>,
) {
    log::trace!("Toggling pause...");
    if input.just_pressed(KeyCode::Escape) {
        log::trace!("Escape pressed[{:?}]", current_state.get());
        match current_state.get() {
            GameState::Paused => state.set(GameState::Running),
            _ => state.set(GameState::Paused),
        }
    }
}

fn on_resume(_event: On<ButtonClicked>, mut state: ResMut<NextState<GameState>>) {
    log::trace!("Resuming...");
    state.set(GameState::Running);
}

pub fn on_back_to_title(_event: On<ButtonClicked>, mut state: ResMut<NextState<AppState>>) {
    log::trace!("Back to title...");
    state.set(AppState::Title);
}
