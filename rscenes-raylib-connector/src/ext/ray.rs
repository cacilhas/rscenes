use crate::rcore::Rcore;
use crate::rmodels::Rmodels;
use raylib_ffi::*;

pub trait RayExt {
    fn get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Self;

    fn get_collision_sphere(self, center: Vector3, radius: f32) -> RayCollision;
    fn get_collision_box(self, box_: BoundingBox) -> RayCollision;
    fn get_collision_mesh(self, mesh: Mesh, transform: Matrix) -> RayCollision;
    fn get_collision_triangle(self, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision;
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
