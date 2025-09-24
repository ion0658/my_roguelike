use crate::{
    AppState, GameState, GoGameResource,
    pause::on_back_to_title,
    ui_button::{ButtonClicked, create_button},
};
use bevy::prelude::*;

pub const STONE_RADIUS: f32 = 22.5;
pub const LINE_COLOR: Color = Color::Srgba(bevy::color::palettes::tailwind::GRAY_800);

#[derive(Component)]
pub struct StonePos {
    x: u8,
    y: u8,
}

fn create_2d_mesh(
    mesh: Handle<Mesh>,
    color: Handle<ColorMaterial>,
    visibility: Visibility,
    position: Vec3,
) -> impl Bundle {
    (
        Mesh2d(mesh),
        MeshMaterial2d(color),
        visibility,
        Transform::from_translation(position),
    )
}

pub fn setup_in_game_ui(
    mut commands: Commands,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<GoGameResource>,
) {
    use bevy::color::palettes::tailwind::*;
    log::trace!("Setting up in-game UI");
    let window_resolution = window.size();
    game.game.reset();
    commands
        .spawn((
            DespawnOnExit(AppState::InGame),
            Transform::default(),
            Visibility::Visible,
            Mesh2d(meshes.add(Rectangle::from_length(window_resolution.y))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(AMBER_500)))),
        ))
        .with_children(|p| {
            let board_size = window_resolution.y * 0.9;

            let line_color = materials.add(ColorMaterial::from(LINE_COLOR));
            let stone_color = materials.add(ColorMaterial::from(Color::BLACK));
            let horizontal_line = meshes.add(Rectangle::new(board_size, 3.));
            let vertical_line = meshes.add(Rectangle::new(3., board_size));
            let star_circle = meshes.add(Circle::new(STONE_RADIUS / 4.));
            let stone_circle = meshes.add(Circle::new(STONE_RADIUS));

            let line_count = game.game.size();
            for i in 0..line_count {
                let pos = -board_size / 2. + (i as f32) * (board_size / (line_count - 1) as f32);
                p.spawn(create_2d_mesh(
                    horizontal_line.clone(),
                    line_color.clone(),
                    Visibility::Visible,
                    Vec3::new(0., pos, 0.),
                ));
                p.spawn(create_2d_mesh(
                    vertical_line.clone(),
                    line_color.clone(),
                    Visibility::Visible,
                    Vec3::new(pos, 0., 0.),
                ));
            }
            let is_star = |x: u8, y: u8| {
                x > 0
                    && y > 0
                    && x < line_count as u8
                    && y < line_count as u8
                    && x % 3 == 0
                    && x % 2 != 0
                    && y % 3 == 0
                    && y % 2 != 0
            };
            for x in 0..line_count {
                for y in 0..line_count {
                    if is_star(x, y) {
                        let pos_x =
                            -board_size / 2. + (x as f32) * (board_size / (line_count - 1) as f32);
                        let pos_y =
                            board_size / 2. - (y as f32) * (board_size / (line_count - 1) as f32);
                        p.spawn(create_2d_mesh(
                            star_circle.clone(),
                            stone_color.clone(),
                            Visibility::Visible,
                            Vec3::new(pos_x, pos_y, 0.),
                        ));
                    }
                    let pos_x =
                        -board_size / 2. + (x as f32) * (board_size / (line_count - 1) as f32);
                    let pos_y =
                        board_size / 2. - (y as f32) * (board_size / (line_count - 1) as f32);
                    p.spawn(create_2d_mesh(
                        stone_circle.clone(),
                        stone_color.clone(),
                        Visibility::Hidden,
                        Vec3::new(pos_x, pos_y, 0.),
                    ))
                    .insert(StonePos { x, y });
                }
            }
        });
}

pub fn tick_game(
    mut game: ResMut<GoGameResource>,
    mut turn: Local<igo_core::Stone>,
    mut state: ResMut<NextState<GameState>>,
) {
    log::trace!("Ticking game, turn: {:?}", *turn);
    let hands = game.game.get_allowed_hands(*turn);
    let hand = if hands.is_empty() {
        igo_core::GameHand::pass(*turn)
    } else {
        let idx = rand::random::<u64>() as usize % hands.len();
        hands[idx]
    };
    log::debug!("Hand: {:?}", hand);
    *turn = (*turn).opposite();
    if !game.game.put_hand(hand) {
        state.set(GameState::GameOver);
    }
}

pub fn update_in_game(
    game: Res<GoGameResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(
        &StonePos,
        &mut MeshMaterial2d<ColorMaterial>,
        &mut Visibility,
    )>,
) {
    log::trace!("Updating in-game UI");
    let board = game.game.board();
    let black_material = materials.add(ColorMaterial::from(Color::BLACK));
    let white_material = materials.add(ColorMaterial::from(Color::WHITE));
    query
        .par_iter_mut()
        .for_each(|(pos, mut material, mut visibility)| {
            if let Some(stone) = board.get_stone(pos.x, pos.y) {
                let color = match stone {
                    igo_core::Stone::Black => black_material.clone(),
                    igo_core::Stone::White => white_material.clone(),
                };
                *material = MeshMaterial2d(color);
                *visibility = Visibility::Visible;
            } else {
                *visibility = Visibility::Hidden;
            }
        });
}

pub fn setup_game_over_ui(mut commands: Commands, game: Res<GoGameResource>) {
    let game_result = game.game.judge();
    log::info!("Game Over: {game_result:?}");
    commands
        .spawn((
            DespawnOnExit(GameState::GameOver),
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
            ZIndex(i32::MAX - 1),
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: percent(100),
                height: percent(50),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10.0),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((
                    Text("Game Over".into()),
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: Justify::Center,
                        ..Default::default()
                    },
                    TextFont {
                        font_size: 40.0,
                        ..Default::default()
                    },
                ));
                p.spawn((
                    Text(format!("Winner: {:?}", game_result)),
                    TextColor(Color::WHITE),
                    TextLayout {
                        justify: Justify::Center,
                        ..Default::default()
                    },
                    TextFont {
                        font_size: 30.0,
                        ..Default::default()
                    },
                ));
            });
            p.spawn(Node {
                width: percent(100),
                height: percent(50),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: px(10.0),
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn((create_button("Reset"),)).observe(on_reset_game);
                p.spawn((create_button("Back To Title"),))
                    .observe(on_back_to_title);
            });
        });
}

fn on_reset_game(
    _event: On<ButtonClicked>,
    mut game: ResMut<GoGameResource>,
    mut state: ResMut<NextState<GameState>>,
) {
    log::trace!("Resetting game");
    log::info!("Resetting game");
    game.game.reset();
    state.set(GameState::Running);
}
