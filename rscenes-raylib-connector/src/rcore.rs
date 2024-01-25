use crate::WindowHandle;
use eyre::*;
use raylib_ffi::*;
use std::ffi::{c_char, CString};

#[derive(Clone, Copy, Debug)]
pub struct Rcore {}

impl Rcore {
    pub(crate) fn new() -> Self {
        Self {}
    }

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
}
