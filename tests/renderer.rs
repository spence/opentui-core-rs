use opentui_core::{CursorStyle, DebugCorner, Renderer, Rgba, WidthMethod};

fn test_renderer() -> Renderer {
  Renderer::with_options(80, 24, true, false).expect("create renderer")
}

#[test]
fn create_and_drop() {
  let _r = test_renderer();
}

#[test]
fn create_with_new() {
  let _r = Renderer::new(120, 40).expect("create renderer");
}

#[test]
fn create_zero_dimensions_fails() {
  assert!(Renderer::new(0, 0).is_err());
  assert!(Renderer::new(80, 0).is_err());
  assert!(Renderer::new(0, 24).is_err());
}

#[test]
fn render_empty_frame() {
  let mut r = test_renderer();
  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  r.render(false);
}

#[test]
fn render_force() {
  let mut r = test_renderer();
  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  r.render(true);

  let output = r.last_output_for_test();
  assert!(!output.is_empty(), "forced render should produce output");
}

#[test]
fn render_produces_ansi() {
  let mut r = test_renderer();
  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  buf.draw_text("test", 0, 0, &Rgba::WHITE, None, 0);
  r.render(true);

  let output = r.last_output_for_test();
  assert!(output.len() > 10, "should produce ANSI sequences");
}

#[test]
fn resize() {
  let mut r = test_renderer();
  r.resize(160, 48);

  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  r.render(false);
}

#[test]
fn set_background_color() {
  let mut r = test_renderer();
  r.set_background_color(&Rgba::from_hex("#1a1a2e").unwrap());
}

#[test]
fn set_render_offset() {
  let mut r = test_renderer();
  r.set_render_offset(5);
}

#[test]
fn cursor_position() {
  let mut r = test_renderer();
  r.set_cursor_position(10, 5, true);

  let state = r.cursor_state();
  assert!(state.visible);
}

#[test]
fn cursor_color() {
  let mut r = test_renderer();
  r.set_cursor_color(&Rgba::WHITE);
}

#[test]
fn cursor_state_roundtrip() {
  let mut r = test_renderer();
  r.set_cursor_position(1, 1, true);
  let state = r.cursor_state();
  assert!(matches!(
    state.style,
    CursorStyle::Block | CursorStyle::Default
  ));
}

#[test]
fn debug_overlay() {
  let mut r = test_renderer();
  r.set_debug_overlay(true, DebugCorner::TopRight);
  r.set_debug_overlay(false, DebugCorner::TopRight);
}

#[test]
fn mouse_enable_disable() {
  let mut r = test_renderer();
  r.enable_mouse(true);
  r.disable_mouse();
}

#[test]
fn kitty_keyboard() {
  let mut r = test_renderer();
  r.enable_kitty_keyboard(0b111);
  let flags = r.kitty_keyboard_flags();
  // flags may be combined with prior state; just verify it's nonzero
  assert!(flags & 0b111 != 0);
  r.set_kitty_keyboard_flags(0);
  r.disable_kitty_keyboard();
}

#[test]
fn terminal_capabilities() {
  let r = test_renderer();
  let caps = r.terminal_capabilities();
  // Just verify it doesn't crash and returns sensible defaults
  let _ = caps.rgb;
  let _ = caps.sync;
  let _ = caps.unicode;
}

#[test]
fn write_out() {
  let mut r = test_renderer();
  r.write_out(b"\x1b[H");
}

#[test]
fn hit_grid() {
  let mut r = test_renderer();
  r.add_to_hit_grid(0, 0, 10, 5, 42);

  // Render to activate the hit grid (it swaps on render)
  let buf = r.next_buffer();
  buf.clear(&Rgba::BLACK);
  r.render(true);

  let hit = r.check_hit(5, 2);
  assert_eq!(hit, 42);

  r.clear_hit_grid();
}

#[test]
fn hit_grid_scissor() {
  let mut r = test_renderer();
  r.hit_grid_push_scissor(0, 0, 40, 12);
  r.add_to_hit_grid_clipped(0, 0, 80, 24, 1);
  r.hit_grid_pop_scissor();
  r.hit_grid_clear_scissors();
}

#[test]
fn hit_grid_dirty() {
  let r = test_renderer();
  let _ = r.hit_grid_dirty();
}

#[test]
fn set_terminal_env_var() {
  let mut r = test_renderer();
  r.set_terminal_env_var("TERM", "xterm-256color");
}

#[test]
fn update_stats() {
  let mut r = test_renderer();
  r.update_stats(16.67, 60, 1.0);
  r.update_memory_stats(1024, 4096, 256);
}

#[test]
fn set_use_thread() {
  let mut r = test_renderer();
  r.set_use_thread(false);
}

#[test]
fn suspend_resume() {
  let mut r = test_renderer();
  r.suspend();
  r.resume();
}

#[test]
fn hyperlinks_capability() {
  let mut r = test_renderer();
  r.set_hyperlinks_capability(true);
  r.set_hyperlinks_capability(false);
}

#[test]
fn clipboard_osc52() {
  let mut r = test_renderer();
  let _ = r.copy_to_clipboard_osc52(0, b"dGVzdA==");
  let _ = r.clear_clipboard_osc52(0);
}

#[test]
fn set_terminal_title() {
  let mut r = test_renderer();
  r.set_terminal_title("test title");
}

#[test]
fn process_capability_response() {
  let mut r = test_renderer();
  r.process_capability_response(b"\x1b[?62;4c");
}

#[test]
fn query_pixel_resolution() {
  let mut r = test_renderer();
  r.query_pixel_resolution();
}

#[test]
fn width_method_values() {
  assert_eq!(WidthMethod::Wcwidth as u8, 0);
  assert_eq!(WidthMethod::Unicode as u8, 1);
}
