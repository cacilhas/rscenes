use crate::ext::vector::Vector2Ext;
use raylib_ffi::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Rshapes;

/// Crate only methods
impl Rshapes {
    pub(crate) fn __set_shapes_texture(texture: Texture2D, source: Rectangle) {
        unsafe { SetShapesTexture(texture, source) }
    }

    // Basic shapes drawing methods

    pub(crate) fn __draw_pixel(x: i32, y: i32, color: Color) {
        unsafe { DrawPixel(x, y, color) }
    }

    pub(crate) fn __draw_pixel_v(position: Vector2, color: Color) {
        unsafe { DrawPixelV(position, color) }
    }

    pub(crate) fn __draw_line(start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        unsafe { DrawLine(start_x, start_y, end_x, end_y, color) }
    }

    pub(crate) fn __draw_line_v(start: Vector2, end: Vector2, color: Color) {
        unsafe { DrawLineV(start, end, color) }
    }

    pub(crate) fn __draw_line_ex(start: Vector2, end: Vector2, thick: f32, color: Color) {
        unsafe { DrawLineEx(start, end, thick, color) }
    }

    pub(crate) fn __draw_line_strip(points: &mut Vec<Vector2>, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawLineStrip(points, count, color)
        }
    }

    pub(crate) fn __draw_line_bezier(start: Vector2, end: Vector2, thick: f32, color: Color) {
        unsafe { DrawLineBezier(start, end, thick, color) }
    }

    pub(crate) fn __draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { DrawCircle(center_x, center_y, radius, color) }
    }

    pub(crate) fn __draw_circle_sector(
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe { DrawCircleSector(center, radius, start_angle, end_angle, segments, color) }
    }

    pub(crate) fn __draw_circle_sector_lines(
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe { DrawCircleSectorLines(center, radius, start_angle, end_angle, segments, color) }
    }

    pub(crate) fn __draw_circle_gradient(
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { DrawCircleGradient(center_x, center_y, radius, color1, color2) }
    }

    pub(crate) fn __draw_circle_v(center: Vector2, radius: f32, color: Color) {
        unsafe { DrawCircleV(center, radius, color) }
    }

    pub(crate) fn __draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
        unsafe { DrawCircleLines(center_x, center_y, radius, color) }
    }

    pub(crate) fn __draw_circle_lines_v(center: Vector2, radius: f32, color: Color) {
        unsafe { DrawCircleLinesV(center, radius, color) }
    }

    pub(crate) fn __draw_ellipse(
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { DrawEllipse(center_x, center_y, radius_h, radius_v, color) }
    }

    pub(crate) fn __draw_ellipse_lines(
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        unsafe { DrawEllipseLines(center_x, center_y, radius_h, radius_v, color) }
    }

    pub(crate) fn __draw_ring(
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            DrawRing(
                center,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    pub(crate) fn __draw_ring_lines(
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe {
            DrawRingLines(
                center,
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments,
                color,
            )
        }
    }

    pub(crate) fn __draw_rectangle(x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe { DrawRectangle(x, y, width, height, color) }
    }

    pub(crate) fn __draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
        unsafe { DrawRectangleV(position, size, color) }
    }

    pub(crate) fn __draw_rectangle_rec(rec: Rectangle, color: Color) {
        unsafe { DrawRectangleRec(rec, color) }
    }

    pub(crate) fn __draw_rectangle_pro(
        rec: Rectangle,
        origin: Vector2,
        rotation: f32,
        color: Color,
    ) {
        unsafe { DrawRectanglePro(rec, origin, rotation, color) }
    }

    pub(crate) fn __draw_rectangle_gradient_v(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { DrawRectangleGradientV(x, y, width, height, color1, color2) }
    }

    pub(crate) fn __draw_rectangle_gradient_h(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        unsafe { DrawRectangleGradientH(x, y, width, height, color1, color2) }
    }

    pub(crate) fn __draw_rectangle_gradient_ex(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    ) {
        unsafe { DrawRectangleGradientEx(rec, col1, col2, col3, col4) }
    }

    pub(crate) fn __draw_rectangle_lines(x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe { DrawRectangleLines(x, y, width, height, color) }
    }

    pub(crate) fn __draw_rectangle_lines_ex(rec: Rectangle, thick: f32, color: Color) {
        unsafe { DrawRectangleLinesEx(rec, thick, color) }
    }

    pub(crate) fn __draw_rectangle_rounded(
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        color: Color,
    ) {
        unsafe { DrawRectangleRounded(rec, roundness, segments, color) }
    }

    pub(crate) fn __draw_rectangle_rounded_lines(
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawRectangleRoundedLines(rec, roundness, segments, thick, color) }
    }

    pub(crate) fn __draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { DrawTriangle(v1, v2, v3, color) }
    }

    pub(crate) fn __draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        unsafe { DrawTriangleLines(v1, v2, v3, color) }
    }

    pub(crate) fn __draw_triangle_fan(points: &mut Vec<Vector2>, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawTriangleFan(points, count, color)
        }
    }

    pub(crate) fn __draw_triangle_strip(points: &mut Vec<Vector2>, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawTriangleStrip(points, count, color)
        }
    }

    pub(crate) fn __draw_poly(
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { DrawPoly(center, sides, radius, rotation, color) }
    }

    pub(crate) fn __draw_poly_lines(
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe { DrawPolyLines(center, sides, radius, rotation, color) }
    }

    pub(crate) fn __draw_poly_lines_ex(
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawPolyLinesEx(center, sides, radius, rotation, thick, color) }
    }

    // Splines drawing methods

    pub(crate) fn __draw_spline_linear(points: &mut Vec<Vector2>, thick: f32, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawSplineLinear(points, count, thick, color)
        }
    }

    pub(crate) fn __draw_spline_basis(points: &mut Vec<Vector2>, thick: f32, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawSplineBasis(points, count, thick, color)
        }
    }

    pub(crate) fn __draw_spline_catmull_rom(points: &mut Vec<Vector2>, thick: f32, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawSplineCatmullRom(points, count, thick, color)
        }
    }

    pub(crate) fn __draw_spline_bezier_quadratic(
        points: &mut Vec<Vector2>,
        thick: f32,
        color: Color,
    ) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawSplineBezierQuadratic(points, count, thick, color)
        }
    }

    pub(crate) fn __draw_spline_bezier_cubic(points: &mut Vec<Vector2>, thick: f32, color: Color) {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            DrawSplineBezierCubic(points, count, thick, color)
        }
    }

    pub(crate) fn __draw_spline_segment_linear(p1: Vector2, p2: Vector2, thick: f32, color: Color) {
        unsafe { DrawSplineSegmentLinear(p1, p2, thick, color) }
    }

    pub(crate) fn __draw_spline_segment_basis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawSplineSegmentBasis(p1, p2, p3, p4, thick, color) }
    }

    pub(crate) fn __draw_spline_segment_catmull_rom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick, color) }
    }

    pub(crate) fn __draw_spline_segment_bezier_quadratic(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawSplineSegmentBezierQuadratic(p1, p2, p3, thick, color) }
    }

    pub(crate) fn __draw_spline_segment_bezier_cubic(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        unsafe { DrawSplineSegmentBezierCubic(p1, p2, p3, p4, thick, color) }
    }

    // Spline segment point evaluation methods, for a given t [0.0f .. 1.0f]

    pub(crate) fn __get_spline_point_linear(start: Vector2, end: Vector2, t: f32) -> Vector2 {
        unsafe { GetSplinePointLinear(start, end, t) }
    }

    pub(crate) fn __get_spline_point_basis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        unsafe { GetSplinePointBasis(p1, p2, p3, p4, t) }
    }

    pub(crate) fn __get_spline_point_catmull_rom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        unsafe { GetSplinePointCatmullRom(p1, p2, p3, p4, t) }
    }

    pub(crate) fn __get_spline_point_bezier_quad(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        t: f32,
    ) -> Vector2 {
        unsafe { GetSplinePointBezierQuad(p1, p2, p3, t) }
    }

    pub(crate) fn __get_spline_point_bezier_cubic(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        unsafe { GetSplinePointBezierCubic(p1, p2, p3, p4, t) }
    }

    // Basic shapes collision detection methods

    pub(crate) fn __check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
        unsafe { CheckCollisionRecs(rec1, rec2) }
    }

    pub(crate) fn __check_collision_circles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool {
        unsafe { CheckCollisionCircles(center1, radius1, center2, radius2) }
    }

    pub(crate) fn __check_collision_circle_rec(
        center: Vector2,
        radius: f32,
        rec: Rectangle,
    ) -> bool {
        unsafe { CheckCollisionCircleRec(center, radius, rec) }
    }

    pub(crate) fn __check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
        unsafe { CheckCollisionPointRec(point, rec) }
    }

    pub(crate) fn __check_collision_point_circle(
        point: Vector2,
        center: Vector2,
        radius: f32,
    ) -> bool {
        unsafe { CheckCollisionPointCircle(point, center, radius) }
    }

    pub(crate) fn __check_collision_point_triangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool {
        unsafe { CheckCollisionPointTriangle(point, p1, p2, p3) }
    }

    pub(crate) fn __check_collision_point_poly(point: Vector2, points: &mut Vec<Vector2>) -> bool {
        unsafe {
            let count = points.len() as i32;
            let points = points.as_mut_ptr();
            CheckCollisionPointPoly(point, points, count)
        }
    }

    pub(crate) fn __check_collision_lines(
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

    pub(crate) fn __check_collision_point_line(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: i32,
    ) -> bool {
        unsafe { CheckCollisionPointLine(point, p1, p2, threshold) }
    }

    pub(crate) fn __get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        unsafe { GetCollisionRec(rec1, rec2) }
    }
}

