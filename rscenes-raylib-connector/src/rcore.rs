use crate::window_handle::WindowHandle;
use eyre::*;
use raylib_ffi::enums::{KeyboardKey, TraceLogLevel};
use raylib_ffi::*;
use std::char;
use std::{
    ffi::{c_char, c_uchar, c_void, CString},
    ptr,
};

#[derive(Clone, Copy, Debug)]
pub struct Rcore;

/// Crate only methods
impl Rcore {
    // Window-related methods

    pub(crate) fn __init_window(width: i32, height: i32, title: &str) {
        unsafe { InitWindow(width, height, rl_str!(title)) }
    }

    pub(crate) fn __close_window() {
        unsafe { CloseWindow() }
    }

    pub(crate) fn __window_should_close() -> bool {
        unsafe { WindowShouldClose() }
    }

    pub(crate) fn __is_window_ready() -> bool {
        unsafe { IsWindowReady() }
    }

    pub(crate) fn __is_window_fullscreen() -> bool {
        unsafe { IsWindowFullscreen() }
    }

    pub(crate) fn __is_window_hidden() -> bool {
        unsafe { IsWindowHidden() }
    }

    pub(crate) fn __is_window_minimized() -> bool {
        unsafe { IsWindowMinimized() }
    }

    pub(crate) fn __is_window_maximized() -> bool {
        unsafe { IsWindowMaximized() }
    }

    pub(crate) fn __is_window_focused() -> bool {
        unsafe { IsWindowFocused() }
    }

    pub(crate) fn __is_window_resized() -> bool {
        unsafe { IsWindowResized() }
    }

    pub(crate) fn __is_window_state(flag: usize) -> bool {
        unsafe { IsWindowState(flag as u32) }
    }

    pub(crate) fn __set_window_state(flag: usize) {
        let flag: usize = flag.into();
        unsafe { SetWindowState(flag as u32) }
    }

    pub(crate) fn __clear_window_state(flag: usize) {
        let flag: usize = flag.into();
        unsafe { ClearWindowState(flag as u32) }
    }

    pub(crate) fn __toggle_fullscreen() {
        unsafe { ToggleFullscreen() }
    }

    pub(crate) fn __toggle_borderless_windowed() {
        unsafe { ToggleBorderlessWindowed() }
    }

    pub(crate) fn __maximize_window() {
        unsafe { MaximizeWindow() }
    }

    pub(crate) fn __minimize_window() {
        unsafe { MinimizeWindow() }
    }

    pub(crate) fn __restore_window() {
        unsafe { RestoreWindow() }
    }

    pub(crate) fn __set_window_icon(image: Image) {
        unsafe { SetWindowIcon(image) }
    }

    pub(crate) fn __set_window_icons(images: &mut Vec<Image>) {
        unsafe {
            let count = images.len() as i32;
            SetWindowIcons(images.as_ptr() as *mut Image, count)
        }
    }

    pub(crate) fn __set_window_title(title: &str) {
        unsafe { SetWindowTitle(rl_str!(title)) }
    }

    pub(crate) fn __set_window_position(x: i32, y: i32) {
        unsafe { SetWindowPosition(x, y) }
    }

    pub(crate) fn __set_window_monitor(monitor: i32) {
        unsafe { SetWindowMonitor(monitor) }
    }

    pub(crate) fn __set_window_min_size(width: i32, height: i32) {
        unsafe { SetWindowMinSize(width, height) }
    }

    pub(crate) fn __set_window_max_size(width: i32, height: i32) {
        unsafe { SetWindowMaxSize(width, height) }
    }

    pub(crate) fn __set_window_size(width: i32, height: i32) {
        unsafe { SetWindowSize(width, height) }
    }

    pub(crate) fn __set_window_opacity(opacity: f32) {
        unsafe { SetWindowOpacity(opacity) }
    }

    pub(crate) fn __set_window_focused() {
        unsafe { SetWindowFocused() }
    }

    pub(crate) fn __get_window_handle<'a>() -> WindowHandle<'a> {
        unsafe { GetWindowHandle().into() }
    }

    pub(crate) fn __get_screen_width() -> i32 {
        unsafe { GetScreenWidth() }
    }

    pub(crate) fn __get_screen_height() -> i32 {
        unsafe { GetScreenHeight() }
    }

    pub(crate) fn __get_render_width() -> i32 {
        unsafe { GetRenderWidth() }
    }

    pub(crate) fn __get_render_height() -> i32 {
        unsafe { GetRenderHeight() }
    }

    pub(crate) fn __get_monitor_count() -> i32 {
        unsafe { GetMonitorCount() }
    }

    pub(crate) fn __get_current_monitor() -> i32 {
        unsafe { GetCurrentMonitor() }
    }

    pub(crate) fn __get_monitor_position(monitor: i32) -> Vector2 {
        unsafe { GetMonitorPosition(monitor) }
    }

    pub(crate) fn __get_monitor_width(monitor: i32) -> i32 {
        unsafe { GetMonitorWidth(monitor) }
    }

    pub(crate) fn __get_monitor_height(monitor: i32) -> i32 {
        unsafe { GetMonitorHeight(monitor) }
    }

