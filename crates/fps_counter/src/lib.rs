use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const FPS_COUNTER_FONT_SIZE: f32 = 12.0;

#[derive(Bundle)]
pub struct FpsTextBundle {
    text: Text,
    color: TextColor,
    layout: TextLayout,
    font: TextFont,
}

#[derive(Component)]
struct FpsCounter;

#[derive(Component)]
struct FpsValueNode;
#[derive(Component)]
struct FpsValue;

#[derive(Component)]
struct FpsAverageNode;
#[derive(Component)]
struct FpsAverageValue;

#[derive(Component)]
struct FpsFrameTimeNode;
#[derive(Component)]
struct FpsFrameTimeValue;

#[derive(Component)]
struct FpsFrameTimeAverageNode;
#[derive(Component)]
struct FpsFrameTimeAverageValue;

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, States)]
#[states(scoped_entities)]
enum FpsCounterVisibilityState {
    Hidden,
    #[default]
    Visible,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, States)]
#[states(scoped_entities)]
enum FpsAverageVisibilityState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, States)]
#[states(scoped_entities)]
enum FpsFrameTimeVisibilityState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, States)]
#[states(scoped_entities)]
enum FpsFrameTimeAverageVisibilityState {
    #[default]
    Hidden,
    Visible,
}

fn spawn_item<C: Component, CN: Component, CS: Component>(
    parent: &mut Commands,
    label: &str,
    component: C,
    node: CN,
    state: CS,
) -> Entity {
    parent
        .spawn((
            state,
            node,
            Node {
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
                left: Val::Auto,
                margin: UiRect::horizontal(Val::Px(5.)),
                ..Default::default()
            },
            BackgroundColor(Color::srgba_u8(0xFF, 0xFF, 0xFF, 0x00)),
            ZIndex(i32::MAX),
        ))
        .with_children(|parent| {
            parent.spawn((FpsTextBundle {
                text: Text(format!("{}:", label)),
                color: TextColor(Color::WHITE),
                layout: TextLayout {
                    justify: Justify::Left,
                    ..Default::default()
                },
                font: TextFont {
                    font_size: FPS_COUNTER_FONT_SIZE,
                    ..Default::default()
                },
            },));
            parent.spawn((
                component,
                FpsTextBundle {
                    text: Text(" N/A".into()),
                    color: TextColor(Color::WHITE),
                    layout: TextLayout {
                        justify: Justify::Right,
                        ..Default::default()
                    },
                    font: TextFont {
                        font_size: FPS_COUNTER_FONT_SIZE,
                        ..Default::default()
                    },
                },
            ));
        })
        .id()
}

fn create_counter_node(mut commands: Commands) {
    log::trace!("Creating FPS counter node");
    let fps_item = spawn_item(
        &mut commands,
        "FPS",
        FpsValue,
        FpsValueNode,
        DespawnOnExit(FpsCounterVisibilityState::Visible),
    );
    commands
        .spawn((
            DespawnOnExit(FpsCounterVisibilityState::Visible),
            FpsCounter,
            Node {
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
                left: Val::Auto,
                padding: UiRect::all(Val::Px(5.)),
                ..Default::default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.5)),
            ZIndex(i32::MAX),
        ))
        .add_child(fps_item);
}

fn add_average_node(mut commands: Commands, parent: Option<Single<Entity, With<FpsCounter>>>) {
    log::trace!("Adding AVG node");
    if let Some(parent) = parent {
        let average_item = spawn_item(
            &mut commands,
            "AVG FPS",
            FpsAverageValue,
            FpsAverageNode,
            DespawnOnExit(FpsAverageVisibilityState::Visible),
        );
        commands.entity(*parent).insert_children(1, &[average_item]);
    } else {
        log::warn!("Cannot add AVG node, parent not found");
    }
}

