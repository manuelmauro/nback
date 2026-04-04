---
name: bevy-ecs-patterns
description: 'Advanced Bevy ECS patterns: states, run conditions, system ordering, generic systems, custom SystemParam, custom QueryData, relationships, hierarchy, change detection, fixed timestep, entity disabling, and error handling. Use when structuring complex game logic, managing game states, or optimizing system execution.'
---

# Bevy ECS Advanced Patterns

## States

States control high-level application flow with enter/exit schedules:

```rust
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Menu,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // Run on state entry/exit
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        // Run only in specific state
        .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
        .add_systems(Update, (movement, combat).run_if(in_state(AppState::InGame)))
        .run();
}

// Transition states via NextState resource
fn menu_input(
    mut next_state: ResMut<NextState<AppState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Enter) {
        next_state.set(AppState::InGame);
    }
}
```

### State-Scoped Entity Cleanup

Entities auto-despawn when entering/exiting states:

```rust
fn setup_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(AppState::Menu),  // despawned when leaving Menu
        Text::new("Press Enter to Play"),
        // ...
    ));
}

fn on_exit_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnEnter(AppState::Menu), // despawned when re-entering Menu
        Text::new("Goodbye menu!"),
    ));
}
```

## Run Conditions

Control when systems execute:

```rust
App::new()
    .add_systems(Update, (
        // Built-in conditions
        my_system.run_if(resource_exists::<MyResource>),

        // Combined with .or()
        my_system.run_if(resource_exists::<Unused>.or(has_user_input)),

        // Combined with .and()
        print_score.run_if(
            resource_exists::<Score>.and(|score: Res<Score>| score.is_changed())
        ),

        // Negation
        show_hint.run_if(not(time_passed(5.0))),
    ))
```

### Custom Run Conditions

A run condition is any system returning `bool` with read-only params:

```rust
fn has_user_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
) -> bool {
    keyboard.just_pressed(KeyCode::Space)
        || mouse.just_pressed(MouseButton::Left)
}

// Parameterized run condition (returns closure)
fn time_passed(threshold: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut elapsed: Local<f32>, time: Res<Time>| {
        *elapsed += time.delta_secs();
        *elapsed >= threshold
    }
}
```

## System Ordering

```rust
App::new()
    // Chain: run in strict sequence
    .add_systems(Update, (system_a, system_b, system_c).chain())

    // Before/After
    .add_systems(Update, (
        system_a.before(system_b),
        system_b,
        system_c.after(system_b),
    ))
```

## Fixed Timestep

Physics and simulation logic belongs in `FixedUpdate`:

```rust
App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
    .add_systems(Update, render_system)         // every frame
    .add_systems(FixedUpdate, physics_system)    // fixed rate
```

### Physics Interpolation Pattern

Separate physical state from visual representation:

```rust
#[derive(Component, Default, Deref, DerefMut)]
struct PhysicalTranslation(Vec3);

#[derive(Component, Default, Deref, DerefMut)]
struct PreviousPhysicalTranslation(Vec3);

#[derive(Component, Default, Deref, DerefMut)]
struct Velocity(Vec3);

// Runs at fixed rate
fn advance_physics(
    mut query: Query<(
        &Velocity,
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
    )>,
    time: Res<Time>,
) {
    for (velocity, mut pos, mut prev_pos) in &mut query {
        **prev_pos = **pos;
        **pos += **velocity * time.delta_secs();
    }
}

// Runs every frame — smooth visual interpolation
fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    let alpha = fixed_time.overstep_fraction();
    for (mut transform, current, previous) in &mut query {
        transform.translation = previous.lerp(**current, alpha);
    }
}

// Schedule setup
App::new()
    .add_systems(FixedUpdate, advance_physics)
    .add_systems(
        RunFixedMainLoop,
        (
            accumulate_input.in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
            interpolate_rendered_transform.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
        ),
    )
```

## Generic Systems

Reuse system logic across multiple component types:

```rust
#[derive(Component)]
struct MenuMarker;

#[derive(Component)]
struct LevelMarker;

// Generic cleanup system
fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

App::new()
    .add_systems(OnExit(AppState::Menu), cleanup::<MenuMarker>)
    .add_systems(OnExit(AppState::InGame), cleanup::<LevelMarker>)
```

## Custom SystemParam

Group commonly used parameters:

