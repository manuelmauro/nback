---
name: bevy-ui
description: Bevy UI layout, buttons, text, menus, interaction handling, state-driven screens, and widget patterns. Use when building game menus, HUDs, settings screens, or any in-game UI.
---

# Bevy UI

## Core Concepts

Bevy UI uses a flexbox-based layout system. UI entities use `Node` for layout, plus optional components like `Button`, `Text`, `ImageNode`, and `BackgroundColor`.

## Text

### Simple Text

```rust
use bevy::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Text::new("Hello, Bevy!"),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
    ));
}
```

### Positioned Text (HUD)

```rust
commands.spawn((
    Text::new("Score: 0"),
    TextFont { font_size: 24.0, ..default() },
    TextColor(Color::WHITE),
    Node {
        position_type: PositionType::Absolute,
        top: px(12),
        left: px(12),
        ..default()
    },
));
```

### Rich Text with TextSpan Children

```rust
commands.spawn((
    Text::default(),
    Node {
        position_type: PositionType::Absolute,
        top: px(12),
        left: px(12),
        ..default()
    },
    children![
        (TextSpan::new("Score: "),
         TextFont { font_size: 30.0, ..default() },
         TextColor(Color::WHITE)),
        (TextSpan::new("0"),
         TextFont { font_size: 30.0, ..default() },
         TextColor(Color::srgb(1.0, 0.5, 0.0))),
    ],
));
```

### Updating Text with TextUiWriter

```rust
#[derive(Component)]
struct ScoreText;

fn update_score(score: Res<Score>, text_entity: Single<Entity, With<ScoreText>>, mut writer: TextUiWriter) {
    // Update the 3rd child span (index 3, 1-indexed skipping parent)
    *writer.text(*text_entity, 3) = format!("{}", score.0);
}
```

## Buttons

### Basic Button with Interaction

```rust
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup_button(mut commands: Commands) {
    commands.spawn((
        Button,
        Node {
            width: px(150),
            height: px(65),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new("Play"),
            TextFont { font_size: 33.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    ));
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => { *color = PRESSED_BUTTON.into(); }
            Interaction::Hovered => { *color = HOVERED_BUTTON.into(); }
            Interaction::None => { *color = NORMAL_BUTTON.into(); }
        }
    }
}
```

### Button with Action Component

```rust
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    Quit,
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Play => next_state.set(AppState::InGame),
                MenuButtonAction::Settings => next_state.set(AppState::Settings),
                MenuButtonAction::Quit => exit.write(AppExit::Success),
            }
        }
    }
}
```

## Layout Patterns

### Centered Full-Screen Container

```rust
commands.spawn((
    Node {
        width: percent(100),
        height: percent(100),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    children![/* centered content */],
));
```

### Vertical Column Layout

```rust
commands.spawn((
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        row_gap: px(10),
        ..default()
    },
    children![
        (Text::new("Title"), TextFont { font_size: 50.0, ..default() }),
        (Button, /* ... */),
        (Button, /* ... */),
    ],
));
```

### Horizontal Row Layout

```rust
commands.spawn((
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: px(10),
        ..default()
    },
    children![/* items laid out horizontally */],
));
```

### Absolute Positioning (Overlay)

```rust
commands.spawn((
    Text::new("FPS: 60"),
    Node {
        position_type: PositionType::Absolute,
        top: px(12),
        right: px(12),
        ..default()
    },
));
```

## Images in UI

```rust
commands.spawn((
    ImageNode::new(asset_server.load("branding/icon.png")),
    Node {
        width: px(200),
        // height auto-adjusts to maintain aspect ratio
        ..default()
    },
));
```

## Complete Menu Example (State-Driven)

```rust
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, (button_visuals, menu_action).run_if(in_state(GameState::Menu)))
        .add_systems(OnEnter(GameState::InGame), setup_game)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(10),
                ..default()
            },
            children![
                (
                    Text::new("My Game"),
                    TextFont { font_size: 67.0, ..default() },
                    TextColor(Color::WHITE),
                    Node { margin: UiRect::bottom(px(30)), ..default() },
                ),
                (
                    Button,
                    MenuButtonAction::Play,
                    Node {
                        width: px(200),
                        height: px(65),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    children![(
                        Text::new("Play"),
                        TextFont { font_size: 33.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )],
                ),
                (
                    Button,
                    MenuButtonAction::Quit,
                    Node {
                        width: px(200),
                        height: px(65),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    children![(
                        Text::new("Quit"),
                        TextFont { font_size: 33.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    )],
                ),
            ]
        )],
    ));
}
```

## Settings with Selected State

```rust
#[derive(Component)]
struct SelectedOption;

fn setting_button<T: Resource + Component + PartialEq + Copy>(
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    let (prev_entity, mut prev_color) = selected.into_inner();
    for (interaction, value, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *value {
            *prev_color = Color::srgb(0.15, 0.15, 0.15).into();
            commands.entity(prev_entity).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *value;
        }
    }
}
```

## Reusable Widget Pattern

```rust
/// Reusable button widget as a function returning a Bundle
fn menu_button(label: &str, action: MenuButtonAction) -> impl Bundle + use<'_> {
    (
        Button,
        action,
        Node {
            width: px(200),
            height: px(65),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        children![(
            Text::new(label),
            TextFont { font_size: 33.0, ..default() },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

// Usage
fn setup(mut commands: Commands) {
    commands.spawn(menu_button("Play", MenuButtonAction::Play));
    commands.spawn(menu_button("Quit", MenuButtonAction::Quit));
}
```

## Splash Screen Pattern

```rust
fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
}

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        DespawnOnExit(GameState::Splash),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            ImageNode::new(asset_server.load("branding/icon.png")),
            Node { width: px(200), ..default() },
        )],
    ));
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        next_state.set(GameState::Menu);
    }
}
```

## Node Properties Reference

Key `Node` fields for layout:

```rust
Node {
    // Sizing
    width: px(100),              // or percent(50), Val::Auto
    height: px(50),
    min_width: px(0),
    max_width: percent(100),

    // Flexbox direction
    flex_direction: FlexDirection::Column,  // Row, ColumnReverse, RowReverse

    // Alignment
    justify_content: JustifyContent::Center,  // FlexStart, FlexEnd, SpaceBetween, SpaceAround
    align_items: AlignItems::Center,          // FlexStart, FlexEnd, Stretch, Baseline
    align_self: AlignSelf::Auto,

    // Gaps
    row_gap: px(10),
    column_gap: px(10),

    // Spacing
    margin: UiRect::all(px(10)),
    padding: UiRect::axes(px(12), px(6)),
    border: UiRect::all(px(1)),

    // Positioning
    position_type: PositionType::Relative,  // or Absolute
    top: px(12),
    bottom: px(12),
    left: px(12),
    right: px(12),

    ..default()
}
```
