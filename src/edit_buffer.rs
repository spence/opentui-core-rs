use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::error::{Error, Result};
use crate::renderer::WidthMethod;

/// An editable text buffer with cursor, undo/redo, and word navigation.
///
/// Wraps a [`TextBuffer`](crate::TextBuffer) with editing operations. The
/// underlying text buffer can be accessed for read-only operations.
pub struct EditBuffer {
  ptr: NonNull<sys::EditBuffer>,
}

/// Logical cursor position in document coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalCursor {
  pub row: u32,
  pub col: u32,
  pub offset: u32,
}

impl EditBuffer {
  pub fn new(width_method: WidthMethod) -> Result<Self> {
    let ptr = unsafe { sys::createEditBuffer(width_method as u8) };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("edit buffer"))
  }

  pub(crate) fn as_ptr(&self) -> *mut sys::EditBuffer {
    self.ptr.as_ptr()
  }

  /// Get the underlying text buffer pointer.
  ///
  /// The returned pointer is owned by this EditBuffer and must not be freed.
  #[allow(dead_code)]
  pub(crate) fn text_buffer_ptr(&self) -> *mut sys::UnifiedTextBuffer {
    unsafe { sys::editBufferGetTextBuffer(self.as_ptr()) }
  }

  // --- Editing ---

  pub fn insert_text(&mut self, text: &str) {
    unsafe {
      sys::editBufferInsertText(self.as_ptr(), text.as_ptr(), text.len());
    }
  }

  pub fn delete_range(&mut self, start_row: u32, start_col: u32, end_row: u32, end_col: u32) {
    unsafe {
      sys::editBufferDeleteRange(self.as_ptr(), start_row, start_col, end_row, end_col);
    }
  }

  pub fn delete_char_backward(&mut self) {
    unsafe { sys::editBufferDeleteCharBackward(self.as_ptr()) }
  }

  pub fn delete_char(&mut self) {
    unsafe { sys::editBufferDeleteChar(self.as_ptr()) }
  }

  pub fn insert_char(&mut self, ch: &str) {
    unsafe {
      sys::editBufferInsertChar(self.as_ptr(), ch.as_ptr(), ch.len());
    }
  }

  pub fn new_line(&mut self) {
    unsafe { sys::editBufferNewLine(self.as_ptr()) }
  }

  pub fn delete_line(&mut self) {
    unsafe { sys::editBufferDeleteLine(self.as_ptr()) }
  }

  pub fn set_text(&mut self, text: &str) {
    unsafe {
      sys::editBufferSetText(self.as_ptr(), text.as_ptr(), text.len());
    }
  }

  pub fn set_text_from_mem(&mut self, mem_id: u8) {
    unsafe { sys::editBufferSetTextFromMem(self.as_ptr(), mem_id) }
  }

  pub fn replace_text(&mut self, text: &str) {
    unsafe {
      sys::editBufferReplaceText(self.as_ptr(), text.as_ptr(), text.len());
    }
  }

  pub fn replace_text_from_mem(&mut self, mem_id: u8) {
    unsafe { sys::editBufferReplaceTextFromMem(self.as_ptr(), mem_id) }
  }

  pub fn clear(&mut self) {
    unsafe { sys::editBufferClear(self.as_ptr()) }
  }

  // --- Cursor movement ---

  pub fn move_cursor_left(&mut self) {
    unsafe { sys::editBufferMoveCursorLeft(self.as_ptr()) }
  }

  pub fn move_cursor_right(&mut self) {
    unsafe { sys::editBufferMoveCursorRight(self.as_ptr()) }
  }

  pub fn move_cursor_up(&mut self) {
    unsafe { sys::editBufferMoveCursorUp(self.as_ptr()) }
  }

  pub fn move_cursor_down(&mut self) {
    unsafe { sys::editBufferMoveCursorDown(self.as_ptr()) }
  }

  // --- Cursor position ---

  pub fn cursor(&self) -> (u32, u32) {
    let mut row = 0u32;
    let mut col = 0u32;
    unsafe { sys::editBufferGetCursor(self.as_ptr(), &mut row, &mut col) };
    (row, col)
  }

  pub fn set_cursor(&mut self, row: u32, col: u32) {
    unsafe { sys::editBufferSetCursor(self.as_ptr(), row, col) }
  }

  pub fn set_cursor_to_line_col(&mut self, row: u32, col: u32) {
    unsafe { sys::editBufferSetCursorToLineCol(self.as_ptr(), row, col) }
  }

  pub fn set_cursor_by_offset(&mut self, offset: u32) {
    unsafe { sys::editBufferSetCursorByOffset(self.as_ptr(), offset) }
  }

  pub fn goto_line(&mut self, line: u32) {
    unsafe { sys::editBufferGotoLine(self.as_ptr(), line) }
  }

  pub fn cursor_position(&self) -> LogicalCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLogicalCursor>::uninit();
    unsafe {
      sys::editBufferGetCursorPosition(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      LogicalCursor {
        row: raw.row,
        col: raw.col,
        offset: raw.offset,
      }
    }
  }

  // --- Word/line navigation ---

  pub fn next_word_boundary(&self) -> LogicalCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLogicalCursor>::uninit();
    unsafe {
      sys::editBufferGetNextWordBoundary(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      LogicalCursor {
        row: raw.row,
        col: raw.col,
        offset: raw.offset,
      }
    }
  }

  pub fn prev_word_boundary(&self) -> LogicalCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLogicalCursor>::uninit();
    unsafe {
      sys::editBufferGetPrevWordBoundary(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      LogicalCursor {
        row: raw.row,
        col: raw.col,
        offset: raw.offset,
      }
    }
  }

  pub fn end_of_line(&self) -> LogicalCursor {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLogicalCursor>::uninit();
    unsafe {
      sys::editBufferGetEOL(self.as_ptr(), out.as_mut_ptr());
      let raw = out.assume_init();
      LogicalCursor {
        row: raw.row,
        col: raw.col,
        offset: raw.offset,
      }
    }
  }

  // --- Offset conversion ---

  pub fn offset_to_position(&self, offset: u32) -> Option<LogicalCursor> {
    let mut out = std::mem::MaybeUninit::<sys::ExternalLogicalCursor>::uninit();
    let ok = unsafe { sys::editBufferOffsetToPosition(self.as_ptr(), offset, out.as_mut_ptr()) };
    if ok {
      let raw = unsafe { out.assume_init() };
      Some(LogicalCursor {
        row: raw.row,
        col: raw.col,
        offset: raw.offset,
      })
    } else {
      None
    }
  }

  pub fn position_to_offset(&self, row: u32, col: u32) -> u32 {
    unsafe { sys::editBufferPositionToOffset(self.as_ptr(), row, col) }
  }

  pub fn line_start_offset(&self, row: u32) -> u32 {
    unsafe { sys::editBufferGetLineStartOffset(self.as_ptr(), row) }
  }

  // --- Text extraction ---

  pub fn text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe { sys::editBufferGetText(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  pub fn text_range(&self, start_offset: u32, end_offset: u32, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe {
      sys::editBufferGetTextRange(
        self.as_ptr(),
        start_offset,
        end_offset,
        buf.as_mut_ptr(),
        buf.len(),
      )
    };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  pub fn text_range_by_coords(
    &self,
    start_row: u32,
    start_col: u32,
    end_row: u32,
    end_col: u32,
    max_len: usize,
  ) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe {
      sys::editBufferGetTextRangeByCoords(
        self.as_ptr(),
        start_row,
        start_col,
        end_row,
        end_col,
        buf.as_mut_ptr(),
        buf.len(),
      )
    };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  // --- Undo/redo ---

  pub fn undo(&mut self, max_len: usize) -> Option<String> {
    let mut buf = vec![0u8; max_len];
    let len = unsafe { sys::editBufferUndo(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    if len == 0 {
      None
    } else {
      buf.truncate(len);
      Some(String::from_utf8_lossy(&buf).into_owned())
    }
  }

  pub fn redo(&mut self, max_len: usize) -> Option<String> {
    let mut buf = vec![0u8; max_len];
    let len = unsafe { sys::editBufferRedo(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    if len == 0 {
      None
    } else {
      buf.truncate(len);
      Some(String::from_utf8_lossy(&buf).into_owned())
    }
  }

  pub fn can_undo(&self) -> bool {
    unsafe { sys::editBufferCanUndo(self.as_ptr()) }
  }

  pub fn can_redo(&self) -> bool {
    unsafe { sys::editBufferCanRedo(self.as_ptr()) }
  }

  pub fn clear_history(&mut self) {
    unsafe { sys::editBufferClearHistory(self.as_ptr()) }
  }

  // --- Metadata ---

  pub fn id(&self) -> u16 {
    unsafe { sys::editBufferGetId(self.as_ptr()) }
  }

  pub fn debug_log_rope(&self) {
    unsafe { sys::editBufferDebugLogRope(self.as_ptr()) }
  }
}

impl Drop for EditBuffer {
  fn drop(&mut self) {
    unsafe { sys::destroyEditBuffer(self.ptr.as_ptr()) }
  }
}

unsafe impl Send for EditBuffer {}
