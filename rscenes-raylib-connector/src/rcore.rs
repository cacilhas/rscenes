use crate::ext::window_handle::WindowHandle;
use crate::utils::{array_from_c, string_from_c};
use raylib_ffi::{enums::*, *};
use std::char;
use std::ffi::{c_char, c_uchar, c_void};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RcoreImpl;

/// Crate only methods
impl RcoreImpl {
    // Window-related methods

    pub fn __init_window(width: i32, height: i32, title: impl Display) {
        unsafe { InitWindow(width, height, rl_str!(title)) }
    }

    pub fn __close_window() {
        unsafe { CloseWindow() }
    }

    pub fn __window_should_close() -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn __is_window_ready() -> bool {
        unsafe { IsWindowReady() }
    }

    pub fn __is_window_fullscreen() -> bool {
        unsafe { IsWindowFullscreen() }
    }

    pub fn __is_window_hidden() -> bool {
        unsafe { IsWindowHidden() }
    }

    pub fn __is_window_minimized() -> bool {
        unsafe { IsWindowMinimized() }
    }

    pub fn __is_window_maximized() -> bool {
        unsafe { IsWindowMaximized() }
    }

    pub fn __is_window_focused() -> bool {
        unsafe { IsWindowFocused() }
    }

    pub fn __is_window_resized() -> bool {
        unsafe { IsWindowResized() }
    }

    pub fn __is_window_state(flag: usize) -> bool {
        unsafe { IsWindowState(flag as u32) }
    }

    pub fn __set_window_state(flag: usize) {
        let flag: usize = flag;
        unsafe { SetWindowState(flag as u32) }
    }

    pub fn __clear_window_state(flag: usize) {
        let flag: usize = flag;
        unsafe { ClearWindowState(flag as u32) }
    }

    pub fn __toggle_fullscreen() {
        unsafe { ToggleFullscreen() }
    }

    pub fn __toggle_borderless_windowed() {
        unsafe { ToggleBorderlessWindowed() }
    }

    pub fn __maximize_window() {
        unsafe { MaximizeWindow() }
    }

    pub fn __minimize_window() {
        unsafe { MinimizeWindow() }
    }

    pub fn __restore_window() {
        unsafe { RestoreWindow() }
    }

    pub fn __set_window_icon(image: Image) {
        unsafe { SetWindowIcon(image) }
    }

    pub fn __set_window_icons(images: &mut Vec<Image>) {
        unsafe {
            let count = images.len() as i32;
            SetWindowIcons(images.as_mut_ptr(), count)
        }
    }

    pub fn __set_window_title(title: impl Display) {
        unsafe { SetWindowTitle(rl_str!(title)) }
    }

    pub fn __set_window_position(x: i32, y: i32) {
        unsafe { SetWindowPosition(x, y) }
    }

    pub fn __set_window_monitor(monitor: i32) {
        unsafe { SetWindowMonitor(monitor) }
    }

    pub fn __set_window_min_size(width: i32, height: i32) {
        unsafe { SetWindowMinSize(width, height) }
    }

    pub fn __set_window_max_size(width: i32, height: i32) {
        unsafe { SetWindowMaxSize(width, height) }
    }

    pub fn __set_window_size(width: i32, height: i32) {
        unsafe { SetWindowSize(width, height) }
    }

    pub fn __set_window_opacity(opacity: f32) {
        unsafe { SetWindowOpacity(opacity) }
    }

    pub fn __set_window_focused() {
        unsafe { SetWindowFocused() }
    }

    pub fn __get_window_handle<'a>() -> Result<WindowHandle<'a>, String> {
        unsafe {
            let raw = GetWindowHandle();
            if raw.is_null() {
                Err("couldn't get window handle".to_owned())
            } else {
                Ok(raw.into())
            }
        }
    }

    pub fn __get_screen_width() -> i32 {
        unsafe { GetScreenWidth() }
    }

    pub fn __get_screen_height() -> i32 {
        unsafe { GetScreenHeight() }
    }

    pub fn __get_render_width() -> i32 {
        unsafe { GetRenderWidth() }
    }

    pub fn __get_render_height() -> i32 {
        unsafe { GetRenderHeight() }
    }

    pub fn __get_monitor_count() -> i32 {
        unsafe { GetMonitorCount() }
    }

    pub fn __get_current_monitor() -> i32 {
        unsafe { GetCurrentMonitor() }
    }

    pub fn __get_monitor_position(monitor: i32) -> Vector2 {
        unsafe { GetMonitorPosition(monitor) }
    }

    pub fn __get_monitor_width(monitor: i32) -> i32 {
        unsafe { GetMonitorWidth(monitor) }
    }

    pub fn __get_monitor_height(monitor: i32) -> i32 {
        unsafe { GetMonitorHeight(monitor) }
    }

