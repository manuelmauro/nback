---
name: bevy-ecs-fundamentals
description: 'Core Bevy ECS patterns: components, resources, systems, queries, commands, and App setup. Use when creating new Bevy projects, defining game data types, writing systems, or structuring App builders.'
---

# Bevy ECS Fundamentals

## Components

Components are plain Rust data types attached to entities. Derive `Component`:

```rust
use bevy::prelude::*;

// Simple struct component
#[derive(Component)]
struct Player {
    name: String,
}

// Newtype component
#[derive(Component)]
struct Health(f32);

// Enum component
#[derive(Component)]
enum PlayerState {
    Idle,
    Running,
    Jumping,
}

// Marker component (zero-sized)
#[derive(Component)]
struct Enemy;

// Component with Deref/DerefMut for ergonomic access to inner value
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

// Required components: Wall automatically requires Sprite + Transform + Collider
#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Sprite, Transform, Collider)]
struct Wall;
```

### Immutable Components

Immutable components cannot be mutated once inserted — only replaced or removed:

```rust
#[derive(Component)]
#[component(immutable)]
struct ImmutableTag(u32);
```

## Resources

Resources are global singletons. Derive `Resource`:

```rust
#[derive(Resource, Default)]
struct GameState {
    score: usize,
    level: u32,
}

#[derive(Resource)]
struct GameConfig {
    max_players: usize,
    difficulty: f32,
}
```

## Systems

Systems are plain functions whose parameters are ECS system params:

```rust
// Simple system with no parameters
fn greet() {
    println!("Hello!");
}

// Read-only resource access
fn print_score(state: Res<GameState>) {
    info!("Score: {}", state.score);
}

// Mutable resource access
fn increment_score(mut state: ResMut<GameState>) {
    state.score += 1;
}

// Query — iterate entities with specific components
fn move_players(mut query: Query<(&Player, &mut Transform, &Velocity)>) {
    for (player, mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

// Startup system — runs once at app start
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

// Exclusive system — full World access (blocks parallelism)
fn exclusive_system(world: &mut World) {
    let count = world.query::<&Player>().iter(world).count();
    info!("{count} players");
}
```

### System with Local State

`Local<T>` provides per-system persistent state:

```rust
fn count_frames(mut frame_count: Local<u32>) {
    *frame_count += 1;
    info!("Frame {}", *frame_count);
}
```

### Fallible Systems

Systems can return `Result` for error handling:

```rust
fn fallible_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result {
    let mesh = Sphere::new(1.0).mesh().ico(7)?;
    commands.spawn(Mesh3d(meshes.add(mesh)));
    Ok(())
}
```

## Queries

### Basic Query Patterns

```rust
// Immutable access
fn read_positions(query: Query<&Transform>) {
    for transform in &query {
        info!("{:?}", transform.translation);
    }
}

// Mutable access
fn reset_positions(mut query: Query<&mut Transform>) {
    for mut transform in &mut query {
        transform.translation = Vec3::ZERO;
    }
}

// Multiple components + Entity ID
fn inspect(query: Query<(Entity, &Player, &Health)>) {
    for (entity, player, health) in &query {
        info!("{}: {} HP={}", entity, player.name, health.0);
    }
}

// Optional components
fn optional_query(query: Query<(&Player, Option<&Health>)>) {
    for (player, maybe_health) in &query {
        if let Some(health) = maybe_health {
            info!("{} has {} HP", player.name, health.0);
        }
    }
}
```

### Query Filters

```rust
// With/Without filters
fn enemies_only(query: Query<&Transform, With<Enemy>>) { /* ... */ }
fn non_enemies(query: Query<&Transform, Without<Enemy>>) { /* ... */ }

// Changed/Added filters
fn on_health_change(query: Query<(&Player, &Health), Changed<Health>>) {
    for (player, health) in &query {
        info!("{} health changed to {}", player.name, health.0);
    }
}

// Combined filters
fn complex_filter(
    query: Query<&Transform, (With<Player>, Without<Enemy>, Changed<Transform>)>,
) { /* ... */ }

// Or filter
fn either(
    query: Query<Entity, Or<(With<Player>, With<Enemy>)>>,
) { /* ... */ }
```

