#![allow(clippy::too_many_arguments)] // Raylib functions has too may arguments ¯\_(ツ)_/¯
#![allow(clippy::wrong_self_convention)] // Raylib is_* methods receive owned objects

mod ext;
mod raudio;
mod rcamera;
mod rcore;
mod rgestures;
mod rmodels;
mod rshapes;
mod rtext;
mod rtextures;
mod utils;

pub mod interface;

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
    pub use crate::ext::vector::{Vector2Ext, Vector3Ext};
    pub use crate::ext::wave::{WaveExt, WaveType};
    pub use crate::ext::window_handle::WindowHandle;
    pub use raylib_ffi::{
        enums::*, AudioStream, AutomationEvent, AutomationEventList, BoneInfo, BoundingBox,
        Camera2D, Camera3D, Color, FilePathList, Font, GlyphInfo, Image, Material, MaterialMap,
        Matrix, Mesh, Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, Rectangle,
        RenderTexture, Shader, Sound, Texture, Transform, Vector2, Vector3, Vector4, VrDeviceInfo,
        VrStereoConfig, Wave,
    };
}
