use crate::anim::AnimationPlugin;
use crate::consts::{CLEAR_COLOR, LAUNCHER_TITLE};
use crate::mov::MovementPlugin;
use crate::setup_plugin::SetupPlugin;
use crate::state::{quit_by_esc, toggle_state, GameState};
use crate::ui::UiPlugin;
use bevy::{prelude::*, render::texture::ImageSettings, window::WindowMode};

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(CLEAR_COLOR)
        .insert_resource(WindowDescriptor {
            title: LAUNCHER_TITLE.into(),
            mode: WindowMode::BorderlessFullscreen,
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SetupPlugin)
        // .add_startup_system(debug_setup)
        // .add_system(debug)
        .add_state(GameState::Game)
        .add_plugin(AnimationPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(UiPlugin)
        .add_system(toggle_state)
        .add_system_set(
            SystemSet::on_update(GameState::Items).with_system(quit_by_esc),
        );

    app
}

// use crate::consts::*;
// use crate::comps::*;
// use crate::assets::Assets;
//
// fn debug_setup(mut cmd: Commands, anim: Query<Entity, With<Animator>>, asset:
// Res<Assets>) {     cmd.entity(anim.single()).with_children(|cmd| {
//         cmd.spawn()
//         .insert(Demon)
//         .insert(HighlighterTexture(asset.demons[3].clone_weak()))
//         .insert_bundle(SpriteBundle {
//             texture: asset.demons[2].clone_weak(),
//             transform: Transform::identity()
//                 .with_translation(Vec3::new(STEP_SIZE, STEP_SIZE,
// DEMON_Z_POS))                 .with_scale(Vec3::new(DEMON_SCALE, DEMON_SCALE,
// 1.0)),                 ..default()
//         });
//     });
// }
//
// fn debug() {}
