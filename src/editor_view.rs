use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::color::Rgba;
use crate::edit_buffer::EditBuffer;
use crate::error::{Error, Result};
use crate::text_buffer::StyledChunk;
use crate::text_buffer_view::{LineInfo, WrapMode};

/// A visual editor view over an [`EditBuffer`].
///
/// Provides viewport management, visual cursor, scroll margins, selection, and
/// line info for rendering an editor-like UI.
pub struct EditorView {
  ptr: NonNull<sys::EditorView>,
}

/// Visual cursor position, combining logical and visual coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualCursor {
  pub visual_row: u32,
  pub visual_col: u32,
  pub logical_row: u32,
  pub logical_col: u32,
  pub offset: u32,
}

impl EditorView {
  pub fn new(edit_buffer: &EditBuffer, viewport_width: u32, viewport_height: u32) -> Result<Self> {
    let ptr =
      unsafe { sys::createEditorView(edit_buffer.as_ptr(), viewport_width, viewport_height) };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("editor view"))
  }

  pub(crate) fn as_ptr(&self) -> *mut sys::EditorView {
    self.ptr.as_ptr()
  }

  // --- Viewport ---

  pub fn set_viewport(&mut self, x: u32, y: u32, width: u32, height: u32, move_cursor: bool) {
    unsafe {
      sys::editorViewSetViewport(self.as_ptr(), x, y, width, height, move_cursor);
    }
  }

  pub fn clear_viewport(&mut self) {
    unsafe { sys::editorViewClearViewport(self.as_ptr()) }
  }

  pub fn viewport(&self) -> Option<(u32, u32, u32, u32)> {
    let (mut x, mut y, mut w, mut h) = (0u32, 0u32, 0u32, 0u32);
    let ok = unsafe { sys::editorViewGetViewport(self.as_ptr(), &mut x, &mut y, &mut w, &mut h) };
    if ok {
      Some((x, y, w, h))
    } else {
      None
    }
  }

  pub fn set_viewport_size(&mut self, width: u32, height: u32) {
    unsafe { sys::editorViewSetViewportSize(self.as_ptr(), width, height) }
  }

  pub fn set_scroll_margin(&mut self, margin: f32) {
    unsafe { sys::editorViewSetScrollMargin(self.as_ptr(), margin) }
  }

  // --- Line info ---

  pub fn virtual_line_count(&self) -> u32 {
    unsafe { sys::editorViewGetVirtualLineCount(self.as_ptr()) }
  }

  pub fn total_virtual_line_count(&self) -> u32 {
    unsafe { sys::editorViewGetTotalVirtualLineCount(self.as_ptr()) }
  }

  pub fn line_info(&self) -> LineInfo {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLineInfo>::uninit();
    unsafe {
      sys::editorViewGetLineInfoDirect(self.as_ptr(), out.as_mut_ptr());
      LineInfo::from_raw(out.assume_init())
    }
  }

  pub fn logical_line_info(&self) -> LineInfo {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLineInfo>::uninit();
    unsafe {
      sys::editorViewGetLogicalLineInfoDirect(self.as_ptr(), out.as_mut_ptr());
      LineInfo::from_raw(out.assume_init())
    }
  }

  // --- Display ---

  pub fn set_wrap_mode(&mut self, mode: WrapMode) {
    unsafe { sys::editorViewSetWrapMode(self.as_ptr(), mode as u8) }
  }

  pub fn set_tab_indicator(&mut self, indicator: u32) {
    unsafe { sys::editorViewSetTabIndicator(self.as_ptr(), indicator) }
  }

  pub fn set_tab_indicator_color(&mut self, color: &Rgba) {
    unsafe {
      sys::editorViewSetTabIndicatorColor(self.as_ptr(), color.as_ptr());
    }
  }

  pub fn set_placeholder_styled_text(&mut self, chunks: &[StyledChunk<'_>]) {
    let raw: Vec<sys::StyledChunk> = chunks.iter().map(|c| c.to_raw()).collect();
    unsafe {
      sys::editorViewSetPlaceholderStyledText(self.as_ptr(), raw.as_ptr(), raw.len());
    }
  }

  // --- Selection ---

  pub fn set_selection(&mut self, start: u32, end: u32, bg: Option<&Rgba>, fg: Option<&Rgba>) {
    unsafe {
      sys::editorViewSetSelection(
        self.as_ptr(),
        start,
        end,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      );
    }
  }

  pub fn reset_selection(&mut self) {
    unsafe { sys::editorViewResetSelection(self.as_ptr()) }
  }

  pub fn selection(&self) -> u64 {
    unsafe { sys::editorViewGetSelection(self.as_ptr()) }
  }

  pub fn set_local_selection(
    &mut self,
    anchor_x: i32,
    anchor_y: i32,
    focus_x: i32,
    focus_y: i32,
    bg: Option<&Rgba>,
    fg: Option<&Rgba>,
    update_cursor: bool,
    follow_cursor: bool,
  ) -> bool {
    unsafe {
      sys::editorViewSetLocalSelection(
        self.as_ptr(),
        anchor_x,
        anchor_y,
        focus_x,
        focus_y,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        update_cursor,
        follow_cursor,
      )
    }
  }

  pub fn update_selection(&mut self, end: u32, bg: Option<&Rgba>, fg: Option<&Rgba>) {
    unsafe {
      sys::editorViewUpdateSelection(
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
    update_cursor: bool,
    follow_cursor: bool,
  ) -> bool {
    unsafe {
      sys::editorViewUpdateLocalSelection(
        self.as_ptr(),
        anchor_x,
        anchor_y,
        focus_x,
        focus_y,
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        update_cursor,
        follow_cursor,
      )
    }
  }

  pub fn reset_local_selection(&mut self) {
    unsafe { sys::editorViewResetLocalSelection(self.as_ptr()) }
  }

  pub fn selected_text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len =
      unsafe { sys::editorViewGetSelectedTextBytes(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  // --- Cursor ---

  pub fn cursor(&self) -> (u32, u32) {
    let mut row = 0u32;
    let mut col = 0u32;
    unsafe { sys::editorViewGetCursor(self.as_ptr(), &mut row, &mut col) };
    (row, col)
  }

  pub fn visual_cursor(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetVisualCursor(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  pub fn move_up_visual(&mut self) {
    unsafe { sys::editorViewMoveUpVisual(self.as_ptr()) }
  }

  pub fn move_down_visual(&mut self) {
    unsafe { sys::editorViewMoveDownVisual(self.as_ptr()) }
  }

  pub fn set_cursor_by_offset(&mut self, offset: u32) {
    unsafe { sys::editorViewSetCursorByOffset(self.as_ptr(), offset) }
  }

  // --- Word/line navigation ---

  pub fn next_word_boundary(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetNextWordBoundary(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  pub fn prev_word_boundary(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetPrevWordBoundary(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  pub fn end_of_line(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetEOL(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  pub fn visual_start_of_line(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetVisualSOL(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  pub fn visual_end_of_line(&self) -> VisualCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalVisualCursor>::uninit();
    unsafe {
      sys::editorViewGetVisualEOL(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      VisualCursor {
        visual_row: raw.visual_row,
        visual_col: raw.visual_col,
        logical_row: raw.logical_row,
        logical_col: raw.logical_col,
        offset: raw.offset,
      }
    }
  }

  // --- Text operations ---

  pub fn delete_selected_text(&mut self) {
    unsafe { sys::editorViewDeleteSelectedText(self.as_ptr()) }
  }

  pub fn text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe { sys::editorViewGetText(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }
}

impl Drop for EditorView {
  fn drop(&mut self) {
    unsafe { sys::destroyEditorView(self.ptr.as_ptr()) }
  }
}

unsafe impl Send for EditorView {}
