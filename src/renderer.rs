use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::buffer::BufferRef;
use crate::color::Rgba;
use crate::error::{Error, Result};

/// The core terminal renderer.
///
/// Manages double-buffered rendering with automatic diff detection. Only changed
/// cells generate ANSI output per frame.
///
/// # Examples
///
/// ```no_run
/// use opentui_core::{Renderer, Rgba};
///
/// let mut renderer = Renderer::new(80, 24).unwrap();
/// let buf = renderer.next_buffer();
/// buf.clear(&Rgba::BLACK);
/// buf.draw_text("Hello, opentui!", 0, 0, &Rgba::WHITE, None, 0);
/// renderer.render(false);
/// ```
pub struct Renderer {
  ptr: NonNull<sys::CliRenderer>,
}

impl Renderer {
  /// Create a new renderer with the given terminal dimensions.
  pub fn new(width: u32, height: u32) -> Result<Self> {
    Self::with_options(width, height, false, false)
  }

  /// Create a renderer with testing/remote options.
  pub fn with_options(width: u32, height: u32, testing: bool, remote: bool) -> Result<Self> {
    // SAFETY: createRenderer returns null on invalid dimensions or OOM.
    let ptr = unsafe { sys::createRenderer(width, height, testing, remote) };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("renderer"))
  }

  /// Get the back buffer to draw into for the next frame.
  pub fn next_buffer(&mut self) -> BufferRef<'_> {
    // SAFETY: Returns a pointer to an internally-managed buffer.
    let ptr = unsafe { sys::getNextBuffer(self.ptr.as_ptr()) };
    BufferRef::from_raw(NonNull::new(ptr).expect("getNextBuffer returned null"))
  }

  /// Get the current (front) buffer.
  pub fn current_buffer(&self) -> BufferRef<'_> {
    // SAFETY: Returns a pointer to an internally-managed buffer.
    let ptr = unsafe { sys::getCurrentBuffer(self.ptr.as_ptr()) };
    BufferRef::from_raw(NonNull::new(ptr).expect("getCurrentBuffer returned null"))
  }

  /// Render the current frame, diffing against the previous frame.
  ///
  /// If `force` is true, redraws all cells regardless of changes.
  pub fn render(&mut self, force: bool) {
    // SAFETY: Renderer pointer is valid for the lifetime of Self.
    unsafe { sys::render(self.ptr.as_ptr(), force) }
  }

  /// Resize the renderer to new terminal dimensions.
  pub fn resize(&mut self, width: u32, height: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::resizeRenderer(self.ptr.as_ptr(), width, height) }
  }

  /// Set the background color used for clearing.
  pub fn set_background_color(&mut self, color: &Rgba) {
    // SAFETY: Color pointer is valid for the call.
    unsafe { sys::setBackgroundColor(self.ptr.as_ptr(), color.as_ptr()) }
  }

  /// Set the render offset (vertical scroll offset).
  pub fn set_render_offset(&mut self, offset: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setRenderOffset(self.ptr.as_ptr(), offset) }
  }

  /// Enable or disable threaded rendering.
  pub fn set_use_thread(&mut self, use_thread: bool) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setUseThread(self.ptr.as_ptr(), use_thread) }
  }

  /// Set a terminal environment variable for capability detection.
  pub fn set_terminal_env_var(&mut self, key: &str, value: &str) -> bool {
    // SAFETY: Pointers are valid for the call duration.
    unsafe {
      sys::setTerminalEnvVar(
        self.ptr.as_ptr(),
        key.as_ptr(),
        key.len(),
        value.as_ptr(),
        value.len(),
      )
    }
  }

  /// Update frame timing statistics.
  pub fn update_stats(&mut self, time: f64, fps: u32, frame_callback_time: f64) {
    // SAFETY: Renderer pointer is valid.
    unsafe {
      sys::updateStats(self.ptr.as_ptr(), time, fps, frame_callback_time);
    }
  }

  /// Update memory usage statistics.
  pub fn update_memory_stats(&mut self, heap_used: u32, heap_total: u32, array_buffers: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe {
      sys::updateMemoryStats(self.ptr.as_ptr(), heap_used, heap_total, array_buffers);
    }
  }

  /// Get the last rendered output bytes (testing only).
  pub fn last_output_for_test(&self) -> &[u8] {
    let mut out = sys::OutputSlice {
      ptr: std::ptr::null(),
      len: 0,
    };
    // SAFETY: Writes to a stack-allocated struct. The returned slice is valid
    // until the next render call.
    unsafe {
      sys::getLastOutputForTest(self.ptr.as_ptr(), &mut out);
      if out.ptr.is_null() || out.len == 0 {
        &[]
      } else {
        std::slice::from_raw_parts(out.ptr, out.len)
      }
    }
  }

  // --- Terminal capabilities ---

  /// Query terminal capabilities.
  pub fn terminal_capabilities(&self) -> Capabilities {
    let mut out = std::mem::MaybeUninit::<sys::ExternalCapabilities>::uninit();
    // SAFETY: Writes to stack-allocated struct.
    let raw = unsafe {
      sys::getTerminalCapabilities(self.ptr.as_ptr(), out.as_mut_ptr());
      out.assume_init()
    };
    Capabilities::from_raw(&raw)
  }

  /// Process a terminal capability response (e.g. from DA1/DA2 queries).
  pub fn process_capability_response(&mut self, response: &[u8]) {
    // SAFETY: Pointer is valid for the call.
    unsafe {
      sys::processCapabilityResponse(self.ptr.as_ptr(), response.as_ptr(), response.len());
    }
  }

  /// Enable or disable hyperlink capability.
  pub fn set_hyperlinks_capability(&mut self, enabled: bool) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setHyperlinksCapability(self.ptr.as_ptr(), enabled) }
  }

  // --- Cursor ---

  /// Set the cursor position and visibility.
  pub fn set_cursor_position(&mut self, x: i32, y: i32, visible: bool) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setCursorPosition(self.ptr.as_ptr(), x, y, visible) }
  }

  /// Set the cursor color.
  pub fn set_cursor_color(&mut self, color: &Rgba) {
    // SAFETY: Pointers are valid.
    unsafe { sys::setCursorColor(self.ptr.as_ptr(), color.as_ptr()) }
  }

  /// Set cursor style options.
  pub fn set_cursor_style(&mut self, options: &CursorStyleOptions) {
    let raw = sys::CursorStyleOptions {
      style: options.style as u8,
      blinking: options.blinking as u8,
      color: options
        .color
        .as_ref()
        .map(|c| c.as_ptr())
        .unwrap_or(std::ptr::null()),
      cursor: options.mouse_cursor,
    };
    // SAFETY: Pointers valid for the call.
    unsafe { sys::setCursorStyleOptions(self.ptr.as_ptr(), &raw) }
  }

  /// Get the current cursor state.
  pub fn cursor_state(&self) -> CursorState {
    let mut out = std::mem::MaybeUninit::<sys::ExternalCursorState>::uninit();
    // SAFETY: Writes to stack-allocated struct.
    let raw = unsafe {
      sys::getCursorState(self.ptr.as_ptr(), out.as_mut_ptr());
      out.assume_init()
    };
    CursorState {
      x: raw.x,
      y: raw.y,
      visible: raw.visible,
      style: CursorStyle::from_u8(raw.style),
      blinking: raw.blinking,
      color: Rgba::new(raw.r, raw.g, raw.b, raw.a),
    }
  }

  // --- Terminal control ---

  pub fn clear_terminal(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::clearTerminal(self.ptr.as_ptr()) }
  }

  pub fn set_terminal_title(&mut self, title: &str) {
    // SAFETY: Pointer valid for the call.
    unsafe {
      sys::setTerminalTitle(self.ptr.as_ptr(), title.as_ptr(), title.len());
    }
  }

  pub fn copy_to_clipboard_osc52(&mut self, target: u8, payload: &[u8]) -> bool {
    // SAFETY: Pointer valid for the call.
    unsafe { sys::copyToClipboardOSC52(self.ptr.as_ptr(), target, payload.as_ptr(), payload.len()) }
  }

  pub fn clear_clipboard_osc52(&mut self, target: u8) -> bool {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::clearClipboardOSC52(self.ptr.as_ptr(), target) }
  }

  pub fn restore_terminal_modes(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::restoreTerminalModes(self.ptr.as_ptr()) }
  }

  pub fn setup_terminal(&mut self, use_alternate_screen: bool) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setupTerminal(self.ptr.as_ptr(), use_alternate_screen) }
  }

  pub fn suspend(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::suspendRenderer(self.ptr.as_ptr()) }
  }

  pub fn resume(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::resumeRenderer(self.ptr.as_ptr()) }
  }

  // --- Debug ---

  pub fn set_debug_overlay(&mut self, enabled: bool, corner: DebugCorner) {
    // SAFETY: Renderer pointer is valid.
    unsafe {
      sys::setDebugOverlay(self.ptr.as_ptr(), enabled, corner as u8);
    }
  }

  pub fn dump_buffers(&self, timestamp: i64) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::dumpBuffers(self.ptr.as_ptr(), timestamp) }
  }

  pub fn dump_stdout_buffer(&self, timestamp: i64) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::dumpStdoutBuffer(self.ptr.as_ptr(), timestamp) }
  }

  // --- Mouse ---

  pub fn enable_mouse(&mut self, enable_movement: bool) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::enableMouse(self.ptr.as_ptr(), enable_movement) }
  }

  pub fn disable_mouse(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::disableMouse(self.ptr.as_ptr()) }
  }

  // --- Keyboard ---

  pub fn enable_kitty_keyboard(&mut self, flags: u8) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::enableKittyKeyboard(self.ptr.as_ptr(), flags) }
  }

  pub fn disable_kitty_keyboard(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::disableKittyKeyboard(self.ptr.as_ptr()) }
  }

  pub fn set_kitty_keyboard_flags(&mut self, flags: u8) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::setKittyKeyboardFlags(self.ptr.as_ptr(), flags) }
  }

  pub fn kitty_keyboard_flags(&self) -> u8 {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::getKittyKeyboardFlags(self.ptr.as_ptr()) }
  }

  // --- Output ---

  /// Write raw bytes to the terminal output.
  pub fn write_out(&mut self, data: &[u8]) {
    // SAFETY: Pointer valid for the call.
    unsafe { sys::writeOut(self.ptr.as_ptr(), data.as_ptr(), data.len()) }
  }

  /// Query the terminal for pixel resolution.
  pub fn query_pixel_resolution(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::queryPixelResolution(self.ptr.as_ptr()) }
  }

  // --- Hit grid ---

  pub fn add_to_hit_grid(&mut self, x: i32, y: i32, width: u32, height: u32, id: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::addToHitGrid(self.ptr.as_ptr(), x, y, width, height, id) }
  }

  pub fn clear_hit_grid(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::clearCurrentHitGrid(self.ptr.as_ptr()) }
  }

  pub fn hit_grid_push_scissor(&mut self, x: i32, y: i32, width: u32, height: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe {
      sys::hitGridPushScissorRect(self.ptr.as_ptr(), x, y, width, height);
    }
  }

  pub fn hit_grid_pop_scissor(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::hitGridPopScissorRect(self.ptr.as_ptr()) }
  }

  pub fn hit_grid_clear_scissors(&mut self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::hitGridClearScissorRects(self.ptr.as_ptr()) }
  }

  pub fn add_to_hit_grid_clipped(&mut self, x: i32, y: i32, width: u32, height: u32, id: u32) {
    // SAFETY: Renderer pointer is valid.
    unsafe {
      sys::addToCurrentHitGridClipped(self.ptr.as_ptr(), x, y, width, height, id);
    }
  }

  pub fn check_hit(&self, x: u32, y: u32) -> u32 {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::checkHit(self.ptr.as_ptr(), x, y) }
  }

  pub fn hit_grid_dirty(&self) -> bool {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::getHitGridDirty(self.ptr.as_ptr()) }
  }

  pub fn dump_hit_grid(&self) {
    // SAFETY: Renderer pointer is valid.
    unsafe { sys::dumpHitGrid(self.ptr.as_ptr()) }
  }
}

