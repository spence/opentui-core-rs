use opentui_core::Rgba;

#[test]
fn constants() {
  assert_eq!(Rgba::BLACK, Rgba::new(0.0, 0.0, 0.0, 1.0));
  assert_eq!(Rgba::WHITE, Rgba::new(1.0, 1.0, 1.0, 1.0));
  assert_eq!(Rgba::TRANSPARENT, Rgba::new(0.0, 0.0, 0.0, 0.0));
}

#[test]
fn from_u8() {
  let c = Rgba::from_u8(255, 128, 0);
  assert_eq!(c.r, 1.0);
  assert!((c.g - 128.0 / 255.0).abs() < 1e-6);
  assert_eq!(c.b, 0.0);
  assert_eq!(c.a, 1.0);
}

#[test]
fn from_u8_black() {
  let c = Rgba::from_u8(0, 0, 0);
  assert_eq!(c.r, 0.0);
  assert_eq!(c.g, 0.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn from_hex_with_hash() {
  let c = Rgba::from_hex("#ff0000").unwrap();
  assert_eq!(c.r, 1.0);
  assert_eq!(c.g, 0.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn from_hex_without_hash() {
  let c = Rgba::from_hex("00ff00").unwrap();
  assert_eq!(c.r, 0.0);
  assert_eq!(c.g, 1.0);
  assert_eq!(c.b, 0.0);
}

#[test]
fn from_hex_mixed_case() {
  let c = Rgba::from_hex("#aaBBcc").unwrap();
  assert_eq!(c, Rgba::from_u8(0xaa, 0xbb, 0xcc));
}

#[test]
fn from_hex_invalid_length() {
  assert!(Rgba::from_hex("#fff").is_none());
  assert!(Rgba::from_hex("").is_none());
  assert!(Rgba::from_hex("#ff00ff00").is_none());
}

#[test]
fn from_hex_invalid_chars() {
  assert!(Rgba::from_hex("#gggggg").is_none());
}

#[test]
fn default_is_black() {
  assert_eq!(Rgba::default(), Rgba::BLACK);
}

#[test]
fn clone_and_copy() {
  let a = Rgba::rgb(0.5, 0.5, 0.5);
  let b = a;
  let c = a.clone();
  assert_eq!(a, b);
  assert_eq!(a, c);
}

#[test]
fn debug_format() {
  let c = Rgba::rgb(1.0, 0.0, 0.0);
  let s = format!("{c:?}");
  assert!(s.contains("Rgba"));
  assert!(s.contains("1.0"));
}
