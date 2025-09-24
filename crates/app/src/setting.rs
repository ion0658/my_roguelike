use crate::AppState;
use bevy::prelude::*;

pub fn update_setting() {}

pub fn next_state(input: Res<ButtonInput<KeyCode>>, mut state: ResMut<NextState<AppState>>) {
    log::trace!("next_state");
    if input.just_pressed(KeyCode::Escape) {
        state.set(AppState::Title)
    }
}
