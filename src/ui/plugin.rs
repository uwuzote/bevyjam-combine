// use crate::comps::*;
// use crate::consts::*;
use crate::state::GameState;
use bevy::prelude::*;

use crate::ui::{
    hotbar::{
        draw_hotbar, hide_hotbar, show_hotbar, update_hotbar, UpdateHotbarEvent,
    },
    menu::{draw_menu, hide_menu, show_menu, toggle_storage_section},
};

#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(draw_menu)
            .add_startup_system(draw_hotbar)
            .add_event::<UpdateHotbarEvent>()
            .add_system_set(
                SystemSet::on_enter(GameState::Items)
                    .with_system(show_menu)
                    .with_system(hide_hotbar),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(hide_menu)
                    .with_system(show_hotbar),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(update_hotbar),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Items)
                    .with_system(toggle_storage_section),
            );
    }
}
