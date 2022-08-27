#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod anim;
pub mod app;
pub mod assets;
pub mod comps;
pub mod consts;
pub mod mov;
pub mod setup_plugin;
pub mod state;
pub mod storage;
pub mod ui;

pub use app::app;
pub use consts::LAUNCHER_TITLE;

// pub mod prelude {
//     pub use crate::{
//         comps::*
//     };
// }
