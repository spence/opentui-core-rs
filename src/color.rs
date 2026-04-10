/// RGBA color with f32 components in [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rgba {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Rgba {
  pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
  pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
  pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

  pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self { r, g, b, a }
  }

  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self::new(r, g, b, 1.0)
  }

  /// Create from 8-bit RGB values.
  pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
    Self::rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
  }

  /// Create from a hex string (e.g. `"#ff0000"` or `"ff0000"`).
  pub fn from_hex(hex: &str) -> Option<Self> {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    if hex.len() != 6 {
      return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Self::from_u8(r, g, b))
  }

  pub(crate) fn as_ptr(&self) -> *const f32 {
    &self.r as *const f32
  }
}

impl Default for Rgba {
  fn default() -> Self {
    Self::BLACK
  }
}
