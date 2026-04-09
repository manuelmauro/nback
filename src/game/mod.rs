use self::{
    session::{
        Session, SessionPlugin, answer::Answer, cue::CueTimer, engine::CueEngine, round::Round,
        score::Score,
    },
    settings::GameSettings,
    tile::{Tile, TileMeshes, TilePlugin},
    ui::{UiPlugin, button::GameButtonPlugin},
};
use crate::{config, state::AppState, theme};
use bevy::prelude::*;
pub mod phase;
pub mod score;
pub mod session;
pub mod settings;
pub mod tile;
pub mod ui;
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<phase::GamePhase>()
            .add_plugins((SessionPlugin, TilePlugin, UiPlugin, GameButtonPlugin))
            .add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(phase::GamePhase::Paused), spawn_pause_overlay)
            .add_systems(OnEnter(phase::GamePhase::Playing), despawn_pause_overlay)
            .add_systems(
                Update,
                pause_button_system.run_if(in_state(phase::GamePhase::Paused)),
            );
    }
}
/// Spawn the arena, the tile with its first cue, and the session entity.
fn setup_game(
    mut commands: Commands,
    settings: Res<GameSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let edge = (config::TILE_SIZE * 3.0) + (config::TILE_SPACING * 4.0);
    let bounds = Vec2::new(edge, edge);
    let marker = DespawnOnExit(AppState::Game);
    // Walls: left, right, bottom, top
    for (x, y, w, h) in [
        (
            -bounds.x / 2.0,
            0.0,
            config::WALL_THICKNESS,
            bounds.y + config::WALL_THICKNESS,
        ),
        (
            bounds.x / 2.0,
            0.0,
            config::WALL_THICKNESS,
            bounds.y + config::WALL_THICKNESS,
        ),
        (
            0.0,
            -bounds.y / 2.0,
            bounds.x + config::WALL_THICKNESS,
            config::WALL_THICKNESS,
        ),
        (
            0.0,
            bounds.y / 2.0,
            bounds.x + config::WALL_THICKNESS,
            config::WALL_THICKNESS,
        ),
    ] {
        commands.spawn((
            Sprite {
                color: config::WALL_COLOR,
                custom_size: Some(Vec2::new(w, h)),
                ..default()
            },
            Transform::from_xyz(x, y, 0.0),
            marker.clone(),
        ));
    }
    // Pre-build mesh handles for every shape variant.
    let tile_meshes = TileMeshes::new(&mut meshes);
    // Create engine and generate the first cue up-front so the player
    // sees a real cue from the start (no phantom round).
    let mut engine = CueEngine::new(
        settings.n,
        settings.position,
        settings.color,
        settings.shape,
        settings.sound,
    );
    let first = engine.new_cue();
    let tile_pos = first.position.unwrap_or_default();
    let tile_color = first.color.unwrap_or_default();
    let tile_shape = first.shape.unwrap_or_default();
    let tile_sound = first.sound.unwrap_or_default();
    let mesh_handle = tile_meshes.get(&tile_shape);
    let mat_handle = materials.add(ColorMaterial::from_color(Color::from(&tile_color)));
    commands.insert_resource(tile_meshes);
    // Spawn tile with the first cue already applied (mesh-based rendering).
    // Change-detection will fire on the first frame, playing the sound
    // and triggering the pop animation.
    commands.spawn((
        Name::new("tile"),
        Tile,
        Mesh2d(mesh_handle),
        MeshMaterial2d(mat_handle),
        Transform::from_translation((&tile_pos).into()),
        tile_pos,
        tile_color,
        tile_shape,
        tile_sound,
        marker.clone(),
    ));
    // Spawn session. The engine already consumed the first cue, and
    // current starts at 1 (round 0 is the cue we just displayed).
    // The timer starts fresh — the player gets the full duration.
    commands.spawn((
        Name::new("session"),
        Session,
        engine,
        CueTimer::with_duration(settings.round_time),
        Round {
            current: 1,
            total: settings.rounds,
        },
        Score::default(),
        Answer::default(),
        marker,
    ));
}
/// Toggle between Playing and Paused on Escape.
fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current: Res<State<phase::GamePhase>>,
    mut next: ResMut<NextState<phase::GamePhase>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        next.set(match current.get() {
            phase::GamePhase::Playing => phase::GamePhase::Paused,
            phase::GamePhase::Paused => phase::GamePhase::Playing,
        });
    }
}
#[derive(Component)]
struct PauseOverlay;
#[derive(Component)]
enum PauseAction {
    Resume,
    Quit,
}
fn spawn_pause_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://fonts/FiraSans-Bold.ttf");
    commands.spawn((
        PauseOverlay,
        DespawnOnExit(AppState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        GlobalZIndex(10),
        // Card
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::axes(theme::SP_XL, theme::SP_LG),
                row_gap: theme::SP_LG,
                min_width: px(300),
                border: UiRect::all(theme::STROKE_MD),
                border_radius: BorderRadius::all(theme::RADIUS_LG),
                ..default()
            },
            BackgroundColor(theme::SURFACE),
            BorderColor::all(theme::BORDER),
            children![
                (
                    Text::new("PAUSED"),
                    TextFont {
                        font: font.clone(),
                        font_size: 64.0,
                        ..default()
                    },
                    TextColor(theme::TEXT_ACCENT),
                ),
                pause_btn(
                    "Resume",
                    PauseAction::Resume,
                    theme::BUTTON_PRIMARY,
                    theme::TEXT_ON_ACCENT,
                    font.clone()
                ),
                pause_btn(
                    "Quit to Menu",
                    PauseAction::Quit,
                    theme::BUTTON_SECONDARY,
                    theme::TEXT,
                    font
                ),
            ],
        )],
    ));
}
fn pause_btn(
    label: &str,
    action: PauseAction,
    palette: theme::ButtonPalette,
    text: Color,
    font: Handle<Font>,
) -> impl Bundle + use<'_> {
    (
        Button,
        action,
        Node {
            width: percent(100),
            height: px(52),
            border: UiRect::all(theme::STROKE_SM),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(theme::RADIUS_MD),
            ..default()
        },
        BackgroundColor(palette.idle_bg),
        BorderColor::all(palette.idle_border),
        palette,
        children![(
            Text::new(label),
            TextFont {
                font,
                font_size: 26.0,
                ..default()
            },
            TextColor(text),
        )],
    )
}
type PauseQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w mut BorderColor,
    &'w PauseAction,
    &'w theme::ButtonPalette,
);

fn pause_button_system(
    mut next_phase: ResMut<NextState<phase::GamePhase>>,
    mut next_app: ResMut<NextState<AppState>>,
    mut query: Query<PauseQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, mut border, action, palette) in &mut query {
        theme::apply_button_palette(interaction, palette, &mut color, &mut border);

        if *interaction == Interaction::Pressed {
            match action {
                PauseAction::Resume => next_phase.set(phase::GamePhase::Playing),
                PauseAction::Quit => next_app.set(AppState::Menu),
            }
        }
    }
}
fn despawn_pause_overlay(mut commands: Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
