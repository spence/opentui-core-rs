use opentui_core_sys as sys;

use crate::renderer::WidthMethod;

/// An encoded unicode character with its display width.
#[derive(Debug, Clone, Copy)]
pub struct EncodedChar {
  pub width: u8,
  pub char_code: u32,
}

/// Encode a text string into display-width-annotated characters.
///
/// Returns `None` if encoding fails.
pub fn encode_unicode(text: &str, width_method: WidthMethod) -> Option<Vec<EncodedChar>> {
  let mut out_ptr: *mut sys::EncodedChar = std::ptr::null_mut();
  let mut out_len: usize = 0;

  let ok = unsafe {
    sys::encodeUnicode(
      text.as_ptr(),
      text.len(),
      &mut out_ptr,
      &mut out_len,
      width_method as u8,
    )
  };

  if !ok || out_ptr.is_null() {
    return None;
  }

  // SAFETY: The Zig core allocated out_len EncodedChar structs.
  let raw = unsafe { std::slice::from_raw_parts(out_ptr, out_len) };
  let result: Vec<EncodedChar> = raw
    .iter()
    .map(|c| EncodedChar {
      width: c.width,
      char_code: c.char_code,
    })
    .collect();

  // SAFETY: Free the Zig-allocated array.
  unsafe { sys::freeUnicode(out_ptr, out_len) };

  Some(result)
}
