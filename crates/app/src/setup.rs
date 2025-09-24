use bevy::{
    prelude::*,
    window::{WindowResized, WindowResolution},
};

const LOGICAL_WINDOW_WIDTH: u32 = 1920;
const LOGICAL_WINDOW_HEIGHT: u32 = 1080;

pub(crate) fn generate_window_settings() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            resizable: false,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            resolution: WindowResolution::new(LOGICAL_WINDOW_WIDTH, LOGICAL_WINDOW_HEIGHT),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub(crate) fn generate_rendere_settings() -> bevy::render::RenderPlugin {
    const BACKEND: Option<bevy::render::settings::Backends> = if cfg!(target_os = "windows") {
        Some(bevy::render::settings::Backends::DX12)
    } else if cfg!(target_os = "linux") {
        Some(bevy::render::settings::Backends::VULKAN)
    } else if cfg!(target_os = "macos") {
        Some(bevy::render::settings::Backends::METAL)
    } else {
        None
    };
    log::debug!("Using backend: {:?}", BACKEND);
    bevy::render::RenderPlugin {
        render_creation: bevy::render::settings::RenderCreation::Automatic(
            bevy::render::settings::WgpuSettings {
                power_preference: bevy::render::settings::PowerPreference::HighPerformance,
                backends: BACKEND,
                ..Default::default()
            },
        ),
        synchronous_pipeline_compilation: false,
        ..Default::default()
    }
}

pub(crate) fn setup_camera(mut commands: Commands) {
    log::trace!("Spawning 2D Camera");
    commands.spawn((Camera2d, Msaa::Sample4));
}

pub(crate) fn track_window_size(
    mut resize_events: MessageReader<WindowResized>,
    mut window: Single<&mut Window>,
) {
    log::trace!("Tracking Window Size");
    for event in resize_events.read() {
        log::debug!("Window resized: {event:?}");
        let resolution = &mut window.resolution;
        log::debug!("Previous Physical Resolution: {:?}", resolution);
        let h_scale = resolution.physical_width() as f64 / LOGICAL_WINDOW_WIDTH as f64;
        let v_scale = resolution.physical_height() as f64 / LOGICAL_WINDOW_HEIGHT as f64;
        let scale = h_scale.min(v_scale) as f32;
        log::debug!("New Scale Factor: {scale}");
        log::info!(
            "Window resized: {}x{} (scale factor: {})",
            resolution.physical_width(),
            resolution.physical_height(),
            scale
        );
        resolution.set_scale_factor(scale);
        log::debug!("New Physical Resolution: {:?}", resolution);
    }
}