/// Exported methods
impl Rshapes {
    pub fn set_shapes_texture(&self, texture: Texture2D, source: Rectangle) {
        Self::__set_shapes_texture(texture, source)
    }

    // Basic shapes drawing methods

    pub fn draw_pixel(&self, x: i32, y: i32, color: Color) {
        Self::__draw_pixel(x, y, color)
    }

    pub fn draw_pixel_v(&self, position: Vector2, color: Color) {
        Self::__draw_pixel_v(position, color)
    }

    pub fn draw_line(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        Self::__draw_line(start_x, start_y, end_x, end_y, color)
    }

    pub fn draw_line_v(&self, start: Vector2, end: Vector2, color: Color) {
        Self::__draw_line_v(start, end, color)
    }

    pub fn draw_line_ex(&self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        Self::__draw_line_ex(start, end, thick, color)
    }

    pub fn draw_line_strip(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_line_strip(points, color)
    }

    pub fn draw_line_bezier(&self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        Self::__draw_line_bezier(start, end, thick, color)
    }

    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        Self::__draw_circle(center_x, center_y, radius, color)
    }

    pub fn draw_circle_sector(
        &self,
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_circle_sector(center, radius, start_angle, end_angle, segments, color)
    }

    pub fn draw_circle_sector_lines(
        &self,
        center: Vector2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_circle_sector_lines(center, radius, start_angle, end_angle, segments, color)
    }

    pub fn draw_circle_gradient(
        &self,
        center_x: i32,
        center_y: i32,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        Self::__draw_circle_gradient(center_x, center_y, radius, color1, color2)
    }

    pub fn draw_circle_v(&self, center: Vector2, radius: f32, color: Color) {
        Self::__draw_circle_v(center, radius, color)
    }

    pub fn draw_circle_lines(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        Self::__draw_circle_lines(center_x, center_y, radius, color)
    }

    pub fn draw_circle_lines_v(&self, center: Vector2, radius: f32, color: Color) {
        Self::__draw_circle_lines_v(center, radius, color)
    }

    pub fn draw_ellipse(
        &self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        Self::__draw_ellipse(center_x, center_y, radius_h, radius_v, color)
    }

    pub fn draw_ellipse_lines(
        &self,
        center_x: i32,
        center_y: i32,
        radius_h: f32,
        radius_v: f32,
        color: Color,
    ) {
        Self::__draw_ellipse_lines(center_x, center_y, radius_h, radius_v, color)
    }

    pub fn draw_ring(
        &self,
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_ring(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    pub fn draw_ring_lines(
        &self,
        center: Vector2,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_ring_lines(
            center,
            inner_radius,
            outer_radius,
            start_angle,
            end_angle,
            segments,
            color,
        )
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        Self::__draw_rectangle(x, y, width, height, color)
    }

    pub fn draw_rectangle_v(&self, position: Vector2, size: Vector2, color: Color) {
        Self::__draw_rectangle_v(position, size, color)
    }

    pub fn draw_rectangle_rec(&self, rec: Rectangle, color: Color) {
        Self::__draw_rectangle_rec(rec, color)
    }

    pub fn draw_rectangle_pro(&self, rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
        Self::__draw_rectangle_pro(rec, origin, rotation, color)
    }

    pub fn draw_rectangle_gradient_v(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        Self::__draw_rectangle_gradient_v(x, y, width, height, color1, color2)
    }

    pub fn draw_rectangle_gradient_h(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color1: Color,
        color2: Color,
    ) {
        Self::__draw_rectangle_gradient_h(x, y, width, height, color1, color2)
    }

    pub fn draw_rectangle_gradient_ex(
        &self,
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    ) {
        Self::__draw_rectangle_gradient_ex(rec, col1, col2, col3, col4)
    }

    pub fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        Self::__draw_rectangle_lines(x, y, width, height, color)
    }

    pub fn draw_rectangle_lines_ex(&self, rec: Rectangle, thick: f32, color: Color) {
        Self::__draw_rectangle_lines_ex(rec, thick, color)
    }

    pub fn draw_rectangle_rounded(
        &self,
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_rectangle_rounded(rec, roundness, segments, color)
    }

    pub fn draw_rectangle_rounded_lines(
        &self,
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_rectangle_rounded_lines(rec, roundness, segments, thick, color)
    }

    pub fn draw_triangle(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        Self::__draw_triangle(v1, v2, v3, color)
    }

    pub fn draw_triangle_lines(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        Self::__draw_triangle_lines(v1, v2, v3, color)
    }

    pub fn draw_triangle_fan(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_triangle_fan(points, color)
    }

    pub fn draw_triangle_strip(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_triangle_strip(points, color)
    }

    pub fn draw_poly(&self, center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
        Self::__draw_poly(center, sides, radius, rotation, color)
    }

    pub fn draw_poly_lines(
        &self,
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        Self::__draw_poly_lines(center, sides, radius, rotation, color)
    }

    pub fn draw_poly_lines_ex(
        &self,
        center: Vector2,
        sides: i32,
        radius: f32,
        rotation: f32,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_poly_lines_ex(center, sides, radius, rotation, thick, color)
    }

    // Splines drawing methods

    pub fn draw_spline_linear(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_linear(points, thick, color)
    }

    pub fn draw_spline_basis(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_basis(points, thick, color)
    }

    pub fn draw_spline_catmull_rom(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_catmull_rom(points, thick, color)
    }

    pub fn draw_spline_berzier_quadratic(
        &self,
        points: &mut Vec<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_bezier_quadratic(points, thick, color)
    }

    pub fn draw_spline_berzier_cubic(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_bezier_cubic(points, thick, color)
    }

    pub fn draw_spline_segment_linear(&self, p1: Vector2, p2: Vector2, thick: f32, color: Color) {
        Self::__draw_spline_segment_linear(p1, p2, thick, color)
    }

    pub fn draw_spline_segment_basis(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_segment_basis(p1, p2, p3, p4, thick, color)
    }

    pub fn draw_spline_segment_catmull_rom(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_segment_catmull_rom(p1, p2, p3, p4, thick, color)
    }

    pub fn draw_spline_segment_bezier_quadratic(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_segment_bezier_quadratic(p1, p2, p3, thick, color)
    }

    pub fn draw_spline_segment_bezier_cubic(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_segment_bezier_cubic(p1, p2, p3, p4, thick, color)
    }

    // Spline segment point evaluation methods, for a given t [0.0f .. 1.0f]

    pub fn get_spline_point_linear(&self, start: Vector2, end: Vector2, t: f32) -> Vector2 {
        Self::__get_spline_point_linear(start, end, t)
    }

    pub fn get_spline_point_basis(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        Self::__get_spline_point_basis(p1, p2, p3, p4, t)
    }

    pub fn get_spline_point_catmull_rom(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        Self::__get_spline_point_catmull_rom(p1, p2, p3, p4, t)
    }

    pub fn get_spline_point_bezier_quad(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        t: f32,
    ) -> Vector2 {
        Self::__get_spline_point_bezier_quad(p1, p2, p3, t)
    }

    pub fn get_spline_point_bezier_cubic(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: f32,
    ) -> Vector2 {
        Self::__get_spline_point_bezier_cubic(p1, p2, p3, p4, t)
    }

    // Basic shapes collision detection methods

    pub fn check_collision_recs(&self, rec1: Rectangle, rec2: Rectangle) -> bool {
        Self::__check_collision_recs(rec1, rec2)
    }

    pub fn check_collision_circles(
        &self,
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool {
        Self::__check_collision_circles(center1, radius1, center2, radius2)
    }

    pub fn check_collision_circle_rec(&self, center: Vector2, radius: f32, rec: Rectangle) -> bool {
        Self::__check_collision_circle_rec(center, radius, rec)
    }

    pub fn check_collision_point_rec(&self, point: Vector2, rec: Rectangle) -> bool {
        Self::__check_collision_point_rec(point, rec)
    }

    pub fn check_collision_point_circle(
        &self,
        point: Vector2,
        center: Vector2,
        radius: f32,
    ) -> bool {
        Self::__check_collision_point_circle(point, center, radius)
    }

    pub fn check_collision_point_triangle(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool {
        Self::__check_collision_point_triangle(point, p1, p2, p3)
    }

    pub fn check_collision_point_poly(&self, point: Vector2, points: &mut Vec<Vector2>) -> bool {
        Self::__check_collision_point_poly(point, points)
    }

    pub fn check_collision_lines(
        &self,
        start1: Vector2,
        end1: Vector2,
        start2: Vector2,
        end2: Vector2,
    ) -> Option<Vector2> {
        Self::__check_collision_lines(start1, end1, start2, end2)
    }

    pub fn check_collision_point_line(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: i32,
    ) -> bool {
        Self::__check_collision_point_line(point, p1, p2, threshold)
    }

    pub fn get_collision_rec(&self, rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        Self::__get_collision_rec(rec1, rec2)
    }
}
