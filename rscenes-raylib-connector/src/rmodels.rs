use raylib_ffi::*;
use std::fmt::Display;

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
}