    pub fn __get_monitor_physical_width(monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalWidth(monitor) }
    }

    pub fn __get_monitor_physical_height(monitor: i32) -> i32 {
        unsafe { GetMonitorPhysicalHeight(monitor) }
    }

    pub fn __get_monitor_refresh_rate(monitor: i32) -> i32 {
        unsafe { GetMonitorRefreshRate(monitor) }
    }

    pub fn __get_window_position() -> Vector2 {
        unsafe { GetWindowPosition() }
    }

    pub fn __get_window_scale_dpi() -> Vector2 {
        unsafe { GetWindowScaleDPI() }
    }

    pub fn __get_monitor_name(monitor: i32) -> Result<String, String> {
        unsafe { string_from_c(GetMonitorName(monitor) as *mut c_char) }
    }

    pub fn __set_clipboard_text(text: impl Display) {
        unsafe { SetClipboardText(rl_str!(text)) }
    }

    pub fn __get_clipboard_text() -> Result<String, String> {
        unsafe { string_from_c(GetClipboardText() as *mut c_char) }
    }

    pub fn __enable_event_waiting() {
        unsafe { EnableEventWaiting() }
    }

    pub fn __disable_event_waiting() {
        unsafe { DisableEventWaiting() }
    }

    // Cursor-related methods

    pub fn __show_cursor() {
        unsafe { ShowCursor() }
    }

    pub fn __hide_cursor() {
        unsafe { HideCursor() }
    }

    pub fn __is_cursor_hiden() -> bool {
        unsafe { IsCursorHidden() }
    }

    pub fn __enable_cursor() {
        unsafe { EnableCursor() }
    }

    pub fn __disable_cursor() {
        unsafe { DisableCursor() }
    }

    pub fn __is_cursor_on_screen() -> bool {
        unsafe { IsCursorOnScreen() }
    }

    // Drawing-related methods

    pub fn __clear_background(color: Color) {
        unsafe { ClearBackground(color) }
    }

    pub fn __begin_drawing() {
        unsafe { BeginDrawing() }
    }

    pub fn __end_drawing() {
        unsafe { EndDrawing() }
    }

    #[allow(non_snake_case)]
    pub fn __begin_mode_2D(camera: Camera2D) {
        unsafe { BeginMode2D(camera) }
    }

    #[allow(non_snake_case)]
    pub fn __end_mode_2D() {
        unsafe { EndMode2D() }
    }

    #[allow(non_snake_case)]
    pub fn __begin_mode_3D(camera: Camera3D) {
        unsafe { BeginMode3D(camera) }
    }

    #[allow(non_snake_case)]
    pub fn __end_mode_3D() {
        unsafe { EndMode3D() }
    }

    pub fn __begin_texture_mode(target: RenderTexture2D) {
        unsafe { BeginTextureMode(target) }
    }

    pub fn __end_texture_mode() {
        unsafe { EndTextureMode() }
    }

    pub fn __begin_shader_mode(shader: Shader) {
        unsafe { BeginShaderMode(shader) }
    }

    pub fn __end_shader_mode() {
        unsafe { EndShaderMode() }
    }

    pub fn __begin_blend_mode(mode: impl Into<usize>) {
        unsafe { BeginBlendMode(mode.into() as i32) }
    }

    pub fn __end_blend_mode() {
        unsafe { EndBlendMode() }
    }

    pub fn __begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
        unsafe { BeginScissorMode(x, y, width, height) }
    }

    pub fn __end_scissor_mode() {
        unsafe { EndScissorMode() }
    }

    pub fn __begin_vr_stereo_mode(config: VrStereoConfig) {
        unsafe { BeginVrStereoMode(config) }
    }

    pub fn __end_vr_stereo_mode() {
        unsafe { EndVrStereoMode() }
    }

    // VR stereo config methods for VR simulator

    pub fn __load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
        unsafe { LoadVrStereoConfig(device) }
    }

    pub fn __unload_vr_stereo_config(config: VrStereoConfig) {
        unsafe { UnloadVrStereoConfig(config) }
    }

    // Shader management methods

    pub fn __load_shader(
        vs_filename: impl Display,
        fs_filename: impl Display,
    ) -> Result<Shader, String> {
        unsafe {
            let shader = LoadShader(rl_str!(vs_filename), rl_str!(fs_filename));
            if shader.locs.is_null() {
                Err(format!(
                    "couldn't load shader from [vs]{} [fs]{}",
                    vs_filename, fs_filename
                ))
            } else {
                Ok(shader)
            }
        }
    }

    pub fn __load_shader_from_memory(
        vs_code: impl Display,
        fs_code: impl Display,
    ) -> Result<Shader, String> {
        unsafe {
            let shader = LoadShaderFromMemory(rl_str!(vs_code), rl_str!(fs_code));
            if shader.locs.is_null() {
                Err("failed to load shader from memory".to_owned())
            } else {
                Ok(shader)
            }
        }
    }

    pub fn __is_shader_ready(shader: Shader) -> bool {
        unsafe { IsShaderReady(shader) }
    }

    pub fn __get_shader_location(shader: Shader, name: impl Display) -> i32 {
        unsafe { GetShaderLocation(shader, rl_str!(name)) }
    }

    pub fn __get_shader_location_attrib(
        shader: Shader,
        name: impl Display,
    ) -> Result<ShaderLocationIndex, String> {
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

    pub fn __set_shader_value<T>(
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

    pub fn __set_shader_value_v<T>(
        shader: Shader,
        index: i32,
        value: &[&T],
        tpe: enums::ShaderUniformDataType,
    ) {
        unsafe {
            let tpe: usize = tpe.into();
            let count = value.len() as i32;
            let value = value.as_ptr() as *const c_void;
            SetShaderValueV(shader, index, value, tpe as i32, count)
        }
    }

    pub fn __set_shader_value_matrix(shader: Shader, loc: i32, mat: Matrix) {
        unsafe { SetShaderValueMatrix(shader, loc, mat) }
    }

    pub fn __set_shader_value_texture(shader: Shader, index: i32, texture: Texture2D) {
        unsafe { SetShaderValueTexture(shader, index, texture) }
    }

    pub fn __unload_shader(shader: Shader) {
        unsafe { UnloadShader(shader) }
    }

    // Screen-space-related methods

    pub fn __get_mouse_ray(mouse_position: Vector2, camera: Camera3D) -> Ray {
        unsafe { GetMouseRay(mouse_position, camera) }
    }

    pub fn __get_camera_matrix_2d(camera: Camera2D) -> Matrix {
        unsafe { GetCameraMatrix2D(camera) }
    }

    pub fn __get_camera_matrix_3d(camera: Camera3D) -> Matrix {
        unsafe { GetCameraMatrix(camera) }
    }

    pub fn __get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetWorldToScreen2D(position, camera) }
    }

    pub fn __get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
        unsafe { GetScreenToWorld2D(position, camera) }
    }

    pub fn __get_world_to_screen_3d(position: Vector3, camera: Camera3D) -> Vector2 {
        unsafe { GetWorldToScreen(position, camera) }
    }

    pub fn __get_world_to_screen_3d_ex(
        position: Vector3,
        camera: Camera3D,
        width: i32,
        height: i32,
    ) -> Vector2 {
        unsafe { GetWorldToScreenEx(position, camera, width, height) }
    }

    // Timing-related methods

    pub fn __set_target_fps(fps: i32) {
        unsafe { SetTargetFPS(fps) }
    }

    pub fn __get_frame_time() -> f32 {
        unsafe { GetFrameTime() }
    }

    pub fn __get_time() -> f64 {
        unsafe { GetTime() }
    }

    pub fn __get_fps() -> i32 {
        unsafe { GetFPS() }
    }

    // Custom frame control methods

    pub fn __swap_screen_buffer() {
        unsafe { SwapScreenBuffer() }
    }

    pub fn __poll_input_events() {
        unsafe { PollInputEvents() }
    }

    pub fn __wait_time(seconds: f64) {
        unsafe { WaitTime(seconds) }
    }

    // Random values generation methods

    pub fn __set_random_seed(seed: u32) {
        unsafe { SetRandomSeed(seed) }
    }

    pub fn __get_random_value(min: i32, max: i32) -> i32 {
        unsafe { GetRandomValue(min, max) }
    }

    pub fn __load_random_sequence(count: usize, min: i32, max: i32) -> Result<Vec<i32>, String> {
        unsafe {
            let raw = LoadRandomSequence(count as u32, min, max);
            let res = array_from_c(raw, count, || {
                "could not generate random sequence".to_owned()
            })?
            // Copy sequence to the stack
            .iter()
            .map(|e| *e)
            .collect::<Vec<_>>();
            UnloadRandomSequence(raw);
            Ok(res)
        }
    }

    // Misc methods

    pub fn __take_screenshot(filename: impl Display) {
        unsafe { TakeScreenshot(rl_str!(filename)) }
    }

    pub fn __set_config_flags(flags: impl Into<usize>) {
        unsafe { SetConfigFlags(flags.into() as u32) }
    }

    pub fn __open_url(url: impl Display) {
        unsafe { OpenURL(rl_str!(url)) }
    }

    pub fn __trace_log(level: impl Into<usize>, text: impl Display) {
        unsafe { TraceLog(level.into() as i32, rl_str!(text)) }
    }

    pub fn __set_trace_log_level(level: impl Into<usize>) {
        unsafe { SetTraceLogLevel(level.into() as i32) }
    }

    // pub fn __mem_alloc(size: usize) -> *mut c_void {
    //     unsafe { MemAlloc(size as u32) }
    // }
    //
    // pub fn __mem_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    //     unsafe { MemRealloc(ptr, size as u32) }
    // }
    //
    // pub fn __mem_free(ptr: *mut c_void) {
    //     unsafe { MemFree(ptr) }
    // }

    // Files management methods

    pub fn __load_file_data(filename: impl Display) -> Result<Vec<u8>, String> {
        unsafe {
            let mut size = 0;
            let raw = LoadFileData(rl_str!(filename), &mut size);
            let res = array_from_c(raw, size as usize, || {
                format!("couldn't load file data from {}", filename)
            })?
            // Copy data to the stack
            .iter()
            .map(|e| *e)
            .collect::<Vec<_>>();
            UnloadFileData(raw);
            Ok(res)
        }
    }

    pub fn __save_file_data(filename: impl Display, data: &mut Vec<u8>) -> bool {
        unsafe {
            let size = data.len() as i32;
            let data = data.as_mut_ptr() as *mut c_void;
            SaveFileData(rl_str!(filename), data, size)
        }
    }

    pub fn __export_data_as_code(data: &str, filename: impl Display) -> bool {
        unsafe {
            let size = data.len() as i32;
            ExportDataAsCode(rl_str!(data) as *const c_uchar, size, rl_str!(filename))
        }
    }

    pub fn __load_file_text(filename: impl Display) -> Result<String, String> {
        unsafe {
            let raw = LoadFileText(rl_str!(filename)) as *mut c_char;
            let res = string_from_c(raw).map(|e| e.clone());
            UnloadFileText(raw);
            res
        }
    }

    pub fn __save_file_text(filename: impl Display, text: impl Display) -> bool {
        unsafe { SaveFileText(rl_str!(filename), rl_str!(text) as *mut c_char) }
    }

    // File system methods

    // pub fn __file_exists(filename: impl Display) -> bool {
    //     unsafe { FileExists(rl_str!(filename)) }
    // }
    //
    // pub fn __directory_exists(dirname: impl Display) -> bool {
    //     unsafe { DirectoryExists(rl_str!(dirname)) }
    // }
    //
    // pub fn __is_file_extension(filename: impl Display, ext: impl Display) -> bool {
    //     unsafe { IsFileExtension(rl_str!(filename), rl_str!(ext)) }
    // }
    //
    // pub fn __get_file_length(filename: impl Display) -> i32 {
    //     unsafe { GetFileLength(rl_str!(filename)) }
    // }
    //
    // pub fn __get_file_extenstion(filename: impl Display) -> Result<String, String> {
    //     unsafe { string_from_c(GetFileExtension(rl_str!(filename)) as *mut c_char) }
    // }
    //
    // pub fn __get_file_name(path: impl Display) -> Result<String, String> {
    //     unsafe { string_from_c(GetFileName(rl_str!(path)) as *mut c_char) }
    // }
    //
    // pub fn __get_file_name_without_ext(path: impl Display) -> Result<String, String> {
    //     unsafe { string_from_c(GetFileNameWithoutExt(rl_str!(path)) as *mut c_char) }
    // }
    //
    // pub fn __get_directory_path(path: impl Display) -> Result<String, String> {
    //     unsafe { string_from_c(GetDirectoryPath(rl_str!(path)) as *mut c_char) }
    // }
    //
    // pub fn __get_prev_directory_path(path: impl Display) -> Result<String, String> {
    //     unsafe { string_from_c(GetPrevDirectoryPath(rl_str!(path)) as *mut c_char) }
    // }
    //
    // pub fn __get_working_directory() -> Result<String, String> {
    //     unsafe { string_from_c(GetWorkingDirectory() as *mut c_char) }
    // }

    pub fn __get_application_directory() -> Result<String, String> {
        unsafe { string_from_c(GetApplicationDirectory() as *mut c_char) }
    }

    // pub fn __change_directory(dir: impl Display) -> bool {
    //     unsafe { ChangeDirectory(rl_str!(dir)) }
    // }
    //
    // pub fn __is_path_file(path: impl Display) -> bool {
    //     unsafe { IsPathFile(rl_str!(path)) }
    // }
    //
    // pub fn __load_directory_files(path: impl Display) -> FilePathList {
    //     unsafe { LoadDirectoryFiles(rl_str!(path)) }
    // }
    //
    // pub fn __load_directory_files_ex(
    //     path: impl Display,
    //     filter: impl Display,
    //     scan_subdirs: bool,
    // ) -> FilePathList {
    //     unsafe { LoadDirectoryFilesEx(rl_str!(path), rl_str!(filter), scan_subdirs) }
    // }
    //
    // pub fn __unload_directory_files(files: FilePathList) {
    //     unsafe { UnloadDirectoryFiles(files) }
    // }
    //
    // pub fn __is_file_dropped() -> bool {
    //     unsafe { IsFileDropped() }
    // }
    //
    // pub fn __load_dropped_files() -> FilePathList {
    //     unsafe { LoadDroppedFiles() }
    // }
    //
    // pub fn __unload_dropped_files(files: FilePathList) {
    //     unsafe { UnloadDroppedFiles(files) }
    // }
    //
    // pub fn __get_file_mod_time(filename: impl Display) -> i64 {
    //     unsafe { GetFileModTime(rl_str!(filename)) }
    // }

    // Compression/Encoding functionality

    // pub fn __compress_data(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
    //     unsafe {
    //         let size = data.len() as i32;
    //         let data = data.as_mut_ptr() as *mut c_uchar;
    //         let mut comp_size = 0;
    //         let raw = CompressData(data, size, &mut comp_size);
    //         array_from_c(raw, size as usize, || {
    //             "error trying to compress data".to_owned()
    //         })
    //     }
    // }
    //
    // pub fn __decompress_data(data: &mut Vec<u8>) -> Result<Vec<u8>, String> {
    //     unsafe {
    //         let size = data.len() as i32;
    //         let data = data.as_mut_ptr() as *mut c_uchar;
    //         let mut decomp_size = 0;
    //         let raw = DecompressData(data, size, &mut decomp_size);
    //         array_from_c(raw, size as usize, || {
    //             "error trying to decompress data".to_owned()
    //         })
    //     }
    // }
    //
    // pub fn __encode_data_base64(data: &mut Vec<u8>) -> Result<String, String> {
    //     unsafe {
    //         let size = data.len() as i32;
    //         let data = data.as_mut_ptr() as *mut c_uchar;
    //         let mut output_size = 0;
    //         string_from_c(EncodeDataBase64(data, size, &mut output_size) as *mut c_char)
    //     }
    // }
    //
    // pub fn __decode_data_base64(data: &str) -> Result<Vec<u8>, String> {
    //     unsafe {
    //         let mut size: i32 = 0;
    //         let raw = DecodeDataBase64(rl_str!(data) as *const c_uchar, &mut size);
    //         array_from_c(raw, size as usize, || {
    //             format!("could not decode as Base64: {}", data)
    //         })
    //     }
    // }

    // Automation events functionality

    pub fn __load_automation_event_list(
        filename: impl Display,
    ) -> Result<AutomationEventList, String> {
        unsafe {
            let list = LoadAutomationEventList(rl_str!(filename));
            if list.events.is_null() {
                Err(format!("couldn't load automation events from {}", filename))
            } else {
                Ok(list)
            }
        }
    }

    pub fn __unload_automation_event_list(mut list: AutomationEventList) {
        unsafe { UnloadAutomationEventList(&mut list) }
    }

    pub fn __export_automation_event_list(
        list: AutomationEventList,
        filename: impl Display,
    ) -> bool {
        unsafe { ExportAutomationEventList(list, rl_str!(filename)) }
    }

    pub fn __set_automation_event_list(mut list: AutomationEventList) {
        unsafe { SetAutomationEventList(&mut list) }
    }

    pub fn __set_automation_event_base_frame(frame: i32) {
        unsafe { SetAutomationEventBaseFrame(frame) }
    }

    pub fn __start_automation_event_record() {
        unsafe { StartAutomationEventRecording() }
    }

    pub fn __stop_automation_event_record() {
        unsafe { StopAutomationEventRecording() }
    }

    pub fn __play_automation_event(event: AutomationEvent) {
        unsafe { PlayAutomationEvent(event) }
    }

    // Input-related methods: keyboard

    pub fn __is_key_pressed(key: impl Into<usize>) -> bool {
        unsafe { IsKeyPressed(key.into() as i32) }
    }

    pub fn __is_key_pressed_repeat(key: impl Into<usize>) -> bool {
        unsafe { IsKeyPressedRepeat(key.into() as i32) }
    }

    pub fn __is_key_down(key: impl Into<usize>) -> bool {
        unsafe { IsKeyDown(key.into() as i32) }
    }

    pub fn __is_key_released(key: impl Into<usize>) -> bool {
        unsafe { IsKeyReleased(key.into() as i32) }
    }

    pub fn __is_key_up(key: impl Into<usize>) -> bool {
        unsafe { IsKeyUp(key.into() as i32) }
    }

    pub fn __get_key_pressed() -> KeyboardKey {
        unsafe {
            match GetKeyPressed() {
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

    // Input-related methods: gamepads

    pub fn __is_gamepad_available(gamepad: i32) -> bool {
        unsafe { IsGamepadAvailable(gamepad) }
    }

    pub fn __get_gamepad_name(gamepad: i32) -> Result<String, String> {
        unsafe { string_from_c(GetGamepadName(gamepad) as *mut c_char) }
    }

    pub fn __is_gamepad_button_pressed(gamepad: i32, button: impl Into<usize>) -> bool {
        unsafe { IsGamepadButtonPressed(gamepad, button.into() as i32) }
    }

    pub fn __is_gamepad_button_down(gamepad: i32, button: impl Into<usize>) -> bool {
        unsafe { IsGamepadButtonDown(gamepad, button.into() as i32) }
    }

    pub fn __is_gamepad_button_released(gamepad: i32, button: impl Into<usize>) -> bool {
        unsafe { IsGamepadButtonReleased(gamepad, button.into() as i32) }
    }

    pub fn __is_gamepad_button_up(gamepad: i32, button: impl Into<usize>) -> bool {
        unsafe { IsGamepadButtonUp(gamepad, button.into() as i32) }
    }

    pub fn __get_gamepad_button_pressed() -> GamepadButton {
        unsafe {
            match GetGamepadButtonPressed() {
                1 => GamepadButton::LeftFaceUp,
                2 => GamepadButton::LeftFaceRight,
                3 => GamepadButton::LeftFaceDown,
                4 => GamepadButton::LeftFaceLeft,
                5 => GamepadButton::RightFaceUp,
                6 => GamepadButton::RightFaceRight,
                7 => GamepadButton::RightFaceDown,
                8 => GamepadButton::RightFaceLeft,
                9 => GamepadButton::LeftTrigger1,
                10 => GamepadButton::LeftTrigger2,
                11 => GamepadButton::RightTrigger1,
                12 => GamepadButton::RightTrigger2,
                13 => GamepadButton::MiddleLeft,
                14 => GamepadButton::Middle,
                15 => GamepadButton::MiddleRight,
                16 => GamepadButton::LeftThumb,
                17 => GamepadButton::RightThumb,
                _ => GamepadButton::Unknown,
            }
        }
    }

    pub fn __get_gamepad_axis_count(gamepad: i32) -> i32 {
        unsafe { GetGamepadAxisCount(gamepad) }
    }

    pub fn __get_gamepad_axis_movement(gamepad: i32, axis: impl Into<usize>) -> f32 {
        unsafe { GetGamepadAxisMovement(gamepad, axis.into() as i32) }
    }

    pub fn __set_gamepad_mappings(mappings: impl Display) -> i32 {
        unsafe { SetGamepadMappings(rl_str!(mappings)) }
    }

    // Input-related methods: mouse

    pub fn __is_mouse_button_pressed(button: impl Into<usize>) -> bool {
        unsafe { IsMouseButtonPressed(button.into() as i32) }
    }

    pub fn __is_mouse_button_down(button: impl Into<usize>) -> bool {
        unsafe { IsMouseButtonDown(button.into() as i32) }
    }

    pub fn __is_mouse_button_released(button: impl Into<usize>) -> bool {
        unsafe { IsMouseButtonReleased(button.into() as i32) }
    }

    pub fn __is_mouse_button_up(button: impl Into<usize>) -> bool {
        unsafe { IsMouseButtonUp(button.into() as i32) }
    }

    pub fn __get_mouse_x() -> i32 {
        unsafe { GetMouseX() }
    }

    pub fn __get_mouse_y() -> i32 {
        unsafe { GetMouseY() }
    }

    pub fn __get_mouse_position() -> Vector2 {
        unsafe { GetMousePosition() }
    }

    pub fn __get_mouse_delta() -> Vector2 {
        unsafe { GetMouseDelta() }
    }

    pub fn __set_mouse_position(x: i32, y: i32) {
        unsafe { SetMousePosition(x, y) }
    }

    pub fn __set_mouse_offset(x: i32, y: i32) {
        unsafe { SetMouseOffset(x, y) }
    }

    pub fn __set_mouse_scale(x: f32, y: f32) {
        unsafe { SetMouseScale(x, y) }
    }

    pub fn __get_mouse_wheel_move() -> f32 {
        unsafe { GetMouseWheelMove() }
    }

    pub fn __get_mouse_wheel_move_v() -> Vector2 {
        unsafe { GetMouseWheelMoveV() }
    }

    pub fn __set_mouse_cursor(cursor: impl Into<usize>) {
        unsafe { SetMouseCursor(cursor.into() as i32) }
    }

    // Input-related methods: touch

    pub fn __get_touch_x() -> i32 {
        unsafe { GetTouchX() }
    }

    pub fn __get_touch_y() -> i32 {
        unsafe { GetTouchY() }
    }

    pub fn __get_touch_position(index: i32) -> Vector2 {
        unsafe { GetTouchPosition(index) }
    }

    pub fn __get_touch_point_id(index: i32) -> i32 {
        unsafe { GetTouchPointId(index) }
    }

    pub fn __get_touch_point_count() -> i32 {
        unsafe { GetTouchPointCount() }
    }
}

/// Exported methods
pub trait Rcore: Debug {
    // Window-related methods

    /// Initialize window and OpenGL context
    fn init_window(&self, width: i32, height: i32, title: impl Display) {
        RcoreImpl::__init_window(width, height, title)
    }

    /// Close window and unload OpenGL context
    fn close_window(&self) {
        RcoreImpl::__close_window()
    }

    /// Check whether application should close (KEY_ESCAPE pressed or windows close icon clicked)
    fn window_should_close(&self) -> bool {
        RcoreImpl::__window_should_close()
    }

    /// Check whether window has been initialized successfully
    fn is_window_ready() -> bool {
        RcoreImpl::__is_window_ready()
    }

    /// Check whether window is currently fullscreen
    fn is_window_fullscreen(&self) -> bool {
        RcoreImpl::__is_window_fullscreen()
    }

    /// Check whether window is currently hidden (only PLATFORM_DESKTOP)
    fn is_window_hidden(&self) -> bool {
        RcoreImpl::__is_window_hidden()
    }

    /// Check whether window is currently minimized (only PLATFORM_DESKTOP)
    fn is_window_minimized(&self) -> bool {
        RcoreImpl::__is_window_minimized()
    }

    /// Check whether window is currently maximized (only PLATFORM_DESKTOP)
    fn is_window_maximized(&self) -> bool {
        RcoreImpl::__is_window_maximized()
    }

    /// Check whether window is currently focused (only PLATFORM_DESKTOP)
    fn is_window_focused(&self) -> bool {
        RcoreImpl::__is_window_focused()
    }

    /// Check whether window has been resized last frame
    fn is_window_resized(&self) -> bool {
        RcoreImpl::__is_window_resized()
    }

    /// Check whether one specific window flag is enabled
    fn is_window_state(&self, flag: usize) -> bool {
        RcoreImpl::__is_window_state(flag)
    }

    /// Set window configuration state using flags (only PLATFORM_DESKTOP)
    fn set_window_state(&self, flag: usize) {
        RcoreImpl::__set_window_state(flag)
    }

    /// Clear window configuration state flags
    fn clear_window_state(&self, flag: usize) {
        RcoreImpl::__clear_window_state(flag)
    }

    /// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
    fn toggle_fullscreen(&self) {
        RcoreImpl::__toggle_fullscreen()
    }

    /// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
    fn toggle_borderless_windowed(&self) {
        RcoreImpl::__toggle_borderless_windowed()
    }

    /// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
    fn maximize_window(&self) {
        RcoreImpl::__maximize_window()
    }

    /// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
    fn minimize_window(&self) {
        RcoreImpl::__minimize_window()
    }

    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
    fn restore_window(&self) {
        RcoreImpl::__restore_window()
    }

    /// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
    fn set_window_icon(&self, image: Image) {
        RcoreImpl::__set_window_icon(image)
    }

    /// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
    fn set_window_icons(&self, images: &mut Vec<Image>) {
        RcoreImpl::__set_window_icons(images)
    }

    /// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
    fn set_window_title(&self, title: impl Display) {
        RcoreImpl::__set_window_title(title)
    }

    /// Set window position on screen (only PLATFORM_DESKTOP)
    fn set_window_position(&self, x: i32, y: i32) {
        RcoreImpl::__set_window_position(x, y)
    }

    /// Set monitor for the current window
    fn set_window_monitor(&self, monitor: i32) {
        RcoreImpl::__set_window_monitor(monitor)
    }

    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
    fn set_window_min_size(&self, width: i32, height: i32) {
        RcoreImpl::__set_window_min_size(width, height)
    }

    /// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
    fn set_window_max_size(&self, width: i32, height: i32) {
        RcoreImpl::__set_window_max_size(width, height)
    }

    /// Set window dimensions
    fn set_window_size(&self, width: i32, height: i32) {
        RcoreImpl::__set_window_size(width, height)
    }

    /// Set window opacity [0.0..1.0] (only PLATFORM_DESKTOP)
    fn set_window_opacity(&self, opacity: f32) {
        RcoreImpl::__set_window_opacity(opacity)
    }

    /// Set window focused (only PLATFORM_DESKTOP)
    fn set_window_focused(&self) {
        RcoreImpl::__set_window_focused()
    }

    /// Get native window handle
    fn get_window_handle(&self) -> Result<WindowHandle<'_>, String> {
        RcoreImpl::__get_window_handle()
    }

    /// Get current screen width
    fn get_screen_width(&self) -> i32 {
        RcoreImpl::__get_screen_width()
    }

    /// Get current screen height
    fn get_screen_height(&self) -> i32 {
        RcoreImpl::__get_screen_height()
    }

    /// Get current render width (it considers HiDPI)
    fn get_render_width(&self) -> i32 {
        RcoreImpl::__get_render_width()
    }

    /// Get current render width (it considers HiDPI)
    fn get_render_height(&self) -> i32 {
        RcoreImpl::__get_render_height()
    }

    /// Get number of connected monitors
    fn get_monitor_count(&self) -> i32 {
        RcoreImpl::__get_monitor_count()
    }

    /// Get current connected monitor
    fn get_current_monitor(&self) -> i32 {
        RcoreImpl::__get_current_monitor()
    }

    /// Get specified monitor position
    fn get_monitor_position(&self, monitor: i32) -> Vector2 {
        RcoreImpl::__get_monitor_position(monitor)
    }

    /// Get specified monitor width (current video mode used by monitor)
    fn get_monitor_width(&self, monitor: i32) -> i32 {
        RcoreImpl::__get_monitor_width(monitor)
    }

    /// Get specified monitor height (current video mode used by monitor)
    fn get_monitor_height(&self, monitor: i32) -> i32 {
        RcoreImpl::__get_monitor_height(monitor)
    }

    /// Get specified monitor physical width in millimetres
    fn get_monitor_physical_width(&self, monitor: i32) -> i32 {
        RcoreImpl::__get_monitor_physical_width(monitor)
    }

    /// Get specified monitor physical height in millimetres
    fn get_monitor_physical_height(&self, monitor: i32) -> i32 {
        RcoreImpl::__get_monitor_physical_height(monitor)
    }

    /// Get specified monitor refresh rate
    fn get_monitor_refresh_rate(&self, monitor: i32) -> i32 {
        RcoreImpl::__get_monitor_refresh_rate(monitor)
    }

    /// Get window position XY on monitor
    fn get_window_position(&self) -> Vector2 {
        RcoreImpl::__get_window_position()
    }

    /// Get window scale DPI factor
    fn get_window_scale_dpi(&self) -> Vector2 {
        RcoreImpl::__get_window_scale_dpi()
    }

    /// Get the human-readable, UTF-8 encoded name of the specified monitor
    fn get_monitor_name(&self, monitor: i32) -> Result<String, String> {
        RcoreImpl::__get_monitor_name(monitor)
    }

    /// Set clipboard text content
    fn set_clipboard_text(&self, text: impl Display) {
        RcoreImpl::__set_clipboard_text(text)
    }

    /// Get clipboard text content
    fn get_clipboard_text(&self) -> Result<String, String> {
        RcoreImpl::__get_clipboard_text()
    }

    /// Enable waiting for events on EndDrawing(), no automatic event polling
    fn enable_event_waiting(&self) {
        RcoreImpl::__enable_event_waiting()
    }

    /// Disable waiting for events on EndDrawing(), automatic events polling
    fn disable_event_waiting(&self) {
        RcoreImpl::__disable_event_waiting()
    }

    // Cursor-related methods

    /// Shows cursor
    fn show_cursor(&self) {
        RcoreImpl::__show_cursor()
    }

    /// Hides cursor
    fn hide_cursor(&self) {
        RcoreImpl::__hide_cursor()
    }

    /// Check whether cursor is not visible
    fn is_cursor_hiden(&self) -> bool {
        RcoreImpl::__is_cursor_hiden()
    }

    /// Enables cursor (unlock cursor)
    fn enable_cursor(&self) {
        RcoreImpl::__enable_cursor()
    }

    /// Disables cursor (lock cursor)
    fn disable_cursor(&self) {
        RcoreImpl::__disable_cursor()
    }

    /// Check whether cursor is on the screen
    fn is_cursor_on_screen(&self) -> bool {
        RcoreImpl::__is_cursor_on_screen()
    }

    // Drawing-related methods

    /// Set background color (framebuffer clear color)
    fn clear_background(&self, color: Color) {
        RcoreImpl::__clear_background(color)
    }

    /// Setup canvas (framebuffer) to start drawing
    fn begin_drawing(&self) {
        RcoreImpl::__begin_drawing()
    }

    /// End canvas drawing and swap buffers (double buffering)
    fn end_drawing(&self) {
        RcoreImpl::__end_drawing()
    }

    /// Begin 2D mode with custom camera (2D)
    fn begin_mode_2d(&self, camera: Camera2D) {
        RcoreImpl::__begin_mode_2D(camera)
    }

    /// Ends 2D mode with custom camera
    fn end_mode_2d(&self) {
        RcoreImpl::__end_mode_2D()
    }

    /// Begin 3D mode with custom camera (3D)
    fn begin_mode_3(&self, camera: Camera3D) {
        RcoreImpl::__begin_mode_3D(camera)
    }

    /// Ends 3D mode and returns to default 2D orthographic mode
    fn end_mode_3d(&self) {
        RcoreImpl::__end_mode_3D()
    }

    /// Begin drawing to render texture
    fn begin_texture_mode(&self, target: RenderTexture2D) {
        RcoreImpl::__begin_texture_mode(target)
    }

    /// Ends drawing to render texture
    fn end_texture_mode(&self) {
        RcoreImpl::__end_texture_mode()
    }

    /// Begin custom shader drawing
    fn begin_shader_mode(&self, shader: Shader) {
        RcoreImpl::__begin_shader_mode(shader)
    }

    /// End custom shader drawing (use default shader)
    fn end_shader_mode(&self) {
        RcoreImpl::__end_shader_mode()
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    fn begin_blend_mode(&self, mode: BlendMode) {
        RcoreImpl::__begin_blend_mode(mode)
    }

    /// End blending mode (reset to default: alpha blending)
    fn end_blend_mode(&self) {
        RcoreImpl::__end_blend_mode()
    }

    /// Begin scissor mode (define screen area for following drawing)
    fn begin_scissor_mode(&self, x: i32, y: i32, width: i32, height: i32) {
        RcoreImpl::__begin_scissor_mode(x, y, width, height)
    }

    /// End scissor mode
    fn end_scissor_mode(&self) {
        RcoreImpl::__end_scissor_mode()
    }

    /// Begin stereo rendering (requires VR simulator)
    fn begin_vr_stereo_mode(&self, config: VrStereoConfig) {
        RcoreImpl::__begin_vr_stereo_mode(config)
    }

    /// End stereo rendering (requires VR simulator)
    fn end_vr_stereo_mode(&self) {
        RcoreImpl::__end_vr_stereo_mode()
    }

    // Mode helpers

    /// Run a closure in texture mode
    fn texture_mode<F, R, E>(&self, target: RenderTexture2D, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.begin_texture_mode(target);
        let res = block();
        self.end_texture_mode();
        res
    }

    /// Run a closure in shader mode
    fn shader_mode<F, R, E>(&self, shader: Shader, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.begin_shader_mode(shader);
        let res = block();
        self.end_shader_mode();
        res
    }

    /// Run a closure in blend mode
    fn blend_mode<F, R, E>(&self, mode: BlendMode, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.begin_blend_mode(mode);
        let res = block();
        self.end_blend_mode();
        res
    }

    /// Run a closure in scissor mode
    fn scissor_mode<F, R, E>(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        block: F,
    ) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.begin_scissor_mode(x, y, width, height);
        let res = block();
        self.end_scissor_mode();
        res
    }

    /// Run a closure in VR stereo mode
    fn vr_stereo_mode<F, R, E>(&self, config: VrStereoConfig, block: F) -> Result<R, E>
    where
        F: FnOnce() -> Result<R, E>,
    {
        self.begin_vr_stereo_mode(config);
        let res = block();
        self.end_vr_stereo_mode();
        res
    }

    // VR stereo config methods for VR simulator

    /// Load VR stereo config for VR simulator device parameters
    fn load_vr_stereo_config(&self, device: VrDeviceInfo) -> VrStereoConfig {
        RcoreImpl::__load_vr_stereo_config(device)
    }

    /// Unload VR stereo config
    fn unload_vr_stereo_config(&self, config: VrStereoConfig) {
        RcoreImpl::__unload_vr_stereo_config(config)
    }

    // Shader management methods

    /// Load shader from files and bind default locations
    fn load_shader(
        &self,
        vs_filename: impl Display,
        fs_filename: impl Display,
    ) -> Result<Shader, String> {
        RcoreImpl::__load_shader(vs_filename, fs_filename)
    }

    /// Load shader from code strings and bind default locations
    fn load_shader_from_memory(
        &self,
        vs_code: impl Display,
        fs_code: impl Display,
    ) -> Result<Shader, String> {
        RcoreImpl::__load_shader_from_memory(vs_code, fs_code)
    }

    /// Check whether a shader is ready
    fn is_shader_ready(&self, shader: Shader) -> bool {
        RcoreImpl::__is_shader_ready(shader)
    }

    /// Get shader uniform location
    fn get_shader_location(&self, shader: Shader, name: impl Display) -> i32 {
        RcoreImpl::__get_shader_location(shader, name)
    }

    /// Get shader attribute location
    fn get_shader_location_attrib(
        &self,
        shader: Shader,
        name: impl Display,
    ) -> Result<enums::ShaderLocationIndex, String> {
        RcoreImpl::__get_shader_location_attrib(shader, name)
    }

    /// Set shader uniform value
    fn set_shader_value<T>(
        &self,
        shader: Shader,
        index: i32,
        value: &T,
        tpe: enums::ShaderUniformDataType,
    ) {
        RcoreImpl::__set_shader_value(shader, index, value, tpe)
    }

    /// Set shader uniform value vector
    fn set_shader_value_v<T>(
        &self,
        shader: Shader,
        index: i32,
        value: &[&T],
        tpe: enums::ShaderUniformDataType,
    ) {
        RcoreImpl::__set_shader_value_v(shader, index, value, tpe)
    }

    /// Set shader uniform value (matrix 4x4)
    fn set_shader_value_matrix(&self, shader: Shader, loc: i32, mat: Matrix) {
        RcoreImpl::__set_shader_value_matrix(shader, loc, mat)
    }

    /// Set shader uniform value for texture (sampler2d)
    fn set_shader_value_texture(&self, shader: Shader, index: i32, texture: Texture2D) {
        RcoreImpl::__set_shader_value_texture(shader, index, texture)
    }

    /// Unload shader from GPU memory (VRAM)
    fn unload_shader(&self, shader: Shader) {
        RcoreImpl::__unload_shader(shader)
    }

    // Screen-space-related methods

    /// Get a ray trace from mouse position
    fn get_mouse_ray(&self, mouse_position: Vector2, camera: Camera3D) -> Ray {
        RcoreImpl::__get_mouse_ray(mouse_position, camera)
    }

    /// Get camera 2D transform matrix
    fn get_camera_matrix_2d(&self, camera: Camera2D) -> Matrix {
        RcoreImpl::__get_camera_matrix_2d(camera)
    }

    /// Get camera transform matrix (view matrix)
    fn get_camera_matrix_3d(&self, camera: Camera3D) -> Matrix {
        RcoreImpl::__get_camera_matrix_3d(camera)
    }

    /// Get the screen space position for a 2D camera world space position
    fn get_world_to_screen_2d(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        RcoreImpl::__get_world_to_screen_2d(position, camera)
    }

    /// Get the world space position for a 2D camera screen space position
    fn get_screen_to_world_2d(&self, position: Vector2, camera: Camera2D) -> Vector2 {
        RcoreImpl::__get_screen_to_world_2d(position, camera)
    }

    /// Get the screen space position for a 3D world space position
    fn get_world_to_screen_3d(&self, position: Vector3, camera: Camera3D) -> Vector2 {
        RcoreImpl::__get_world_to_screen_3d(position, camera)
    }

    /// Get size position for a 3D world space position
    fn get_world_to_screen_3d_ex(
        &self,
        position: Vector3,
        camera: Camera3D,
        width: i32,
        height: i32,
    ) -> Vector2 {
        RcoreImpl::__get_world_to_screen_3d_ex(position, camera, width, height)
    }

    // Timing-related methods

    /// Set target FPS (maximum)
    fn set_target_fps(&self, fps: i32) {
        RcoreImpl::__set_target_fps(fps)
    }

    /// Get time in seconds for last frame drawn (delta time)
    fn get_frame_time(&self) -> f32 {
        RcoreImpl::__get_frame_time()
    }

    /// Get elapsed time in seconds since InitWindow()
    fn get_time(&self) -> f64 {
        RcoreImpl::__get_time()
    }

    /// Get current FPS
    fn get_fps(&self) -> i32 {
        RcoreImpl::__get_fps()
    }

    // Custom frame control methods

    /// Swap back buffer with front buffer (screen drawing)
    fn swap_screen_buffer(&self) {
        dbg!("avoid rcore.swap_screen_buffer(), use rcore.end_drawing() instead");
        RcoreImpl::__swap_screen_buffer()
    }

    /// Register all input events
    fn poll_input_events(&self) {
        dbg!("avoid rcore.poll_input_events(), use rcore.end_drawing() instead");
        RcoreImpl::__poll_input_events()
    }

    /// Wait for some time (halt program execution)
    fn wait_time(&self, seconds: f64) {
        dbg!("halting execution", seconds);
        RcoreImpl::__wait_time(seconds)
    }

    // Random values generation methods

    /// Set the seed for the random number generator
    fn set_random_seed(&self, seed: u32) {
        RcoreImpl::__set_random_seed(seed)
    }

    /// Get a random value between min and max (both included)
    fn get_random_value(&self, min: i32, max: i32) -> i32 {
        RcoreImpl::__get_random_value(min, max)
    }

    /// Load random values sequence, no values repeated
    fn load_random_sequence(&self, count: usize, min: i32, max: i32) -> Vec<i32> {
        RcoreImpl::__load_random_sequence(count, min, max).unwrap_or_else(|_| vec![])
    }

    // Misc methods

    /// Takes a screenshot of current screen (filename extension defines format)
    fn take_screenshot(&self, filename: impl Display) {
        RcoreImpl::__take_screenshot(filename)
    }

    /// Setup init configuration flags (use ConfigFlags)
    fn set_config_flags(&self, flags: impl Into<usize>) {
        RcoreImpl::__set_config_flags(flags)
    }

    /// Open URL with default system browser (if available)
    fn open_url(&self, url: impl Display) {
        RcoreImpl::__open_url(url)
    }

    /// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
    fn trace_log(&self, level: TraceLogLevel, text: impl Display) {
        RcoreImpl::__trace_log(level, text)
    }

    /// Set the current threshold (minimum) log level
    fn set_trace_log_level(&self, level: TraceLogLevel) {
        RcoreImpl::__set_trace_log_level(level)
    }

    // Files management methods

    /// Load file data as byte array (read)
    fn load_file_data(&self, filename: impl Display) -> Result<Vec<u8>, String> {
        RcoreImpl::__load_file_data(filename)
    }

    /// Save data to file from byte array (write), returns true on success
    fn save_file_data(&self, filename: impl Display, data: &mut Vec<u8>) -> bool {
        RcoreImpl::__save_file_data(filename, data)
    }

    /// Export data to code (.h), returns true on success
    fn export_data_as_code(&self, data: &str, filename: impl Display) -> bool {
        RcoreImpl::__export_data_as_code(data, filename)
    }

    /// Load text data from file (read)
    fn load_file_text(&self, filename: impl Display) -> Result<String, String> {
        RcoreImpl::__load_file_text(filename)
    }

    /// Save text data to file (write), string must be '\0' terminated, returns true on success
    fn save_file_text(&self, filename: impl Display, text: impl Display) -> bool {
        RcoreImpl::__save_file_text(filename, text)
    }

    // File system methods

    /// Get the directory of the running application
    fn get_application_directory(&self) -> Result<String, String> {
        RcoreImpl::__get_application_directory()
    }

    // Automation events functionality

    /// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
    fn load_automation_event_list(
        &self,
        filename: impl Display,
    ) -> Result<AutomationEventList, String> {
        RcoreImpl::__load_automation_event_list(filename)
    }

    /// Unload automation events list from file
    fn unload_automation_event_list(&self, list: AutomationEventList) {
        RcoreImpl::__unload_automation_event_list(list)
    }

    /// Export automation events list as text file
    fn export_automation_event_list(
        &self,
        list: AutomationEventList,
        filename: impl Display,
    ) -> bool {
        RcoreImpl::__export_automation_event_list(list, filename)
    }

    /// Set automation event list to record to
    fn set_automation_event_list(&self, list: AutomationEventList) {
        RcoreImpl::__set_automation_event_list(list)
    }

    /// Set automation event internal base frame to start recording
    fn set_automation_event_base_frame(&self, frame: i32) {
        RcoreImpl::__set_automation_event_base_frame(frame)
    }

    /// Start recording automation events (AutomationEventList must be set)
    fn start_automation_event_record(&self) {
        RcoreImpl::__start_automation_event_record()
    }

    /// Stop recording automation events
    fn stop_automation_event_record(&self) {
        RcoreImpl::__stop_automation_event_record()
    }

    /// Play a recorded automation event
    fn play_automation_event(&self, event: AutomationEvent) {
        RcoreImpl::__play_automation_event(event)
    }

    // Input-related methods: keyboard

    /// Check whether a key has been pressed once
    fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        RcoreImpl::__is_key_pressed(key)
    }

    /// Check whether a key has been pressed again (Only PLATFORM_DESKTOP)
    fn is_key_pressed_repeat(&self, key: KeyboardKey) -> bool {
        RcoreImpl::__is_key_pressed_repeat(key)
    }

    /// Check whether a key is being pressed
    fn is_key_down(&self, key: KeyboardKey) -> bool {
        RcoreImpl::__is_key_down(key)
    }

    /// Check whether a key has been released once
    fn is_key_released(&self, key: KeyboardKey) -> bool {
        RcoreImpl::__is_key_released(key)
    }

    /// Check whether a key is NOT being pressed
    fn is_key_up(&self, key: KeyboardKey) -> bool {
        RcoreImpl::__is_key_up(key)
    }

    /// Get key pressed, call it multiple times for keys queued, returns KeyboardKey::Null when the queue is empty
    fn get_key_pressed(&self) -> KeyboardKey {
        RcoreImpl::__get_key_pressed()
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns empty when the queue is empty
    fn get_char_pressed(&self) -> String {
        RcoreImpl::__get_char_pressed()
    }

    /// Set a custom key to exit program (default is KeyboardKey::Escape)
    fn set_exit_key(&self, key: KeyboardKey) {
        RcoreImpl::__set_exit_key(key)
    }

    // Input-related methods: gamepads

    /// Check whether a gamepad is available
    fn is_gamepad_available(&self, gamepad: i32) -> bool {
        RcoreImpl::__is_gamepad_available(gamepad)
    }

    /// Get gamepad internal name id
    fn get_gamepad_name(&self, gamepad: i32) -> Result<String, String> {
        RcoreImpl::__get_gamepad_name(gamepad)
    }

    /// Check whether a gamepad button has been pressed once
    fn is_gamepad_button_pressed(&self, gamepad: i32, button: GamepadButton) -> bool {
        RcoreImpl::__is_gamepad_button_pressed(gamepad, button)
    }

    /// Check whether a gamepad button is being pressed
    fn is_gamepad_button_down(&self, gamepad: i32, button: GamepadButton) -> bool {
        RcoreImpl::__is_gamepad_button_down(gamepad, button)
    }

    /// Check whether a gamepad button has been released once
    fn is_gamepad_button_released(&self, gamepad: i32, button: GamepadButton) -> bool {
        RcoreImpl::__is_gamepad_button_released(gamepad, button)
    }

    /// Check whether a gamepad button is NOT being pressed
    fn is_gamepad_button_up(&self, gamepad: i32, button: GamepadButton) -> bool {
        RcoreImpl::__is_gamepad_button_up(gamepad, button)
    }

    /// Get the last gamepad button pressed
    fn get_gamepad_button_pressed(&self) -> GamepadButton {
        RcoreImpl::__get_gamepad_button_pressed()
    }

    /// Get gamepad axis count for a gamepad
    fn get_gamepad_axis_count(&self, gamepad: i32) -> i32 {
        RcoreImpl::__get_gamepad_axis_count(gamepad)
    }

    /// Get axis movement value for a gamepad axis
    fn get_gamepad_axis_movement(&self, gamepad: i32, axis: GamepadAxis) -> f32 {
        RcoreImpl::__get_gamepad_axis_movement(gamepad, axis)
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB)
    fn set_gamepad_mappings(&self, mappings: impl Display) -> i32 {
        RcoreImpl::__set_gamepad_mappings(mappings)
    }

    // Input-related methods: mouse

    /// Check whether a mouse button has been pressed once
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        RcoreImpl::__is_mouse_button_pressed(button)
    }

    /// Check whether a mouse button is being pressed
    fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        RcoreImpl::__is_mouse_button_down(button)
    }

    /// Check whether a mouse button has been released once
    fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        RcoreImpl::__is_mouse_button_released(button)
    }

    /// Check whether a mouse button is NOT being pressed
    fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        RcoreImpl::__is_mouse_button_up(button)
    }

    /// Get mouse position X
    fn get_mouse_x(&self) -> i32 {
        RcoreImpl::__get_mouse_x()
    }

    /// Get mouse position Y
    fn get_mouse_y(&self) -> i32 {
        RcoreImpl::__get_mouse_y()
    }

    // Get mouse position XY
    fn get_mouse_position(&self) -> Vector2 {
        RcoreImpl::__get_mouse_position()
    }

    /// Get mouse delta between frames
    fn get_mouse_delta(&self) -> Vector2 {
        RcoreImpl::__get_mouse_delta()
    }

    /// Set mouse position XY
    fn set_mouse_position(&self, x: i32, y: i32) {
        RcoreImpl::__set_mouse_position(x, y)
    }

    // Set mouse offset
    fn set_mouse_offset(&self, x: i32, y: i32) {
        RcoreImpl::__set_mouse_offset(x, y)
    }

    /// Set mouse scaling
    fn set_mouse_scale(&self, x: f32, y: f32) {
        RcoreImpl::__set_mouse_scale(x, y)
    }

    // Get mouse wheel movement for X or Y, whichever is larger
    fn get_mouse_wheel_move(&self) -> f32 {
        RcoreImpl::__get_mouse_wheel_move()
    }

    /// Get mouse wheel movement for both X and Y
    fn get_mouse_wheel_move_v(&self) -> Vector2 {
        RcoreImpl::__get_mouse_wheel_move_v()
    }

    /// Set mouse cursor
    fn set_mouse_cursor(&self, cursor: MouseCursor) {
        RcoreImpl::__set_mouse_cursor(cursor)
    }

    // Input-related methods: touch

    /// Get touch position X for touch point 0 (relative to screen size)
    fn get_touch_x(&self) -> i32 {
        RcoreImpl::__get_touch_x()
    }

    /// Get touch position Y for touch point 0 (relative to screen size)
    fn get_touch_y(&self) -> i32 {
        RcoreImpl::__get_touch_y()
    }

    /// Get touch position XY for a touch point index (relative to screen size)
    fn get_touch_position(&self, index: i32) -> Vector2 {
        RcoreImpl::__get_touch_position(index)
    }

    /// Get touch point identifier for given index
    fn get_touch_point_id(&self, index: i32) -> i32 {
        RcoreImpl::__get_touch_point_id(index)
    }

    /// Get number of touch points
    fn get_touch_point_count(&self) -> i32 {
        RcoreImpl::__get_touch_point_count()
    }
}
