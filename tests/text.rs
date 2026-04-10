use opentui_core::{
  EditBuffer, EditorView, Highlight, Rgba, StyledChunk, SyntaxStyle, TextBuffer, TextBufferView,
  WidthMethod, WrapMode,
};

// --- TextBuffer ---

#[test]
fn text_buffer_create() {
  let tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  assert_eq!(tb.length(), 0);
  // Empty buffer has 1 line (the empty line)
  assert!(tb.line_count() <= 1);
}

#[test]
fn text_buffer_append() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("hello world");
  assert!(tb.length() > 0);
  assert!(tb.byte_size() > 0);
}

#[test]
fn text_buffer_plain_text() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("line one\nline two");
  let text = tb.plain_text(1024);
  assert!(text.contains("line one"));
  assert!(text.contains("line two"));
}

#[test]
fn text_buffer_clear() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("some content");
  assert!(tb.length() > 0);
  tb.clear();
  assert_eq!(tb.length(), 0);
}

#[test]
fn text_buffer_reset() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("content");
  tb.reset();
  assert_eq!(tb.length(), 0);
}

#[test]
fn text_buffer_line_count() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("a\nb\nc");
  assert!(tb.line_count() >= 3);
}

#[test]
fn text_buffer_styled_text() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let red = Rgba::from_hex("#ff0000").unwrap();
  let chunks = [StyledChunk {
    text: "styled",
    fg: Some(&red),
    bg: None,
    attributes: 0,
    link: None,
  }];
  tb.set_styled_text(&chunks);
  assert!(tb.length() > 0);
}

#[test]
fn text_buffer_styled_text_with_link() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let chunks = [StyledChunk {
    text: "click me",
    fg: None,
    bg: None,
    attributes: 0,
    link: Some("https://example.com"),
  }];
  tb.set_styled_text(&chunks);
  assert!(tb.length() > 0);
}

#[test]
fn text_buffer_defaults() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.set_default_fg(Some(&Rgba::WHITE));
  tb.set_default_bg(Some(&Rgba::BLACK));
  tb.set_default_attributes(Some(1));
  tb.reset_defaults();
}

#[test]
fn text_buffer_tabs() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  assert_eq!(tb.tab_width(), 2); // default
  tb.set_tab_width(4);
  assert_eq!(tb.tab_width(), 4);
}

#[test]
fn text_buffer_text_range() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("abcdefghij");
  let range = tb.text_range(2, 5, 1024);
  assert_eq!(range.len(), 3);
}

#[test]
fn text_buffer_text_range_by_coords() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("line one\nline two");
  let range = tb.text_range_by_coords(0, 0, 0, 4, 1024);
  assert!(!range.is_empty());
}

// --- Highlights ---

#[test]
fn text_buffer_highlights() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("some highlighted text");

  tb.add_highlight_by_char_range(&Highlight {
    start: 0,
    end: 4,
    style_id: 1,
    priority: 0,
    hl_ref: 100,
  });

  assert_eq!(tb.highlight_count(), 1);

  tb.remove_highlights_by_ref(100);
  assert_eq!(tb.highlight_count(), 0);
}

#[test]
fn text_buffer_line_highlights() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("line one\nline two");

  tb.add_highlight(
    0,
    &Highlight {
      start: 0,
      end: 4,
      style_id: 1,
      priority: 0,
      hl_ref: 0,
    },
  );

  let highlights = tb.line_highlights(0);
  assert!(!highlights.is_empty());

  tb.clear_line_highlights(0);
  tb.clear_all_highlights();
}

// --- Memory buffer registry ---

#[test]
fn text_buffer_mem_registry() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let id = tb.register_mem_buffer(b"hello from mem", false);
  assert_ne!(id, 0xFFFF);

  tb.set_text_from_mem(id as u8);
  assert!(tb.length() > 0);

  tb.clear_mem_registry();
}

// --- TextBufferView ---

#[test]
fn text_buffer_view_create() {
  let tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let _view = TextBufferView::new(&tb).unwrap();
}

#[test]
fn text_buffer_view_wrap_modes() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("a long line that should wrap when the viewport is narrow enough");
  let mut view = TextBufferView::new(&tb).unwrap();

  view.set_wrap_mode(WrapMode::Word);
  view.set_wrap_width(20);
  view.set_viewport_size(20, 10);

  let count = view.virtual_line_count();
  assert!(count >= 1);
}

#[test]
fn text_buffer_view_viewport() {
  let tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_viewport(0, 0, 40, 10);
}

#[test]
fn text_buffer_view_selection() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("select this text");
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_viewport_size(40, 10);

  view.set_selection(0, 6, Some(&Rgba::from_u8(50, 50, 150)), None);
  let info = view.selection_info();
  assert_ne!(info, 0);

  view.reset_selection();
}

#[test]
fn text_buffer_view_local_selection() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("select locally");
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_viewport_size(40, 10);

  view.set_local_selection(0, 0, 5, 0, None, None);
  view.reset_local_selection();
}

