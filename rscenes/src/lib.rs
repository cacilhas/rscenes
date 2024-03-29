#![feature(trait_alias)]
mod connectors;
mod macros;
mod manager;
mod scene;
mod state;

pub mod extras;

pub mod prelude {
    pub use super::setup;
    pub use crate::connectors::*;
    pub use crate::manager::*;
    pub use crate::scene::Scene;
    pub use crate::state::State;
    pub use rscenes_macros::draw;
    pub use rscenes_raylib_connector::{assets::*, interface::*};
}
