use raylib_ffi::*;
use std::{ffi::c_void, fmt::Display};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rmodels;

/// Crate only methods
impl Rmodels {
    // Basic geometric 3D shapes drawing methods

    pub(crate) fn __draw_line_3d(start: Vector3, end: Vector3, color: Color) {
        unsafe { DrawLine3D(start, end, color) }
    }

    pub(crate) fn __draw_point_3d(position: Vector3, color: Color) {
        unsafe { DrawPoint3D(position, color) }
    }

    pub(crate) fn __draw_circle_3d(
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    ) {
        unsafe { DrawCircle3D(center, radius, rotation_axis, rotation_angle, color) }
    }

    pub(crate) fn __draw_triangle_3d(v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        unsafe { DrawTriangle3D(v1, v2, v3, color) }
    }

    pub(crate) fn __draw_triangle_strip_3d(points: Vec<Vector3>, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_ptr() as *mut Vector3;
            DrawTriangleStrip3D(points, count, color)
        }
    }

    pub(crate) fn __draw_cube(
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe { DrawCube(position, width, height, length, color) }
    }

    pub(crate) fn __draw_cube_v(position: Vector3, size: Vector3, color: Color) {
        unsafe { DrawCubeV(position, size, color) }
    }

    pub(crate) fn __draw_cube_wires(
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe { DrawCubeWires(position, width, height, length, color) }
    }

    pub(crate) fn __draw_cube_wires_v(position: Vector3, size: Vector3, color: Color) {
        unsafe { DrawCubeWiresV(position, size, color) }
    }

    pub(crate) fn __draw_sphere(center: Vector3, radius: f32, color: Color) {
        unsafe { DrawSphere(center, radius, color) }
    }

    pub(crate) fn __draw_sphere_ex(
        center: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        unsafe { DrawSphereEx(center, radius, rings, slices, color) }
    }

    pub(crate) fn __draw_sphere_wires(
        center: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        unsafe { DrawSphereWires(center, radius, rings, slices, color) }
    }

    pub(crate) fn __draw_cylinder(
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        unsafe { DrawCylinder(position, radius_top, radius_bottom, height, slices, color) }
    }

    pub(crate) fn __draw_cylinder_ex(
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: i32,
        color: Color,
    ) {
        unsafe { DrawCylinderEx(start_pos, end_pos, start_radius, end_radius, sides, color) }
    }

    pub(crate) fn __draw_cylinder_wires(
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        unsafe { DrawCylinderWires(position, radius_top, radius_bottom, height, slices, color) }
    }

    pub(crate) fn __draw_cylinder_wires_ex(
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: i32,
        color: Color,
    ) {
        unsafe { DrawCylinderWiresEx(start_pos, end_pos, start_radius, end_radius, sides, color) }
    }

    pub(crate) fn __draw_capsule(
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        unsafe { DrawCapsule(start_pos, end_pos, radius, slices, rings, color) }
    }

    pub(crate) fn __draw_capsule_wires(
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        unsafe { DrawCapsuleWires(start_pos, end_pos, radius, slices, rings, color) }
    }

    pub(crate) fn __draw_plane(center: Vector3, size: Vector2, color: Color) {
        unsafe { DrawPlane(center, size, color) }
    }

    pub(crate) fn __draw_ray(ray: Ray, color: Color) {
        unsafe { DrawRay(ray, color) }
    }

    pub(crate) fn __draw_grid(slices: i32, spacing: f32) {
        unsafe { DrawGrid(slices, spacing) }
    }

    // Model management methods

    pub(crate) fn __load_model(filename: impl Display) -> Model {
        unsafe { LoadModel(rl_str!(filename)) }
    }

    pub(crate) fn __load_model_from_mesh(mesh: Mesh) -> Model {
        unsafe { LoadModelFromMesh(mesh) }
    }

    pub(crate) fn __is_model_ready(model: Model) -> bool {
        unsafe { IsModelReady(model) }
    }

    pub(crate) fn __unload_model(model: Model) {
        unsafe { UnloadModel(model) }
    }

    pub(crate) fn __get_model_bounding_box(model: Model) -> BoundingBox {
        unsafe { GetModelBoundingBox(model) }
    }

    // Model drawing methods

    pub(crate) fn __draw_model(model: Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { DrawModel(model, position, scale, tint) }
    }

    pub(crate) fn __draw_model_ex(
        model: Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe { DrawModelEx(model, position, rotation_axis, rotation_angle, scale, tint) }
    }

    pub(crate) fn __draw_model_wires(model: Model, position: Vector3, scale: f32, tint: Color) {
        unsafe { DrawModelWires(model, position, scale, tint) }
    }

    pub(crate) fn __draw_model_wires_ex(
        model: Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        unsafe { DrawModelWiresEx(model, position, rotation_axis, rotation_angle, scale, tint) }
    }

    pub(crate) fn __draw_bounding_box(box_: BoundingBox, color: Color) {
        unsafe { DrawBoundingBox(box_, color) }
    }

    pub(crate) fn __draw_billboard(
        camera: Camera3D,
        texture: Texture2D,
        position: Vector3,
        size: f32,
        tint: Color,
    ) {
        unsafe { DrawBillboard(camera, texture, position, size, tint) }
    }

    pub(crate) fn __draw_billboard_rec(
        camera: Camera3D,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    ) {
        unsafe { DrawBillboardRec(camera, texture, source, position, size, tint) }
    }

    pub(crate) fn __draw_billboard_pro(
        camera: Camera3D,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            DrawBillboardPro(
                camera, texture, source, position, up, size, origin, rotation, tint,
            )
        }
    }

    // Mesh management methods

    pub(crate) fn __upload_mesh(mesh: &mut Mesh, dynamic: bool) {
        unsafe { UploadMesh(mesh, dynamic) }
    }

    pub(crate) fn __update_mesh_buffer(mesh: Mesh, index: i32, data: Vec<u8>, offset: i32) {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_void;
            UpdateMeshBuffer(mesh, index, data, size, offset)
        }
    }

    pub(crate) fn __unload_mesh(mesh: Mesh) {
        unsafe { UnloadMesh(mesh) }
    }

    pub(crate) fn __draw_mesh(mesh: Mesh, material: Material, transform: Matrix) {
        unsafe { DrawMesh(mesh, material, transform) }
    }

    // TODO: DrawMeshInstanced

    pub(crate) fn __export_mesh(mesh: Mesh, filename: impl Display) -> bool {
        unsafe { ExportMesh(mesh, rl_str!(filename)) }
    }

    pub(crate) fn __get_mesh_bounding_box(mesh: Mesh) -> BoundingBox {
        unsafe { GetMeshBoundingBox(mesh) }
    }

    pub(crate) fn __gen_mesh_tangents(mesh: &mut Mesh) {
        unsafe { GenMeshTangents(mesh) }
    }

    // Mesh generation methods

    pub(crate) fn __gen_mesh_poly(sides: i32, radius: f32) -> Mesh {
        unsafe { GenMeshPoly(sides, radius) }
    }

    pub(crate) fn __gen_mesh_plane(width: f32, height: f32, x: i32, z: i32) -> Mesh {
        unsafe { GenMeshPlane(width, height, x, z) }
    }

    pub(crate) fn __gen_mesh_cube(width: f32, height: f32, length: f32) -> Mesh {
        unsafe { GenMeshCube(width, height, length) }
    }

    pub(crate) fn __gen_mesh_sphere(radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { GenMeshSphere(radius, rings, slices) }
    }

    pub(crate) fn __gen_mesh_hemisphere(radius: f32, rings: i32, slices: i32) -> Mesh {
        unsafe { GenMeshHemiSphere(radius, rings, slices) }
    }

    pub(crate) fn __gen_mesh_cylinder(radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe { GenMeshCylinder(radius, height, slices) }
    }

    pub(crate) fn __gen_mesh_cone(radius: f32, height: f32, slices: i32) -> Mesh {
        unsafe { GenMeshCone(radius, height, slices) }
    }

    pub(crate) fn __gen_mesh_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe { GenMeshTorus(radius, size, rad_seg, sides) }
    }

    pub(crate) fn __gen_mesh_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        unsafe { GenMeshKnot(radius, size, rad_seg, sides) }
    }

    pub(crate) fn __gen_mesh_heightmap(heightmap: Image, size: Vector3) -> Mesh {
        unsafe { GenMeshHeightmap(heightmap, size) }
    }

    pub(crate) fn __gen_mesh_cubicmap(heightmap: Image, size: Vector3) -> Mesh {
        unsafe { GenMeshCubicmap(heightmap, size) }
    }

    // Material loading/unloading methods
}

