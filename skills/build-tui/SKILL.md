---
description: Build a terminal UI application using the opentui-core Rust rendering engine. Use when building TUIs, terminal dashboards, CLI tools with rich output, or any interactive terminal application with Rust.
---

Build a TUI application using the opentui-core crate based on the following description:

$ARGUMENTS

---

Before writing any code, read `examples/leto.rs` from the opentui-core-rs repo as the reference implementation for how to build TUI applications with this crate. Follow the same patterns.

## Crate overview

opentui-core is a Rust FFI wrapper around the sst/opentui Zig rendering engine. It is a **rendering engine, not a framework** — you control the render loop, layout, and input handling yourself. It provides:

- Double-buffered rendering with automatic diff detection (only changed cells produce ANSI output)
- Cell-level drawing operations (text, rectangles, boxes, grids)
- Scissor clipping (nested rectangular viewport masking)
- Opacity stacking (hierarchical transparency)
- Alpha blending (Porter-Duff compositing with f32 RGBA)
- Buffer compositing (draw one buffer onto another)
- Text buffers with styled text, highlights, and syntax styles
- Edit buffers with cursor, undo/redo, word navigation
- Editor views with viewport, wrapping, and visual cursor
- Hit grid for mouse interaction regions
- Hyperlink support (OSC 8)
- Kitty keyboard protocol
- Unicode/grapheme-aware text handling

## Architecture

```
Your app
  │
  ├─ renderer.next_buffer()  →  BufferRef (back buffer)
  │       │
  │       ├── clear(bg)
  │       ├── draw_text(text, x, y, fg, bg, attrs)
  │       ├── fill_rect(x, y, w, h, color)
  │       ├── draw_box(...)
  │       ├── push_scissor_rect / pop_scissor_rect
  │       ├── push_opacity / pop_opacity
  │       ├── draw_frame_buffer (composite another buffer)
  │       ├── draw_editor_view / draw_text_buffer_view
  │       └── ... (all drawing ops)
  │
  └─ drop(buf)  →  renderer.render(force)
                      │
                      ├── diff back vs front buffer
                      ├── emit ANSI only for changed cells
                      └── swap buffers
```

**Critical pattern**: The `BufferRef` borrows the renderer mutably. You MUST drop it before calling `renderer.render()`:

```rust
{
    let buf = renderer.next_buffer();
    // ... draw everything ...
}  // buf dropped here
renderer.render(false);
```

## Core types

### Rgba
```rust
use opentui_core::Rgba;

// Constants
Rgba::BLACK, Rgba::WHITE, Rgba::TRANSPARENT

// Constructors
Rgba::new(r, g, b, a)        // f32 components [0.0, 1.0]
Rgba::rgb(r, g, b)           // opaque (a = 1.0)
Rgba::from_u8(r, g, b)       // from 8-bit values
Rgba::from_hex("#ff0000")    // from hex string, returns Option
```

### Renderer
```rust
use opentui_core::Renderer;

let mut renderer = Renderer::new(width, height)?;

// Terminal lifecycle
renderer.setup_terminal(true);       // enter alternate screen
renderer.restore_terminal_modes();   // exit alternate screen

// Render loop
let buf = renderer.next_buffer();    // get back buffer (BufferRef)
// ... draw into buf ...
drop(buf);
renderer.render(false);              // diff render (true = force full)

// Resize
renderer.resize(new_width, new_height);

// Cursor
renderer.set_cursor_position(x, y, visible);
renderer.set_cursor_color(&color);

// Mouse
renderer.enable_mouse(enable_movement);
renderer.disable_mouse();

// Hit grid (clickable regions)
renderer.add_to_hit_grid(x, y, w, h, id);
renderer.check_hit(x, y)  // returns region id or 0

// Kitty keyboard
renderer.enable_kitty_keyboard(flags);
renderer.disable_kitty_keyboard();

// Testing
let output = renderer.last_output_for_test();  // raw ANSI bytes
```

