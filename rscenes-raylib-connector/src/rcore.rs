use crate::window_handle::WindowHandle;
use eyre::*;
use raylib_ffi::*;
use std::ffi::c_void;
use std::ffi::{c_char, CString};

#[derive(Clone, Copy, Debug)]
pub struct Rcore;

impl Rcore {
    // Window-related methods

    pub fn init_window(&self, width: i32, height: i32, title: &str) {
        unsafe { InitWindow(width, height, rl_str!(title)) }
    }

    pub fn close_window(&self) {
        unsafe { CloseWindow() }
    }

    pub fn window_should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn is_window_ready(&self) -> bool {
        unsafe { IsWindowReady() }
    }

    pub fn is_window_fullscreen(&self) -> bool {
        unsafe { IsWindowFullscreen() }
    }

    pub fn is_window_hidden(&self) -> bool {
        unsafe { IsWindowHidden() }
    }

    pub fn is_window_minimized(&self) -> bool {
        unsafe { IsWindowMinimized() }
    }

    pub fn is_window_maximized(&self) -> bool {
        unsafe { IsWindowMaximized() }
    }

    pub fn is_window_focused(&self) -> bool {
        unsafe { IsWindowFocused() }
    }

    pub fn is_window_resized(&self) -> bool {
        unsafe { IsWindowResized() }
    }

    pub fn is_window_state(&self, flag: enums::ConfigFlags) -> bool {
        let flag: usize = flag.into();
        unsafe { IsWindowState(flag as u32) }
    }

    pub fn set_window_state(&self, flag: enums::ConfigFlags) {
        let flag: usize = flag.into();
        unsafe { SetWindowState(flag as u32) }
    }

    pub fn clear_window_state(&self, flag: enums::ConfigFlags) {
        let flag: usize = flag.into();
        unsafe { ClearWindowState(flag as u32) }
    }

    pub fn toggle_fullscreen(&self) {
        unsafe { ToggleFullscreen() }
    }

    pub fn toggle_borderless_windowed(&self) {
        unsafe { ToggleBorderlessWindowed() }
    }

    pub fn maximize_window(&self) {
        unsafe { MaximizeWindow() }
    }

    pub fn minimize_window(&self) {
        unsafe { MinimizeWindow() }
    }

    pub fn restore_window(&self) {
        unsafe { RestoreWindow() }
    }

    pub fn set_window_icon(&self, image: Image) {
        unsafe { SetWindowIcon(image) }
    }

    pub fn set_window_icons(&self, images: &mut Vec<Image>) {
        unsafe {
            let count = images.len() as i32;
            SetWindowIcons(images.as_ptr() as *mut Image, count)
        }
    }

    pub fn set_window_title(&self, title: &str) {
        unsafe { SetWindowTitle(rl_str!(title)) }
    }

    pub fn set_window_position(&self, x: i32, y: i32) {
        unsafe { SetWindowPosition(x, y) }
    }

    pub fn set_window_monitor(&self, monitor: i32) {
        unsafe { SetWindowMonitor(monitor) }
    }

    pub fn set_window_min_size(&self, width: i32, height: i32) {
        unsafe { SetWindowMinSize(width, height) }
    }

    pub fn set_window_max_size(&self, width: i32, height: i32) {
        unsafe { SetWindowMaxSize(width, height) }
    }

    pub fn set_window_size(&self, width: i32, height: i32) {
        unsafe { SetWindowSize(width, height) }
    }

    pub fn set_window_opacity(&self, opacity: f32) {
        unsafe { SetWindowOpacity(opacity) }
    }

    pub fn set_window_focused(&self) {
        unsafe { SetWindowFocused() }
    }