impl Drop for Renderer {
  fn drop(&mut self) {
    // SAFETY: We own the renderer and it hasn't been destroyed yet.
    unsafe { sys::destroyRenderer(self.ptr.as_ptr()) }
  }
}

// SAFETY: The Zig core synchronizes internally when threaded rendering is on.
unsafe impl Send for Renderer {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CursorStyle {
  Block = 0,
  Line = 1,
  Underline = 2,
  Default = 3,
}

impl CursorStyle {
  fn from_u8(v: u8) -> Self {
    match v {
      0 => Self::Block,
      1 => Self::Line,
      2 => Self::Underline,
      _ => Self::Default,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DebugCorner {
  TopLeft = 0,
  TopRight = 1,
  BottomLeft = 2,
  BottomRight = 3,
}

#[derive(Debug, Clone)]
pub struct CursorStyleOptions {
  pub style: CursorStyle,
  pub blinking: bool,
  pub color: Option<Rgba>,
  /// Mouse pointer style (0-5). Values > 5 are ignored.
  pub mouse_cursor: u8,
}

#[derive(Debug, Clone)]
pub struct CursorState {
  pub x: u32,
  pub y: u32,
  pub visible: bool,
  pub style: CursorStyle,
  pub blinking: bool,
  pub color: Rgba,
}

/// Detected terminal capabilities.
#[derive(Debug, Clone)]
pub struct Capabilities {
  pub kitty_keyboard: bool,
  pub kitty_graphics: bool,
  pub rgb: bool,
  pub unicode: WidthMethod,
  pub sgr_pixels: bool,
  pub color_scheme_updates: bool,
  pub explicit_width: bool,
  pub scaled_text: bool,
  pub sixel: bool,
  pub focus_tracking: bool,
  pub sync: bool,
  pub bracketed_paste: bool,
  pub hyperlinks: bool,
  pub osc52: bool,
  pub explicit_cursor_positioning: bool,
  pub term_name: String,
  pub term_version: String,
  pub term_from_xtversion: bool,
}

impl Capabilities {
  fn from_raw(raw: &sys::ExternalCapabilities) -> Self {
    // SAFETY: The pointers are valid for the duration of this conversion.
    let term_name = if raw.term_name_len > 0 && !raw.term_name_ptr.is_null() {
      unsafe {
        let bytes = std::slice::from_raw_parts(raw.term_name_ptr, raw.term_name_len);
        String::from_utf8_lossy(bytes).into_owned()
      }
    } else {
      String::new()
    };

    let term_version = if raw.term_version_len > 0 && !raw.term_version_ptr.is_null() {
      unsafe {
        let bytes = std::slice::from_raw_parts(raw.term_version_ptr, raw.term_version_len);
        String::from_utf8_lossy(bytes).into_owned()
      }
    } else {
      String::new()
    };

    Self {
      kitty_keyboard: raw.kitty_keyboard,
      kitty_graphics: raw.kitty_graphics,
      rgb: raw.rgb,
      unicode: if raw.unicode == 0 {
        WidthMethod::Wcwidth
      } else {
        WidthMethod::Unicode
      },
      sgr_pixels: raw.sgr_pixels,
      color_scheme_updates: raw.color_scheme_updates,
      explicit_width: raw.explicit_width,
      scaled_text: raw.scaled_text,
      sixel: raw.sixel,
      focus_tracking: raw.focus_tracking,
      sync: raw.sync,
      bracketed_paste: raw.bracketed_paste,
      hyperlinks: raw.hyperlinks,
      osc52: raw.osc52,
      explicit_cursor_positioning: raw.explicit_cursor_positioning,
      term_name,
      term_version,
      term_from_xtversion: raw.term_from_xtversion,
    }
  }
}

/// Unicode width calculation method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WidthMethod {
  Wcwidth = 0,
  Unicode = 1,
}
