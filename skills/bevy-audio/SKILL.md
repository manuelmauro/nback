---
name: bevy-audio
description: Bevy audio playback, control, spatial audio, soundtracks, volume, pause, and fade effects. Use when adding sound effects, music, spatial audio, or audio state management to a Bevy game.
---

# Bevy Audio

## Basic Audio Playback

### Play Audio on Startup

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioPlayer::new(
        asset_server.load("sounds/music.ogg"),
    ));
}
```

### Looping Background Music

```rust
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/background.ogg")),
        PlaybackSettings::LOOP,
    ));
}
```

### One-Shot Sound Effect

```rust
fn play_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/hit.ogg")),
        PlaybackSettings::DESPAWN, // despawn entity when done
    ));
}
```

## Audio Control with AudioSink

Once audio starts playing, the `AudioSink` component is added automatically. Use it to control playback:

```rust
#[derive(Component)]
struct MyMusic;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("sounds/music.ogg")),
        PlaybackSettings::LOOP,
        MyMusic,
    ));
}
```

### Pause / Resume

```rust
fn toggle_playback(
    keyboard: Res<ButtonInput<KeyCode>>,
    music: Query<&AudioSink, With<MyMusic>>,
) {
    let Ok(sink) = music.single() else { return };
    if keyboard.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
}
```

### Mute / Unmute

```rust
fn toggle_mute(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut music: Query<&mut AudioSink, With<MyMusic>>,
) {
    let Ok(mut sink) = music.single_mut() else { return };
    if keyboard.just_pressed(KeyCode::KeyM) {
        sink.toggle_mute();
    }
}
```

### Volume Control

```rust
fn adjust_volume(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut music: Query<&mut AudioSink, With<MyMusic>>,
) {
    let Ok(mut sink) = music.single_mut() else { return };
    if keyboard.just_pressed(KeyCode::Equal) {
        let current = sink.volume();
        sink.set_volume(current.increase_by_percentage(10.0));
    } else if keyboard.just_pressed(KeyCode::Minus) {
        let current = sink.volume();
        sink.set_volume(current.increase_by_percentage(-10.0));
    }
}
```

### Speed Control

```rust
fn update_speed(music: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
    let Ok(sink) = music.single() else { return };
    if sink.is_paused() { return; }
    // Oscillate speed between 0.1 and 2.0
    sink.set_speed((ops::sin(time.elapsed_secs() / 5.0) + 1.0).max(0.1));
}
```

### Track Position

```rust
fn show_progress(music: Single<&AudioSink, With<MyMusic>>) {
    info!("Progress: {}s", music.position().as_secs_f32());
}
```

## Sound Effects via Observers

Play sounds in response to game events:

```rust
#[derive(Event)]
struct BallCollided;

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound = asset_server.load("sounds/collision.ogg");
    commands.insert_resource(CollisionSound(sound));
}

fn play_collision_sound(
    _trigger: On<BallCollided>,
    mut commands: Commands,
    sound: Res<CollisionSound>,
) {
    commands.spawn(AudioPlayer::new(sound.0.clone()));
}

// Register as observer
App::new()
    .add_observer(play_collision_sound)
```

## Soundtrack System (State-Based Music)

Switch music tracks based on game state with crossfade:

```rust
use bevy::audio::Volume;

#[derive(Resource, Default)]
enum GameState {
    #[default]
    Peaceful,
    Battle,
}

#[derive(Resource)]
struct SoundtrackPlayer {
    track_list: Vec<Handle<AudioSource>>,
}

#[derive(Component)]
struct FadeIn;

#[derive(Component)]
struct FadeOut;

const FADE_TIME: f32 = 2.0;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tracks = vec![
        asset_server.load::<AudioSource>("sounds/peaceful.ogg"),
        asset_server.load::<AudioSource>("sounds/battle.ogg"),
    ];
    commands.insert_resource(SoundtrackPlayer { track_list: tracks });
    commands.insert_resource(GameState::default());
}

/// Triggered when GameState changes
fn change_track(
    mut commands: Commands,
    soundtrack: Res<SoundtrackPlayer>,
    playing: Query<Entity, With<AudioSink>>,
    game_state: Res<GameState>,
) {
    if !game_state.is_changed() { return; }

    // Fade out current tracks
    for entity in &playing {
        commands.entity(entity).insert(FadeOut);
    }

    // Start new track at silent volume
    let track = match *game_state {
        GameState::Peaceful => soundtrack.track_list[0].clone(),
        GameState::Battle => soundtrack.track_list[1].clone(),
    };

    commands.spawn((
        AudioPlayer(track),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::SILENT,
            ..default()
        },
        FadeIn,
    ));
}

