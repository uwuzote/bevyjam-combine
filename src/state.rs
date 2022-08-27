use bevy::{app::AppExit, prelude::*};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, StageLabel)]
pub enum GameState {
    #[default]
    Game,
    Items,
}

impl GameState {
    pub fn invert(&self) -> Self {
        use GameState::*;

        match *self {
            Game => Items,
            Items => Game,
        }
    }
}

pub fn toggle_state(
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::E) {
        let new = state.current().invert();

        state.set(new).expect("Other state-changers queued");
    }
}

pub fn quit_by_esc(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
