---
name: bevy-ecs-events
description: 'Bevy message/event patterns: MessageWriter, MessageReader, MessageMutator, observers, EntityEvent, event propagation, and one-shot systems. Use when implementing communication between systems, reacting to component lifecycle, or creating event-driven architectures.'
---

# Bevy ECS Events & Observers

## Messages (formerly Events)

Messages are the primary way to communicate between systems. Define with `#[derive(Message)]`:

```rust
use bevy::prelude::*;

#[derive(Message, Debug)]
struct DealDamage {
    pub amount: i32,
}

#[derive(Message, Debug, Default)]
struct DamageReceived;
```

### Registering Messages

Messages must be registered on the App:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<DealDamage>()
        .add_message::<DamageReceived>()
        .run();
}
```

### Sending Messages with MessageWriter

```rust
fn deal_damage(
    time: Res<Time>,
    mut timer: ResMut<DamageTimer>,
    mut writer: MessageWriter<DealDamage>,
) {
    if timer.0.tick(time.delta()).is_finished() {
        writer.write(DealDamage { amount: 10 });
    }
}

// For Default types:
fn send_default(mut writer: MessageWriter<DamageReceived>) {
    writer.write_default();
}
```

### Reading Messages with MessageReader

Each system independently tracks which messages it has read:

```rust
fn receive_damage(mut reader: MessageReader<DealDamage>) {
    for damage in reader.read() {
        info!("Took {} damage", damage.amount);
    }
}

// Multiple systems can read the same message type independently
fn play_sound(mut reader: MessageReader<DamageReceived>) {
    for _ in reader.read() {
        info!("Playing damage sound");
    }
}

fn spawn_particles(mut reader: MessageReader<DamageReceived>) {
    for _ in reader.read() {
        info!("Spawning particle effect");
    }
}
```

### Mutating Messages with MessageMutator

Modify messages in-flight between writer and reader:

```rust
fn apply_armor(
    mut damage_messages: MessageMutator<DealDamage>,
    mut blocked_writer: MessageWriter<ArmorBlocked>,
) {
    for message in damage_messages.read() {
        message.amount -= 5; // reduce by armor value
        if message.amount <= 0 {
            blocked_writer.write(ArmorBlocked);
        }
    }
}
```

### System Ordering for Messages

Writers must run before readers. Use `.chain()`:

```rust
App::new()
    .add_systems(Update, (
        deal_damage,
        apply_armor,
        receive_damage,
    ).chain())
    .add_systems(Update, (
        play_sound,
        spawn_particles,
    )) // these can run in parallel, may have 1-frame delay
```

## Observers

Observers are systems that run reactively when events are triggered:

### Custom Events

```rust
#[derive(Event)]
struct ExplodeMines {
    pos: Vec2,
    radius: f32,
}
```

### Entity Events

`EntityEvent` targets a specific entity:

```rust
#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}
```

### Registering Global Observers

```rust
App::new()
    .add_observer(|trigger: On<ExplodeMines>, mut commands: Commands| {
        info!("Explosion at {:?}", trigger.pos);
    })
    .add_observer(on_add_mine)    // component lifecycle
    .add_observer(on_remove_mine)
```

### Component Lifecycle Observers

React to `Add`, `Insert`, `Replace`, `Remove` events on components:

```rust
#[derive(Component)]
struct Mine { pos: Vec2, size: f32 }

#[derive(Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), HashSet<Entity>>,
}

// Triggered when Mine component is added to an entity
fn on_add_mine(
    trigger: On<Add, Mine>,
    query: Query<&Mine>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity).unwrap();
    let tile = ((mine.pos.x / 64.0).floor() as i32, (mine.pos.y / 64.0).floor() as i32);
    index.map.entry(tile).or_default().insert(trigger.entity);
}

