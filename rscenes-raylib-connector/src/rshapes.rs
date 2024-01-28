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
    /// Set texture and rectangle to be used on shapes drawing
    pub fn set_shapes_texture(&self, texture: Texture2D, source: Rectangle) {
        Self::__set_shapes_texture(texture, source)
    }

    // Basic shapes drawing methods

    /// Draw a pixel
    pub fn draw_pixel(&self, x: i32, y: i32, color: Color) {
        Self::__draw_pixel(x, y, color)
    }

    /// Draw a pixel (Vector version)
    pub fn draw_pixel_v(&self, position: Vector2, color: Color) {
        Self::__draw_pixel_v(position, color)
    }

    /// Draw a line
    pub fn draw_line(&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, color: Color) {
        Self::__draw_line(start_x, start_y, end_x, end_y, color)
    }

    /// Draw a line (using GL lines)
    pub fn draw_line_v(&self, start: Vector2, end: Vector2, color: Color) {
        Self::__draw_line_v(start, end, color)
    }

    /// Draw a line (using triangles/quads)
    pub fn draw_line_ex(&self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        Self::__draw_line_ex(start, end, thick, color)
    }

    /// Draw lines sequence (using GL lines)
    pub fn draw_line_strip(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_line_strip(points, color)
    }

    /// Draw line segment cubic-bezier in-out interpolation
    pub fn draw_line_bezier(&self, start: Vector2, end: Vector2, thick: f32, color: Color) {
        Self::__draw_line_bezier(start, end, thick, color)
    }

    /// Draw a color-filled circle
    pub fn draw_circle(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        Self::__draw_circle(center_x, center_y, radius, color)
    }

    /// Draw a piece of a circle
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

    /// Draw circle sector outline
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

    /// Draw a gradient-filled circle
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

    /// Draw a color-filled circle (Vector version)
    pub fn draw_circle_v(&self, center: Vector2, radius: f32, color: Color) {
        Self::__draw_circle_v(center, radius, color)
    }

    /// Draw circle outline
    pub fn draw_circle_lines(&self, center_x: i32, center_y: i32, radius: f32, color: Color) {
        Self::__draw_circle_lines(center_x, center_y, radius, color)
    }

    /// Draw circle outline (Vector version)
    pub fn draw_circle_lines_v(&self, center: Vector2, radius: f32, color: Color) {
        Self::__draw_circle_lines_v(center, radius, color)
    }

    /// Draw ellipse
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

    /// Draw ellipse outline
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

    /// Draw ring
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

    /// Draw ring outline
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

    /// Draw a color-filled rectangle
    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        Self::__draw_rectangle(x, y, width, height, color)
    }

    /// Draw a color-filled rectangle (Vector version)
    pub fn draw_rectangle_v(&self, position: Vector2, size: Vector2, color: Color) {
        Self::__draw_rectangle_v(position, size, color)
    }

    /// Draw a color-filled rectangle
    pub fn draw_rectangle_rec(&self, rec: Rectangle, color: Color) {
        Self::__draw_rectangle_rec(rec, color)
    }

    /// Draw a color-filled rectangle with pro parameters
    pub fn draw_rectangle_pro(&self, rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
        Self::__draw_rectangle_pro(rec, origin, rotation, color)
    }

    /// Draw a vertical-gradient-filled rectangle
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

    /// Draw a horizontal-gradient-filled rectangle
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

    /// Draw a gradient-filled rectangle with custom vertex colors
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

    /// Draw rectangle outline
    pub fn draw_rectangle_lines(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        Self::__draw_rectangle_lines(x, y, width, height, color)
    }

    /// Draw rectangle outline with extended parameters
    pub fn draw_rectangle_lines_ex(&self, rec: Rectangle, thick: f32, color: Color) {
        Self::__draw_rectangle_lines_ex(rec, thick, color)
    }

    /// Draw rectangle with rounded edges
    pub fn draw_rectangle_rounded(
        &self,
        rec: Rectangle,
        roundness: f32,
        segments: i32,
        color: Color,
    ) {
        Self::__draw_rectangle_rounded(rec, roundness, segments, color)
    }

    /// Draw rectangle with rounded edges outline
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

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    pub fn draw_triangle(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        Self::__draw_triangle(v1, v2, v3, color)
    }

    /// Draw triangle outline (vertex in counter-clockwise order!)
    pub fn draw_triangle_lines(&self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        Self::__draw_triangle_lines(v1, v2, v3, color)
    }

    /// Draw a triangle fan defined by points (first vertex is the center)
    pub fn draw_triangle_fan(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_triangle_fan(points, color)
    }

    /// Draw a triangle strip defined by points
    pub fn draw_triangle_strip(&self, points: &mut Vec<Vector2>, color: Color) {
        Self::__draw_triangle_strip(points, color)
    }

    /// Draw a regular polygon (Vector version)
    pub fn draw_poly(&self, center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
        Self::__draw_poly(center, sides, radius, rotation, color)
    }

    /// Draw a polygon outline of n sides
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

    /// Draw a polygon outline of n sides with extended parameters
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

    /// Draw spline: Linear, minimum 2 points
    pub fn draw_spline_linear(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_linear(points, thick, color)
    }

    /// Draw spline: B-Spline, minimum 4 points
    pub fn draw_spline_basis(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_basis(points, thick, color)
    }

    /// Draw spline: Catmull-Rom, minimum 4 points
    pub fn draw_spline_catmull_rom(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_catmull_rom(points, thick, color)
    }

    /// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
    pub fn draw_spline_berzier_quadratic(
        &self,
        points: &mut Vec<Vector2>,
        thick: f32,
        color: Color,
    ) {
        Self::__draw_spline_bezier_quadratic(points, thick, color)
    }

    /// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
    pub fn draw_spline_berzier_cubic(&self, points: &mut Vec<Vector2>, thick: f32, color: Color) {
        Self::__draw_spline_bezier_cubic(points, thick, color)
    }

    /// Draw spline segment: Linear, 2 points
    pub fn draw_spline_segment_linear(&self, p1: Vector2, p2: Vector2, thick: f32, color: Color) {
        Self::__draw_spline_segment_linear(p1, p2, thick, color)
    }

    /// Draw spline segment: B-Spline, 4 points
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

    /// Draw spline segment: Catmull-Rom, 4 points
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

    /// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
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

    /// Draw spline segment: Cubic Bezier, 2 points, 2 control points
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

    /// Get (evaluate) spline point: Linear
    pub fn get_spline_point_linear(&self, start: Vector2, end: Vector2, t: f32) -> Vector2 {
        Self::__get_spline_point_linear(start, end, t)
    }

    /// Get (evaluate) spline point: B-Spline
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

    /// Get (evaluate) spline point: Catmull-Rom
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

    /// Get (evaluate) spline point: Quadratic Bezier
    pub fn get_spline_point_bezier_quad(
        &self,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        t: f32,
    ) -> Vector2 {
        Self::__get_spline_point_bezier_quad(p1, p2, p3, t)
    }

    // Get (evaluate) spline point: Cubic Bezier
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

    /// Check collision between two rectangles
    pub fn check_collision_recs(&self, rec1: Rectangle, rec2: Rectangle) -> bool {
        Self::__check_collision_recs(rec1, rec2)
    }

    /// Check collision between two circles
    pub fn check_collision_circles(
        &self,
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool {
        Self::__check_collision_circles(center1, radius1, center2, radius2)
    }

    /// Check collision between circle and rectangle
    pub fn check_collision_circle_rec(&self, center: Vector2, radius: f32, rec: Rectangle) -> bool {
        Self::__check_collision_circle_rec(center, radius, rec)
    }

    /// Check if point is inside rectangle
    pub fn check_collision_point_rec(&self, point: Vector2, rec: Rectangle) -> bool {
        Self::__check_collision_point_rec(point, rec)
    }

    /// Check if point is inside circle
    pub fn check_collision_point_circle(
        &self,
        point: Vector2,
        center: Vector2,
        radius: f32,
    ) -> bool {
        Self::__check_collision_point_circle(point, center, radius)
    }

    /// Check if point is inside a triangle
    pub fn check_collision_point_triangle(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool {
        Self::__check_collision_point_triangle(point, p1, p2, p3)
    }

    /// Check if point is within a polygon described by array of vertices
    pub fn check_collision_point_poly(&self, point: Vector2, points: &mut Vec<Vector2>) -> bool {
        Self::__check_collision_point_poly(point, points)
    }

    /// Check the collision between two lines defined by two points each, returns collision point by reference
    pub fn check_collision_lines(
        &self,
        start1: Vector2,
        end1: Vector2,
        start2: Vector2,
        end2: Vector2,
    ) -> Option<Vector2> {
        Self::__check_collision_lines(start1, end1, start2, end2)
    }

    /// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
    pub fn check_collision_point_line(
        &self,
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: i32,
    ) -> bool {
        Self::__check_collision_point_line(point, p1, p2, threshold)
    }

    /// Get collision rectangle for two rectangles collision
    pub fn get_collision_rec(&self, rec1: Rectangle, rec2: Rectangle) -> Rectangle {
        Self::__get_collision_rec(rec1, rec2)
    }
}
