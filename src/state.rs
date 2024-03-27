use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Game,
}

/// Tag component used to tag entities added on the splash screen
#[derive(Component)]
pub struct OnSplashScreen;

// Tag component used to tag entities added on the game screen
#[derive(Component)]
pub struct OnGameScreen;

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
