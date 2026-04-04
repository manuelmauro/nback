use crate::state::AppState;
use bevy::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetLoading), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("embedded://icon.png");

    commands.spawn((
        DespawnOnExit(AppState::AssetLoading),
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![(
            ImageNode::new(icon),
            Node {
                width: px(200),
                ..default()
            },
        )],
    ));
}