    pub(crate) fn __get_monitor_physical_width(monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalWidth(monitor) }
    }

    pub(crate) fn __get_monitor_physical_height(monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalHeight(monitor) }
    }

    pub(crate) fn __get_monitor_refresh_rate(monitor: i32) -> i32 {
        unsafe { GetMonitorRefreshRate(monitor) }
    }

    pub(crate) fn __get_window_position() -> Vector2 {
        unsafe { GetWindowPosition() }
    }

    pub(crate) fn __get_window_scale_dpi() -> Vector2 {
        unsafe { GetWindowScaleDPI() }
    }

    pub(crate) fn __get_monitor_name(monitor: i32) -> Result<String> {
        unsafe {
            let res = GetMonitorName(monitor) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __set_clipboard_text(text: &str) {
        unsafe { SetClipboardText(rl_str!(text)) }
    }

    pub(crate) fn __enable_event_waiting() {
        unsafe { EnableEventWaiting() }
    }

    pub(crate) fn __disable_event_waiting() {
        unsafe { DisableEventWaiting() }
    }

    // Cursor-related methods

    pub(crate) fn __show_cursor() {
        unsafe { ShowCursor() }
    }

    pub(crate) fn __hide_cursor() {
        unsafe { HideCursor() }
    }

    pub(crate) fn __is_cursor_hiden() -> bool {
        unsafe { IsCursorHidden() }
    }

    pub(crate) fn __enable_cursor() {
        unsafe { EnableCursor() }
    }

    pub(crate) fn __disable_cursor() {
        unsafe { DisableCursor() }
    }

    pub(crate) fn __is_cursor_on_screen() -> bool {
        unsafe { IsCursorOnScreen() }
    }

    // Drawing-related methods

    pub(crate) fn __clear_background(color: Color) {
        unsafe { ClearBackground(color) }
    }

    pub(crate) fn __begin_drawing() {
        unsafe { BeginDrawing() }
    }

    pub(crate) fn __end_drawing() {
        unsafe { EndDrawing() }
    }

    #[allow(non_snake_case)]
    pub(crate) fn __begin_mode_2D(camera: Camera2D) {
        unsafe { BeginMode2D(camera) }
    }

    #[allow(non_snake_case)]
    pub(crate) fn __end_mode_2D() {
        unsafe { EndMode2D() }
    }

    #[allow(non_snake_case)]
    pub(crate) fn __begin_mode_3D(camera: Camera3D) {
        unsafe { BeginMode3D(camera) }
    }

    #[allow(non_snake_case)]
    pub(crate) fn __end_mode_3D() {
        unsafe { EndMode3D() }
    }

    pub(crate) fn __begin_texture_mode(target: RenderTexture2D) {
        unsafe { BeginTextureMode(target) }
    }

    pub(crate) fn __end_texture_mode() {
        unsafe { EndTextureMode() }
    }

    pub(crate) fn __begin_shader_mode(shader: Shader) {
        unsafe { BeginShaderMode(shader) }
    }

    pub(crate) fn __end_shader_mode() {
        unsafe { EndShaderMode() }
    }

    pub(crate) fn __begin_blend_mode(mode: i32) {
        unsafe { BeginBlendMode(mode) }
    }

    pub(crate) fn __end_blend_mode() {
        unsafe { EndBlendMode() }
    }

    pub(crate) fn __begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
        unsafe { BeginScissorMode(x, y, width, height) }
    }

    pub(crate) fn __end_scissor_mode() {
        unsafe { EndScissorMode() }
    }

    pub(crate) fn __begin_vr_stereo_mode(config: VrStereoConfig) {
        unsafe { BeginVrStereoMode(config) }
    }

    pub(crate) fn __end_vr_stereo_mode() {
        unsafe { EndVrStereoMode() }
    }

    // VR stereo config methods for VR simulator

    pub(crate) fn __load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
        unsafe { LoadVrStereoConfig(device) }
    }

    pub(crate) fn __unload_vr_stereo_config(config: VrStereoConfig) {
        unsafe { UnloadVrStereoConfig(config) }
    }

    // Shader management functions

    pub(crate) fn __load_shader(vs_filename: &str, fs_filename: &str) -> Shader {
        unsafe { LoadShader(rl_str!(vs_filename), rl_str!(fs_filename)) }
    }

    pub(crate) fn __load_shader_from_memory(vs_code: &str, fs_code: &str) -> Shader {
        unsafe { LoadShaderFromMemory(rl_str!(vs_code), rl_str!(fs_code)) }
    }

    pub(crate) fn __is_shader_ready(shader: Shader) -> bool {
        unsafe { IsShaderReady(shader) }
    }

    pub(crate) fn __get_shader_location(shader: Shader, name: &str) -> i32 {
        unsafe { GetShaderLocation(shader, rl_str!(name)) }
    }

    pub(crate) fn __get_shader_location_attrib(
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

    pub(crate) fn __set_shader_value<T>(
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

    pub(crate) fn __set_shader_value_v<T>(
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

    pub(crate) fn __set_shader_value_matrix(shader: Shader, loc: i32, mat: Matrix) {
        unsafe { SetShaderValueMatrix(shader, loc, mat) }
    }

    pub(crate) fn __set_shader_value_texture(shader: Shader, index: i32, texture: Texture2D) {
        unsafe { SetShaderValueTexture(shader, index, texture) }
    }

    pub(crate) fn __unload_shader(shader: Shader) {
        unsafe { UnloadShader(shader) }
    }

    // Screen-space-related methods

    pub(crate) fn __get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Ray {
        unsafe { GetMouseRay(mouse_position, camera) }
    }

    pub(crate) fn __get_camera_matrix_2d(camera: Camera2D) -> Matrix {
        unsafe { GetCameraMatrix2D(camera) }
    }

    pub(crate) fn __get_camera_matrix_3d(camera: Camera3D) -> Matrix {
        unsafe { GetCameraMatrix(camera) }
    }

    pub(crate) fn __get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetWorldToScreen2D(position, camera) }
    }

    pub(crate) fn __get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetScreenToWorld2D(position, camera) }
    }

    pub(crate) fn __get_world_to_screen_3d(position: Vector3, camera: Camera3D) -> Vector2 {
        unsafe { GetWorldToScreen(position, camera) }
    }

    pub(crate) fn __get_world_to_screen_3d_ex(
        position: Vector3,
        camera: Camera3D,
        width: i32,
        height: i32,
    ) -> Vector2 {
        unsafe { GetWorldToScreenEx(position, camera, width, height) }
    }

    // Timing-related functions

    pub(crate) fn __set_target_fps(fps: i32) {
        unsafe { SetTargetFPS(fps) }
    }

    pub(crate) fn __get_frame_time() -> f32 {
        unsafe { GetFrameTime() }
    }

    pub(crate) fn __get_time() -> f64 {
        unsafe { GetTime() }
    }

    pub(crate) fn __get_fps() -> i32 {
        unsafe { GetFPS() }
    }

    // Custom frame control functions

    pub(crate) fn __swap_screen_buffer() {
        unsafe { SwapScreenBuffer() }
    }

    pub(crate) fn __poll_input_events() {
        unsafe { PollInputEvents() }
    }

    pub(crate) fn __wait_time(seconds: f64) {
        unsafe { WaitTime(seconds) }
    }

    // Random values generation functions

    pub(crate) fn __set_random_seed(seed: u32) {
        unsafe { SetRandomSeed(seed) }
    }

    pub(crate) fn __get_random_value(min: i32, max: i32) -> i32 {
        unsafe { GetRandomValue(min, max) }
    }

    // TODO: LoadRandomSequence
    // TODO: UnloadRandomSequence

    // Misc functions

    pub(crate) fn __take_screenshot(filename: &str) {
        unsafe { TakeScreenshot(rl_str!(filename)) }
    }

    pub(crate) fn __set_config_flags(flags: usize) {
        unsafe { SetConfigFlags(flags as u32) }
    }

    pub(crate) fn __open_url(url: &str) {
        unsafe { OpenURL(rl_str!(url)) }
    }

    pub(crate) fn __trace_log(level: TraceLogLevel, text: &str) {
        unsafe {
            let level: usize = level.into();
            TraceLog(level as i32, rl_str!(text))
        }
    }

    pub(crate) fn __set_trace_log_level(level: TraceLogLevel) {
        unsafe {
            let level: usize = level.into();
            SetTraceLogLevel(level as i32)
        }
    }

    pub(crate) fn __mem_alloc(size: usize) -> *mut c_void {
        unsafe { MemAlloc(size as u32) }
    }

    pub(crate) fn __mem_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
        unsafe { MemRealloc(ptr, size as u32) }
    }

    pub(crate) fn __mem_free(ptr: *mut c_void) {
        unsafe { MemFree(ptr) }
    }

    // Files management functions

    pub(crate) fn __load_file_data(filename: &str) -> Vec<u8> {
        unsafe {
            let mut size = 0;
            let data = LoadFileData(rl_str!(filename), &mut size);
            let array = ptr::slice_from_raw_parts_mut(data, size as usize);
            (*array).to_owned()
        }
    }

    // TODO: UnloadFileData

    pub(crate) fn __save_file_data(filename: &str, data: Vec<u8>) -> bool {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_void;
            SaveFileData(rl_str!(filename), data, size)
        }
    }

    pub(crate) fn __export_data_as_code(data: &str, filename: &str) -> bool {
        unsafe {
            let size = data.len() as i32;
            ExportDataAsCode(rl_str!(data) as *const c_uchar, size, rl_str!(filename))
        }
    }

    pub(crate) fn __load_file_text(filename: &str) -> Result<String> {
        unsafe {
            let res = LoadFileText(rl_str!(filename)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    // TODO: UnloadFileText

    pub(crate) fn __save_file_text(filename: &str, text: &str) -> bool {
        unsafe { SaveFileText(rl_str!(filename), rl_str!(text) as *mut c_char) }
    }

    // File system functions

    pub(crate) fn __file_exists(filename: &str) -> bool {
        unsafe { FileExists(rl_str!(filename)) }
    }

    pub(crate) fn __directory_exists(dirname: &str) -> bool {
        unsafe { DirectoryExists(rl_str!(dirname)) }
    }

    pub(crate) fn __is_file_extension(filename: &str, ext: &str) -> bool {
        unsafe { IsFileExtension(rl_str!(filename), rl_str!(ext)) }
    }

    pub(crate) fn __get_file_length(filename: &str) -> i32 {
        unsafe { GetFileLength(rl_str!(filename)) }
    }

    pub(crate) fn __get_file_extenstion(filename: &str) -> Result<String> {
        unsafe {
            let res = GetFileExtension(rl_str!(filename)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_file_name(path: &str) -> Result<String> {
        unsafe {
            let res = GetFileName(rl_str!(path)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_file_name_without_ext(path: &str) -> Result<String> {
        unsafe {
            let res = GetFileNameWithoutExt(rl_str!(path)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_directory_path(path: &str) -> Result<String> {
        unsafe {
            let res = GetDirectoryPath(rl_str!(path)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_prev_directory_path(path: &str) -> Result<String> {
        unsafe {
            let res = GetPrevDirectoryPath(rl_str!(path)) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_working_directory() -> Result<String> {
        unsafe {
            let res = GetWorkingDirectory() as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __get_application_directory() -> Result<String> {
        unsafe {
            let res = GetApplicationDirectory() as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __change_directory(dir: &str) -> bool {
        unsafe { ChangeDirectory(rl_str!(dir)) }
    }

    pub(crate) fn __is_path_file(path: &str) -> bool {
        unsafe { IsPathFile(rl_str!(path)) }
    }

    pub(crate) fn __load_directory_files(path: &str) -> FilePathList {
        unsafe { LoadDirectoryFiles(rl_str!(path)) }
    }

    pub(crate) fn __load_directory_files_ex(
        path: &str,
        filter: &str,
        scan_subdirs: bool,
    ) -> FilePathList {
        unsafe { LoadDirectoryFilesEx(rl_str!(path), rl_str!(filter), scan_subdirs) }
    }

    pub(crate) fn __unload_directory_files(files: FilePathList) {
        unsafe { UnloadDirectoryFiles(files) }
    }

    pub(crate) fn __is_file_dropped() -> bool {
        unsafe { IsFileDropped() }
    }

    pub(crate) fn __load_dropped_files() -> FilePathList {
        unsafe { LoadDroppedFiles() }
    }

    pub(crate) fn __unload_dropped_files(files: FilePathList) {
        unsafe { UnloadDroppedFiles(files) }
    }

    pub(crate) fn __get_file_mod_time(filename: &str) -> i64 {
        unsafe { GetFileModTime(rl_str!(filename)) }
    }

    // Compression/Encoding functionality

    pub(crate) fn __compress_data(data: Vec<u8>) -> Vec<u8> {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            let mut comp_size = 0;
            let res = CompressData(data, size, &mut comp_size);
            let array = ptr::slice_from_raw_parts_mut(res, comp_size as usize);
            (*array).to_owned()
        }
    }

    pub(crate) fn __decompress_data(data: Vec<u8>) -> Vec<u8> {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            let mut decomp_size = 0;
            let res = DecompressData(data, size, &mut decomp_size);
            let array = ptr::slice_from_raw_parts_mut(res, decomp_size as usize);
            (*array).to_owned()
        }
    }

    pub(crate) fn __encode_data_base64(data: Vec<u8>) -> Result<String> {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_ptr() as *mut c_uchar;
            let mut output_size = 0;
            let res = EncodeDataBase64(data, size, &mut output_size) as *mut c_char;
            Ok(CString::from_raw(res).into_string()?)
        }
    }

    pub(crate) fn __decode_data_base64(data: &str) -> Vec<u8> {
        unsafe {
            let mut size = 0;
            let res = DecodeDataBase64(rl_str!(data) as *const c_uchar, &mut size);
            let array = ptr::slice_from_raw_parts_mut(res, size as usize);
            (*array).to_owned()
        }
    }

    // Automation events functionality

    pub(crate) fn __load_automation_event_list(filename: &str) -> AutomationEventList {
        unsafe { LoadAutomationEventList(rl_str!(filename)) }
    }

    pub(crate) fn __unload_automation_event_list(mut list: AutomationEventList) {
        unsafe { UnloadAutomationEventList(&mut list) }
    }

    pub(crate) fn __export_automation_event_list(
        list: AutomationEventList,
        filename: &str,
    ) -> bool {
        unsafe { ExportAutomationEventList(list, rl_str!(filename)) }
    }

    pub(crate) fn __set_automation_event_list(mut list: AutomationEventList) {
        unsafe { SetAutomationEventList(&mut list) }
    }

    pub(crate) fn __set_automation_event_base_frame(frame: i32) {
        unsafe { SetAutomationEventBaseFrame(frame) }
    }

    pub(crate) fn __start_automation_event_record() {
        unsafe { StartAutomationEventRecording() }
    }

    pub(crate) fn __stop_automation_event_record() {
        unsafe { StopAutomationEventRecording() }
    }

    pub(crate) fn __play_automation_event(event: AutomationEvent) {
        unsafe { PlayAutomationEvent(event) }
    }

    // Input-related functions: keyboard

    pub(crate) fn __is_key_pressed(key: impl Into<usize>) -> bool {
        unsafe { IsKeyPressed(key.into() as i32) }
    }

    pub(crate) fn __is_key_pressed_repeat(key: impl Into<usize>) -> bool {
        unsafe { IsKeyPressedRepeat(key.into() as i32) }
    }

    pub(crate) fn __is_key_down(key: impl Into<usize>) -> bool {
        unsafe { IsKeyDown(key.into() as i32) }
    }

    pub(crate) fn __is_key_released(key: impl Into<usize>) -> bool {
        unsafe { IsKeyReleased(key.into() as i32) }
    }

    pub(crate) fn __is_key_up(key: impl Into<usize>) -> bool {
        unsafe { IsKeyUp(key.into() as i32) }
    }

    pub(crate) fn __get_key_pressed() -> KeyboardKey {
        unsafe {
            match GetKeyPressed() {
                0 => KeyboardKey::Null,
                39 => KeyboardKey::Apostrophe,
                44 => KeyboardKey::Comma,
                45 => KeyboardKey::Minus,
                46 => KeyboardKey::Period,
                47 => KeyboardKey::Slash,
                48 => KeyboardKey::Zero,
                49 => KeyboardKey::One,
                50 => KeyboardKey::Two,
                51 => KeyboardKey::Three,
                52 => KeyboardKey::Four,
                53 => KeyboardKey::Five,
                54 => KeyboardKey::Six,
                55 => KeyboardKey::Seven,
                56 => KeyboardKey::Eight,
                57 => KeyboardKey::Nine,
                59 => KeyboardKey::Semicolon,
                61 => KeyboardKey::Equal,
                65 => KeyboardKey::A,
                66 => KeyboardKey::B,
                67 => KeyboardKey::C,
                68 => KeyboardKey::D,
                69 => KeyboardKey::E,
                70 => KeyboardKey::F,
                71 => KeyboardKey::G,
                72 => KeyboardKey::H,
                73 => KeyboardKey::I,
                74 => KeyboardKey::J,
                75 => KeyboardKey::K,
                76 => KeyboardKey::L,
                77 => KeyboardKey::M,
                78 => KeyboardKey::N,
                79 => KeyboardKey::O,
                80 => KeyboardKey::P,
                81 => KeyboardKey::Q,
                82 => KeyboardKey::R,
                83 => KeyboardKey::S,
                84 => KeyboardKey::T,
                85 => KeyboardKey::U,
                86 => KeyboardKey::V,
                87 => KeyboardKey::W,
                88 => KeyboardKey::X,
                89 => KeyboardKey::Y,
                90 => KeyboardKey::Z,
                91 => KeyboardKey::LeftBracket,
                92 => KeyboardKey::Backslash,
                93 => KeyboardKey::RightBracket,
                96 => KeyboardKey::Grave,
                32 => KeyboardKey::Space,
                256 => KeyboardKey::Escape,
                257 => KeyboardKey::Enter,
                258 => KeyboardKey::Tab,
                259 => KeyboardKey::Backspace,
                260 => KeyboardKey::Insert,
                261 => KeyboardKey::Delete,
                262 => KeyboardKey::Right,
                263 => KeyboardKey::Left,
                264 => KeyboardKey::Down,
                265 => KeyboardKey::Up,
                266 => KeyboardKey::PageUp,
                267 => KeyboardKey::PageDown,
                268 => KeyboardKey::Home,
                269 => KeyboardKey::End,
                280 => KeyboardKey::CapsLock,
                281 => KeyboardKey::ScrollLock,
                282 => KeyboardKey::NumLock,
                283 => KeyboardKey::PrintScreen,
                284 => KeyboardKey::Pause,
                290 => KeyboardKey::F1,
                291 => KeyboardKey::F2,
                292 => KeyboardKey::F3,
                293 => KeyboardKey::F4,
                294 => KeyboardKey::F5,
                295 => KeyboardKey::F6,
                296 => KeyboardKey::F7,
                297 => KeyboardKey::F8,
                298 => KeyboardKey::F9,
                299 => KeyboardKey::F10,
                300 => KeyboardKey::F11,
                301 => KeyboardKey::F12,
                340 => KeyboardKey::LeftShift,
                341 => KeyboardKey::LeftControl,
                342 => KeyboardKey::LeftAlt,
                343 => KeyboardKey::LeftSuper,
                344 => KeyboardKey::RightShift,
                345 => KeyboardKey::RightControl,
                346 => KeyboardKey::RightAlt,
                347 => KeyboardKey::RightSuper,
                348 => KeyboardKey::KbMenu,
                320 => KeyboardKey::Kp0,
                321 => KeyboardKey::Kp1,
                322 => KeyboardKey::Kp2,
                323 => KeyboardKey::Kp3,
                324 => KeyboardKey::Kp4,
                325 => KeyboardKey::Kp5,
                326 => KeyboardKey::Kp6,
                327 => KeyboardKey::Kp7,
                328 => KeyboardKey::Kp8,
                329 => KeyboardKey::Kp9,
                330 => KeyboardKey::KpDecimal,
                331 => KeyboardKey::KpDivide,
                332 => KeyboardKey::KpMultiply,
                333 => KeyboardKey::KpSubtract,
                334 => KeyboardKey::KpAdd,
                335 => KeyboardKey::KpEnter,
                336 => KeyboardKey::KpEqual,
                4 => KeyboardKey::Back,
                24 => KeyboardKey::VolumeUp,
                25 => KeyboardKey::VolumeDown,
                _ => KeyboardKey::Null,
            }
        }
    }

    pub fn __get_char_pressed() -> String {
        unsafe {
            char::from_u32(GetCharPressed() as u32)
                .map(|c| c.to_string())
                .unwrap_or("".to_owned())
        }
    }

    pub fn __set_exit_key(key: impl Into<usize>) {
        unsafe { SetExitKey(key.into() as i32) }
    }
}

/// Exported methods
impl Rcore {
    // Window-related methods

    pub fn init_window(&self, width: i32, height: i32, title: &str) {
        Self::__init_window(width, height, title)
    }

    pub fn close_window(&self) {
        Self::__close_window()
    }

    pub fn window_should_close(&self) -> bool {
        Self::__window_should_close()
    }

    pub fn is_window_ready() -> bool {
        Self::__is_window_ready()
    }

    pub fn is_window_fullscreen(&self) -> bool {
        Self::__is_window_fullscreen()
    }

    pub fn is_window_hidden(&self) -> bool {
        Self::__is_window_hidden()
    }

    pub fn is_window_minimized(&self) -> bool {
        Self::__is_window_minimized()
    }

    pub fn is_window_maximized(&self) -> bool {
        Self::__is_window_maximized()
    }

    pub fn is_window_focused(&self) -> bool {
        Self::__is_window_focused()
    }

    pub fn is_window_resized(&self) -> bool {
        Self::__is_window_resized()
    }

    pub fn is_window_state(&self, flag: usize) -> bool {
        Self::__is_window_state(flag)
    }

    pub fn set_window_state(&self, flag: usize) {
        Self::__set_window_state(flag)
    }

    pub fn clear_window_state(&self, flag: usize) {
        Self::__clear_window_state(flag)
    }

    pub fn toggle_fullscreen(&self) {
        Self::__toggle_fullscreen()
    }

    pub fn toggle_borderless_windowed(&self) {
        Self::__toggle_borderless_windowed()
    }

    pub fn maximize_window(&self) {
        Self::__maximize_window()
    }

    pub fn minimize_window(&self) {
        Self::__minimize_window()
    }

    pub fn restore_window(&self) {
        Self::__restore_window()
    }

    pub fn set_window_icon(&self, image: Image) {
        Self::__set_window_icon(image)
    }

    pub fn set_window_icons(&self, images: &mut Vec<Image>) {
        Self::__set_window_icons(images)
    }

    pub fn set_window_title(&self, title: &str) {
        Self::__set_window_title(title)
    }

    pub fn set_window_position(&self, x: i32, y: i32) {
        Self::__set_window_position(x, y)
    }

    pub fn set_window_monitor(&self, monitor: i32) {
        Self::__set_window_monitor(monitor)
    }

    pub fn set_window_min_size(&self, width: i32, height: i32) {
        Self::__set_window_min_size(width, height)
    }

    pub fn set_window_max_size(&self, width: i32, height: i32) {
        Self::__set_window_max_size(width, height)
    }

    pub fn set_window_size(&self, width: i32, height: i32) {
        Self::__set_window_size(width, height)
    }

    pub fn set_window_opacity(&self, opacity: f32) {
        Self::__set_window_opacity(opacity)
    }

    pub fn set_window_focused(&self) {
        Self::__set_window_focused()
    }

    pub fn get_window_handle(&self) -> WindowHandle<'_> {
        Self::__get_window_handle()
    }

    pub fn get_screen_width(&self) -> i32 {
        Self::__get_screen_width()
    }

    pub fn get_screen_height(&self) -> i32 {
        Self::__get_screen_height()
    }

    pub fn get_render_width(&self) -> i32 {
        Self::__get_render_width()
    }

    pub fn get_render_height(&self) -> i32 {
        Self::__get_render_height()
    }

    pub fn get_monitor_count(&self) -> i32 {
        Self::__get_monitor_count()
    }

    pub fn get_current_monitor(&self) -> i32 {
        Self::__get_current_monitor()
    }

    pub fn get_monitor_position(&self, monitor: i32) -> Vector2 {
        Self::__get_monitor_position(monitor)
    }

    pub fn get_monitor_width(&self, monitor: i32) -> i32 {
        Self::__get_monitor_width(monitor)
    }

    pub fn get_monitor_height(&self, monitor: i32) -> i32 {
        Self::__get_monitor_height(monitor)
    }

    pub fn get_monitor_physical_width(&self, monitor: i32) -> i32 {
        Self::__get_monitor_physical_width(monitor)
    }

    pub fn get_monitor_physical_height(&self, monitor: i32) -> i32 {
        Self::__get_monitor_physical_height(monitor)
    }

    pub fn get_monitor_refresh_rate(&self, monitor: i32) -> i32 {
        Self::__get_monitor_refresh_rate(monitor)
    }

    pub fn get_window_position(&self) -> Vector2 {
        Self::__get_window_position()
    }

    pub fn get_window_scale_dpi(&self) -> Vector2 {
        Self::__get_window_scale_dpi()
    }

    pub fn get_monitor_name(&self, monitor: i32) -> Result<String> {
        Self::__get_monitor_name(monitor)
    }

    pub fn set_clipboard_text(&self, text: &str) {
        Self::__set_clipboard_text(text)
    }

    pub fn enable_event_waiting(&self) {
        Self::__enable_event_waiting()
    }

    pub fn disable_event_waiting(&self) {
        Self::__disable_event_waiting()
    }

    // Cursor-related methods

    pub fn show_cursor(&self) {
        Self::__show_cursor()
    }

    pub fn hide_cursor(&self) {
        Self::__hide_cursor()
    }

    pub fn is_cursor_hiden(&self) -> bool {
        Self::__is_cursor_hiden()
    }

    pub fn enable_cursor(&self) {
        Self::__enable_cursor()
    }

    pub fn disable_cursor(&self) {
        Self::__disable_cursor()
    }

    pub fn is_cursor_on_screen(&self) -> bool {
        Self::__is_cursor_on_screen()
    }

    // Drawing-related methods

    pub fn clear_background(&self, color: Color) {
        Self::__clear_background(color)
    }

    pub fn begin_drawing(&self) {
        Self::__begin_drawing()
    }

    pub fn end_drawing(&self) {
        Self::__end_drawing()
    }

    #[allow(non_snake_case)]
    pub fn begin_mode_2D(&self, camera: Camera2D) {
        Self::__begin_mode_2D(camera)
    }

    #[allow(non_snake_case)]
    pub fn end_mode_2D(&self) {
        Self::__end_mode_2D()
    }

    #[allow(non_snake_case)]
    pub fn begin_mode_3D(&self, camera: Camera3D) {
        Self::__begin_mode_3D(camera)
    }

    #[allow(non_snake_case)]
    pub fn end_mode_3D(&self) {
        Self::__end_mode_3D()
    }

    pub fn begin_texture_mode(&self, target: RenderTexture2D) {
        Self::__begin_texture_mode(target)
    }

    pub fn end_texture_mode(&self) {
        Self::__end_texture_mode()
    }

    pub fn begin_shader_mode(&self, shader: Shader) {
        Self::__begin_shader_mode(shader)
    }

    pub fn end_shader_mode(&self) {
        Self::__end_shader_mode()
    }

    pub fn begin_blend_mode(&self, mode: i32) {
        Self::__begin_blend_mode(mode)
    }

    pub fn end_blend_mode(&self) {
        Self::__end_blend_mode()
    }

    pub fn begin_scissor_mode(&self, x: i32, y: i32, width: i32, height: i32) {
        Self::__begin_scissor_mode(x, y, width, height)
    }

    pub fn end_scissor_mode(&self) {
        Self::__end_scissor_mode()
    }

    pub fn begin_vr_stereo_mode(&self, config: VrStereoConfig) {
        Self::__begin_vr_stereo_mode(config)
    }

    pub fn end_vr_stereo_mode(&self) {
        Self::__end_vr_stereo_mode()
    }

    // VR stereo config methods for VR simulator

    pub fn load_vr_stereo_config(&self, device: VrDeviceInfo) -> VrStereoConfig {
        Self::__load_vr_stereo_config(device)
    }

    pub fn unload_vr_stereo_config(&self, config: VrStereoConfig) {
        Self::__unload_vr_stereo_config(config)
    }

    // Shader management functions

    pub fn load_shader(&self, vs_filename: &str, fs_filename: &str) -> Shader {
        Self::__load_shader(vs_filename, fs_filename)
    }

    pub fn load_shader_from_memory(&self, vs_code: &str, fs_code: &str) -> Shader {
        Self::__load_shader_from_memory(vs_code, fs_code)
    }

    pub fn is_shader_ready(&self, shader: Shader) -> bool {
        Self::__is_shader_ready(shader)
    }

    pub fn get_shader_location(&self, shader: Shader, name: &str) -> i32 {
        Self::__get_shader_location(shader, name)
    }

    pub fn get_shader_location_attrib(
        &self,
        shader: Shader,
        name: &str,
    ) -> Result<enums::ShaderLocationIndex, String> {
        Self::__get_shader_location_attrib(shader, name)
    }

    pub fn set_shader_value<T>(
        &self,
        shader: Shader,
        index: i32,
        value: &T,
        tpe: enums::ShaderUniformDataType,
    ) {
        Self::__set_shader_value(shader, index, value, tpe)
    }

    pub fn set_shader_value_v<T>(
        &self,
        shader: Shader,
        index: i32,
        value: Vec<&T>,
        tpe: enums::ShaderUniformDataType,
    ) {
        Self::__set_shader_value_v(shader, index, value, tpe)
    }

    pub fn set_shader_value_matrix(&self, shader: Shader, loc: i32, mat: Matrix) {
        Self::__set_shader_value_matrix(shader, loc, mat)
    }

    pub fn set_shader_value_texture(&self, shader: Shader, index: i32, texture: Texture2D) {
        Self::__set_shader_value_texture(shader, index, texture)
    }

    pub fn unload_shader(&self, shader: Shader) {
        Self::__unload_shader(shader)
    }

    // Screen-space-related methods

    pub fn get_mouse_ray(&self, mouse_position: Vector2, camera: Camera3D) -> Ray {
        Self::__get_mouse_ray(mouse_position, camera)
    }

    #[allow(non_snake_case)]
    pub fn get_camera_matrix_2D(&self, camera: Camera2D) -> Matrix {
        Self::__get_camera_matrix_2d(camera)
    }
    #[allow(non_snake_case)]
    pub fn get_camera_matrix_3D(&self, camera: Camera3D) -> Matrix {
        Self::__get_camera_matrix_3d(camera)
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_2D(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        Self::__get_world_to_screen_2d(position, camera)
    }

    #[allow(non_snake_case)]
    pub fn get_screen_to_world_2D(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        Self::__get_screen_to_world_2d(position, camera)
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_3D(&self, position: Vector3, camera: Camera3D) -> Vector2 {
        Self::__get_world_to_screen_3d(position, camera)
    }

    #[allow(non_snake_case)]
    pub fn get_world_to_screen_3D_ex(
        &self,
        position: Vector3,
        camera: Camera3D,
        width: i32,
        height: i32,
    ) -> Vector2 {
        Self::__get_world_to_screen_3d_ex(position, camera, width, height)
    }

    // Timing-related functions

    pub fn set_target_fps(&self, fps: i32) {
        Self::__set_target_fps(fps)
    }

    pub fn get_frame_time(&self) -> f32 {
        Self::__get_frame_time()
    }

    pub fn get_time(&self) -> f64 {
        Self::__get_time()
    }

    pub fn get_fps(&self) -> i32 {
        Self::__get_fps()
    }

    // Custom frame control functions

    pub fn swap_screen_buffer(&self) {
        Self::__swap_screen_buffer()
    }

    pub fn poll_input_events(&self) {
        Self::__poll_input_events()
    }

    pub fn wait_time(&self, seconds: f64) {
        Self::__wait_time(seconds)
    }

    // Random values generation functions

    pub fn set_random_seed(&self, seed: u32) {
        Self::__set_random_seed(seed)
    }

    pub fn get_random_value(&self, min: i32, max: i32) -> i32 {
        Self::__get_random_value(min, max)
    }

    // TODO: LoadRandomSequence
    // TODO: UnloadRandomSequence

    // Misc functions

    pub fn take_screenshot(&self, filename: &str) {
        Self::__take_screenshot(filename)
    }

    pub fn set_config_flags(&self, flags: usize) {
        Self::__set_config_flags(flags)
    }

    pub fn open_url(&self, url: &str) {
        Self::__open_url(url)
    }

    pub fn trace_log(&self, level: TraceLogLevel, text: &str) {
        Self::__trace_log(level, text)
    }

    pub fn set_trace_log_level(&self, level: TraceLogLevel) {
        Self::__set_trace_log_level(level)
    }

    pub fn mem_alloc(&self, size: usize) -> *mut c_void {
        Self::__mem_alloc(size)
    }

    pub fn mem_realloc(&self, ptr: *mut c_void, size: usize) -> *mut c_void {
        Self::__mem_realloc(ptr, size)
    }

    pub fn mem_free(&self, ptr: *mut c_void) {
        Self::__mem_free(ptr)
    }

    // Files management functions

    pub fn load_file_data(&self, filename: &str) -> Vec<u8> {
        Self::__load_file_data(filename)
    }

    // TODO: UnloadFileData

    pub fn save_file_data(&self, filename: &str, data: Vec<u8>) -> bool {
        Self::__save_file_data(filename, data)
    }

    pub fn export_data_as_code(&self, data: &str, filename: &str) -> bool {
        Self::__export_data_as_code(data, filename)
    }

    pub fn load_file_text(&self, filename: &str) -> Result<String> {
        Self::__load_file_text(filename)
    }

    // TODO: UnloadFileText

    pub fn save_file_text(&self, filename: &str, text: &str) -> bool {
        Self::__save_file_text(filename, text)
    }

    // File system functions

    pub fn file_exists(&self, filename: &str) -> bool {
        Self::__file_exists(filename)
    }

    pub fn directory_exists(&self, dirname: &str) -> bool {
        Self::__directory_exists(dirname)
    }

    pub fn is_file_extension(&self, filename: &str, ext: &str) -> bool {
        Self::__is_file_extension(filename, ext)
    }

    pub fn get_file_length(&self, filename: &str) -> i32 {
        Self::__get_file_length(filename)
    }

    pub fn get_file_extenstion(&self, filename: &str) -> Result<String> {
        Self::__get_file_extenstion(filename)
    }

    pub fn get_file_name(&self, path: &str) -> Result<String> {
        Self::__get_file_name(path)
    }

    pub fn get_file_name_without_ext(&self, path: &str) -> Result<String> {
        Self::__get_file_name_without_ext(path)
    }

    pub fn get_directory_path(&self, path: &str) -> Result<String> {
        Self::__get_directory_path(path)
    }

    pub fn get_prev_directory_path(&self, path: &str) -> Result<String> {
        Self::__get_prev_directory_path(path)
    }

    pub fn get_working_directory(&self) -> Result<String> {
        Self::__get_working_directory()
    }

    pub fn get_application_directory(&self) -> Result<String> {
        Self::__get_application_directory()
    }

    pub fn change_directory(&self, dir: &str) -> bool {
        Self::__change_directory(dir)
    }

    pub fn is_path_file(&self, path: &str) -> bool {
        Self::__is_path_file(path)
    }

    pub fn load_directory_files(&self, path: &str) -> FilePathList {
        Self::__load_directory_files(path)
    }

    pub fn load_directory_files_ex(
        &self,
        path: &str,
        filter: &str,
        scan_subdirs: bool,
    ) -> FilePathList {
        Self::__load_directory_files_ex(path, filter, scan_subdirs)
    }

    pub fn unload_directory_files(&self, files: FilePathList) {
        Self::__unload_directory_files(files)
    }

    pub fn is_file_dropped(&self) -> bool {
        Self::__is_file_dropped()
    }

    pub fn load_dropped_files() -> FilePathList {
        Self::__load_dropped_files()
    }

    pub fn unload_dropped_files(&self, files: FilePathList) {
        Self::__unload_dropped_files(files)
    }

    pub fn get_file_mod_time(&self, filename: &str) -> i64 {
        Self::__get_file_mod_time(filename)
    }

    // Compression/Encoding functionality

    pub fn compress_data(&self, data: Vec<u8>) -> Vec<u8> {
        Self::__compress_data(data)
    }

    pub fn decompress_data(&self, data: Vec<u8>) -> Vec<u8> {
        Self::__decompress_data(data)
    }

    pub fn encode_data_base64(&self, data: Vec<u8>) -> Result<String> {
        Self::__encode_data_base64(data)
    }

    pub fn decode_data_base64(&self, data: &str) -> Vec<u8> {
        Self::__decode_data_base64(data)
    }

    // Automation events functionality

    pub fn load_automation_event_list(&self, filename: &str) -> AutomationEventList {
        Self::__load_automation_event_list(filename)
    }

    pub fn unload_automation_event_list(&self, list: AutomationEventList) {
        Self::__unload_automation_event_list(list)
    }

    pub fn export_automation_event_list(&self, list: AutomationEventList, filename: &str) -> bool {
        Self::__export_automation_event_list(list, filename)
    }

    pub fn set_automation_event_list(&self, list: AutomationEventList) {
        Self::__set_automation_event_list(list)
    }

    pub fn set_automation_event_base_frame(&self, frame: i32) {
        Self::__set_automation_event_base_frame(frame)
    }

    pub fn start_automation_event_record(&self) {
        Self::__start_automation_event_record()
    }

    pub fn stop_automation_event_record(&self) {
        Self::__stop_automation_event_record()
    }

    pub fn play_automation_event(&self, event: AutomationEvent) {
        Self::__play_automation_event(event)
    }

    // Input-related functions: keyboard

    pub fn is_key_pressed(&self, key: impl Into<usize>) -> bool {
        Self::__is_key_pressed(key)
    }

    pub fn is_key_pressed_repeat(&self, key: impl Into<usize>) -> bool {
        Self::__is_key_pressed_repeat(key)
    }

    pub fn is_key_down(&self, key: impl Into<usize>) -> bool {
        Self::__is_key_down(key)
    }

    pub fn is_key_released(&self, key: impl Into<usize>) -> bool {
        Self::__is_key_released(key)
    }

    pub fn is_key_up(&self, key: impl Into<usize>) -> bool {
        Self::__is_key_up(key)
    }

    pub fn get_key_pressed(&self) -> KeyboardKey {
        Self::__get_key_pressed()
    }

    pub fn get_char_pressed(&self) -> String {
        Self::__get_char_pressed()
    }

    pub fn set_exit_key(&self, key: impl Into<usize>) {
        Self::__set_exit_key(key)
    }
}
