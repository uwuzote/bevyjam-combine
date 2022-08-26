#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod anim;
pub mod app;
pub mod assets;
pub mod comps;
pub mod consts;
pub mod setup_plugin;
pub mod storage;

pub use app::app;

// pub mod prelude {
//     pub use crate::{
//         comps::*
//     };
// }
