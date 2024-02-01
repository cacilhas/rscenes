#![allow(clippy::too_many_arguments)] // Raylib functions has too may arguments ¯\_(ツ)_/¯
#![allow(clippy::wrong_self_convention)] // Raylib is_* methods receive owned objects

mod ext;
mod raudio;
mod rcamera;
mod rcore;
mod rgestures;
mod rmodels;
mod rmodels_collisions;
mod rshapes;
mod rshapes_collisions;
mod rtext;
mod rtextures;
mod utils;

pub mod assets {
    pub use crate::ext::audio_stream::AudioStreamExt;
    pub use crate::ext::camera::{Camera2DExt, Camera3DExt};
    pub use crate::ext::codepoints::Codepoints;
    pub use crate::ext::color::ColorExt;
    pub use crate::ext::font::FontExt;
    pub use crate::ext::image::{ImageExt, ImageType};
    pub use crate::ext::key::KeyboardKeyExt;
    pub use crate::ext::material::MaterialExt;
    pub use crate::ext::mesh::MeshExt;
    pub use crate::ext::model::ModelExt;
    pub use crate::ext::model_animation::ModelAnimationExt;
    pub use crate::ext::music::MusicExt;
    pub use crate::ext::ray::RayExt;
    pub use crate::ext::sound::SoundExt;
    pub use crate::ext::texture::{RenderTextureExt, TextureCubemapExt, TextureExt};
    pub use crate::ext::trace_log_level::TraceLogLevelExt;
    pub use crate::ext::vector::{Vector2Ext, Vector3Ext};
    pub use crate::ext::wave::{WaveExt, WaveType};
    pub use crate::ext::window_handle::WindowHandle;
    pub use raylib_ffi::{
        enums::*, AudioStream, AutomationEvent, AutomationEventList, BoneInfo, BoundingBox,
        Camera2D, Camera3D, Color, FilePathList, Font, GlyphInfo, Image, Material, MaterialMap,
        Matrix, Mesh, Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, Rectangle,
        RenderTexture, Shader, Sound, Texture2D, TextureCubemap, Transform, Vector2, Vector3,
        Vector4, VrDeviceInfo, VrStereoConfig, Wave,
    };
}

pub mod interface {
    pub use crate::raudio::Raudio;
    pub use crate::rcamera::Rcamera;
    pub use crate::rcore::Rcore;
    pub use crate::rgestures::Rgestures;
    pub use crate::rmodels::Rmodels;
    pub use crate::rmodels_collisions::RmodelsCollisions;
    pub use crate::rshapes::Rshapes;
    pub use crate::rshapes_collisions::RshapesCollisions;
    pub use crate::rtext::Rtext;
    pub use crate::rtextures::Rtextures;
}
