use bevy::prelude::*;

use crate::{game::settings::GameSettings, state::AppState, theme};

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
    IncreaseN,
    DecreaseN,
}

/// Stores the button's resting background color so it can be restored
/// after hover/press.
#[derive(Component, Deref)]
pub struct RestingColor(pub Color);

type MenuButtonQuery<'w> = (
    &'w Interaction,
    &'w mut BackgroundColor,
    &'w MenuButtonAction,
    &'w RestingColor,
);

pub fn menu_button_system(
    mut app_state: ResMut<NextState<AppState>>,
    mut exit: MessageWriter<AppExit>,
    mut settings: ResMut<GameSettings>,
    mut query: Query<MenuButtonQuery, (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color, action, resting) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                *color = theme::ACCENT_PRESS.into();
                match action {
                    MenuButtonAction::Play => {
                        app_state.set(AppState::Game);
                    }
                    MenuButtonAction::Quit => {
                        exit.write(AppExit::Success);
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
                *color = theme::ACCENT_HOVER.into();
            }
            Interaction::None => {
                *color = (**resting).into();
            }
        }
    }
}
