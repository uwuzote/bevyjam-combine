use crate::comps::*;
use crate::consts::*;
use bevy::prelude::*;

#[derive(
    Debug, Clone, Copy, Component, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(anim_animator)
            .add_system(anim_highlighter_update)
            .add_system(anim_highlighter_wave.after(anim_highlighter_update));
    }
}

pub fn anim_animator(
    mut query: Query<&mut Transform, With<Animator>>,
    time: Res<Time>,
) {
    let mut anim = query.single_mut();
    let smth = (time.seconds_since_startup() as f32).sin();

    anim.translation = Vec3::new(0.0, smth * STEP_SIZE / 6.0, 0.0);
}

pub fn anim_highlighter_wave(
    mut high: Query<&mut Transform, With<Highlighter>>,
    time: Res<Time>,
) {
    let mut high = high.single_mut();
    let sin = ((time.seconds_since_startup() as f32) / 1.4).sin();
    // let smth = 1.0 + (sin / 2.0 + 0.5) / 7.0 + 0.1;
    let smth = 1.172 + (sin / 14.0);

    high.scale = Vec3::new(DEMON_SCALE * smth, DEMON_SCALE * smth, 1.0);
}

pub fn anim_highlighter_update(
    mut high: Query<
        (&mut Transform, &mut Handle<Image>),
        (With<Highlighter>, Without<ActiveDemon>),
    >,
    active: Query<
        (&Transform, &HighlighterTexture),
        (With<ActiveDemon>, Without<Highlighter>),
    >,
) {
    let mut high = high.single_mut();
    let (new_trans, handle) = active.single();

    high.0.translation.x = new_trans.translation.x;
    high.0.translation.y = new_trans.translation.y;

    *high.1 = handle.0.clone_weak();
}