#[test]
fn text_buffer_view_plain_text() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("hello view");
  let view = TextBufferView::new(&tb).unwrap();
  let text = view.plain_text(1024);
  assert!(text.contains("hello view"));
}

#[test]
fn text_buffer_view_line_info() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("line1\nline2\nline3");
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_viewport_size(40, 10);

  let info = view.line_info();
  assert!(info.width_cols_max > 0 || info.start_cols().is_empty());
}

#[test]
fn text_buffer_view_measure() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  tb.append("measure me");
  let view = TextBufferView::new(&tb).unwrap();

  let result = view.measure_for_dimensions(40, 10);
  assert!(result.is_some());
}

#[test]
fn text_buffer_view_tab_indicator() {
  let tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_tab_indicator('→' as u32);
  view.set_tab_indicator_color(&Rgba::from_u8(100, 100, 100));
}

#[test]
fn text_buffer_view_truncate() {
  let tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = TextBufferView::new(&tb).unwrap();
  view.set_truncate(true);
  view.set_truncate(false);
}

// --- EditBuffer ---

#[test]
fn edit_buffer_create() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  assert_eq!(eb.cursor(), (0, 0));
}

#[test]
fn edit_buffer_insert_and_read() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.insert_text("hello");
  let text = eb.text(1024);
  assert_eq!(text, "hello");
}

#[test]
fn edit_buffer_set_text() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("initial");
  assert_eq!(eb.text(1024), "initial");
  eb.set_text("replaced");
  assert_eq!(eb.text(1024), "replaced");
}

#[test]
fn edit_buffer_cursor_movement() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abc\ndef");

  eb.set_cursor(0, 0);
  eb.move_cursor_right();
  let (row, col) = eb.cursor();
  assert_eq!(row, 0);
  assert_eq!(col, 1);

  eb.move_cursor_down();
  let (row, _) = eb.cursor();
  assert_eq!(row, 1);

  eb.move_cursor_left();
  eb.move_cursor_up();
}

#[test]
fn edit_buffer_delete() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abcdef");
  eb.set_cursor(0, 3);
  eb.delete_char_backward();
  let text = eb.text(1024);
  assert_eq!(text, "abdef");
}

#[test]
fn edit_buffer_delete_forward() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abcdef");
  eb.set_cursor(0, 2);
  eb.delete_char();
  assert_eq!(eb.text(1024), "abdef");
}

#[test]
fn edit_buffer_new_line() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("ab");
  eb.set_cursor(0, 1);
  eb.new_line();
  let text = eb.text(1024);
  assert!(text.contains('\n'));
}

#[test]
fn edit_buffer_delete_line() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("line1\nline2\nline3");
  eb.set_cursor(1, 0);
  eb.delete_line();
  let text = eb.text(1024);
  assert!(!text.contains("line2"));
}

#[test]
fn edit_buffer_undo_redo() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("original");

  if eb.can_undo() {
    eb.undo(1024);
    if eb.can_redo() {
      eb.redo(1024);
    }
  }
}

#[test]
fn edit_buffer_clear_history() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("text");
  eb.clear_history();
  assert!(!eb.can_undo());
}

#[test]
fn edit_buffer_clear() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("content");
  eb.clear();
  assert_eq!(eb.text(1024), "");
}

#[test]
fn edit_buffer_cursor_position() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abc\ndef");
  eb.set_cursor(1, 2);
  let pos = eb.cursor_position();
  assert_eq!(pos.row, 1);
  assert_eq!(pos.col, 2);
}

#[test]
fn edit_buffer_word_boundaries() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello world foo");
  eb.set_cursor(0, 0);
  let next = eb.next_word_boundary();
  assert!(next.col > 0);
}

#[test]
fn edit_buffer_end_of_line() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello");
  eb.set_cursor(0, 0);
  let eol = eb.end_of_line();
  assert_eq!(eol.col, 5);
}

#[test]
fn edit_buffer_goto_line() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("line0\nline1\nline2");
  eb.goto_line(2);
  let (row, _) = eb.cursor();
  assert_eq!(row, 2);
}

#[test]
fn edit_buffer_offset_conversion() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abc\ndef");

  let offset = eb.position_to_offset(1, 0);
  assert!(offset > 0);

  let pos = eb.offset_to_position(offset);
  assert!(pos.is_some());
  assert_eq!(pos.unwrap().row, 1);
}

#[test]
fn edit_buffer_line_start_offset() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abc\ndef");
  let offset = eb.line_start_offset(1);
  assert!(offset > 0);
}

#[test]
fn edit_buffer_text_range() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abcdefgh");
  let range = eb.text_range(2, 5, 1024);
  assert_eq!(range.len(), 3);
}

#[test]
fn edit_buffer_replace_text() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello");
  eb.set_cursor(0, 2);
  eb.replace_text("XY");
  // replace_text replaces selected text or inserts at cursor
}

