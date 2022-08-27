use crate::assets::Assets;
use crate::comps::*;
use bevy::prelude::*;

pub const FULL_SIZE: Size<Val> = Size {
    width: Val::Percent(100.0),
    height: Val::Percent(100.0),
};

pub fn show_menu(mut menu: Query<&mut Style, With<UiMenuNode>>) {
    menu.single_mut().display = Display::Flex;

    // TODO: update storag
}

pub fn hide_menu(mut menu: Query<&mut Style, With<UiMenuNode>>) {
    menu.single_mut().display = Display::None;
}

pub fn toggle_storage_section(
    mut section: Query<&mut Style, With<UiStorageSection>>,
    keys: Res<Input<KeyCode>>,
) {
    use Display::*;

    if keys.just_pressed(KeyCode::Tab) {
        let mut section = section.single_mut();

        section.display = match section.display {
            Flex => None,
            None => Flex,
        };
    }
}

pub fn draw_menu(mut cmd: Commands, assets: Res<Assets>) {
    let font = assets.font.clone_weak();

    cmd.spawn_bundle(NodeBundle {
        color: Color::rgba(0.15, 0.4, 0.3, 0.25).into(),
        style: Style {
            size: FULL_SIZE,
            // justify_content: JustifyContent::SpaceBetween,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            flex_direction: FlexDirection::ColumnReverse,
            display: Display::None,
            ..default()
        },
        ..default()
    })
    .insert(UiMenuNode)
    .with_children(|cmd| {
        cmd.spawn_bundle(
            TextBundle::from_section(
                "INVENTORY",
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            }),
        );

        cmd.spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: FULL_SIZE,
                flex_direction: FlexDirection::Row,
                margin: UiRect {
                    top: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                style: Style {
                    size: FULL_SIZE,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            })
            .with_children(|cmd| {
                cmd.spawn_bundle(
                    TextBundle::from_section(
                        "DEMON'S",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::BLUE,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                cmd.spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        size: FULL_SIZE,
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                })
                .insert(UiDemonInvNode);
            });

            cmd.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                style: Style {
                    size: FULL_SIZE,
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Stretch,
                    ..default()
                },
                ..default()
            })
            .insert(UiStorageSection)
            .with_children(|cmd| {
                cmd.spawn_bundle(
                    TextBundle::from_section(
                        "STORAGE",
                        TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::BLUE,
                        },
                    )
                    .with_style(Style {
                        align_self: AlignSelf::Center,
                        margin: UiRect {
                            top: Val::Px(10.0),
                            ..default()
                        },
                        ..default()
                    }),
                );

                cmd.spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style {
                        size: FULL_SIZE,
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    ..default()
                })
                .insert(UiStorageInvNode);
            });
        });
    });
}