fn fade_in(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeIn>>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    for (mut sink, entity) in &mut audio_sink {
        sink.set_volume(
            Volume::SILENT.fade_towards(Volume::Linear(1.0), *elapsed / FADE_TIME),
        );
        if *elapsed >= FADE_TIME {
            sink.set_volume(Volume::Linear(1.0));
            commands.entity(entity).remove::<FadeIn>();
            *elapsed = 0.0;
        }
    }
}

fn fade_out(
    mut commands: Commands,
    mut audio_sink: Query<(&mut AudioSink, Entity), With<FadeOut>>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();
    for (mut sink, entity) in &mut audio_sink {
        sink.set_volume(
            Volume::Linear(1.0).fade_towards(Volume::SILENT, *elapsed / FADE_TIME),
        );
        if *elapsed >= FADE_TIME {
            commands.entity(entity).despawn();
            *elapsed = 0.0;
        }
    }
}
```

## Spatial Audio (2D)

Audio that changes based on distance and position:

```rust
use bevy::audio::{AudioPlugin, SpatialScale};

const AUDIO_SCALE: f32 = 1.0 / 100.0; // 100 pixels = 1 audio unit

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AudioPlugin {
            default_spatial_scale: SpatialScale::new_2d(AUDIO_SCALE),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Sound emitter (moving entity)
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        AudioPlayer::new(asset_server.load("sounds/ambient.ogg")),
        PlaybackSettings::LOOP.with_spatial(true),
    ));

    // Spatial listener with left/right ear gap
    let gap = 400.0;
    commands.spawn((
        Transform::default(),
        Visibility::default(),
        SpatialListener::new(gap),
        children![
            // Left ear
            (
                Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::splat(20.0)),
                Transform::from_xyz(-gap / 2.0, 0.0, 0.0),
            ),
            // Right ear
            (
                Sprite::from_color(Color::srgb(0.0, 1.0, 0.0), Vec2::splat(20.0)),
                Transform::from_xyz(gap / 2.0, 0.0, 0.0),
            ),
        ],
    ));
}
```

### Moving the Listener

```rust
fn update_listener(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut listener: Single<&mut Transform, With<SpatialListener>>,
) {
    let speed = 200.0;
    if keyboard.pressed(KeyCode::ArrowRight) { listener.translation.x += speed * time.delta_secs(); }
    if keyboard.pressed(KeyCode::ArrowLeft)  { listener.translation.x -= speed * time.delta_secs(); }
    if keyboard.pressed(KeyCode::ArrowUp)    { listener.translation.y += speed * time.delta_secs(); }
    if keyboard.pressed(KeyCode::ArrowDown)  { listener.translation.y -= speed * time.delta_secs(); }
}
```

## Spatial Audio (3D)

For 3D, use `SpatialScale::new` with a 3D factor:

```rust
App::new()
    .add_plugins(DefaultPlugins.set(AudioPlugin {
        default_spatial_scale: SpatialScale::new(Vec3::splat(1.0)),
        ..default()
    }))
```

The emitter and listener work the same — just use 3D `Transform` positions.

## PlaybackSettings Reference

```rust
// One-shot (default): plays once, entity stays
PlaybackSettings::ONCE

// Loop forever
PlaybackSettings::LOOP

// Play once, then despawn the entity
PlaybackSettings::DESPAWN

// Remove the AudioPlayer component when done (keeps entity)
PlaybackSettings::REMOVE

// Custom
PlaybackSettings {
    mode: bevy::audio::PlaybackMode::Loop,
    volume: Volume::new(0.5),
    speed: 1.0,
    paused: false,
    spatial: true,
    ..default()
}

// Spatial modifier
PlaybackSettings::LOOP.with_spatial(true)
```

## App Setup Checklist

```rust
App::new()
    .add_plugins(DefaultPlugins)
    // For spatial audio, configure AudioPlugin:
    // .add_plugins(DefaultPlugins.set(AudioPlugin {
    //     default_spatial_scale: SpatialScale::new_2d(1.0 / 100.0),
    //     ..default()
    // }))
    .add_systems(Startup, setup_audio)
    .add_systems(Update, (
        toggle_playback,
        adjust_volume,
    ))
    .add_observer(play_collision_sound)
    .run();
```