    pub fn get_window_handle(&self) -> WindowHandle<'_> {
        unsafe { GetWindowHandle().into() }
    }

    pub fn get_screen_width(&self) -> i32 {
        unsafe { GetScreenWidth() }
    }

    pub fn get_screen_height(&self) -> i32 {
        unsafe { GetScreenHeight() }
    }

    pub fn get_render_width(&self) -> i32 {
        unsafe { GetRenderWidth() }
    }

    pub fn get_render_height(&self) -> i32 {
        unsafe { GetRenderHeight() }
    }

    pub fn get_monitor_count(&self) -> i32 {
        unsafe { GetMonitorCount() }
    }

    pub fn get_current_monitor(&self) -> i32 {
        unsafe { GetCurrentMonitor() }
    }

    pub fn get_monitor_position(&self, monitor: i32) -> Vector2 {
        unsafe { GetMonitorPosition(monitor) }
    }

    pub fn get_monitor_width(&self, monitor: i32) -> i32 {
        unsafe { GetMonitorWidth(monitor) }
    }

    pub fn get_monitor_height(&self, monitor: i32) -> i32 {
        unsafe { GetMonitorHeight(monitor) }
    }

    pub fn get_monitor_physical_width(&self, monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalWidth(monitor) }
    }

    pub fn get_monitor_physical_height(&self, monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalHeight(monitor) }
    }

    pub fn get_monitor_refresh_rate(&self, monitor: i32) -> i32 {
        unsafe { GetMonitorRefreshRate(monitor) }
    }

    pub fn get_window_position(&self) -> Vector2 {
        unsafe { GetWindowPosition() }
    }

    pub fn get_window_scale_dpi(&self) -> Vector2 {
        unsafe { GetWindowScaleDPI() }
    }

    pub fn get_monitor_name(&self, monitor: i32) -> Result<String> {
        unsafe {
            let res = GetMonitorName(monitor) as *mut c_char;
            let res = CString::from_raw(res);
            Ok(res.into_string()?)
        }
    }

    pub fn set_clipboard_text(&self, text: &str) {
        unsafe { SetClipboardText(rl_str!(text)) }
    }

    pub fn enable_event_waiting(&self) {
        unsafe { EnableEventWaiting() }
    }

    pub fn disable_event_waiting(&self) {
        unsafe { DisableEventWaiting() }
    }

    // Cursor-related methods

    pub fn show_cursor(&self) {
        unsafe { ShowCursor() }
    }

    pub fn hide_cursor(&self) {
        unsafe { HideCursor() }
    }

    pub fn is_cursor_hiden(&self) -> bool {
        unsafe { IsCursorHidden() }
    }

    pub fn enable_cursor(&self) {
        unsafe { EnableCursor() }
    }

    pub fn disable_cursor(&self) {
        unsafe { DisableCursor() }
    }

    pub fn is_cursor_on_screen(&self) -> bool {
        unsafe { IsCursorOnScreen() }
    }

    // Drawing-related methods

    pub fn clear_background(&self, color: Color) {
        unsafe { ClearBackground(color) }
    }

    pub fn begin_drawing(&self) {
        unsafe { BeginDrawing() }
    }

    pub fn end_drawing(&self) {
        unsafe { EndDrawing() }
    }

    #[allow(non_snake_case)]
    pub fn begin_mode_2D(&self, camera: Camera2D) {
        unsafe { BeginMode2D(camera) }
    }

    #[allow(non_snake_case)]
    pub fn end_mode_2D(&self) {
        unsafe { EndMode2D() }
    }

    #[allow(non_snake_case)]
    pub fn begin_mode_3D(&self, camera: Camera3D) {
        unsafe { BeginMode3D(camera) }
    }

    #[allow(non_snake_case)]
    pub fn end_mode_3D(&self) {
        unsafe { EndMode3D() }
    }

    pub fn begin_texture_mode(&self, target: RenderTexture2D) {
        unsafe { BeginTextureMode(target) }
    }

    pub fn end_texture_mode(&self) {
        unsafe { EndTextureMode() }
    }

    pub fn begin_shader_mode(&self, shader: Shader) {
        unsafe { BeginShaderMode(shader) }
    }

    pub fn end_shader_mode(&self) {
        unsafe { EndShaderMode() }
    }

    pub fn begin_blend_mode(&self, mode: i32) {
        unsafe { BeginBlendMode(mode) }
    }

    pub fn end_blend_mode(&self) {
        unsafe { EndBlendMode() }
    }

    pub fn begin_scissor_mode(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { BeginScissorMode(x, y, width, height) }
    }

    pub fn end_scissor_mode(&self) {
        unsafe { EndScissorMode() }
    }

    pub fn begin_vr_stereo_mode(&self, config: VrStereoConfig) {
        unsafe { BeginVrStereoMode(config) }
    }

    pub fn end_vr_stereo_mode(&self) {
        unsafe { EndVrStereoMode() }
    }

    // VR stereo config methods for VR simulator

    pub fn load_vr_stereo_config(&self, device: VrDeviceInfo) -> VrStereoConfig {
        unsafe { LoadVrStereoConfig(device) }
    }

    pub fn unload_vr_stereo_config(&self, config: VrStereoConfig) {
        unsafe { UnloadVrStereoConfig(config) }
    }

    // Shader management functions

    pub fn load_shader(&self, vs_filename: &str, fs_filename: &str) -> Shader {
        unsafe { LoadShader(rl_str!(vs_filename), rl_str!(fs_filename)) }
    }

    pub fn load_shader_from_memory(&self, vs_code: &str, fs_code: &str) -> Shader {
        unsafe { LoadShaderFromMemory(rl_str!(vs_code), rl_str!(fs_code)) }
    }

    pub fn is_shader_ready(&self, shader: Shader) -> bool {
        unsafe { IsShaderReady(shader) }
    }

    pub fn get_shader_location(&self, shader: Shader, name: &str) -> i32 {
        unsafe { GetShaderLocation(shader, rl_str!(name)) }
    }

    pub fn get_shader_location_attrib(
        &self,
        shader: Shader,
        name: &str,
    ) -> Result<enums::ShaderLocationIndex, String> {
        unsafe {
            match GetShaderLocationAttrib(shader, rl_str!(name)) {
                0 => Ok(enums::ShaderLocationIndex::VertexPosition),
                1 => Ok(enums::ShaderLocationIndex::VertexTexcoord01),
                2 => Ok(enums::ShaderLocationIndex::VertexTexcoord02),
                3 => Ok(enums::ShaderLocationIndex::VertexNormal),
                4 => Ok(enums::ShaderLocationIndex::VertexTangent),
                5 => Ok(enums::ShaderLocationIndex::VertexColor),
                6 => Ok(enums::ShaderLocationIndex::MatrixMvp),
                7 => Ok(enums::ShaderLocationIndex::MatrixView),
                8 => Ok(enums::ShaderLocationIndex::MatrixProjection),
                9 => Ok(enums::ShaderLocationIndex::MatrixModel),
                10 => Ok(enums::ShaderLocationIndex::MatrixNormal),
                11 => Ok(enums::ShaderLocationIndex::VectorView),
                12 => Ok(enums::ShaderLocationIndex::ColorDiffuse),
                13 => Ok(enums::ShaderLocationIndex::ColorSpecular),
                14 => Ok(enums::ShaderLocationIndex::ColorAmbient),
                15 => Ok(enums::ShaderLocationIndex::MapAlbedo),
                16 => Ok(enums::ShaderLocationIndex::MapMetalness),
                17 => Ok(enums::ShaderLocationIndex::MapNormal),
                18 => Ok(enums::ShaderLocationIndex::MapRoughness),
                19 => Ok(enums::ShaderLocationIndex::MapOcclusion),
                20 => Ok(enums::ShaderLocationIndex::MapEmission),
                21 => Ok(enums::ShaderLocationIndex::MapHeight),
                22 => Ok(enums::ShaderLocationIndex::MapCubemap),
                23 => Ok(enums::ShaderLocationIndex::MapIrradiance),
                24 => Ok(enums::ShaderLocationIndex::MapPrefilter),
                25 => Ok(enums::ShaderLocationIndex::MapBrdf),
                num => Err(format!("could not translate location {}", num)),
            }
        }
    }

    pub fn set_shader_value<T>(
        &self,
        shader: Shader,
        index: i32,
        value: &T,
        tpe: enums::ShaderUniformDataType,
    ) {
        unsafe {
            let tpe: usize = tpe.into();
            let value = value as *const T as *const c_void;
            SetShaderValue(shader, index, value, tpe as i32)
        }
    }

    pub fn set_shader_value_v<T>(
        &self,
        shader: Shader,
        index: i32,
        value: Vec<&T>,
        tpe: enums::ShaderUniformDataType,
    ) {
        unsafe {
            let tpe: usize = tpe.into();
            let count = value.len() as i32;
            let value = value.as_ptr() as *const c_void;
            SetShaderValueV(shader, index, value, tpe as i32, count)
        }
    }

    pub fn set_shader_value_matrix(&self, shader: Shader, loc: i32, mat: Matrix) {
        unsafe { SetShaderValueMatrix(shader, loc, mat) }
    }

    pub fn set_shader_value_texture(&self, shader: Shader, index: i32, texture: Texture2D) {
        unsafe { SetShaderValueTexture(shader, index, texture) }
    }

    pub fn unload_shader(&self, shader: Shader) {
        unsafe { UnloadShader(shader) }
    }

    // Screen-space-related methods

    pub fn get_mouse_ray(&self, mouse_position: Vector2, camera: Camera3D) -> Ray {
        unsafe { GetMouseRay(mouse_position, camera) }
    }

    #[allow(non_snake_case)]
    pub fn get_camera_matrix_2D(&self, camera: Camera2D) -> Matrix {
        unsafe { GetCameraMatrix2D(camera) }
    }
    #[allow(non_snake_case)]
    pub fn get_camera_matrix_3D(&self, camera: Camera3D) -> Matrix {
        unsafe { GetCameraMatrix(camera) }
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_2D(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetWorldToScreen2D(position, camera) }
    }

    #[allow(non_snake_case)]
    pub fn get_screen_to_world_2D(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetScreenToWorld2D(position, camera) }
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_3D(&self, position: Vector3, camera: Camera3D) -> Vector2 {
        unsafe { GetWorldToScreen(position, camera) }
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_3D_ex(
        &self,
        position: Vector3,
        camera: Camera3D,
        width: i32,
        height: i32,
    ) -> Vector2 {
        unsafe { GetWorldToScreenEx(position, camera, width, height) }
    }

    // Timing-related functions

    pub fn set_target_fps(&self, fps: i32) {
        unsafe { SetTargetFPS(fps) }
    }

    pub fn get_frame_time(&self) -> f32 {
        unsafe { GetFrameTime() }
    }

    pub fn get_time(&self) -> f64 {
        unsafe { GetTime() }
    }

    pub fn get_fps(&self) -> i32 {
        unsafe { GetFPS() }
    }

    // Custom frame control functions

    pub fn swap_screen_buffer(&self) {
        unsafe { SwapScreenBuffer() }
    }

    pub fn poll_input_events(&self) {
        unsafe { PollInputEvents() }
    }

    pub fn wait_time(&self, seconds: f64) {
        unsafe { WaitTime(seconds) }
    }

    // Random values generation functions

    pub fn set_random_seed(&self, seed: u32) {
        unsafe { SetRandomSeed(seed) }
    }

    pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
        unsafe { GetRandomValue(min, max) }
    }

    // TODO: LoadRandomSequence
    // TODO: UnloadRandomSequence
}
