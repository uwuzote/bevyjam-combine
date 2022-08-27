use crate::assets::Assets;
use crate::consts::*;
use crate::storage::*;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub const CELL_SIZE_ARR: [f32; 2] =
    [ITEM_SCALE * ITEM_SIZE, ITEM_SCALE * ITEM_SIZE];
pub const CELL_SIZE: Size<Val> = Size {
    width: Val::Px(CELL_SIZE_ARR[0]),
    height: Val::Px(CELL_SIZE_ARR[1]),
};

pub fn spawn_icon_ui<'w, 's, 'a>(
    mut cmd: &mut EntityCommands<'w, 's, 'a>,
    color: UiColor,
    cell: &Cell,
    assets: &Assets,
) {
    cmd.insert_bundle(NodeBundle {
        color,
        style: Style {
            size: CELL_SIZE,
            ..default()
        },
        ..default()
    })
    .with_children(|cmd| {
        if cell.0.texture_id().is_none() {
            return;
        }

        cmd.spawn()
            .insert_bundle(ImageBundle {
                image: assets.items[cell.0.texture_id().unwrap()]
                    .clone_weak()
                    .into(),
                style: Style {
                    min_size: CELL_SIZE,
                    ..default()
                },
                ..default()
            })
            .with_children(|cmd| {
                cmd.spawn().insert_bundle(TextBundle::from_section(
                    format!("{}", cell.1),
                    TextStyle {
                        font: assets.font.clone_weak(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            });
    });
}
