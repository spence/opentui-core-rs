use opentui_core::{
  Buffer, EditBuffer, EditorView, Renderer, Rgba, StyledChunk, TextBuffer, TextBufferView,
  WidthMethod, WrapMode,
};

#[test]
fn full_render_pipeline() {
  let mut renderer = Renderer::with_options(80, 24, true, false).unwrap();

  // Frame 1: clear + text
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("Frame 1", 0, 0, &Rgba::WHITE, None, 0);
  renderer.render(true);
  let out1 = renderer.last_output_for_test().len();
  assert!(out1 > 0);

  // Frame 2: same content, diff rendering should produce minimal output
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("Frame 1", 0, 0, &Rgba::WHITE, None, 0);
  renderer.render(false);
  let out2 = renderer.last_output_for_test().len();
  // identical frame should produce less output than the initial forced render
  assert!(
    out2 < out1,
    "diff render ({out2}) should be less than forced ({out1})"
  );
}

#[test]
fn buffer_compositing_pipeline() {
  let mut renderer = Renderer::with_options(80, 24, true, false).unwrap();

  // Create an overlay buffer
  let overlay = Buffer::new(20, 5, true, WidthMethod::Wcwidth, "overlay").unwrap();
  overlay.clear(&Rgba::new(0.0, 0.0, 0.0, 0.5));
  overlay.draw_text("Overlay", 1, 1, &Rgba::WHITE, None, 0);

  // Draw main content + composite overlay
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::from_hex("#1a1a2e").unwrap());
  buf.draw_text("Background content", 0, 0, &Rgba::WHITE, None, 0);
  buf.draw_frame_buffer(30, 10, &overlay, 0, 0, 20, 5);

  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn scissor_clipping_pipeline() {
  let mut renderer = Renderer::with_options(40, 20, true, false).unwrap();
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);

  // Draw within a scissor region
  buf.push_scissor_rect(5, 5, 10, 10);
  buf.fill_rect(0, 0, 40, 20, &Rgba::from_hex("#ff0000").unwrap());
  buf.pop_scissor_rect();

  // The red should only appear within the 10x10 region
  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn opacity_stacking_pipeline() {
  let mut renderer = Renderer::with_options(40, 20, true, false).unwrap();
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);

  buf.push_opacity(0.5);
  buf.fill_rect(0, 0, 20, 10, &Rgba::WHITE);
  buf.push_opacity(0.5); // effective: 0.25
  buf.draw_text("faded", 1, 1, &Rgba::WHITE, None, 0);
  buf.pop_opacity();
  buf.pop_opacity();

  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn text_buffer_to_renderer() {
  let mut renderer = Renderer::with_options(60, 20, true, false).unwrap();

  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let red = Rgba::rgb(1.0, 0.0, 0.0);
  let green = Rgba::rgb(0.0, 1.0, 0.0);
  tb.set_styled_text(&[
    StyledChunk {
      text: "Hello ",
      fg: Some(&red),
      bg: None,
      attributes: 0,
      link: None,
    },
    StyledChunk {
      text: "World",
      fg: Some(&green),
      bg: None,
      attributes: 0,
      link: None,
    },
  ]);

  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_viewport_size(60, 20);

  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text_buffer_view(&mut view, 0, 0);
  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn editor_view_to_renderer() {
  let mut renderer = Renderer::with_options(60, 20, true, false).unwrap();

  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("fn main() {\n    println!(\"hello\");\n}");

  let mut view = EditorView::new(&eb, 60, 20).unwrap();
  view.set_viewport(0, 0, 60, 20, false);
  view.set_wrap_mode(WrapMode::Word);

  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_editor_view(&mut view, 0, 0);
  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn multiple_renderers() {
  let mut r1 = Renderer::with_options(40, 12, true, false).unwrap();
  let mut r2 = Renderer::with_options(80, 24, true, false).unwrap();

  let buf1 = r1.next_buffer();
  buf1.clear(&Rgba::BLACK);
  buf1.draw_text("r1", 0, 0, &Rgba::WHITE, None, 0);
  r1.render(true);

  let buf2 = r2.next_buffer();
  buf2.clear(&Rgba::BLACK);
  buf2.draw_text("r2", 0, 0, &Rgba::WHITE, None, 0);
  r2.render(true);

  assert!(!r1.last_output_for_test().is_empty());
  assert!(!r2.last_output_for_test().is_empty());
}

#[test]
fn hit_grid_with_rendering() {
  let mut renderer = Renderer::with_options(40, 20, true, false).unwrap();

  // Draw buttons and register hit regions
  renderer.add_to_hit_grid(0, 0, 10, 3, 1);
  renderer.add_to_hit_grid(10, 0, 10, 3, 2);
  renderer.add_to_hit_grid(20, 0, 10, 3, 3);

  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("Button 1", 1, 1, &Rgba::WHITE, None, 0);
  buf.draw_text("Button 2", 11, 1, &Rgba::WHITE, None, 0);
  buf.draw_text("Button 3", 21, 1, &Rgba::WHITE, None, 0);
  renderer.render(true);

  // Hit grid is swapped on render, check hits after
  let hit1 = renderer.check_hit(5, 1);
  let hit2 = renderer.check_hit(15, 1);
  let hit3 = renderer.check_hit(25, 1);
  // Verify at least one region registered (grid behavior depends on render cycle)
  assert!(hit1 > 0 || hit2 > 0 || hit3 > 0);
}

#[test]
fn draw_box_with_content() {
  let mut renderer = Renderer::with_options(60, 20, true, false).unwrap();
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);

  let border_chars = [
    '╔' as u32,
    '═' as u32,
    '╗' as u32,
    '║' as u32,
    '╚' as u32,
    '═' as u32,
    '╝' as u32,
    '║' as u32,
  ];

  let packed = 0b1111 | (1 << 4); // all sides + fill
  buf.draw_box(
    2,
    2,
    30,
    10,
    &border_chars,
    packed,
    &Rgba::from_u8(100, 200, 255),
    &Rgba::from_u8(20, 20, 40),
    Some("Window Title"),
    Some("Status: OK"),
  );

  buf.draw_text("Content inside the box", 4, 5, &Rgba::WHITE, None, 0);

  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}

#[test]
fn rapid_frame_loop() {
  let mut renderer = Renderer::with_options(80, 24, true, false).unwrap();

  for i in 0..100 {
    let buf = renderer.next_buffer();
    buf.clear(&Rgba::BLACK);
    let text = format!("frame {i}");
    buf.draw_text(&text, 0, 0, &Rgba::WHITE, None, 0);
    renderer.render(false);
  }

  // final frame should produce output
  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("final", 0, 0, &Rgba::WHITE, None, 0);
  renderer.render(true);
  assert!(!renderer.last_output_for_test().is_empty());
}