### BufferRef / Buffer
```rust
// BufferRef: borrowed from renderer (no Drop)
let buf = renderer.next_buffer();

// Buffer: standalone, owned (freed on Drop)
let buf = Buffer::new(width, height, respect_alpha, WidthMethod::Wcwidth, "id")?;

// Both share the same drawing API:
buf.clear(&bg_color);
buf.draw_text("hello", x, y, &fg, Some(&bg), attributes);
buf.draw_text("no bg", x, y, &fg, None, 0);
buf.draw_char(char_code, x, y, &fg, &bg, attributes);
buf.set_cell(x, y, char_code, &fg, &bg, attributes);
buf.set_cell_blended(x, y, char_code, &fg, &bg, attributes);  // alpha blend
buf.fill_rect(x, y, width, height, &color);
buf.draw_box(x, y, w, h, &border_chars, packed_opts, &border_color, &bg, title, bottom_title);
buf.draw_grid(&border_chars, &border_fg, &border_bg, &col_offsets, &row_offsets, inner, outer);
buf.draw_frame_buffer(dest_x, dest_y, &source_buf, src_x, src_y, src_w, src_h);
buf.resize(new_w, new_h);
buf.width();
buf.height();

// Scissor clipping
buf.push_scissor_rect(x, y, w, h);
// ... draws here are clipped to the rect ...
buf.pop_scissor_rect();

// Opacity
buf.push_opacity(0.5);
// ... draws here are 50% transparent ...
buf.pop_opacity();
buf.current_opacity();  // returns effective opacity

// Color matrix effects
buf.color_matrix_uniform(&matrix_4x4, strength, target);  // target: 1=FG, 2=BG

// Rendering text views
buf.draw_editor_view(&mut editor_view, x, y);
buf.draw_text_buffer_view(&mut text_buffer_view, x, y);
```

### draw_box packed_options format
```rust
let sides = 0b1111;      // top=8, right=4, bottom=2, left=1
let fill = 1 << 4;       // bit 4: fill interior
let title_align = 0 << 5;      // bits 5-6: 0=left, 1=center, 2=right
let bottom_align = 0 << 7;     // bits 7-8: same
let packed = sides | fill | title_align | bottom_align;

// border_chars: [top-left, top, top-right, right, bottom-left, bottom, bottom-right, left]
let single = ['┌','─','┐','│','└','─','┘','│'].map(|c| c as u32);
let double = ['╔','═','╗','║','╚','═','╝','║'].map(|c| c as u32);
let rounded = ['╭','─','╮','│','╰','─','╯','│'].map(|c| c as u32);
```

### TextBuffer
```rust
use opentui_core::{TextBuffer, WidthMethod, StyledChunk, Highlight};

let mut tb = TextBuffer::new(WidthMethod::Wcwidth)?;
tb.append("hello world");
tb.set_text_from_mem(mem_id);
tb.load_file("path/to/file");
tb.set_styled_text(&[
    StyledChunk { text: "red", fg: Some(&red), bg: None, attributes: 0, link: None },
    StyledChunk { text: " normal", fg: None, bg: None, attributes: 0, link: None },
]);
tb.plain_text(max_len);
tb.length();
tb.line_count();
tb.set_tab_width(4);

// Highlights
tb.add_highlight_by_char_range(&Highlight { start, end, style_id, priority, hl_ref });
tb.remove_highlights_by_ref(hl_ref);
tb.clear_all_highlights();

// With syntax styles
tb.set_syntax_style(Some(&mut syntax_style));
```

### TextBufferView
```rust
use opentui_core::{TextBufferView, WrapMode};

let mut view = TextBufferView::new(&tb)?;
view.set_viewport_size(width, height);
view.set_viewport(x, y, width, height);
view.set_wrap_mode(WrapMode::Word);  // None, Char, Word
view.virtual_line_count();

// Selection
view.set_selection(start, end, Some(&sel_bg), None);
view.reset_selection();

// Draw into buffer
buf.draw_text_buffer_view(&mut view, x, y);
```

### EditBuffer
```rust
use opentui_core::{EditBuffer, WidthMethod};

let mut eb = EditBuffer::new(WidthMethod::Wcwidth)?;
eb.set_text("initial content");
eb.insert_text("more");
eb.delete_char_backward();  // backspace
eb.delete_char();            // delete forward
eb.new_line();
eb.move_cursor_left/right/up/down();
eb.set_cursor(row, col);
let (row, col) = eb.cursor();
eb.undo(max_len);
eb.redo(max_len);
eb.can_undo();
```

### EditorView
```rust
use opentui_core::EditorView;

let mut view = EditorView::new(&eb, viewport_width, viewport_height)?;
view.set_viewport(x, y, w, h, move_cursor);
view.set_wrap_mode(WrapMode::Word);
view.set_scroll_margin(3.0);
let vc = view.visual_cursor();  // VisualCursor { visual_row, visual_col, logical_row, logical_col, offset }
view.move_up_visual();
view.move_down_visual();

// Draw into buffer
buf.draw_editor_view(&mut view, x, y);
```

