use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::color::Rgba;
use crate::error::{Error, Result};
use crate::renderer::WidthMethod;
use crate::syntax_style::SyntaxStyle;

/// A styled, rope-backed text buffer.
///
/// Stores text content with per-character or per-span styling, highlights, and
/// syntax style integration. Used as the backing store for
/// [`TextBufferView`](crate::TextBufferView) and
/// [`EditBuffer`](crate::EditBuffer).
pub struct TextBuffer {
  ptr: NonNull<sys::UnifiedTextBuffer>,
}

impl TextBuffer {
  pub fn new(width_method: WidthMethod) -> Result<Self> {
    // SAFETY: Returns null on OOM.
    let ptr = unsafe { sys::createTextBuffer(width_method as u8) };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("text buffer"))
  }

  pub(crate) fn as_ptr(&self) -> *mut sys::UnifiedTextBuffer {
    self.ptr.as_ptr()
  }

  // --- Properties ---

  pub fn length(&self) -> u32 {
    unsafe { sys::textBufferGetLength(self.as_ptr()) }
  }

  pub fn byte_size(&self) -> u32 {
    unsafe { sys::textBufferGetByteSize(self.as_ptr()) }
  }

  pub fn line_count(&self) -> u32 {
    unsafe { sys::textBufferGetLineCount(self.as_ptr()) }
  }

  // --- Content ---

  pub fn reset(&mut self) {
    unsafe { sys::textBufferReset(self.as_ptr()) }
  }

  pub fn clear(&mut self) {
    unsafe { sys::textBufferClear(self.as_ptr()) }
  }

  pub fn append(&mut self, data: &str) {
    unsafe { sys::textBufferAppend(self.as_ptr(), data.as_ptr(), data.len()) }
  }

  pub fn load_file(&mut self, path: &str) -> bool {
    unsafe { sys::textBufferLoadFile(self.as_ptr(), path.as_ptr(), path.len()) }
  }

  /// Set content from styled text chunks.
  pub fn set_styled_text(&mut self, chunks: &[StyledChunk<'_>]) {
    let raw: Vec<sys::StyledChunk> = chunks.iter().map(|c| c.to_raw()).collect();
    unsafe {
      sys::textBufferSetStyledText(self.as_ptr(), raw.as_ptr(), raw.len());
    }
  }

  // --- Memory buffer registry ---

  pub fn register_mem_buffer(&mut self, data: &[u8], owned: bool) -> u16 {
    unsafe { sys::textBufferRegisterMemBuffer(self.as_ptr(), data.as_ptr(), data.len(), owned) }
  }

  pub fn replace_mem_buffer(&mut self, id: u8, data: &[u8], owned: bool) -> bool {
    unsafe { sys::textBufferReplaceMemBuffer(self.as_ptr(), id, data.as_ptr(), data.len(), owned) }
  }

  pub fn clear_mem_registry(&mut self) {
    unsafe { sys::textBufferClearMemRegistry(self.as_ptr()) }
  }

  pub fn set_text_from_mem(&mut self, id: u8) {
    unsafe { sys::textBufferSetTextFromMem(self.as_ptr(), id) }
  }

  pub fn append_from_mem(&mut self, id: u8) {
    unsafe { sys::textBufferAppendFromMemId(self.as_ptr(), id) }
  }

  // --- Style defaults ---

  pub fn set_default_fg(&mut self, fg: Option<&Rgba>) {
    unsafe {
      sys::textBufferSetDefaultFg(
        self.as_ptr(),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      );
    }
  }

  pub fn set_default_bg(&mut self, bg: Option<&Rgba>) {
    unsafe {
      sys::textBufferSetDefaultBg(
        self.as_ptr(),
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      );
    }
  }

  pub fn set_default_attributes(&mut self, attr: Option<u32>) {
    unsafe {
      let val = attr.unwrap_or(0);
      let ptr = if attr.is_some() {
        &val as *const u32
      } else {
        std::ptr::null()
      };
      sys::textBufferSetDefaultAttributes(self.as_ptr(), ptr);
    }
  }

  pub fn reset_defaults(&mut self) {
    unsafe { sys::textBufferResetDefaults(self.as_ptr()) }
  }

  // --- Tabs ---

  pub fn tab_width(&self) -> u8 {
    unsafe { sys::textBufferGetTabWidth(self.as_ptr()) }
  }

  pub fn set_tab_width(&mut self, width: u8) {
    unsafe { sys::textBufferSetTabWidth(self.as_ptr(), width) }
  }

  // --- Text extraction ---

  pub fn plain_text(&self, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe { sys::textBufferGetPlainText(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    buf.truncate(len);
    String::from_utf8_lossy(&buf).into_owned()
  }

  pub fn text_range(&self, start_offset: u32, end_offset: u32, max_len: usize) -> String {
    let mut buf = vec![0u8; max_len];
    let len = unsafe {
      sys::textBufferGetTextRange(
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
      sys::textBufferGetTextRangeByCoords(
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

  // --- Highlights ---

  pub fn add_highlight_by_char_range(&mut self, highlight: &Highlight) {
    let raw = highlight.to_raw();
    unsafe { sys::textBufferAddHighlightByCharRange(self.as_ptr(), &raw) }
  }

  pub fn add_highlight(&mut self, line_idx: u32, highlight: &Highlight) {
    let raw = highlight.to_raw();
    unsafe { sys::textBufferAddHighlight(self.as_ptr(), line_idx, &raw) }
  }

  pub fn remove_highlights_by_ref(&mut self, hl_ref: u16) {
    unsafe { sys::textBufferRemoveHighlightsByRef(self.as_ptr(), hl_ref) }
  }

  pub fn clear_line_highlights(&mut self, line_idx: u32) {
    unsafe { sys::textBufferClearLineHighlights(self.as_ptr(), line_idx) }
  }

  pub fn clear_all_highlights(&mut self) {
    unsafe { sys::textBufferClearAllHighlights(self.as_ptr()) }
  }

  pub fn highlight_count(&self) -> u32 {
    unsafe { sys::textBufferGetHighlightCount(self.as_ptr()) }
  }

  /// Get highlights for a specific line.
  ///
  /// The returned highlights are allocated by the Zig core and freed when
  /// the returned `Vec` is dropped (via [`textBufferFreeLineHighlights`]).
  pub fn line_highlights(&self, line_idx: u32) -> Vec<Highlight> {
    let mut count: usize = 0;
    let ptr = unsafe { sys::textBufferGetLineHighlightsPtr(self.as_ptr(), line_idx, &mut count) };
    if ptr.is_null() || count == 0 {
      return Vec::new();
    }
    // SAFETY: The Zig core allocated `count` ExternalHighlight structs.
    let slice = unsafe { std::slice::from_raw_parts(ptr, count) };
    let highlights: Vec<Highlight> = slice
      .iter()
      .map(|h| Highlight {
        start: h.start,
        end: h.end,
        style_id: h.style_id,
        priority: h.priority,
        hl_ref: h.hl_ref,
      })
      .collect();
    // SAFETY: Free the Zig-allocated memory.
    unsafe { sys::textBufferFreeLineHighlights(ptr, count) };
    highlights
  }

  // --- Syntax ---

  pub fn set_syntax_style(&mut self, style: Option<&mut SyntaxStyle>) {
    unsafe {
      sys::textBufferSetSyntaxStyle(
        self.as_ptr(),
        style.map(|s| s.as_ptr()).unwrap_or(std::ptr::null_mut()),
      );
    }
  }
}

impl Drop for TextBuffer {
  fn drop(&mut self) {
    unsafe { sys::destroyTextBuffer(self.ptr.as_ptr()) }
  }
}

unsafe impl Send for TextBuffer {}

/// A highlight range applied to text.
#[derive(Debug, Clone, Copy)]
pub struct Highlight {
  pub start: u32,
  pub end: u32,
  pub style_id: u32,
  pub priority: u8,
  pub hl_ref: u16,
}

impl Highlight {
  fn to_raw(&self) -> sys::ExternalHighlight {
    sys::ExternalHighlight {
      start: self.start,
      end: self.end,
      style_id: self.style_id,
      priority: self.priority,
      hl_ref: self.hl_ref,
    }
  }
}

/// A chunk of styled text for [`TextBuffer::set_styled_text`].
pub struct StyledChunk<'a> {
  pub text: &'a str,
  pub fg: Option<&'a Rgba>,
  pub bg: Option<&'a Rgba>,
  pub attributes: u32,
  pub link: Option<&'a str>,
}

impl StyledChunk<'_> {
  pub(crate) fn to_raw(&self) -> sys::StyledChunk {
    sys::StyledChunk {
      text_ptr: self.text.as_ptr(),
      text_len: self.text.len(),
      fg_ptr: self.fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      bg_ptr: self.bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
      attributes: self.attributes,
      link_ptr: self.link.map(|l| l.as_ptr()).unwrap_or(std::ptr::null()),
      link_len: self.link.map(|l| l.len()).unwrap_or(0),
    }
  }
}
