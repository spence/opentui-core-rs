use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::color::Rgba;
use crate::error::{Error, Result};
use crate::text_buffer::TextBuffer;

/// A viewport into a [`TextBuffer`] with wrapping, selection, and layout.
pub struct TextBufferView {
  ptr: NonNull<sys::UnifiedTextBufferView>,
}

/// Wrap mode for text layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WrapMode {
  None = 0,
  Char = 1,
  Word = 2,
}

impl TextBufferView {
  pub fn new(tb: &TextBuffer) -> Result<Self> {
    // SAFETY: Returns null on failure.
    let ptr = unsafe { sys::createTextBufferView(tb.as_ptr()) };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("text buffer view"))
  }

  pub(crate) fn as_ptr(&self) -> *mut sys::UnifiedTextBufferView {
    self.ptr.as_ptr()
  }

  // --- Selection ---

  pub fn set_selection(&mut self, start: u32, end: u32, bg: Option<&Rgba>, fg: Option<&Rgba>) {
    unsafe {
      sys::textBufferViewSetSelection(
        self.as_ptr(),
        start,
        end,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      );
    }
  }

  pub fn reset_selection(&mut self) {
    unsafe { sys::textBufferViewResetSelection(self.as_ptr()) }
  }

  pub fn selection_info(&self) -> u64 {
    unsafe { sys::textBufferViewGetSelectionInfo(self.as_ptr()) }
  }

  pub fn set_local_selection(
    &mut self,
    anchor_x: i32,
    anchor_y: i32,
    focus_x: i32,
    focus_y: i32,
    bg: Option<&Rgba>,
    fg: Option<&Rgba>,
  ) -> bool {
    unsafe {
      sys::textBufferViewSetLocalSelection(
        self.as_ptr(),
        anchor_x,
        anchor_y,
        focus_x,
        focus_y,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      )
    }
  }

  pub fn update_selection(&mut self, end: u32, bg: Option<&Rgba>, fg: Option<&Rgba>) {
    unsafe {
      sys::textBufferViewUpdateSelection(
        self.as_ptr(),
        end,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      );
    }
  }

  pub fn update_local_selection(
    &mut self,
    anchor_x: i32,
    anchor_y: i32,
    focus_x: i32,
    focus_y: i32,
    bg: Option<&Rgba>,
    fg: Option<&Rgba>,
  ) -> bool {
    unsafe {
      sys::textBufferViewUpdateLocalSelection(
        self.as_ptr(),
        anchor_x,
        anchor_y,
        focus_x,
        focus_y,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      )
    }
  }

  pub fn reset_local_selection(&mut self) {
    unsafe { sys::textBufferViewResetLocalSelection(self.as_ptr()) }
  }

  // --- Layout ---

  pub fn set_wrap_width(&mut self, width: u32) {
    unsafe { sys::textBufferViewSetWrapWidth(self.as_ptr(), width) }
  }

  pub fn set_wrap_mode(&mut self, mode: WrapMode) {
    unsafe { sys::textBufferViewSetWrapMode(self.as_ptr(), mode as u8) }
  }

  pub fn set_viewport_size(&mut self, width: u32, height: u32) {
    unsafe {
      sys::textBufferViewSetViewportSize(self.as_ptr(), width, height);
    }
  }

  pub fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32) {
    unsafe {
      sys::textBufferViewSetViewport(self.as_ptr(), x, y, width, height);
    }
  }

  pub fn virtual_line_count(&self) -> u32 {
    unsafe { sys::textBufferViewGetVirtualLineCount(self.as_ptr()) }
  }

  // --- Line info ---

  pub fn line_info(&self) -> LineInfo {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLineInfo>::uninit();
    unsafe {
      sys::textBufferViewGetLineInfoDirect(self.as_ptr(), out.as_mut_ptr());
      LineInfo::from_raw(out.assume_init())
    }
  }

  pub fn logical_line_info(&self) -> LineInfo {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLineInfo>::uninit();
    unsafe {
      sys::textBufferViewGetLogicalLineInfoDirect(self.as_ptr(), out.as_mut_ptr());
      LineInfo::from_raw(out.assume_init())
    }
  }

  // --- Text extraction ---

  pub fn selected_text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len =
      unsafe { sys::textBufferViewGetSelectedText(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  pub fn plain_text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len =
      unsafe { sys::textBufferViewGetPlainText(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  // --- Display ---

  pub fn set_tab_indicator(&mut self, indicator: u32) {
    unsafe { sys::textBufferViewSetTabIndicator(self.as_ptr(), indicator) }
  }

  pub fn set_tab_indicator_color(&mut self, color: &Rgba) {
    unsafe {
      sys::textBufferViewSetTabIndicatorColor(self.as_ptr(), color.as_ptr());
    }
  }

  pub fn set_truncate(&mut self, truncate: bool) {
    unsafe { sys::textBufferViewSetTruncate(self.as_ptr(), truncate) }
  }

  // --- Measurement ---

  pub fn measure_for_dimensions(&self, width: u32, height: u32) -> Option<MeasureResult> {
    let mut out = std::mem::MaybeUninit::<sys::ExternalMeasureResult>::uninit();
    let ok = unsafe {
      sys::textBufferViewMeasureForDimensions(self.as_ptr(), width, height, out.as_mut_ptr())
    };
    if ok {
      let raw = unsafe { out.assume_init() };
      Some(MeasureResult {
        line_count: raw.line_count,
        width_cols_max: raw.width_cols_max,
      })
    } else {
      None
    }
  }
}

impl Drop for TextBufferView {
  fn drop(&mut self) {
    unsafe { sys::destroyTextBufferView(self.ptr.as_ptr()) }
  }
}

unsafe impl Send for TextBufferView {}

/// Line layout information from the view.
///
/// The slices point into Zig-managed memory and are only valid until the next
/// layout-affecting operation on the view.
#[derive(Debug)]
pub struct LineInfo {
  pub width_cols_max: u32,
  start_cols: Vec<u32>,
  width_cols: Vec<u32>,
  sources: Vec<u32>,
  wraps: Vec<u32>,
}

impl LineInfo {
  pub(crate) unsafe fn from_raw(raw: sys::ExternalLineInfo) -> Self {
    let start_cols = if raw.start_cols_len > 0 {
      std::slice::from_raw_parts(raw.start_cols_ptr, raw.start_cols_len as usize).to_vec()
    } else {
      Vec::new()
    };
    let width_cols = if raw.width_cols_len > 0 {
      std::slice::from_raw_parts(raw.width_cols_ptr, raw.width_cols_len as usize).to_vec()
    } else {
      Vec::new()
    };
    let sources = if raw.sources_len > 0 {
      std::slice::from_raw_parts(raw.sources_ptr, raw.sources_len as usize).to_vec()
    } else {
      Vec::new()
    };
    let wraps = if raw.wraps_len > 0 {
      std::slice::from_raw_parts(raw.wraps_ptr, raw.wraps_len as usize).to_vec()
    } else {
      Vec::new()
    };
    Self {
      width_cols_max: raw.width_cols_max,
      start_cols,
      width_cols,
      sources,
      wraps,
    }
  }

  pub fn start_cols(&self) -> &[u32] {
    &self.start_cols
  }

  pub fn width_cols(&self) -> &[u32] {
    &self.width_cols
  }

  pub fn sources(&self) -> &[u32] {
    &self.sources
  }

  pub fn wraps(&self) -> &[u32] {
    &self.wraps
  }
}

#[derive(Debug, Clone, Copy)]
pub struct MeasureResult {
  pub line_count: u32,
  pub width_cols_max: u32,
}
