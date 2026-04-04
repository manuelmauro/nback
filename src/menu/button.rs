use bevy::prelude::*;

use crate::{game::settings::GameSettings, palette, state::AppState};

pub const NORMAL_BUTTON: Color = palette::SLATE_800;
pub const HOVERED_BUTTON: Color = palette::LIME_900;
pub const PRESSED_BUTTON: Color = palette::LIME_500;
pub const BUTTON_BORDER_COLOR: Color = palette::WHITE;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    IncreaseN,
    DecreaseN,
}

type MenuButtonQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w MenuButtonAction,
);

pub fn menu_button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut settings: ResMut<GameSettings>,
    mut query: Query<MenuButtonQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, action) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                match action {
                    MenuButtonAction::Play => {
                        app_state.set(AppState::Game);
                    }
                    MenuButtonAction::IncreaseN => {
                        settings.n += 1;
                        settings.set_rounds_from_n();
                    }
                    MenuButtonAction::DecreaseN => {
                        settings.n = settings.n.saturating_sub(1).max(1);
                        settings.set_rounds_from_n();
                    }
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
