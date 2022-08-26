use crate::assets::Assets;
use crate::comps::*;
use crate::consts::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<Assets>()
            .add_startup_system_to_stage(
                StartupStage::PreStartup,
                setup_spawn_specials.exclusive_system(),
            )
            .add_startup_system(setup_spawn_camera);
    }
}

pub fn setup_spawn_camera(mut cmd: Commands) {
    cmd.spawn_bundle(Camera2dBundle::default());
}

pub fn setup_spawn_specials(world: &mut World) {
    let assets = world
        .get_resource::<Assets>()
        .expect("Cannot acquire Assets resource");

    let koci4 = assets.demons[0].clone_weak();
    let koci4_shade = assets.demons[1].clone_weak();

    world
        .spawn()
        .insert_bundle(SpatialBundle::visible_identity())
        .insert(Animator)
        .with_children(|cmd| {
            cmd.spawn()
                .insert(Demon)
                .insert(ActiveDemon)
                .insert(HighlighterTexture(koci4_shade.clone_weak()))
                .insert_bundle(SpriteBundle {
                    texture: koci4,
                    transform: Transform::identity()
                        .with_translation(Vec3::new(0.0, 0.0, DEMON_Z_POS))
                        .with_scale(Vec3::new(DEMON_SCALE, DEMON_SCALE, 1.0)),
                    ..default()
                });

            cmd.spawn().insert(Highlighter).insert_bundle(SpriteBundle {
                texture: koci4_shade,
                transform: Transform::identity().with_translation(Vec3::new(
                    0.0,
                    0.0,
                    HIGHLIGHTER_Z_POS,
                )),
                ..default()
            });
        });
}