fn add_frametime_node(mut commands: Commands, parent: Option<Single<Entity, With<FpsCounter>>>) {
    log::trace!("Adding FrameTime node");
    if let Some(parent) = parent {
        let frame_time_item = spawn_item(
            &mut commands,
            "FrameTime [ms]",
            FpsFrameTimeValue,
            FpsFrameTimeNode,
            DespawnOnExit(FpsFrameTimeVisibilityState::Visible),
        );
        commands
            .entity(*parent)
            .insert_children(2, &[frame_time_item]);
    } else {
        log::warn!("Cannot add FrameTime node, parent not found");
    }
}

fn add_frametime_average_node(
    mut commands: Commands,
    parent: Option<Single<Entity, With<FpsCounter>>>,
) {
    log::trace!("Adding FrameTime AVG node");
    if let Some(parent) = parent {
        let frame_time_average_item = spawn_item(
            &mut commands,
            "FrameTime AVG [ms]",
            FpsFrameTimeAverageValue,
            FpsFrameTimeAverageNode,
            DespawnOnExit(FpsFrameTimeAverageVisibilityState::Visible),
        );
        commands
            .entity(*parent)
            .insert_children(3, &[frame_time_average_item]);
    } else {
        log::warn!("Cannot add FrameTime AVG node, parent not found");
    }
}

const fn get_color(fps: f64) -> Color {
    use bevy::color::palettes::basic::*;
    if fps < 29. {
        Color::Srgba(RED)
    } else if fps < 59. {
        Color::Srgba(YELLOW)
    } else if fps < 119. {
        Color::Srgba(LIME)
    } else {
        Color::Srgba(AQUA)
    }
}

fn fps_value_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut text: Single<(&mut Text, &mut TextColor), With<FpsValue>>,
) {
    // try to get a "smoothed" FPS value from Bevy
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        text.0.0 = format!("{value:>4.0}");
        text.1.0 = get_color(value);
    } else {
        // display "N/A" if we can't get a FPS measurement
        text.0.0 = " N/A".into();
        text.1.0 = Color::WHITE;
    }
}
fn fps_average_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut text: Single<(&mut Text, &mut TextColor), With<FpsAverageValue>>,
) {
    // try to get a "smoothed" FPS value from Bevy
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    {
        text.0.0 = format!("{value:>4.0}");
        text.1.0 = get_color(value);
    } else {
        // display "N/A" if we can't get a FPS measurement
        text.0.0 = " N/A".into();
        text.1.0 = Color::WHITE;
    }
}
fn fps_frame_time_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut text: Single<(&mut Text, &mut TextColor), With<FpsFrameTimeValue>>,
) {
    // try to get a "smoothed" frame time value from Bevy
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|ft| ft.smoothed())
    {
        text.0.0 = format!("{value:>6.2}");
        text.1.0 = get_color(1000. / value);
    } else {
        // display "N/A" if we can't get a frame time measurement
        text.0.0 = " N/A".into();
        text.1.0 = Color::WHITE;
    }
}

fn fps_frame_time_average_update_system(
    diagnostics: Res<DiagnosticsStore>,
    mut text: Single<(&mut Text, &mut TextColor), With<FpsFrameTimeAverageValue>>,
) {
    // try to get a "smoothed" frame time value from Bevy
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|ft| ft.average())
    {
        text.0.0 = format!("{value:>6.2}");
        text.1.0 = get_color(1000. / value);
    } else {
        // display "N/A" if we can't get a frame time measurement
        text.0.0 = " N/A".into();
        text.1.0 = Color::WHITE;
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Resource, serde::Serialize, serde::Deserialize,
)]
pub struct FpsCounterOption {
    visible: bool,
    show_average: bool,
    show_frame_time: bool,
    show_average_frame_time: bool,
}
impl Default for FpsCounterOption {
    fn default() -> Self {
        Self {
            visible: true,
            show_average: true,
            show_frame_time: true,
            show_average_frame_time: true,
        }
    }
}

