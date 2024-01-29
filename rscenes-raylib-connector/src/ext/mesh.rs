use crate::rmodels::Rmodels;
use raylib_ffi::*;
use std::fmt::Display;

pub trait MeshExt: Sized {
    // Generate polygonal mesh
    fn gen_poly(sides: i32, radius: f32) -> Self;
    /// Generate plane mesh (with subdivisions)
    fn gen_plane(width: f32, height: f32, x: i32, z: i32) -> Self;
    /// Generate cuboid mesh
    fn gen_cube(width: f32, height: f32, length: f32) -> Self;
    /// Generate sphere mesh (standard sphere)
    fn gen_sphere(radius: f32, rings: i32, slices: i32) -> Self;
    /// Generate half-sphere mesh (no bottom cap)
    fn gen_hemisphere(radius: f32, rings: i32, slices: i32) -> Self;
    /// Generate cylinder mesh
    fn gen_cylinder(radius: f32, height: f32, slices: i32) -> Self;
    /// Generate cone/pyramid mesh
    fn gen_cone(radius: f32, height: f32, slices: i32) -> Self;
    /// Generate torus mesh
    fn gen_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self;
    /// Generate trefoil knot mesh
    fn gen_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self;
    /// Generate heightmap mesh from image data
    fn gen_heightmap(heightmap: Image, size: Vector3) -> Self;
    /// Generate cubes-based map mesh from image data
    fn gen_cubicmap(heightmap: Image, size: Vector3) -> Self;

    /// Upload mesh vertex data in GPU and provide VAO/VBO ids
    fn upload(&mut self, dynamic: bool);
    /// Update mesh vertex data in GPU for a specific buffer index
    fn update_buffer(self, index: i32, data: &mut Vec<u8>, offset: i32);
    /// Unload mesh data from CPU and GPU
    fn unload(self);
    /// Export mesh data to file, returns true on success
    fn export(self, filename: impl Display) -> bool;
    /// Compute mesh bounding box limits
    fn get_bounding_box(self) -> BoundingBox;
    /// Compute mesh tangents
    fn gen_tangents(&mut self);
}

impl MeshExt for Mesh {
    fn gen_poly(sides: i32, radius: f32) -> Self {
        Rmodels::__gen_mesh_poly(sides, radius)
    }

    fn gen_plane(width: f32, height: f32, x: i32, z: i32) -> Self {
        Rmodels::__gen_mesh_plane(width, height, x, z)
    }

    fn gen_cube(width: f32, height: f32, length: f32) -> Self {
        Rmodels::__gen_mesh_cube(width, height, length)
    }

    fn gen_sphere(radius: f32, rings: i32, slices: i32) -> Self {
        Rmodels::__gen_mesh_sphere(radius, rings, slices)
    }

    fn gen_hemisphere(radius: f32, rings: i32, slices: i32) -> Self {
        Rmodels::__gen_mesh_hemisphere(radius, rings, slices)
    }

    fn gen_cylinder(radius: f32, height: f32, slices: i32) -> Self {
        Rmodels::__gen_mesh_cylinder(radius, height, slices)
    }

    fn gen_cone(radius: f32, height: f32, slices: i32) -> Self {
        Rmodels::__gen_mesh_cone(radius, height, slices)
    }

    fn gen_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self {
        Rmodels::__gen_mesh_torus(radius, size, rad_seg, sides)
    }

    fn gen_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Self {
        Rmodels::__gen_mesh_knot(radius, size, rad_seg, sides)
    }

    fn gen_heightmap(heightmap: Image, size: Vector3) -> Self {
        Rmodels::__gen_mesh_heightmap(heightmap, size)
    }

    fn gen_cubicmap(heightmap: Image, size: Vector3) -> Self {
        Rmodels::__gen_mesh_cubicmap(heightmap, size)
    }

    fn upload(&mut self, dynamic: bool) {
        Rmodels::__upload_mesh(self, dynamic)
    }

    fn update_buffer(self, index: i32, data: &mut Vec<u8>, offset: i32) {
        Rmodels::__update_mesh_buffer(self, index, data, offset)
    }

    fn unload(self) {
        Rmodels::__unload_mesh(self)
    }

    fn export(self, filename: impl Display) -> bool {
        Rmodels::__export_mesh(self, filename)
    }

    fn get_bounding_box(self) -> BoundingBox {
        Rmodels::__get_mesh_bounding_box(self)
    }
    fn gen_tangents(&mut self) {
        Rmodels::__gen_mesh_tangents(self)
    }
}
