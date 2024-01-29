use crate::rcore::Rcore;
use crate::rmodels::Rmodels;
use raylib_ffi::*;

pub trait RayExt: Sized {
    /// Get a ray trace from mouse position
    fn get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Self;

    /// Get collision info between ray and sphere
    fn get_collision_sphere(self, center: Vector3, radius: f32) -> RayCollision;
    /// Get collision info between ray and box
    fn get_collision_box(self, box_: BoundingBox) -> RayCollision;
    /// Get collision info between ray and mesh
    fn get_collision_mesh(self, mesh: Mesh, transform: Matrix) -> RayCollision;
    /// Get collision info between ray and triangle
    fn get_collision_triangle(self, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision;
    /// Get collision info between ray and quad
    fn get_collision_quad(self, p1: Vector3, p2: Vector3, p3: Vector3, p4: Vector3)
        -> RayCollision;
}

impl RayExt for Ray {
    fn get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Self {
        Rcore::__get_mouse_ray(mouse_position, camera)
    }

    fn get_collision_sphere(self, center: Vector3, radius: f32) -> RayCollision {
        Rmodels::__get_raycollision_sphere(self, center, radius)
    }

    fn get_collision_box(self, box_: BoundingBox) -> RayCollision {
        Rmodels::__get_raycollision_box(self, box_)
    }

    fn get_collision_mesh(self, mesh: Mesh, transform: Matrix) -> RayCollision {
        Rmodels::__get_raycollision_mesh(self, mesh, transform)
    }

    fn get_collision_triangle(self, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision {
        Rmodels::__get_raycollision_triangle(self, p1, p2, p3)
    }

    fn get_collision_quad(
        self,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision {
        Rmodels::__get_raycollision_quad(self, p1, p2, p3, p4)
    }
}
