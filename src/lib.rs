mod manager;
mod scene;
mod status;

pub mod prelude {
    pub use crate::manager::SceneManager;
    pub use crate::scene::Scene;
    pub use crate::status::Status;
    pub use raylib;
    pub use raylib::prelude::*;
}
