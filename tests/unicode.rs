use opentui_core::{encode_unicode, WidthMethod};

#[test]
fn encode_ascii() {
  let result = encode_unicode("hello", WidthMethod::Wcwidth).unwrap();
  assert_eq!(result.len(), 5);
  for (i, ch) in result.iter().enumerate() {
    assert_eq!(ch.width, 1);
    assert_eq!(ch.char_code, b"hello"[i] as u32);
  }
}

#[test]
fn encode_empty() {
  let result = encode_unicode("", WidthMethod::Wcwidth);
  // empty string should return Some with empty vec or None
  if let Some(chars) = result {
    assert_eq!(chars.len(), 0);
  }
}

#[test]
fn encode_unicode_method() {
  let result = encode_unicode("abc", WidthMethod::Unicode).unwrap();
  assert_eq!(result.len(), 3);
}

#[test]
fn encode_multibyte() {
  let result = encode_unicode("café", WidthMethod::Wcwidth).unwrap();
  // "café" has 4 graphemes, é is a multi-byte char
  assert!(result.len() >= 4);
}

#[test]
fn encode_cjk() {
  let result = encode_unicode("中文", WidthMethod::Wcwidth).unwrap();
  assert_eq!(result.len(), 2);
  // CJK characters are typically width 2
  assert_eq!(result[0].width, 2);
  assert_eq!(result[1].width, 2);
}

#[test]
fn encode_emoji() {
  let result = encode_unicode("🎉", WidthMethod::Wcwidth).unwrap();
  assert!(!result.is_empty());
  assert!(result[0].width >= 1);
}

#[test]
fn encode_mixed() {
  let result = encode_unicode("A中🎉", WidthMethod::Wcwidth).unwrap();
  assert!(result.len() >= 3);
  assert_eq!(result[0].width, 1); // ASCII
  assert_eq!(result[1].width, 2); // CJK
}
