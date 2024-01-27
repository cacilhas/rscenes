mod ext;
mod raudio;
mod rcamera;
mod rcore;
mod rgestures;
mod rmodels;
mod rshapes;
mod rtext;
mod rtextures;

pub mod interface;

pub mod assets {
    pub use crate::ext::camera::{Camera2DExt, Camera3DExt};
    pub use crate::ext::codepoints::Codepoints;
    pub use crate::ext::color::ColorExt;
    pub use crate::ext::font::FontExt;
    pub use crate::ext::image::ImageExt;
    pub use crate::ext::key::KeyboardKeyExt;
    pub use crate::ext::material::MaterialExt;
    pub use crate::ext::mesh::MeshExt;
    pub use crate::ext::model::ModelExt;
    pub use crate::ext::model_animation::ModelAnimationExt;
    pub use crate::ext::texture::{RenderTextureExt, TextureCubemapExt, TextureExt};
    pub use crate::ext::vector::{Vector2Ext, Vector3Ext};
    pub use crate::ext::window_handle::WindowHandle;
    pub use raylib_ffi::{
        enums::*, AudioStream, AutomationEvent, AutomationEventList, BoneInfo, BoundingBox,
        Camera2D, Camera3D, Color, FilePathList, Font, GlyphInfo, Image, Material, MaterialMap,
        Matrix, Mesh, Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, Rectangle,
        RenderTexture, Shader, Sound, Texture, Transform, Vector2, Vector3, Vector4, VrDeviceInfo,
        VrStereoConfig, Wave,
    };
}
