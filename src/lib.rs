//! Rust bindings to the [sst/opentui](https://github.com/sst/opentui) terminal
//! rendering engine.
//!
//! opentui-core provides a safe Rust API over the high-performance Zig core,
//! giving you direct access to the rendering engine without a JavaScript/TypeScript
//! runtime.
//!
//! # Architecture
//!
//! ```text
//! Your app
//!   │
//!   ├─→ renderer.next_buffer() ──→ BufferRef
//!   │        │                       │
//!   │        └── draw_text()         ├── scissor clipping
//!   │        └── set_cell()          ├── opacity stacking
//!   │        └── fill_rect()         ├── alpha blending
//!   │        └── draw_box()          └── cell grid
//!   │
//!   └─→ renderer.render(force)
//!            │
//!            ├── diff: back vs front buffers
//!            ├── ANSI output (only changed cells)
//!            └── swap buffers
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use opentui_core::{Renderer, Rgba};
//!
//! let mut renderer = Renderer::new(80, 24).unwrap();
//! renderer.setup_terminal(true);
//!
//! let buf = renderer.next_buffer();
//! buf.clear(&Rgba::BLACK);
//! buf.draw_text("Hello, opentui!", 0, 0, &Rgba::WHITE, None, 0);
//! renderer.render(false);
//! ```

mod buffer;
mod color;
mod edit_buffer;
mod editor_view;
mod error;
mod event;
mod link;
mod renderer;
mod syntax_style;
mod text_buffer;
mod text_buffer_view;
mod unicode;

pub use buffer::{Buffer, BufferRef};
pub use color::Rgba;
pub use edit_buffer::{EditBuffer, LogicalCursor};
pub use editor_view::{EditorView, VisualCursor};
pub use error::{Error, Result};
pub use event::{
  allocator_stats, arena_allocated_bytes, build_options, set_event_callback, set_log_callback,
  AllocatorStats, BuildOptions, LogLevel,
};
pub use link::{
  attributes_get_link_id, attributes_with_link, clear_global_link_pool, link_alloc, link_url,
};
pub use renderer::{
  Capabilities, CursorState, CursorStyle, CursorStyleOptions, DebugCorner, Renderer, WidthMethod,
};
pub use syntax_style::SyntaxStyle;
pub use text_buffer::{Highlight, StyledChunk, TextBuffer};
pub use text_buffer_view::{LineInfo, MeasureResult, TextBufferView, WrapMode};
pub use unicode::{encode_unicode, EncodedChar};
