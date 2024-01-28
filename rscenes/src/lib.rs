mod connectors;
mod scene;
mod state;

pub mod prelude {
    pub use crate::connectors::{Connector2D, Connector3D};
    pub use crate::scene::Scene;
    pub use crate::state::State;
    pub use rscenes_macros::draw;
    pub use rscenes_raylib_connector::assets::*;
}
