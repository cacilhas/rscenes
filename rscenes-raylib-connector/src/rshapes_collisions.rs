use crate::ext::vector::Vector2Ext;
use raylib_ffi::*;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RshapesCollisionsImpl;

/// Crate only methods
impl RshapesCollisionsImpl {
    // Basic shapes collision detection methods

    pub fn __check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
        unsafe { CheckCollisionRecs(rec1, rec2) }
    }

    pub fn __check_collision_circles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool {
        unsafe { CheckCollisionCircles(center1, radius1, center2, radius2) }
    }

    pub fn __check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
        unsafe { CheckCollisionCircleRec(center, radius, rec) }
    }

    pub fn __check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
        unsafe { CheckCollisionPointRec(point, rec) }
    }

    pub fn __check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
        unsafe { CheckCollisionPointCircle(point, center, radius) }
    }

    pub fn __check_collision_point_triangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool {
        unsafe { CheckCollisionPointTriangle(point, p1, p2, p3) }
    }

    pub fn __check_collision_point_poly(point: Vector2, points: &mut Vec<Vector2>) -> bool {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            CheckCollisionPointPoly(point, points, count)
        }
    }

    pub fn __check_collision_lines(
        start1: Vector2,
        end1: Vector2,
        start2: Vector2,
        end2: Vector2,
    ) -> Option<Vector2> {
        unsafe {
            let mut collision_point: Vector2 = Vector2::ZERO;
            if CheckCollisionLines(
                start1,
                end1,
                start2,
                end2,
                &mut collision_point as *mut Vector2,
            ) {
                Some(collision_point)
            } else {
                None
            }
        }
    }

    pub fn __check_collision_point_line(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: i32,
    ) -> bool {
        unsafe { CheckCollisionPointLine(point, p1, p2, threshold) }
    }

    pub fn __get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        unsafe { GetCollisionRec(rec1, rec2) }
    }
}

/// Exported methods
pub trait RshapesCollisions: Debug {
    // Basic shapes collision detection methods

    /// Check collision between two rectangles
    fn check_collision_recs(&self, rec1: Rectangle, rec2: Rectangle) -> bool {
        RshapesCollisionsImpl::__check_collision_recs(rec1, rec2)
    }

    /// Check collision between two circles
    fn check_collision_circles(
        &self,
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool {
        RshapesCollisionsImpl::__check_collision_circles(center1, radius1, center2, radius2)
    }

    /// Check collision between circle and rectangle
    fn check_collision_circle_rec(&self, center: Vector2, radius: f32, rec: Rectangle) -> bool {
        RshapesCollisionsImpl::__check_collision_circle_rec(center, radius, rec)
    }

    /// Check if point is inside rectangle
    fn check_collision_point_rec(&self, point: Vector2, rec: Rectangle) -> bool {
        RshapesCollisionsImpl::__check_collision_point_rec(point, rec)
    }

    /// Check if point is inside circle
    fn check_collision_point_circle(&self, point: Vector2, center: Vector2, radius: f32) -> bool {
        RshapesCollisionsImpl::__check_collision_point_circle(point, center, radius)
    }

    /// Check if point is inside a triangle
    fn check_collision_point_triangle(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool {
        RshapesCollisionsImpl::__check_collision_point_triangle(point, p1, p2, p3)
    }

    /// Check if point is within a polygon described by array of vertices
    fn check_collision_point_poly(&self, point: Vector2, points: &mut Vec<Vector2>) -> bool {
        RshapesCollisionsImpl::__check_collision_point_poly(point, points)
    }

    /// Check the collision between two lines defined by two points each, returns collision point by reference
    fn check_collision_lines(
        &self,
        start1: Vector2,
        end1: Vector2,
        start2: Vector2,
        end2: Vector2,
    ) -> Option<Vector2> {
        RshapesCollisionsImpl::__check_collision_lines(start1, end1, start2, end2)
    }

    /// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
    fn check_collision_point_line(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: i32,
    ) -> bool {
        RshapesCollisionsImpl::__check_collision_point_line(point, p1, p2, threshold)
    }

    /// Get collision rectangle for two rectangles collision
    fn get_collision_rec(&self, rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        RshapesCollisionsImpl::__get_collision_rec(rec1, rec2)
    }
}
