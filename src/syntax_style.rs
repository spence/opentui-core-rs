use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::color::Rgba;
use crate::error::{Error, Result};

/// A registry of named syntax styles for highlighting.
///
/// Register styles by name and resolve them to IDs for use with
/// [`TextBuffer`](crate::TextBuffer) highlights.
pub struct SyntaxStyle {
  ptr: NonNull<sys::SyntaxStyle>,
}

impl SyntaxStyle {
  pub fn new() -> Result<Self> {
    let ptr = unsafe { sys::createSyntaxStyle() };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("syntax style"))
  }

  pub(crate) fn as_ptr(&self) -> *mut sys::SyntaxStyle {
    self.ptr.as_ptr()
  }

  /// Register a named style, returning its ID.
  pub fn register(
    &mut self,
    name: &str,
    fg: Option<&Rgba>,
    bg: Option<&Rgba>,
    attributes: u32,
  ) -> u32 {
    unsafe {
      sys::syntaxStyleRegister(
        self.as_ptr(),
        name.as_ptr(),
        name.len(),
        fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
        attributes,
      )
    }
  }

  /// Resolve a style name to its ID. Returns 0 if not found.
  pub fn resolve_by_name(&self, name: &str) -> u32 {
    unsafe { sys::syntaxStyleResolveByName(self.as_ptr(), name.as_ptr(), name.len()) }
  }

  pub fn style_count(&self) -> usize {
    unsafe { sys::syntaxStyleGetStyleCount(self.as_ptr()) }
  }
}

impl Drop for SyntaxStyle {
  fn drop(&mut self) {
    unsafe { sys::destroySyntaxStyle(self.ptr.as_ptr()) }
  }
}

unsafe impl Send for SyntaxStyle {}
