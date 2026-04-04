---
name: bevy-assets-rendering
description: 'Bevy asset loading and rendering setup: AssetServer, Mesh3d, MeshMaterial3d, Sprite, Camera, Transform, lighting, and scene composition. Use when loading assets, setting up 3D/2D scenes, or working with materials and meshes.'
---

# Bevy Assets & Rendering

## Asset Loading

### AssetServer Basics

Assets load from the `assets/` directory by default:

```rust
use bevy::prelude::*;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load a texture
    let texture: Handle<Image> = asset_server.load("textures/player.png");

    // Load a glTF mesh (specific primitive)
    let mesh = asset_server.load(
        GltfAssetLabel::Primitive { mesh: 0, primitive: 0 }
            .from_asset("models/cube/cube.gltf"),
    );

    // Load an audio file
    let sound: Handle<AudioSource> = asset_server.load("sounds/hit.ogg");

    // Load entire folder
    let _folder: Handle<LoadedFolder> = asset_server.load_folder("models/");
}
```

### Creating Assets Programmatically

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    // 3D mesh from primitive
    let sphere = meshes.add(Sphere::new(1.0));
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let plane = meshes.add(Plane3d::default().mesh().size(10.0, 10.0));

    // 3D PBR material
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.3),
        metallic: 0.5,
        perceptual_roughness: 0.3,
        ..default()
    });

    // 2D color material
    let color_mat = color_materials.add(Color::srgb(0.5, 0.5, 1.0));
}
```

### Modifying Assets After Spawn

```rust
fn modify_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d>,
) {
    for mesh_handle in &query {
        if let Some(mesh) = meshes.get_mut(mesh_handle) {
            // Modify the mesh data
        }
    }
}
```

## 3D Scene Setup

### Minimal 3D Scene

```rust
fn setup_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    // Sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.8),
            metallic: 0.8,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));
}
```

### Lighting Types

```rust
fn setup_lights(mut commands: Commands) {
    // Point light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));

    // Spot light
    commands.spawn((
        SpotLight {
            intensity: 1_000_000.0,
            range: 30.0,
            outer_angle: 0.8,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
```

### Required Components Pattern for 3D Entities

```rust
// Wall auto-requires Sprite + Transform + Collider
#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;

impl Wall {
    fn new(position: Vec2, size: Vec2) -> (Wall, Sprite, Transform) {
        (
            Wall,
            Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), Vec2::ONE),
            Transform {
                translation: position.extend(0.0),
                scale: size.extend(1.0),
                ..default()
            },
        )
    }
}
```

## 2D Scene Setup

### Minimal 2D Scene

```rust
fn setup_2d(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // Sprite from image
    commands.spawn(Sprite::from_image(asset_server.load("textures/player.png")));

    // Sprite with color
    commands.spawn((
        Sprite::from_color(Color::srgb(0.3, 0.3, 0.8), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}
```

### Sprite Sheet Animation

```rust
#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

fn setup_spritesheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/spritesheet.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(texture, TextureAtlas {
            layout: layout_handle,
            index: 0,
        }),
        AnimationConfig {
            first_sprite_index: 0,
            last_sprite_index: 6,
            fps: 12,
            frame_timer: Timer::from_seconds(1.0 / 12.0, TimerMode::Repeating),
        },
    ));
}

fn animate_sprites(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());
        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= config.last_sprite_index {
                    config.first_sprite_index
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}
```

### 2D Mesh

```rust
fn setup_2d_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // 2D shapes
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.3))),
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(50.0, 6))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));
}
```

## Transforms

```rust
// Position
Transform::from_xyz(1.0, 2.0, 3.0)

// Scale
Transform::from_scale(Vec3::splat(2.0))

// Rotation
Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_4))

// Combined
Transform {
    translation: Vec3::new(1.0, 2.0, 0.0),
    rotation: Quat::from_rotation_z(0.5),
    scale: Vec3::splat(0.75),
    ..default()
}

// Look at target
Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y)

// Parent-child transforms propagate automatically
fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Spinner>>) {
    for mut transform in &mut query {
        transform.rotate_z(std::f32::consts::PI * time.delta_secs());
    }
}
```

## Camera Patterns

```rust
// 3D camera with custom projection
commands.spawn((
    Camera3d::default(),
    Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
));

// 2D camera
commands.spawn(Camera2d);

// Orthographic 3D
commands.spawn((
    Camera3d::default(),
    Projection::from(OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
            viewport_height: 10.0,
        },
        ..OrthographicProjection::default_3d()
    }),
    Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
));

// Viewport to world ray (3D picking)
fn click_to_world(
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let (camera, transform) = *camera;
    if let Ok(window) = windows.single() {
        if let Some(world_pos) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            info!("World position: {world_pos}");
        }
    }
}
```

## Audio

```rust
fn setup_audio(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Play background music
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/background.ogg")),
        PlaybackSettings::LOOP,
    ));
}

// Play sound effect via observer
fn play_collision_sound(
    _trigger: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>,
) {
    commands.spawn(AudioPlayer::new(sound.0.clone()));
}
```