### Single Entity Query

Use `Single` when exactly one entity matches:

```rust
fn update_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    camera.translation = player.translation;
}
```

### Ref for Change Detection Metadata

```rust
fn detect_changes(query: Query<Ref<Health>, Changed<Health>>) {
    for health in &query {
        info!(
            "HP={}, added={}, changed={}, changed_by={}",
            health.0,
            health.is_added(),
            health.is_changed(),
            health.changed_by()
        );
    }
}
```

### Parallel Query Iteration

```rust
fn parallel_move(mut query: Query<(&Velocity, &mut Transform)>) {
    query.par_iter_mut().for_each(|(velocity, mut transform)| {
        transform.translation += velocity.extend(0.0);
    });
}
```

## Commands

Commands defer mutations to the World:

```rust
fn spawn_entities(mut commands: Commands) {
    // Spawn with a bundle (tuple of components)
    commands.spawn((
        Player { name: "Alice".into() },
        Health(100.0),
        Velocity(Vec2::ZERO),
        Transform::default(),
    ));

    // Spawn batch
    commands.spawn_batch(vec![
        (Player { name: "Bob".into() }, Health(80.0)),
        (Player { name: "Carol".into() }, Health(90.0)),
    ]);

    // Insert/remove on existing entity
    let entity = commands.spawn(Enemy).id();
    commands.entity(entity).insert(Health(50.0));
    commands.entity(entity).remove::<Health>();

    // Despawn entity
    commands.entity(entity).despawn();

    // Insert resource
    commands.insert_resource(GameConfig {
        max_players: 4,
        difficulty: 1.0,
    });
}
```

### Hierarchy with Commands

```rust
fn spawn_hierarchy(mut commands: Commands) {
    // Using with_children
    commands
        .spawn((Name::new("Parent"), Transform::default()))
        .with_children(|parent| {
            parent.spawn((Name::new("Child"), Transform::from_xyz(1.0, 0.0, 0.0)));
        });

    // Using add_child
    let parent = commands.spawn(Name::new("Parent")).id();
    let child = commands.spawn(Name::new("Child")).id();
    commands.entity(parent).add_child(child);

    // Using children! macro
    commands.spawn((
        Name::new("Parent"),
        children![
            (Name::new("Child A"),),
            (Name::new("Child B"),),
        ],
    ));
}
```

## App Builder

```rust
fn main() {
    App::new()
        // Plugin groups (DefaultPlugins includes window, rendering, input, etc.)
        .add_plugins(DefaultPlugins)
        // Resources
        .init_resource::<GameState>()              // requires Default
        .insert_resource(GameConfig {              // explicit value
            max_players: 4,
            difficulty: 1.0,
        })
        // Startup systems (run once)
        .add_systems(Startup, setup)
        // Update systems (run every frame)
        .add_systems(Update, (
            move_players,
            check_collisions,
            update_score,
        ))
        // Fixed timestep systems
        .add_systems(FixedUpdate, physics_step)
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        // System ordering
        .add_systems(Update, (
            read_input,
            apply_input,
            render,
        ).chain())  // runs in sequence
        .run();
}
```

## SystemSet for Ordering

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameSystems {
    Input,
    Physics,
    Render,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .configure_sets(Update, (
            GameSystems::Input,
            GameSystems::Physics,
            GameSystems::Render,
        ).chain())
        .add_systems(Update, read_input.in_set(GameSystems::Input))
        .add_systems(Update, apply_physics.in_set(GameSystems::Physics))
        .add_systems(Update, draw.in_set(GameSystems::Render))
        .run();
}
```