// Triggered when Mine component is removed (including despawn)
fn on_remove_mine(
    trigger: On<Remove, Mine>,
    query: Query<&Mine>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity).unwrap();
    let tile = ((mine.pos.x / 64.0).floor() as i32, (mine.pos.y / 64.0).floor() as i32);
    index.map.entry(tile).and_modify(|set| { set.remove(&trigger.entity); });
}
```

### Entity-Scoped Observers

Watch events on specific entities:

```rust
fn setup(mut commands: Commands) {
    // Observer on a single entity
    commands
        .spawn(Mine { pos: Vec2::ZERO, size: 4.0 })
        .observe(|trigger: On<Explode>, query: Query<&Mine>, mut commands: Commands| {
            info!("Mine {} exploded!", trigger.entity);
            commands.entity(trigger.entity).despawn();
        });

    // Shared observer for multiple entities
    let mut observer = Observer::new(explode_handler);
    for _ in 0..100 {
        let entity = commands.spawn(Mine { pos: Vec2::ZERO, size: 4.0 }).id();
        observer.watch_entity(entity);
    }
    commands.spawn(observer);
}
```

### Triggering Events from Systems

```rust
fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
) {
    if mouse.just_pressed(MouseButton::Left) {
        // Trigger a global event
        commands.trigger(ExplodeMines { pos: Vec2::ZERO, radius: 10.0 });

        // Trigger an entity event
        commands.trigger(Explode { entity: some_entity });
    }
}
```

### Event Propagation (Bubbling)

Events can bubble up the entity hierarchy:

```rust
// Enable propagation with entity_event attribute
#[derive(Clone, Component, EntityEvent)]
#[entity_event(propagate, auto_propagate)]
struct Attack {
    entity: Entity,
    damage: u16,
}

#[derive(Component, Deref, DerefMut)]
struct HitPoints(u16);

#[derive(Component, Deref)]
struct Armor(u16);

fn setup(mut commands: Commands) {
    commands
        .spawn((Name::new("Goblin"), HitPoints(50)))
        .observe(take_damage)           // final handler
        .with_children(|parent| {
            parent
                .spawn((Name::new("Helmet"), Armor(5)))
                .observe(block_attack); // intercept before parent
        });
}

fn block_attack(mut attack: On<Attack>, armor: Query<(&Armor, &Name)>) {
    let (armor, name) = armor.get(attack.entity).unwrap();
    let remaining = attack.damage.saturating_sub(**armor);
    if remaining > 0 {
        attack.damage = remaining;
        // Event continues propagating to parent
    } else {
        attack.propagate(false); // Stop propagation
    }
}

fn take_damage(attack: On<Attack>, mut hp: Query<&mut HitPoints>) {
    let mut hp = hp.get_mut(attack.entity).unwrap();
    **hp = hp.saturating_sub(attack.damage);
}
```

## Component Hooks

Hooks enforce correctness at the component level (prefer observers/events when possible):

```rust
#[derive(Component)]
#[component(on_add = my_on_add, on_insert = my_on_insert, on_replace = my_on_replace, on_remove = my_on_remove)]
struct Tracked(u32);

fn my_on_add(mut world: DeferredWorld, ctx: HookContext) {
    let val = world.get::<Tracked>(ctx.entity).unwrap().0;
    info!("Tracked added to {} with value {}", ctx.entity, val);
}
```

Or register hooks dynamically:

```rust
fn setup(world: &mut World) {
    world
        .register_component_hooks::<MyComponent>()
        .on_add(|mut world, ctx| { /* ... */ })
        .on_insert(|world, _| { /* ... */ })
        .on_replace(|mut world, ctx| { /* ... */ })
        .on_remove(|mut world, ctx| { /* ... */ });
}
```

## One-Shot Systems

Register and run systems on demand:

```rust
#[derive(Component)]
struct Callback(SystemId);

fn setup(mut commands: Commands) {
    // Register a system and get its ID
    let system_id = commands.register_system(my_one_shot_system);
    commands.spawn(Callback(system_id));
}

fn trigger_callback(query: Query<&Callback>, mut commands: Commands) {
    for callback in &query {
        commands.run_system(callback.0);
    }
}

fn my_one_shot_system(/* regular system params */) {
    info!("One-shot system executed!");
}

// Can also run directly on World
fn exclusive(world: &mut World) {
    world.run_system_once(my_one_shot_system).unwrap();
}
```
