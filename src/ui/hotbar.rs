use crate::assets::Assets;
use crate::comps::*;
use crate::consts::*;
use crate::storage::*;
use crate::ui::cell::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UpdateHotbarEvent;

pub const HOTBAR_SIZE: Size<Val> = Size {
    width: Val::Px(CELL_SIZE_ARR[0] * 3.0 + HOTBAR_PADDING * 4.0),
    height: Val::Px(CELL_SIZE_ARR[1] + HOTBAR_PADDING * 2.0),
};

pub fn draw_hotbar(
    mut cmd: Commands,
    mut update: EventWriter<UpdateHotbarEvent>,
) {
    cmd.spawn()
        .insert(UiHotbarNode)
        .insert_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: crate::ui::menu::FULL_SIZE,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn()
                .insert(UiHotbarInvNode)
                .insert_bundle(NodeBundle {
                    // color: Color::rgba(0.4, 0.5, 0.45, 0.8).into(),
                    color: Color::GREEN.into(),
                    style: Style {
                        size: HOTBAR_SIZE,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                });
        });

    update.send(UpdateHotbarEvent);
}

pub fn show_hotbar(mut hotbar: Query<&mut Style, With<UiHotbarNode>>) {
    hotbar.single_mut().display = Display::Flex;
}

pub fn hide_hotbar(mut hotbar: Query<&mut Style, With<UiHotbarNode>>) {
    hotbar.single_mut().display = Display::None;
}

pub fn update_hotbar(
    mut updates: EventReader<UpdateHotbarEvent>,
    mut cmd: Commands,
    active: Query<(&DemonInventory, &ActiveSlot), With<ActiveDemon>>,
    assets: Res<Assets>,
    section: Query<Entity, With<UiHotbarInvNode>>,
) {
    if let None = updates.iter().next() {
        return;
    }

    let (inv, active) = active.single();

    let (h0, h1, h2) = if let [h0, h1, h2, ..] = inv.inventory.view() {
        (h0, h1, h2)
    } else {
        panic!("Active demon's inventory view should contain at least 3 cells")
    };

    let mut section = cmd.entity(section.single());

    section.despawn_descendants();

    spawn_icon_ui(
        &mut section,
        get_cell_color(*active, ActiveSlot::First),
        h0,
        &assets,
    );
    spawn_icon_ui(
        &mut section,
        get_cell_color(*active, ActiveSlot::Second),
        h1,
        &assets,
    );
    spawn_icon_ui(
        &mut section,
        get_cell_color(*active, ActiveSlot::Third),
        h2,
        &assets,
    );
}

fn get_cell_color(lhs: ActiveSlot, rhs: ActiveSlot) -> UiColor {
    if lhs == rhs {
        Color::rgba(1.0, 1.0, 1.0, 0.5).into()
    } else {
        Color::NONE.into()
    }
}
