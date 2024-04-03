use crate::state::{despawn_screen, AppState, OnSplashScreen};
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetLoading), setup)
            .add_systems(
                OnExit(AppState::AssetLoading),
                despawn_screen::<OnSplashScreen>,
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("icon.png");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
}