### SyntaxStyle
```rust
use opentui_core::SyntaxStyle;

let mut ss = SyntaxStyle::new()?;
let keyword_id = ss.register("keyword", Some(&blue), None, 0);
let string_id = ss.register("string", Some(&green), None, 0);
ss.resolve_by_name("keyword");  // returns id
tb.set_syntax_style(Some(&mut ss));
```

### Hyperlinks
```rust
use opentui_core::{link_alloc, link_url, attributes_with_link};

let link_id = link_alloc("https://example.com");
let attrs = attributes_with_link(0, link_id);
buf.draw_text("click me", x, y, &fg, None, attrs);
```

## Input handling

Use `crossterm` (dev-dependency) for raw mode and keyboard/mouse input. opentui-core handles all rendering.

```rust
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

// Setup
terminal::enable_raw_mode()?;
crossterm::execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;
let (cols, rows) = terminal::size()?;
let mut renderer = Renderer::new(cols as u32, rows as u32)?;

// Render loop
loop {
    render_frame(&mut renderer, &state, w, h);

    match event::read()? {
        Event::Key(key) => match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Up => { /* ... */ }
            KeyCode::Down => { /* ... */ }
            KeyCode::Enter => { /* ... */ }
            KeyCode::Esc => { /* ... */ }
            _ => {}
        },
        Event::Resize(cols, rows) => {
            w = cols as u32;
            h = rows as u32;
            renderer.resize(w, h);
        }
        _ => {}
    }
}

// Cleanup
crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
terminal::disable_raw_mode()?;
```

**Panic safety** — always restore the terminal on panic:
```rust
let default_hook = std::panic::take_hook();
std::panic::set_hook(Box::new(move |info| {
    let _ = terminal::disable_raw_mode();
    let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
    default_hook(info);
}));
```

## Layout patterns

opentui-core has no layout engine. You calculate positions manually. Common patterns:

**Right-aligned text:**
```rust
fn draw_right(buf: &BufferRef, text: &str, right_x: u32, y: u32, fg: &Rgba) {
    let x = right_x.saturating_sub(text.len() as u32);
    buf.draw_text(text, x, y, fg, None, 0);
}
```

**Sections with fixed bottom panel:**
```rust
let panel_height = 5;
let panel_y = height - panel_height;
// Draw main content in rows 0..panel_y
// Draw bottom panel in rows panel_y..height
```

**Selected row highlighting:**
```rust
if selected {
    buf.fill_rect(0, row_y, width, 1, &SELECTION_BG);
}
let row_bg = if selected { Some(&SELECTION_BG) } else { None };
buf.draw_text(text, x, row_y, &fg, row_bg, 0);
```

**Scrollable list:**
```rust
let visible_start = scroll_offset;
let visible_end = (scroll_offset + viewport_height).min(items.len());
for (i, item) in items[visible_start..visible_end].iter().enumerate() {
    let y = start_y + i as u32;
    buf.draw_text(&item.label, x, y, &fg, None, 0);
}
```

**Composited overlays (e.g. modal dialogs):**
```rust
let overlay = Buffer::new(dialog_w, dialog_h, true, WidthMethod::Wcwidth, "dialog")?;
overlay.clear(&Rgba::new(0.1, 0.1, 0.2, 0.9));
overlay.draw_box(0, 0, dialog_w, dialog_h, &rounded, packed, &border, &bg, Some("Title"), None);
overlay.draw_text("Content", 2, 2, &white, None, 0);
// Composite onto main buffer
buf.draw_frame_buffer(center_x, center_y, &overlay, 0, 0, dialog_w, dialog_h);
```

## Constraints

- The Zig core uses global allocator state that is not thread-safe. Run tests with `--test-threads=1` (configured in `.cargo/config.toml`).
- `BufferRef` borrows the renderer mutably — drop it before calling `render()`.
- `Buffer::new` and `Renderer::new` return `Err` for zero dimensions.
- All coordinates are `u32` (cell positions, not pixels).
- Colors are `f32` RGBA in `[0.0, 1.0]` range.
- `draw_text` bg parameter is `Option<&Rgba>` — pass `None` to leave background unchanged.
- Color matrix target values: `1` = foreground, `2` = background (not `0`).
- Link pool, syntax styles, and text buffers require a renderer to be created first (initializes Zig globals).
- `crossterm` is a dev-dependency — add it if the TUI needs interactive input.

## Styling conventions

Follow the crate's style guide:
- 2-space indentation, 100-char max line width
- `cargo fmt` with `.rustfmt.toml`
- Minimal comments (explain WHY, not WHAT)
- SAFETY comments mandatory for `unsafe` blocks
- Conventional Commits for git messages