/// Exported methods
impl Rmodels {
    // Basic geometric 3D shapes drawing methods

    pub fn draw_line_3d(&self, start: Vector3, end: Vector3, color: Color) {
        Self::__draw_line_3d(start, end, color)
    }

    pub fn draw_point_3d(&self, position: Vector3, color: Color) {
        Self::__draw_point_3d(position, color)
    }

    pub fn draw_circle_3d(
        &self,
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    ) {
        Self::__draw_circle_3d(center, radius, rotation_axis, rotation_angle, color)
    }

    pub fn draw_triangle_3d(&self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        Self::__draw_triangle_3d(v1, v2, v3, color)
    }

    pub fn draw_triangle_strip_3d(&self, points: Vec<Vector3>, color: Color) {
        Self::__draw_triangle_strip_3d(points, color)
    }

    pub fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        Self::__draw_cube(position, width, height, length, color)
    }

    pub fn draw_cube_v(&self, position: Vector3, size: Vector3, color: Color) {
        Self::__draw_cube_v(position, size, color)
    }

    pub fn draw_cube_wires(
        &self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        Self::__draw_cube_wires(position, width, height, length, color)
    }

    pub fn draw_cube_wires_v(&self, position: Vector3, size: Vector3, color: Color) {
        Self::__draw_cube_wires_v(position, size, color)
    }

    pub fn draw_sphere(&self, center: Vector3, radius: f32, color: Color) {
        Self::__draw_sphere(center, radius, color)
    }

    pub fn draw_sphere_ex(
        &self,
        center: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        Self::__draw_sphere_ex(center, radius, rings, slices, color)
    }

    pub fn draw_sphere_wires(
        &self,
        center: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        Self::__draw_sphere_wires(center, radius, rings, slices, color)
    }

    pub fn draw_cylinder(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        Self::__draw_cylinder(position, radius_top, radius_bottom, height, slices, color)
    }

    pub fn draw_cylinder_ex(
        &self,
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: i32,
        color: Color,
    ) {
        Self::__draw_cylinder_ex(start_pos, end_pos, start_radius, end_radius, sides, color)
    }

    pub fn draw_cylinder_wires(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        Self::__draw_cylinder_wires(position, radius_top, radius_bottom, height, slices, color)
    }

    pub fn draw_cylinder_wires_ex(
        &self,
        start_pos: Vector3,
        end_pos: Vector3,
        start_radius: f32,
        end_radius: f32,
        sides: i32,
        color: Color,
    ) {
        Self::__draw_cylinder_wires_ex(start_pos, end_pos, start_radius, end_radius, sides, color)
    }

    pub fn draw_capsule(
        &self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        Self::__draw_capsule(start_pos, end_pos, radius, slices, rings, color)
    }

    pub fn draw_capsule_wires(
        &self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        Self::__draw_capsule_wires(start_pos, end_pos, radius, slices, rings, color)
    }

    pub fn draw_plane(&self, center: Vector3, size: Vector2, color: Color) {
        Self::__draw_plane(center, size, color)
    }

    pub fn draw_ray(&self, ray: Ray, color: Color) {
        Self::__draw_ray(ray, color)
    }

    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        Self::__draw_grid(slices, spacing)
    }

    // Model management methods

    pub fn load_model(&self, filename: impl Display) -> Model {
        Self::__load_model(filename)
    }

    pub fn load_model_from_mesh(&self, mesh: Mesh) -> Model {
        Self::__load_model_from_mesh(mesh)
    }

    pub fn is_model_ready(&self, model: Model) -> bool {
        Self::__is_model_ready(model)
    }

    pub fn unload_model(&self, model: Model) {
        Self::__unload_model(model)
    }

    pub fn get_model_bounding_box(&self, model: Model) -> BoundingBox {
        Self::__get_model_bounding_box(model)
    }

    // Model drawing methods

    pub fn draw_model(&self, model: Model, position: Vector3, scale: f32, tint: Color) {
        Self::__draw_model(model, position, scale, tint)
    }

    pub fn draw_model_ex(
        &self,
        model: Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        Self::__draw_model_ex(model, position, rotation_axis, rotation_angle, scale, tint)
    }

    pub fn draw_model_wires(&self, model: Model, position: Vector3, scale: f32, tint: Color) {
        Self::__draw_model_wires(model, position, scale, tint)
    }

    pub fn draw_model_wires_ex(
        &self,
        model: Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        Self::__draw_model_wires_ex(model, position, rotation_axis, rotation_angle, scale, tint)
    }

    pub fn draw_bounding_box(&self, box_: BoundingBox, color: Color) {
        Self::__draw_bounding_box(box_, color)
    }

    pub fn draw_billboard(
        &self,
        camera: Camera3D,
        texture: Texture2D,
        position: Vector3,
        size: f32,
        tint: Color,
    ) {
        Self::__draw_billboard(camera, texture, position, size, tint)
    }

    pub fn draw_billboard_rec(
        &self,
        camera: Camera3D,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    ) {
        Self::__draw_billboard_rec(camera, texture, source, position, size, tint)
    }

    pub fn draw_billboard_pro(
        &self,
        camera: Camera3D,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        Self::__draw_billboard_pro(
            camera, texture, source, position, up, size, origin, rotation, tint,
        )
    }

    // Mesh management methods

    pub fn upload_mesh(&self, mesh: &mut Mesh, dynamic: bool) {
        Self::__upload_mesh(mesh, dynamic)
    }

    pub fn update_mesh_buffer(&self, mesh: Mesh, index: i32, data: Vec<u8>, offset: i32) {
        Self::__update_mesh_buffer(mesh, index, data, offset)
    }

    pub fn unload_mesh(&self, mesh: Mesh) {
        Self::__unload_mesh(mesh)
    }

    pub fn draw_mesh(&self, mesh: Mesh, material: Material, transform: Matrix) {
        Self::__draw_mesh(mesh, material, transform)
    }

    pub fn export_mesh(&self, mesh: Mesh, filename: impl Display) -> bool {
        Self::__export_mesh(mesh, filename)
    }

    pub fn get_mesh_bounding_box(&self, mesh: Mesh) -> BoundingBox {
        Self::__get_mesh_bounding_box(mesh)
    }

    pub fn gen_mesh_tangents(&self, mesh: &mut Mesh) {
        Self::__gen_mesh_tangents(mesh)
    }

    // Mesh generation methods

    pub fn gen_mesh_poly(&self, sides: i32, radius: f32) -> Mesh {
        Self::__gen_mesh_poly(sides, radius)
    }

    pub fn gen_mesh_plane(&self, width: f32, height: f32, x: i32, z: i32) -> Mesh {
        Self::__gen_mesh_plane(width, height, x, z)
    }

    pub fn gen_mesh_cube(&self, width: f32, height: f32, length: f32) -> Mesh {
        Self::__gen_mesh_cube(width, height, length)
    }

    pub fn gen_mesh_sphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        Self::__gen_mesh_sphere(radius, rings, slices)
    }

    pub fn gen_mesh_hemisphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        Self::__gen_mesh_hemisphere(radius, rings, slices)
    }

    pub fn gen_mesh_cylinder(&self, radius: f32, height: f32, slices: i32) -> Mesh {
        Self::__gen_mesh_cylinder(radius, height, slices)
    }

    pub fn gen_mesh_cone(&self, radius: f32, height: f32, slices: i32) -> Mesh {
        Self::__gen_mesh_cone(radius, height, slices)
    }

    pub fn gen_mesh_torus(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        Self::__gen_mesh_torus(radius, size, rad_seg, sides)
    }

    pub fn gen_mesh_knot(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        Self::__gen_mesh_knot(radius, size, rad_seg, sides)
    }

    pub fn gen_mesh_heightmap(&self, heightmap: Image, size: Vector3) -> Mesh {
        Self::__gen_mesh_heightmap(heightmap, size)
    }

    pub fn gen_mesh_cubicmap(&self, heightmap: Image, size: Vector3) -> Mesh {
        Self::__gen_mesh_cubicmap(heightmap, size)
    }

    // Material loading/unloading methods
}