```rust
use bevy::ecs::system::SystemParam;

#[derive(SystemParam)]
struct PlayerCounter<'w, 's> {
    players: Query<'w, 's, &'static Player>,
    count: ResMut<'w, PlayerCount>,
}

impl<'w, 's> PlayerCounter<'w, 's> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

fn update_count(mut counter: PlayerCounter) {
    counter.count();
    info!("{} players", counter.count.0);
}
```

## Custom QueryData

Named structs for complex queries:

```rust
use bevy::ecs::query::{QueryData, QueryFilter};

#[derive(QueryData)]
#[query_data(derive(Debug))]
struct PlayerQuery {
    entity: Entity,
    name: &'static Name,
    health: &'static Health,
    transform: &'static Transform,
}

// Mutable version
#[derive(QueryData)]
#[query_data(mutable)]
struct PlayerQueryMut {
    entity: Entity,
    health: &'static mut Health,
    transform: &'static mut Transform,
}

// Custom filter
#[derive(QueryFilter)]
struct ActivePlayerFilter {
    _player: With<Player>,
    _alive: Without<Dead>,
    _or: Or<(Changed<Health>, Changed<Transform>)>,
}

fn inspect_players(query: Query<PlayerQuery, ActivePlayerFilter>) {
    for player in &query {
        info!("{}: {} HP at {:?}", player.name, player.health.0, player.transform.translation);
    }
}
```

## Relationships

Custom entity-to-entity relationships:

```rust
/// Source of truth: who this entity targets
#[derive(Component)]
#[relationship(relationship_target = TargetedBy)]
struct Targeting(Entity);

/// Reverse: who is targeting this entity (auto-maintained)
#[derive(Component)]
#[relationship_target(relationship = Targeting)]
struct TargetedBy(Vec<Entity>);

fn setup(mut commands: Commands) {
    let alice = commands.spawn(Name::new("Alice")).id();
    let bob = commands.spawn((Name::new("Bob"), Targeting(alice))).id();

    // Relationship components are immutable — replace to change
    commands.entity(bob).insert(Targeting(some_other_entity));

    // Remove relationship
    commands.entity(bob).remove::<Targeting>();
}

// Traverse relationships
fn find_cycles(query: Query<&Targeting>) {
    for targeting in query.iter_ancestors(start_entity) {
        // walks up the Targeting chain
    }
}
```

## Change Detection

```rust
fn detect_changes(
    changed: Query<Ref<Health>, Changed<Health>>,
    resource: Res<GameState>,
) {
    for health in &changed {
        info!(
            "HP={}, added={}, changed={}, by={}",
            health.0,
            health.is_added(),
            health.is_changed(),
            health.changed_by() // requires track_location feature
        );
    }

    if resource.is_changed() {
        info!("GameState changed");
    }
}

// Avoid spurious change detection with set_if_neq
fn update_carefully(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        let new_value = calculate_health();
        health.set_if_neq(Health(new_value));
    }
}
```

## Entity Disabling

Hide entities from ECS without deleting:

```rust
use bevy::ecs::entity_disabling::Disabled;

// Disable an entity (hidden from all default queries)
fn disable(mut commands: Commands, entity: Entity) {
    commands.entity(entity).insert(Disabled);
}

// Re-enable
fn enable(mut commands: Commands, entity: Entity) {
    commands.entity(entity).remove::<Disabled>();
}

// Query that explicitly includes disabled entities
fn find_all(query: Query<Entity, With<Disabled>>) {
    for entity in &query {
        info!("Found disabled entity: {}", entity);
    }
}

// Has<Disabled> also bypasses the default filter
fn check_status(query: Query<(Entity, Has<Disabled>)>) {
    for (entity, is_disabled) in &query {
        info!("{}: disabled={}", entity, is_disabled);
    }
}
```

## Error Handling

```rust
// Set global error handler
App::new()
    .set_error_handler(bevy::ecs::error::warn)  // warn instead of panic

// Fallible systems return Result
fn risky_system(resource: Option<Res<MyResource>>) -> Result {
    let res = resource.ok_or("Resource missing")?;
    // ... use res
    Ok(())
}

// Per-system error handling with pipe
App::new()
    .add_systems(Update, risky_system.pipe(|result: In<Result>| {
        if let Err(e) = result.0 {
            warn!("Handled: {e}");
        }
    }))
```

## Hierarchy Traversal

```rust
fn traverse(
    parents_query: Query<(Entity, &Children), With<Sprite>>,
    transform_query: Query<&mut Transform>,
) {
    for (parent, children) in &parents_query {
        for child in children {
            if let Ok(transform) = transform_query.get(*child) {
                // process child
            }
        }
    }
}
```
