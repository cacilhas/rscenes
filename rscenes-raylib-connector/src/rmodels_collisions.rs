use crate::utils::array_from_c;
use raylib_ffi::*;
use std::{
    ffi::c_void,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RmodelsCollisionsImpl;

/// Crate only methods
impl RmodelsCollisionsImpl {
    // Collision detection methods

    pub fn __check_collision_spheres(
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool {
        unsafe { CheckCollisionSpheres(center1, radius1, center2, radius2) }
    }

    pub fn __check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
        unsafe { CheckCollisionBoxes(box1, box2) }
    }

    pub fn __check_collision_box_sphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool {
        unsafe { CheckCollisionBoxSphere(box_, center, radius) }
    }

    pub fn __get_raycollision_sphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision {
        unsafe { GetRayCollisionSphere(ray, center, radius) }
    }

    pub fn __get_raycollision_box(ray: Ray, box_: BoundingBox) -> RayCollision {
        unsafe { GetRayCollisionBox(ray, box_) }
    }

    pub fn __get_raycollision_mesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision {
        unsafe { GetRayCollisionMesh(ray, mesh, transform) }
    }

    pub fn __get_raycollision_triangle(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
    ) -> RayCollision {
        unsafe { GetRayCollisionTriangle(ray, p1, p2, p3) }
    }

    pub fn __get_raycollision_quad(
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
pub trait RmodelsCollisions: Debug {
    // Collision detection methods

    // Check collision between two spheres
    fn check_collision_spheres(
        &self,
        center1: Vector3,
        radius1: f32,
        center2: Vector3,
        radius2: f32,
    ) -> bool {
        RmodelsCollisionsImpl::__check_collision_spheres(center1, radius1, center2, radius2)
    }

    /// Check collision between two bounding boxes
    fn check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
        RmodelsCollisionsImpl::__check_collision_boxes(box1, box2)
    }

    /// Check collision between box and sphere
    fn check_collision_box_sphere(&self, box_: BoundingBox, center: Vector3, radius: f32) -> bool {
        RmodelsCollisionsImpl::__check_collision_box_sphere(box_, center, radius)
    }

    /// Get collision info between ray and sphere
    fn get_raycollision_sphere(&self, ray: Ray, center: Vector3, radius: f32) -> RayCollision {
        RmodelsCollisionsImpl::__get_raycollision_sphere(ray, center, radius)
    }

    /// Get collision info between ray and box
    fn get_raycollision_box(&self, ray: Ray, box_: BoundingBox) -> RayCollision {
        RmodelsCollisionsImpl::__get_raycollision_box(ray, box_)
    }

    /// Get collision info between ray and mesh
    fn get_raycollision_mesh(&self, ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision {
        RmodelsCollisionsImpl::__get_raycollision_mesh(ray, mesh, transform)
    }

    /// Get collision info between ray and triangle
    fn get_raycollision_triangle(
        &self,
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
    ) -> RayCollision {
        RmodelsCollisionsImpl::__get_raycollision_triangle(ray, p1, p2, p3)
    }

    /// Get collision info between ray and quad
    fn get_raycollision_quad(
        &self,
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision {
        RmodelsCollisionsImpl::__get_raycollision_quad(ray, p1, p2, p3, p4)
    }
}
