use opentui_core::{Buffer, Renderer, Rgba, WidthMethod};

fn test_renderer() -> Renderer {
  Renderer::with_options(80, 24, true, false).unwrap()
}

// --- Owned buffer ---

#[test]
fn create_owned_buffer() {
  let buf = Buffer::new(40, 12, false, WidthMethod::Wcwidth, "test").unwrap();
  assert_eq!(buf.width(), 40);
  assert_eq!(buf.height(), 12);
}

#[test]
fn create_buffer_zero_dimensions_fails() {
  assert!(Buffer::new(0, 10, false, WidthMethod::Wcwidth, "bad").is_err());
  assert!(Buffer::new(10, 0, false, WidthMethod::Wcwidth, "bad").is_err());
}

#[test]
fn buffer_id() {
  let buf = Buffer::new(10, 10, false, WidthMethod::Wcwidth, "mybuf").unwrap();
  assert_eq!(buf.id(), "mybuf");
}

#[test]
fn buffer_clear_and_fill() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  buf.fill_rect(0, 0, 10, 5, &Rgba::from_hex("#ff0000").unwrap());
}

#[test]
fn buffer_draw_text() {
  let buf = Buffer::new(40, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("hello world", 0, 0, &Rgba::WHITE, None, 0);
}

#[test]
fn buffer_draw_text_with_bg() {
  let buf = Buffer::new(40, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  buf.draw_text(
    "highlighted",
    0,
    0,
    &Rgba::WHITE,
    Some(&Rgba::from_u8(50, 50, 100)),
    0,
  );
}

#[test]
fn buffer_draw_char() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  buf.draw_char('A' as u32, 5, 5, &Rgba::WHITE, &Rgba::BLACK, 0);
}

#[test]
fn buffer_set_cell() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.set_cell(0, 0, 'X' as u32, &Rgba::WHITE, &Rgba::BLACK, 0);
}

#[test]
fn buffer_set_cell_blended() {
  let buf = Buffer::new(20, 10, true, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  let semi_transparent = Rgba::new(1.0, 0.0, 0.0, 0.5);
  buf.set_cell_blended(0, 0, 'X' as u32, &semi_transparent, &Rgba::BLACK, 0);
}

#[test]
fn buffer_resize() {
  let buf = Buffer::new(10, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  assert_eq!(buf.width(), 10);
  buf.resize(20, 15);
  assert_eq!(buf.width(), 20);
  assert_eq!(buf.height(), 15);
}

#[test]
fn buffer_respect_alpha() {
  let buf = Buffer::new(10, 10, true, WidthMethod::Wcwidth, "test").unwrap();
  assert!(buf.respect_alpha());
  buf.set_respect_alpha(false);
  assert!(!buf.respect_alpha());
}

#[test]
fn buffer_real_char_size() {
  let buf = Buffer::new(10, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  let _ = buf.real_char_size();
}

// --- Scissor stack ---

#[test]
fn scissor_push_pop() {
  let buf = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.push_scissor_rect(5, 5, 10, 10);
  buf.draw_text("clipped", 0, 0, &Rgba::WHITE, None, 0);
  buf.pop_scissor_rect();
}

#[test]
fn scissor_nested() {
  let buf = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.push_scissor_rect(0, 0, 20, 10);
  buf.push_scissor_rect(5, 5, 10, 5);
  buf.pop_scissor_rect();
  buf.pop_scissor_rect();
}

#[test]
fn scissor_clear() {
  let buf = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.push_scissor_rect(0, 0, 10, 10);
  buf.push_scissor_rect(0, 0, 5, 5);
  buf.clear_scissor_rects();
}

// --- Opacity stack ---

#[test]
fn opacity_push_pop() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  assert_eq!(buf.current_opacity(), 1.0);
  buf.push_opacity(0.5);
  assert!((buf.current_opacity() - 0.5).abs() < 1e-6);
  buf.pop_opacity();
  assert_eq!(buf.current_opacity(), 1.0);
}

#[test]
fn opacity_nested_multiplies() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.push_opacity(0.5);
  buf.push_opacity(0.5);
  assert!((buf.current_opacity() - 0.25).abs() < 1e-6);
  buf.pop_opacity();
  buf.pop_opacity();
}

#[test]
fn opacity_clear() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.push_opacity(0.5);
  buf.push_opacity(0.3);
  buf.clear_opacity();
  assert_eq!(buf.current_opacity(), 1.0);
}

// --- Renderer buffer ref ---

#[test]
fn renderer_buffer_ref_draw() {
  let mut r = test_renderer();
  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("via renderer", 0, 0, &Rgba::WHITE, None, 0);
  assert_eq!(buf.width(), 80);
  assert_eq!(buf.height(), 24);
}

#[test]
fn current_buffer() {
  let r = test_renderer();
  let buf = r.current_buffer();
  let _ = buf.width();
}

// --- Draw box ---

#[test]
fn draw_box_basic() {
  let buf = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  let border_chars = [
    '┌' as u32,
    '─' as u32,
    '┐' as u32,
    '│' as u32,
    '└' as u32,
    '─' as u32,
    '┘' as u32,
    '│' as u32,
  ];
  let packed = 0b1111 | (1 << 4); // all sides + fill
  buf.draw_box(
    1,
    1,
    20,
    10,
    &border_chars,
    packed,
    &Rgba::WHITE,
    &Rgba::BLACK,
    Some("Title"),
    None,
  );
}

#[test]
fn draw_box_with_bottom_title() {
  let buf = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  let border_chars = [
    '┌' as u32,
    '─' as u32,
    '┐' as u32,
    '│' as u32,
    '└' as u32,
    '─' as u32,
    '┘' as u32,
    '│' as u32,
  ];
  buf.draw_box(
    1,
    1,
    20,
    10,
    &border_chars,
    0b1111,
    &Rgba::WHITE,
    &Rgba::BLACK,
    Some("Top"),
    Some("Bottom"),
  );
}

// --- Frame buffer compositing ---

#[test]
fn draw_frame_buffer() {
  let target = Buffer::new(40, 20, false, WidthMethod::Wcwidth, "target").unwrap();
  let source = Buffer::new(10, 5, false, WidthMethod::Wcwidth, "source").unwrap();
  source.clear(&Rgba::from_hex("#ff0000").unwrap());
  target.clear(&Rgba::BLACK);
  target.draw_frame_buffer(5, 5, &source, 0, 0, 10, 5);
}

// --- Write resolved chars ---

#[test]
fn write_resolved_chars() {
  let buf = Buffer::new(20, 5, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("ABCD", 0, 0, &Rgba::WHITE, None, 0);
  let mut output = vec![0u8; 1024];
  let written = buf.write_resolved_chars(&mut output, true);
  assert!(written > 0);
}

// --- Color matrix ---

#[test]
fn color_matrix_uniform() {
  let buf = Buffer::new(10, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.clear(&Rgba::WHITE);
  // Identity-ish matrix (won't change much)
  let matrix = [
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
  ];
  buf.color_matrix_uniform(&matrix, 1.0, 1); // 1 = FG, 2 = BG
}

// --- Draw packed/grayscale ---

#[test]
fn draw_packed_buffer_empty() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  buf.draw_packed_buffer(&[], 0, 0, 20, 10);
}

#[test]
fn draw_grayscale_buffer() {
  let buf = Buffer::new(20, 10, false, WidthMethod::Wcwidth, "test").unwrap();
  let intensities: Vec<f32> = (0..20).map(|i| i as f32 / 20.0).collect();
  buf.draw_grayscale_buffer(0, 0, &intensities, 5, 4, None, None);
}
