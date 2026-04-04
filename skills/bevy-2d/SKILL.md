---
name: bevy-2d
description: 'Bevy 2D rendering and gameplay patterns: sprites, sprite sheets, 2D shapes, meshes, movement, rotation, transparency, bloom, cameras, viewport-to-world, and top-down camera tracking. Use when building 2D games or rendering 2D content.'
---

# Bevy 2D Rendering & Gameplay

## Camera

```rust
use bevy::prelude::*;

// Basic 2D camera
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

## Sprites

### Basic Sprite from Image

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(
        asset_server.load("textures/player.png"),
    ));
}
```

### Sprite with Color and Custom Size

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Tinted sprite
    commands.spawn(Sprite {
        image: asset_server.load("textures/icon.png"),
        color: Color::srgba(0.0, 0.0, 1.0, 0.7), // alpha for transparency
        custom_size: Some(Vec2::splat(160.0)),
        ..default()
    });
}
```

### Sprite Flipping

```rust
commands.spawn(Sprite {
    image: asset_server.load("textures/player.png"),
    flip_x: true,
    flip_y: false,
    ..default()
});
```

## Sprite Movement

Use `Time` for frame-rate independent movement:

```rust
#[derive(Component)]
enum Direction { Left, Right }

fn sprite_movement(time: Res<Time>, mut query: Query<(&mut Direction, &mut Transform)>) {
    for (mut dir, mut transform) in &mut query {
        match *dir {
            Direction::Right => transform.translation.x += 150.0 * time.delta_secs(),
            Direction::Left  => transform.translation.x -= 150.0 * time.delta_secs(),
        }
        if transform.translation.x > 200.0 {
            *dir = Direction::Left;
        } else if transform.translation.x < -200.0 {
            *dir = Direction::Right;
        }
    }
}
```

## Sprite Sheets & Animation

### Setting Up a Texture Atlas

```rust
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture = asset_server.load("textures/spritesheet.png");
    // Grid: each frame 24x24, 7 columns, 1 row
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 1, last: 6 };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: layout_handle,
                index: animation_indices.first,
            },
        ),
        Transform::from_scale(Vec3::splat(6.0)),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
```

### Looping Animation System

```rust
#[derive(Component)]
struct AnimationIndices { first: usize, last: usize }

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
```

### Event-Triggered Animation (Non-Looping)

```rust
use std::time::Duration;

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Timer::new(
                Duration::from_secs_f32(1.0 / fps as f32),
                TimerMode::Once,
            ),
        }
    }
}

// Trigger animation on input
fn trigger_animation(mut anim: Single<&mut AnimationConfig>) {
    anim.frame_timer = Timer::new(
        Duration::from_secs_f32(1.0 / anim.fps as f32),
        TimerMode::Once,
    );
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                atlas.index = config.first_sprite_index; // reset
            } else {
                atlas.index += 1;
                config.frame_timer = Timer::new(
                    Duration::from_secs_f32(1.0 / config.fps as f32),
                    TimerMode::Once,
                );
            }
        }
    }
}
```

## 2D Shapes (Mesh2d)

Render 2D vector shapes with `Mesh2d` + `MeshMaterial2d`:

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Circle
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));

    // Hexagon
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(50.0, 6))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
    ));

    // Rectangle
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.3, 0.8))),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));

    // Capsule, Ellipse, Annulus, Triangle, Rhombus also available
    commands.spawn((
        Mesh2d(meshes.add(Capsule2d::new(25.0, 50.0))),
        MeshMaterial2d(materials.add(Color::hsl(120.0, 0.95, 0.7))),
        Transform::from_xyz(0.0, 150.0, 0.0),
    ));
}
```

## Rotation

### Rotate Toward Cursor

```rust
use std::f32::consts::FRAC_PI_2;

fn rotate_to_cursor(
    mut player: Single<&mut Transform, With<Player>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
) {
    let (camera, camera_transform) = *camera;
    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        // FRAC_PI_2 offset if sprite faces "up" by default
        player.rotation = Quat::from_rotation_z(
            (world_pos - player.translation.xy()).to_angle() - FRAC_PI_2,
        );
    }
}
```

### Continuous Rotation

```rust
fn rotate_shapes(mut query: Query<&mut Transform, With<Mesh2d>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() / 2.0);
    }
}
```

## Viewport to World (2D Picking)

Convert screen coordinates to world coordinates:

```rust
fn draw_cursor(
    camera: Single<(&Camera, &GlobalTransform)>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera;
    if let Some(cursor_pos) = window.cursor_position()
        && let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos)
    {
        gizmos.circle_2d(world_pos, 10.0, Color::WHITE);
    }
}
```

## Top-Down Camera with Smooth Tracking

```rust
const PLAYER_SPEED: f32 = 100.0;
const CAMERA_DECAY_RATE: f32 = 2.0;

#[derive(Component)]
struct Player;

fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) { direction.y += 1.0; }
    if input.pressed(KeyCode::KeyS) { direction.y -= 1.0; }
    if input.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if input.pressed(KeyCode::KeyD) { direction.x += 1.0; }
    player.translation += direction.normalize_or_zero().extend(0.0) * PLAYER_SPEED * time.delta_secs();
}

fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let target = Vec3::new(player.translation.x, player.translation.y, camera.translation.z);
    // Smooth interpolation
    camera.translation.smooth_nudge(&target, CAMERA_DECAY_RATE, time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, update_camera).chain())
        .run();
}
```

## 2D Bloom

Bright colors in a dark scene produce glow:

```rust
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::post_process::bloom::Bloom;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
    ));

    // Bright sprite glows
    commands.spawn(Sprite {
        image: asset_server.load("icon.png"),
        color: Color::srgb(5.0, 5.0, 5.0), // values > 1.0 for bloom
        ..default()
    });

    // Bright mesh glows
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(materials.add(Color::srgb(7.5, 0.0, 7.5))),
    ));
}
```

## Z-Ordering

In 2D, `Transform.translation.z` determines draw order (higher z = on top):

```rust
// Background at z=0, player at z=1, UI at z=2
commands.spawn((sprite_bg, Transform::from_xyz(0.0, 0.0, 0.0)));
commands.spawn((sprite_player, Transform::from_xyz(0.0, 0.0, 1.0)));
commands.spawn((sprite_overlay, Transform::from_xyz(0.0, 0.0, 2.0)));
```

## Pixel Art Setup

Use nearest-neighbor filtering:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```
