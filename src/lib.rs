mod colors;
mod manager;
mod scene;
mod state;

pub mod prelude {
    pub use crate::manager::SceneManager;
    pub use crate::scene::Scene;
    pub use crate::state::State;
    pub use raylib;
    pub use raylib::consts;
    pub use raylib::prelude::*;

    /// Raylib 4.5 colours
    pub mod colors {
        pub use crate::colors::*;
    }
}
