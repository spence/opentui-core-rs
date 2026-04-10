use opentui_core::{Renderer, Rgba};

fn main() {
  let mut renderer =
    Renderer::with_options(80, 24, true, false).expect("failed to create renderer");

  let buf = renderer.next_buffer();
  buf.clear(&Rgba::BLACK);

  buf.draw_text("Hello, opentui!", 2, 1, &Rgba::WHITE, None, 0);

  let green = Rgba::from_hex("#00ff00").unwrap();
  buf.draw_text("Rendered from Rust via Zig FFI", 2, 3, &green, None, 0);

  let blue = Rgba::from_u8(100, 149, 237);
  buf.fill_rect(2, 5, 40, 3, &blue);
  buf.draw_text(
    "Alpha-blended box",
    4,
    6,
    &Rgba::WHITE,
    Some(&Rgba::TRANSPARENT),
    0,
  );

  renderer.render(true);

  let output = renderer.last_output_for_test();
  println!("Rendered {} bytes of ANSI output", output.len());
}
