use raylib_ffi::*;
use std::{ffi::c_void, fmt::Display};

use crate::utils::array_from_c;

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

    pub(crate) fn __draw_triangle_strip_3d(points: &mut Vec<Vector3>, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
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

    pub(crate) fn __load_model(filename: impl Display) -> Result<Model, String> {
        unsafe {
            let model = LoadModel(rl_str!(filename));
            if model.meshCount > 0 {
                Ok(model)
            } else {
                Err(format!("couldn't load model from {}", filename))
            }
        }
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

    pub(crate) fn __update_mesh_buffer(mesh: Mesh, index: i32, data: &mut Vec<u8>, offset: i32) {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_mut_ptr() as *mut c_void;
            UpdateMeshBuffer(mesh, index, data, size, offset)
        }
    }

    pub(crate) fn __unload_mesh(mesh: Mesh) {
        unsafe { UnloadMesh(mesh) }
    }

    pub(crate) fn __draw_mesh(mesh: Mesh, material: Material, transform: Matrix) {
        unsafe { DrawMesh(mesh, material, transform) }
    }

    pub(crate) fn __draw_mesh_instanced(mesh: Mesh, material: Material, transforms: &[Matrix]) {
        unsafe {
            let instances = transforms.len() as i32;
            let transforms = transforms.as_ptr();
            DrawMeshInstanced(mesh, material, transforms, instances)
        }
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

    pub(crate) fn __load_materials(filename: impl Display) -> Result<Vec<Material>, String> {
        unsafe {
            let mut count: i32 = 0;
            let raw = LoadMaterials(rl_str!(filename), &mut count);
            array_from_c(raw, count as usize, || {
                format!("couldn't load material from {}", filename)
            })
        }
    }

    pub(crate) fn __load_material_default() -> Material {
        unsafe { LoadMaterialDefault() }
    }

    pub(crate) fn __is_material_ready(material: Material) -> bool {
        unsafe { IsMaterialReady(material) }
    }

    pub(crate) fn __unload_material(material: Material) {
        unsafe { UnloadMaterial(material) }
    }

    pub(crate) fn __set_material_texture(
        material: &mut Material,
        map_tpe: i32,
        texture: Texture2D,
    ) {
        unsafe { SetMaterialTexture(material, map_tpe, texture) }
    }

    pub(crate) fn __set_model_mesh_material(model: &mut Model, mesh_id: i32, material_id: i32) {
        unsafe { SetModelMeshMaterial(model, mesh_id, material_id) }
    }

    // Model animations loading/unloading methods

    pub(crate) fn __load_model_animations(
        filename: impl Display,
    ) -> Result<Vec<ModelAnimation>, String> {
        unsafe {
            let mut count: i32 = 0;
            let raw = LoadModelAnimations(rl_str!(filename), &mut count);
            array_from_c(raw, count as usize, || {
                format!("couldn't load model animations from {}", filename)
            })
        }
    }

    pub(crate) fn __update_model_animation(model: Model, anim: ModelAnimation, frame: i32) {
        unsafe { UpdateModelAnimation(model, anim, frame) }
    }

    pub(crate) fn __unload_model_animation(anim: ModelAnimation) {
        unsafe { UnloadModelAnimation(anim) }
    }

    pub(crate) fn __unload_model_animations(mut anims: Vec<ModelAnimation>) {
        unsafe {
            let count = anims.len() as i32;
            UnloadModelAnimations(anims.as_mut_ptr(), count)
        }
    }

    pub(crate) fn __is_model_animation_invalid(model: Model, anim: ModelAnimation) -> bool {
        unsafe { IsModelAnimationValid(model, anim) }
    }

    // Collision detection methods

    pub(crate) fn __check_collision_spheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool {
        unsafe { CheckCollisionSpheres(center1, radius1, center2, radius2) }
    }

    pub(crate) fn __check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
        unsafe { CheckCollisionBoxes(box1, box2) }
    }

    pub(crate) fn __check_collision_box_sphere(
        box_: BoundingBox,
        center: Vector3,
        radius: f32,
    ) -> bool {
        unsafe { CheckCollisionBoxSphere(box_, center, radius) }
    }

    pub(crate) fn __get_raycollision_sphere(
        ray: Ray,
        center: Vector3,
        radius: f32,
    ) -> RayCollision {
        unsafe { GetRayCollisionSphere(ray, center, radius) }
    }

    pub(crate) fn __get_raycollision_box(ray: Ray, box_: BoundingBox) -> RayCollision {
        unsafe { GetRayCollisionBox(ray, box_) }
    }

    pub(crate) fn __get_raycollision_mesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision {
        unsafe { GetRayCollisionMesh(ray, mesh, transform) }
    }

    pub(crate) fn __get_raycollision_triangle(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
    ) -> RayCollision {
        unsafe { GetRayCollisionTriangle(ray, p1, p2, p3) }
    }

    pub(crate) fn __get_raycollision_quad(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision {
        unsafe { GetRayCollisionQuad(ray, p1, p2, p3, p4) }
    }
}

/// Exported methods
impl Rmodels {
    // Basic geometric 3D shapes drawing methods

    /// Draw a line in 3D world space
    pub fn draw_line_3d(&self, start: Vector3, end: Vector3, color: Color) {
        Self::__draw_line_3d(start, end, color)
    }

    /// Draw a point in 3D space, actually a small line
    pub fn draw_point_3d(&self, position: Vector3, color: Color) {
        Self::__draw_point_3d(position, color)
    }

    /// Draw a circle in 3D world space
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

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    pub fn draw_triangle_3d(&self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        Self::__draw_triangle_3d(v1, v2, v3, color)
    }

    /// Draw a triangle strip defined by points
    pub fn draw_triangle_strip_3d(&self, points: &mut Vec<Vector3>, color: Color) {
        Self::__draw_triangle_strip_3d(points, color)
    }

    /// Draw cube
    pub fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        Self::__draw_cube(position, width, height, length, color)
    }

    /// Draw cube (Vector version)
    pub fn draw_cube_v(&self, position: Vector3, size: Vector3, color: Color) {
        Self::__draw_cube_v(position, size, color)
    }

    /// Draw cube wires
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

    /// Draw cube wires (Vector version)
    pub fn draw_cube_wires_v(&self, position: Vector3, size: Vector3, color: Color) {
        Self::__draw_cube_wires_v(position, size, color)
    }

    /// Draw sphere
    pub fn draw_sphere(&self, center: Vector3, radius: f32, color: Color) {
        Self::__draw_sphere(center, radius, color)
    }

    /// Draw sphere with extended parameters
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

    /// Draw sphere wires
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

    /// Draw a cylinder/cone
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

    /// Draw a cylinder with base at startPos and top at endPos
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

    /// Draw a cylinder/cone wires
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

    /// Draw a cylinder wires with base at startPos and top at endPos
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

    /// Draw a capsule with the center of its sphere caps at startPos and endPos
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

    /// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
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

    // Draw a plane XZ
    pub fn draw_plane(&self, center: Vector3, size: Vector2, color: Color) {
        Self::__draw_plane(center, size, color)
    }

    /// Draw a ray line
    pub fn draw_ray(&self, ray: Ray, color: Color) {
        Self::__draw_ray(ray, color)
    }

    /// Draw a grid (centered at (0, 0, 0))
    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        Self::__draw_grid(slices, spacing)
    }

    // Model management methods

    /// Load model from files (meshes and materials)
    pub fn load_model(&self, filename: impl Display) -> Result<Model, String> {
        Self::__load_model(filename)
    }

    /// Load model from generated mesh (default material)
    pub fn load_model_from_mesh(&self, mesh: Mesh) -> Model {
        Self::__load_model_from_mesh(mesh)
    }

    /// check whether a model is ready
    pub fn is_model_ready(&self, model: Model) -> bool {
        Self::__is_model_ready(model)
    }

    /// Unload model (including meshes) from memory (RAM and/or VRAM)
    pub fn unload_model(&self, model: Model) {
        Self::__unload_model(model)
    }

    /// Compute model bounding box limits (considers all meshes)
    pub fn get_model_bounding_box(&self, model: Model) -> BoundingBox {
        Self::__get_model_bounding_box(model)
    }

    // Model drawing methods

    /// Draw a model (with texture if set)
    pub fn draw_model(&self, model: Model, position: Vector3, scale: f32, tint: Color) {
        Self::__draw_model(model, position, scale, tint)
    }

    /// Draw a model with extended parameters
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

    /// Draw a model wires (with texture if set)
    pub fn draw_model_wires(&self, model: Model, position: Vector3, scale: f32, tint: Color) {
        Self::__draw_model_wires(model, position, scale, tint)
    }

    /// Draw a model wires (with texture if set) with extended parameters
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

    /// Draw bounding box (wires)
    pub fn draw_bounding_box(&self, box_: BoundingBox, color: Color) {
        Self::__draw_bounding_box(box_, color)
    }

    /// Draw a billboard texture
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

    /// Draw a billboard texture defined by source
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

    /// Draw a billboard texture defined by source and rotation
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

    /// Upload mesh vertex data in GPU and provide VAO/VBO ids
    pub fn upload_mesh(&self, mesh: &mut Mesh, dynamic: bool) {
        Self::__upload_mesh(mesh, dynamic)
    }

    /// Update mesh vertex data in GPU for a specific buffer index
    pub fn update_mesh_buffer(&self, mesh: Mesh, index: i32, data: &mut Vec<u8>, offset: i32) {
        Self::__update_mesh_buffer(mesh, index, data, offset)
    }

    /// Unload mesh data from CPU and GPU
    pub fn unload_mesh(&self, mesh: Mesh) {
        Self::__unload_mesh(mesh)
    }

    /// Draw a 3D mesh with material and transform
    pub fn draw_mesh(&self, mesh: Mesh, material: Material, transform: Matrix) {
        Self::__draw_mesh(mesh, material, transform)
    }

    /// Draw multiple mesh instances with material and different transforms
    pub fn draw_mesh_instanced(&self, mesh: Mesh, material: Material, transforms: &[Matrix]) {
        Self::__draw_mesh_instanced(mesh, material, transforms)
    }

    /// Export mesh data to file, returns true on success
    pub fn export_mesh(&self, mesh: Mesh, filename: impl Display) -> bool {
        Self::__export_mesh(mesh, filename)
    }

    /// Compute mesh bounding box limits
    pub fn get_mesh_bounding_box(&self, mesh: Mesh) -> BoundingBox {
        Self::__get_mesh_bounding_box(mesh)
    }

    /// Compute mesh tangents
    pub fn gen_mesh_tangents(&self, mesh: &mut Mesh) {
        Self::__gen_mesh_tangents(mesh)
    }

    // Mesh generation methods

    // Generate polygonal mesh
    pub fn gen_mesh_poly(&self, sides: i32, radius: f32) -> Mesh {
        Self::__gen_mesh_poly(sides, radius)
    }

    /// Generate plane mesh (with subdivisions)
    pub fn gen_mesh_plane(&self, width: f32, height: f32, x: i32, z: i32) -> Mesh {
        Self::__gen_mesh_plane(width, height, x, z)
    }

    /// Generate cuboid mesh
    pub fn gen_mesh_cube(&self, width: f32, height: f32, length: f32) -> Mesh {
        Self::__gen_mesh_cube(width, height, length)
    }

    /// Generate sphere mesh (standard sphere)
    pub fn gen_mesh_sphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        Self::__gen_mesh_sphere(radius, rings, slices)
    }

    /// Generate half-sphere mesh (no bottom cap)
    pub fn gen_mesh_hemisphere(&self, radius: f32, rings: i32, slices: i32) -> Mesh {
        Self::__gen_mesh_hemisphere(radius, rings, slices)
    }

    /// Generate cylinder mesh
    pub fn gen_mesh_cylinder(&self, radius: f32, height: f32, slices: i32) -> Mesh {
        Self::__gen_mesh_cylinder(radius, height, slices)
    }

    /// Generate cone/pyramid mesh
    pub fn gen_mesh_cone(&self, radius: f32, height: f32, slices: i32) -> Mesh {
        Self::__gen_mesh_cone(radius, height, slices)
    }

    /// Generate torus mesh
    pub fn gen_mesh_torus(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        Self::__gen_mesh_torus(radius, size, rad_seg, sides)
    }

    /// Generate trefoil knot mesh
    pub fn gen_mesh_knot(&self, radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
        Self::__gen_mesh_knot(radius, size, rad_seg, sides)
    }

    /// Generate heightmap mesh from image data
    pub fn gen_mesh_heightmap(&self, heightmap: Image, size: Vector3) -> Mesh {
        Self::__gen_mesh_heightmap(heightmap, size)
    }

    /// Generate cubes-based map mesh from image data
    pub fn gen_mesh_cubicmap(&self, heightmap: Image, size: Vector3) -> Mesh {
        Self::__gen_mesh_cubicmap(heightmap, size)
    }

    // Material loading/unloading methods

    /// Load materials from model file
    pub fn load_materials(&self, filename: impl Display) -> Result<Vec<Material>, String> {
        Self::__load_materials(filename)
    }

    /// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
    pub fn load_material_default(&self) -> Material {
        Self::__load_material_default()
    }

    /// check whether a material is ready
    pub fn is_material_ready(&self, material: Material) -> bool {
        Self::__is_material_ready(material)
    }

    /// Unload material from GPU memory (VRAM)
    pub fn unload_material(&self, material: Material) {
        Self::__unload_material(material)
    }

    /// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
    pub fn set_material_texture(&self, material: &mut Material, map_tpe: i32, texture: Texture2D) {
        Self::__set_material_texture(material, map_tpe, texture)
    }

    /// Set material for a mesh
    pub fn set_model_mesh_material(&self, model: &mut Model, mesh_id: i32, material_id: i32) {
        Self::__set_model_mesh_material(model, mesh_id, material_id)
    }

    // Model animations loading/unloading methods

    /// Load model animations from file
    pub fn load_model_animations(
        &self,
        filename: impl Display,
    ) -> Result<Vec<ModelAnimation>, String> {
        Self::__load_model_animations(filename)
    }

    /// Update model animation pose
    pub fn update_model_animation(&self, model: Model, anim: ModelAnimation, frame: i32) {
        Self::__update_model_animation(model, anim, frame)
    }

    /// Unload animation data
    pub fn unload_model_animation(&self, anim: ModelAnimation) {
        Self::__unload_model_animation(anim)
    }

    /// Unload animation array data
    pub fn unload_model_animations(&self, anims: Vec<ModelAnimation>) {
        Self::__unload_model_animations(anims)
    }

    /// Check model animation skeleton match
    pub fn is_model_animation_invalid(&self, model: Model, anim: ModelAnimation) -> bool {
        Self::__is_model_animation_invalid(model, anim)
    }

    // Collision detection methods

    // Check collision between two spheres
    pub fn check_collision_spheres(
        &self,
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool {
        Self::__check_collision_spheres(center1, radius1, center2, radius2)
    }

    /// Check collision between two bounding boxes
    pub fn check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
        Self::__check_collision_boxes(box1, box2)
    }

    /// Check collision between box and sphere
    pub fn check_collision_box_sphere(
        &self,
        box_: BoundingBox,
        center: Vector3,
        radius: f32,
    ) -> bool {
        Self::__check_collision_box_sphere(box_, center, radius)
    }

    /// Get collision info between ray and sphere
    pub fn get_raycollision_sphere(&self, ray: Ray, center: Vector3, radius: f32) -> RayCollision {
        Self::__get_raycollision_sphere(ray, center, radius)
    }

    /// Get collision info between ray and box
    pub fn get_raycollision_box(&self, ray: Ray, box_: BoundingBox) -> RayCollision {
        Self::__get_raycollision_box(ray, box_)
    }

    /// Get collision info between ray and mesh
    pub fn get_raycollision_mesh(&self, ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision {
        Self::__get_raycollision_mesh(ray, mesh, transform)
    }

    /// Get collision info between ray and triangle
    pub fn get_raycollision_triangle(
        &self,
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
    ) -> RayCollision {
        Self::__get_raycollision_triangle(ray, p1, p2, p3)
    }

    /// Get collision info between ray and quad
    pub fn get_raycollision_quad(
        &self,
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision {
        Self::__get_raycollision_quad(ray, p1, p2, p3, p4)
    }
}