fn on_change_visibility(
    config: Res<FpsCounterOption>,
    visibility_state_current: Res<State<FpsCounterVisibilityState>>,
    mut vislibility_state: ResMut<NextState<FpsCounterVisibilityState>>,
) {
    match (config.visible, visibility_state_current.get()) {
        (true, FpsCounterVisibilityState::Hidden) => {
            vislibility_state.set(FpsCounterVisibilityState::Visible);
        }
        (false, FpsCounterVisibilityState::Visible) => {
            vislibility_state.set(FpsCounterVisibilityState::Hidden);
        }
        _ => {}
    }
}
fn on_change_average_visibility(
    config: Res<FpsCounterOption>,
    visibility_state_current: Res<State<FpsAverageVisibilityState>>,
    mut vislibility_state: ResMut<NextState<FpsAverageVisibilityState>>,
) {
    match (config.show_average, visibility_state_current.get()) {
        (true, FpsAverageVisibilityState::Hidden) => {
            vislibility_state.set(FpsAverageVisibilityState::Visible);
        }
        (false, FpsAverageVisibilityState::Visible) => {
            vislibility_state.set(FpsAverageVisibilityState::Hidden);
        }
        _ => {}
    }
}
fn on_change_frame_time_visibility(
    config: Res<FpsCounterOption>,
    visibility_state_current: Res<State<FpsFrameTimeVisibilityState>>,
    mut vislibility_state: ResMut<NextState<FpsFrameTimeVisibilityState>>,
) {
    match (config.show_frame_time, visibility_state_current.get()) {
        (true, FpsFrameTimeVisibilityState::Hidden) => {
            vislibility_state.set(FpsFrameTimeVisibilityState::Visible);
        }
        (false, FpsFrameTimeVisibilityState::Visible) => {
            vislibility_state.set(FpsFrameTimeVisibilityState::Hidden);
        }
        _ => {}
    }
}
fn on_change_average_frame_time_visibility(
    config: Res<FpsCounterOption>,
    visibility_state_current: Res<State<FpsFrameTimeAverageVisibilityState>>,
    mut vislibility_state: ResMut<NextState<FpsFrameTimeAverageVisibilityState>>,
) {
    match (
        config.show_average_frame_time,
        visibility_state_current.get(),
    ) {
        (true, FpsFrameTimeAverageVisibilityState::Hidden) => {
            vislibility_state.set(FpsFrameTimeAverageVisibilityState::Visible);
        }
        (false, FpsFrameTimeAverageVisibilityState::Visible) => {
            vislibility_state.set(FpsFrameTimeAverageVisibilityState::Hidden);
        }
        _ => {}
    }
}

#[derive(Default)]
pub struct FpsCounterPlugin {
    config: FpsCounterOption,
}

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(self.config)
            .init_state::<FpsCounterVisibilityState>()
            .init_state::<FpsAverageVisibilityState>()
            .init_state::<FpsFrameTimeVisibilityState>()
            .init_state::<FpsFrameTimeAverageVisibilityState>()
            .add_systems(
                Update,
                fps_value_update_system.run_if(in_state(FpsCounterVisibilityState::Visible)),
            )
            .add_systems(
                Update,
                fps_average_update_system.run_if(in_state(FpsAverageVisibilityState::Visible)),
            )
            .add_systems(
                Update,
                fps_frame_time_update_system.run_if(in_state(FpsFrameTimeVisibilityState::Visible)),
            )
            .add_systems(
                Update,
                fps_frame_time_average_update_system
                    .run_if(in_state(FpsFrameTimeAverageVisibilityState::Visible)),
            )
            .add_systems(
                Update,
                (
                    on_change_visibility,
                    (
                        on_change_average_visibility,
                        on_change_frame_time_visibility,
                        on_change_average_frame_time_visibility,
                    )
                        .after(on_change_visibility),
                )
                    .run_if(resource_changed::<FpsCounterOption>),
            )
            .add_systems(
                OnEnter(FpsCounterVisibilityState::Visible),
                create_counter_node,
            )
            .add_systems(
                OnEnter(FpsAverageVisibilityState::Visible),
                add_average_node,
            )
            .add_systems(
                OnEnter(FpsFrameTimeVisibilityState::Visible),
                add_frametime_node,
            )
            .add_systems(
                OnEnter(FpsFrameTimeAverageVisibilityState::Visible),
                add_frametime_average_node,
            );
    }
}
