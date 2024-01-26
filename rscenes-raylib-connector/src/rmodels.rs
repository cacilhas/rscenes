use raylib_ffi::*;

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
}