#[test]
fn edit_buffer_id() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  let _ = eb.id(); // just verify it doesn't crash
}

#[test]
fn edit_buffer_insert_char() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.insert_char("X");
  assert_eq!(eb.text(1024), "X");
}

#[test]
fn edit_buffer_delete_range() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abcdef");
  eb.delete_range(0, 1, 0, 4);
  let text = eb.text(1024);
  assert_eq!(text, "aef");
}

// --- EditorView ---

#[test]
fn editor_view_create() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  let _view = EditorView::new(&eb, 40, 20).unwrap();
}

#[test]
fn editor_view_viewport() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = EditorView::new(&eb, 40, 20).unwrap();

  view.set_viewport(0, 0, 40, 20, false);
  let vp = view.viewport();
  assert!(vp.is_some());
  let (x, y, w, h) = vp.unwrap();
  assert_eq!((x, y, w, h), (0, 0, 40, 20));

  view.clear_viewport();
  assert!(view.viewport().is_none());
}

#[test]
fn editor_view_cursor() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello\nworld");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, false);

  let (row, col) = view.cursor();
  let _ = (row, col);

  let vc = view.visual_cursor();
  let _ = vc.visual_row;
  let _ = vc.logical_row;
}

#[test]
fn editor_view_selection() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("select me");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, false);

  view.set_selection(0, 6, None, None);
  let sel = view.selection();
  assert_ne!(sel, 0);

  view.reset_selection();
}

#[test]
fn editor_view_wrap_mode() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("a long line that wraps");
  let mut view = EditorView::new(&eb, 10, 5).unwrap();
  view.set_wrap_mode(WrapMode::Word);
}

#[test]
fn editor_view_scroll_margin() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_scroll_margin(3.0);
}

#[test]
fn editor_view_line_counts() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("a\nb\nc\nd\ne");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, false);

  let vl = view.virtual_line_count();
  let tvl = view.total_virtual_line_count();
  assert!(vl > 0 || tvl > 0);
}

#[test]
fn editor_view_visual_navigation() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("line1\nline2\nline3");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, true);

  view.move_down_visual();
  view.move_up_visual();
}

#[test]
fn editor_view_word_boundaries() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello world foo");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, false);

  let next = view.next_word_boundary();
  let prev = view.prev_word_boundary();
  let _ = (next.offset, prev.offset);
}

#[test]
fn editor_view_line_navigation() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("hello world");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_viewport(0, 0, 40, 20, false);

  let eol = view.end_of_line();
  let sol = view.visual_start_of_line();
  let veol = view.visual_end_of_line();
  let _ = (eol.offset, sol.offset, veol.offset);
}

#[test]
fn editor_view_text() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("get this text");
  let view = EditorView::new(&eb, 40, 20).unwrap();
  let text = view.text(1024);
  assert!(text.contains("get this text"));
}

#[test]
fn editor_view_set_cursor_by_offset() {
  let mut eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  eb.set_text("abcdef");
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_cursor_by_offset(3);
}

#[test]
fn editor_view_tab_indicator() {
  let eb = EditBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut view = EditorView::new(&eb, 40, 20).unwrap();
  view.set_tab_indicator('→' as u32);
  view.set_tab_indicator_color(&Rgba::from_u8(100, 100, 100));
}

// --- SyntaxStyle ---

#[test]
fn syntax_style_create() {
  let _ss = SyntaxStyle::new().unwrap();
}

#[test]
fn syntax_style_register_and_resolve() {
  let mut ss = SyntaxStyle::new().unwrap();
  let id = ss.register(
    "keyword",
    Some(&Rgba::from_hex("#ff6600").unwrap()),
    None,
    1, // bold
  );
  assert!(id > 0);

  let resolved = ss.resolve_by_name("keyword");
  assert_eq!(resolved, id);

  assert!(ss.style_count() >= 1);
}

#[test]
fn syntax_style_resolve_unknown() {
  let ss = SyntaxStyle::new().unwrap();
  let id = ss.resolve_by_name("nonexistent");
  assert_eq!(id, 0);
}

#[test]
fn syntax_style_multiple() {
  let mut ss = SyntaxStyle::new().unwrap();
  ss.register("keyword", Some(&Rgba::rgb(1.0, 0.0, 0.0)), None, 0);
  ss.register("string", Some(&Rgba::rgb(0.0, 1.0, 0.0)), None, 0);
  ss.register("comment", Some(&Rgba::rgb(0.5, 0.5, 0.5)), None, 0);
  assert_eq!(ss.style_count(), 3);
}

#[test]
fn text_buffer_with_syntax_style() {
  let mut tb = TextBuffer::new(WidthMethod::Wcwidth).unwrap();
  let mut ss = SyntaxStyle::new().unwrap();
  ss.register("keyword", Some(&Rgba::rgb(1.0, 0.0, 0.0)), None, 1);

  tb.set_syntax_style(Some(&mut ss));
  tb.append("fn main() {}");

  tb.set_syntax_style(None);
}
