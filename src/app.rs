use crate::anim::AnimationPlugin;
use crate::consts::{CLEAR_COLOR, LAUNCHER_TITLE};
use crate::setup_plugin::SetupPlugin;
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
        .add_plugin(AnimationPlugin);

    app
}
