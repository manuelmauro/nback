---
name: bevy-plugins
description: 'Bevy plugin architecture: Plugin trait, PluginGroup, configurable plugins, and modular app composition. Use when organizing game code into reusable modules or creating library crates for Bevy.'
---

# Bevy Plugin Architecture

## Plugin Trait

Plugins are the primary way to organize and share Bevy functionality. A plugin is any type implementing `Plugin`:

```rust
use bevy::prelude::*;
use core::time::Duration;

struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DealDamage>()
            .add_message::<DamageReceived>()
            .init_resource::<DamageConfig>()
            .add_systems(Update, (
                apply_damage,
                process_damage_effects,
            ).chain());
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DamagePlugin))
        .run();
}
```

## Configurable Plugins

Plugins can accept configuration via fields:

```rust
struct PrintMessagePlugin {
    wait_duration: Duration,
    message: String,
}

impl Plugin for PrintMessagePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrintMessageState {
            message: self.message.clone(),
            timer: Timer::new(self.wait_duration, TimerMode::Repeating),
        })
        .add_systems(Update, print_message_system);
    }
}

#[derive(Resource)]
struct PrintMessageState {
    message: String,
    timer: Timer,
}

fn print_message_system(mut state: ResMut<PrintMessageState>, time: Res<Time>) {
    if state.timer.tick(time.delta()).is_finished() {
        info!("{}", state.message);
    }
}

// Usage
App::new()
    .add_plugins((
        DefaultPlugins,
        PrintMessagePlugin {
            wait_duration: Duration::from_secs(1),
            message: "Hello from plugin!".to_string(),
        },
    ))
    .run();
```

## PluginGroup

Group related plugins together:

```rust
use bevy::app::PluginGroupBuilder;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InputPlugin)
            .add(PhysicsPlugin)
            .add(AudioPlugin)
            .add(UiPlugin)
    }
}

// Usage — add all plugins in the group
App::new()
    .add_plugins((DefaultPlugins, GamePlugins))
    .run();

// Or selectively disable plugins
App::new()
    .add_plugins((
        DefaultPlugins,
        GamePlugins
            .build()
            .disable::<AudioPlugin>()
            .add_before::<UiPlugin>(DebugPlugin),
    ))
    .run();
```

## Plugin Best Practices

### 1. One Responsibility Per Plugin

```rust
// Good: focused plugin
struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_message::<ScoreChanged>()
            .add_systems(Update, update_score);
    }
}

// Group related plugins
pub struct GameplayPlugins;
impl PluginGroup for GameplayPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ScorePlugin)
            .add(HealthPlugin)
            .add(CombatPlugin)
    }
}
```

### 2. Plugin Encapsulates Its Resources, Components, and Systems

```rust
struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Register all types this plugin needs
        app.init_resource::<Inventory>()
            .add_message::<ItemPickedUp>()
            .add_message::<ItemDropped>()
            .add_systems(Startup, setup_inventory)
            .add_systems(Update, (
                handle_pickup,
                handle_drop,
                update_inventory_ui,
            ));
    }
}
```

### 3. State-Aware Plugins

```rust
struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_level)
            .add_systems(OnExit(AppState::InGame), cleanup_level)
            .add_systems(
                Update,
                (player_movement, enemy_ai, collision_check)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
```

### 4. DefaultPlugins Configuration

```rust
App::new()
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "My Game".into(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // pixel art
    )
    .run();
```

## Typical Project Structure

```
src/
├── main.rs              // App::new(), add_plugins, run()
├── plugins/
│   ├── mod.rs           // GamePlugins group
│   ├── player.rs        // PlayerPlugin
│   ├── enemy.rs         // EnemyPlugin
│   ├── combat.rs        // CombatPlugin
│   └── ui.rs            // UiPlugin
├── components/
│   ├── mod.rs
│   ├── player.rs        // Player, Health, Inventory components
│   └── world.rs         // Tile, Obstacle components
├── resources/
│   ├── mod.rs
│   └── game_state.rs    // GameConfig, Score resources
└── systems/
    ├── mod.rs
    ├── movement.rs
    └── combat.rs
```

```rust
// main.rs
use bevy::prelude::*;
mod plugins;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            plugins::GamePlugins,
        ))
        .run();
}

// plugins/mod.rs
use bevy::{app::PluginGroupBuilder, prelude::*};
mod player;
mod enemy;
mod combat;
mod ui;

pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(player::PlayerPlugin)
            .add(enemy::EnemyPlugin)
            .add(combat::CombatPlugin)
            .add(ui::UiPlugin)
    }
}
```
