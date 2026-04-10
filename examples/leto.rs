use std::io;

use crossterm::{
  cursor,
  event::{self, Event, KeyCode},
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use opentui_core::{Renderer, Rgba};

// --- Colors ---

const BG: Rgba = Rgba {
  r: 0.07,
  g: 0.07,
  b: 0.12,
  a: 1.0,
};
const HEADER_BG: Rgba = Rgba {
  r: 0.09,
  g: 0.09,
  b: 0.14,
  a: 1.0,
};
const ORANGE: Rgba = Rgba {
  r: 0.95,
  g: 0.65,
  b: 0.18,
  a: 1.0,
};
const AMBER: Rgba = Rgba {
  r: 0.91,
  g: 0.63,
  b: 0.16,
  a: 1.0,
};
const WHITE: Rgba = Rgba {
  r: 0.82,
  g: 0.82,
  b: 0.82,
  a: 1.0,
};
const BRIGHT: Rgba = Rgba {
  r: 0.92,
  g: 0.92,
  b: 0.92,
  a: 1.0,
};
const GRAY: Rgba = Rgba {
  r: 0.50,
  g: 0.50,
  b: 0.50,
  a: 1.0,
};
const DIM: Rgba = Rgba {
  r: 0.30,
  g: 0.30,
  b: 0.30,
  a: 1.0,
};
const BLUE: Rgba = Rgba {
  r: 0.35,
  g: 0.55,
  b: 0.82,
  a: 1.0,
};
const GREEN: Rgba = Rgba {
  r: 0.30,
  g: 0.78,
  b: 0.40,
  a: 1.0,
};
const RED: Rgba = Rgba {
  r: 0.90,
  g: 0.35,
  b: 0.30,
  a: 1.0,
};
const SEL_BG: Rgba = Rgba {
  r: 0.12,
  g: 0.20,
  b: 0.32,
  a: 1.0,
};
const PANEL_BG: Rgba = Rgba {
  r: 0.08,
  g: 0.08,
  b: 0.13,
  a: 1.0,
};

// --- Data ---

struct SpanRow {
  name: &'static str,
  start_ms: f64,
  duration_ms: f64,
  connector: &'static str,
  tags: &'static [(&'static str, &'static str)],
  status: &'static str,
}

struct LocalTrace {
  method: &'static str,
  path: &'static str,
  trace_id: &'static str,
  duration_ms: f64,
  span_count: u32,
  time_ago: &'static str,
  is_error: bool,
}

#[derive(PartialEq)]
enum View {
  Waterfall,
  SpanDetail,
}

struct App {
  selected: usize,
  view: View,
  spans: Vec<SpanRow>,
  local_traces: Vec<LocalTrace>,
}

fn mock_app() -> App {
  App {
    selected: 0,
    view: View::Waterfall,
    spans: vec![
      SpanRow {
        name: "GET /api/feed",
        start_ms: 0.0,
        duration_ms: 91.6,
        connector: "· ",
        tags: &[
          ("http.method", "GET"),
          ("http.url", "/api/feed"),
          ("http.status_code", "200"),
        ],
        status: "ok",
      },
      SpanRow {
        name: "auth.verify",
        start_ms: 0.5,
        duration_ms: 9.10,
        connector: "├─ ",
        tags: &[("auth.method", "jwt"), ("auth.result", "valid")],
        status: "ok",
      },
      SpanRow {
        name: "jwt.decode",
        start_ms: 1.0,
        duration_ms: 2.20,
        connector: "│  ├─ ",
        tags: &[("jwt.alg", "RS256")],
        status: "ok",
      },
      SpanRow {
        name: "jwt.verify-signature",
        start_ms: 3.4,
        duration_ms: 6.21,
        connector: "│  └─ ",
        tags: &[("jwt.alg", "RS256"), ("jwt.kid", "key-2024-03")],
        status: "ok",
      },
      SpanRow {
        name: "feed.assemble",
        start_ms: 9.6,
        duration_ms: 82.0,
        connector: "└─ ",
        tags: &[("feed.items", "20"), ("feed.strategy", "ranked")],
        status: "ok",
      },
      SpanRow {
        name: "postgres SELECT posts",
        start_ms: 12.0,
        duration_ms: 35.9,
        connector: "   ├─ ",
        tags: &[
          (
            "db.statement",
            "SELECT p.* FROM posts p JOIN follows f ON ... LIMIT 20",
          ),
          ("db.system", "postgresql"),
        ],
        status: "ok",
      },
      SpanRow {
        name: "postgres SELECT comments",
        start_ms: 48.0,
        duration_ms: 23.5,
        connector: "   ├─ ",
        tags: &[
          (
            "db.statement",
            "SELECT c.* FROM comments c WHERE post_id IN (...)",
          ),
          ("db.system", "postgresql"),
          ("otel.scope.name", "demo-api"),
          ("span.kind", "internal"),
        ],
        status: "ok",
      },
      SpanRow {
        name: "redis MGET likes",
        start_ms: 72.0,
        duration_ms: 8.53,
        connector: "   ├─ ",
        tags: &[
          ("db.system", "redis"),
          ("db.operation", "MGET"),
          ("redis.keys", "20"),
        ],
        status: "ok",
      },
      SpanRow {
        name: "render.templates",
        start_ms: 78.4,
        duration_ms: 13.2,
        connector: "   └─ ",
        tags: &[
          ("template.name", "feed/index"),
          ("template.engine", "handlebars"),
        ],
        status: "ok",
      },
    ],
    local_traces: vec![
      LocalTrace {
        method: "GET",
        path: "/api/feed",
        trace_id: "#b671de",
        duration_ms: 91.6,
        span_count: 9,
        time_ago: "1h",
        is_error: false,
      },
      LocalTrace {
        method: "GET",
        path: "/api/users/456",
        trace_id: "#de94a0",
        duration_ms: 25.3,
        span_count: 3,
        time_ago: "1h",
        is_error: true,
      },
      LocalTrace {
        method: "POST",
        path: "/api/users/create",
        trace_id: "#0799cc",
        duration_ms: 66.5,
        span_count: 8,
        time_ago: "1h",
        is_error: false,
      },
    ],
  }
}

// --- Helpers ---

fn fmt_dur(ms: f64) -> String {
  if ms >= 1000.0 {
    format!("{:.2}s", ms / 1000.0)
  } else if ms >= 100.0 {
    format!("{:.0}ms", ms)
  } else if ms >= 10.0 {
    format!("{:.1}ms", ms)
  } else {
    format!("{:.2}ms", ms)
  }
}

fn draw_text_right(buf: &opentui_core::BufferRef<'_>, text: &str, right_x: u32, y: u32, fg: &Rgba) {
  let x = right_x.saturating_sub(text.len() as u32);
  buf.draw_text(text, x, y, fg, None, 0);
}

// --- Drawing ---

fn draw_header(buf: &opentui_core::BufferRef<'_>, w: u32) {
  buf.fill_rect(0, 0, w, 1, &HEADER_BG);
  buf.draw_text("LETO OTEL", 1, 0, &BRIGHT, Some(&HEADER_BG), 0);
  buf.draw_text("service:", 12, 0, &GRAY, Some(&HEADER_BG), 0);
  buf.draw_text("demo-api", 21, 0, &WHITE, Some(&HEADER_BG), 0);
  let ts = "updated 4/2 5:43 pm";
  draw_text_right(buf, ts, w - 1, 0, &GRAY);
}

fn draw_trace_info(buf: &opentui_core::BufferRef<'_>, w: u32, y: u32) {
  buf.draw_text("TRACE DETAILS", 1, y, &ORANGE, None, 0);
  let status = "healthy · 91.6ms";
  draw_text_right(buf, status, w - 1, y, &GREEN);

  buf.draw_text("GET /api/feed", 1, y + 1, &BRIGHT, None, 0);

  buf.draw_text("demo-api", 1, y + 2, &BLUE, None, 0);
  buf.draw_text("· 9 spans", 10, y + 2, &GRAY, None, 0);
  let ts = "4/2 4:39 pm";
  draw_text_right(buf, ts, w - 1, y + 2, &GRAY);

  let trace_id = "d74ea8e4f8aa2ba0";
  buf.draw_text(trace_id, 1, y + 3, &BLUE, None, 0);
  let url = "http://127.0.0.1:27686/trace/d74ea8e4f8aa2ba08e054c1ae4b671de";
  let max_url = (w as usize).saturating_sub(20);
  let url_display = if url.len() > max_url {
    &url[..max_url]
  } else {
    url
  };
  buf.draw_text(url_display, 19, y + 3, &GRAY, None, 0);
}

fn draw_waterfall(buf: &opentui_core::BufferRef<'_>, w: u32, y: u32, app: &App) {
  let total_ms: f64 = 91.6;
  let name_cols: u32 = 28;
  let dur_cols: u32 = 8;
  let chart_start = name_cols;
  let chart_end = w.saturating_sub(dur_cols + 1);
  let chart_width = chart_end.saturating_sub(chart_start);

  if chart_width < 10 {
    return;
  }

  // Timeline header
  buf.draw_text("0", chart_start, y, &GRAY, None, 0);
  let mid = fmt_dur(total_ms / 2.0);
  let mid_x = chart_start + chart_width / 2 - mid.len() as u32 / 2;
  buf.draw_text(&mid, mid_x, y, &GRAY, None, 0);
  let end_label = fmt_dur(total_ms);
  draw_text_right(buf, &end_label, w - 1, y, &GRAY);

  // Spans
  for (i, span) in app.spans.iter().enumerate() {
    let row_y = y + 1 + i as u32;
    let selected = i == app.selected && app.view == View::Waterfall;

    if selected {
      buf.fill_rect(0, row_y, w, 1, &SEL_BG);
    }

    let row_bg = if selected { Some(&SEL_BG) } else { None };

    // Tree connector
    buf.draw_text(span.connector, 1, row_y, &DIM, row_bg, 0);

    // Span name
    let name_x = 1 + span.connector.chars().count() as u32;
    let name_color = if selected { &BRIGHT } else { &WHITE };
    buf.draw_text(span.name, name_x, row_y, name_color, row_bg, 0);

    // Bar
    let bar_start_f = (span.start_ms / total_ms) * chart_width as f64;
    let bar_width_f = (span.duration_ms / total_ms) * chart_width as f64;
    let bar_start = chart_start + bar_start_f as u32;
    let bar_width = (bar_width_f as u32).max(1);

    // Dots before bar
    if bar_start > chart_start {
      let dot_count = (bar_start - chart_start) as usize;
      let dots: String = "·".repeat(dot_count);
      buf.draw_text(&dots, chart_start, row_y, &DIM, row_bg, 0);
    }

    // Bar
    let bar_end = (bar_start + bar_width).min(chart_end);
    let actual_bar_width = bar_end.saturating_sub(bar_start);
    if actual_bar_width > 0 {
      let bar: String = "█".repeat(actual_bar_width as usize);
      buf.draw_text(&bar, bar_start, row_y, &AMBER, row_bg, 0);
    }

    // Duration
    let dur = fmt_dur(span.duration_ms);
    draw_text_right(buf, &dur, w - 1, row_y, &AMBER);
  }
}

fn draw_detail_preview(buf: &opentui_core::BufferRef<'_>, _w: u32, y: u32, app: &App) {
  let span = &app.spans[app.selected];
  for (i, (key, val)) in span.tags.iter().enumerate().take(2) {
    buf.draw_text(key, 1, y + i as u32, &GRAY, None, 0);
    let val_x = 16_u32.max(key.len() as u32 + 2);
    buf.draw_text(val, val_x, y + i as u32, &WHITE, None, 0);
  }
}

fn draw_span_detail(buf: &opentui_core::BufferRef<'_>, w: u32, y: u32, app: &App) {
  let span = &app.spans[app.selected];

  buf.draw_text("SPAN DETAIL", 1, y, &ORANGE, None, 0);
  let status_text = format!("{} · {}", span.status, fmt_dur(span.duration_ms));
  draw_text_right(buf, &status_text, w - 1, y, &GREEN);

  buf.draw_text(span.name, 1, y + 1, &BRIGHT, None, 0);

  let meta = format!("demo-api · {} · {}", fmt_dur(span.duration_ms), span.status);
  buf.draw_text(&meta, 1, y + 2, &BLUE, None, 0);

  buf.draw_text("TAGS", 1, y + 4, &ORANGE, None, 0);

  for (i, (key, val)) in span.tags.iter().enumerate() {
    let tag_y = y + 5 + i as u32;
    buf.draw_text(key, 1, tag_y, &GRAY, None, 0);
    let val_x = 16_u32.max(key.len() as u32 + 2);
    let display_val = if val.len() > (w as usize - val_x as usize - 2) {
      &val[..(w as usize - val_x as usize - 2).min(val.len())]
    } else {
      val
    };
    buf.draw_text(display_val, val_x, tag_y, &WHITE, None, 0);
  }
}

fn draw_local_traces(buf: &opentui_core::BufferRef<'_>, w: u32, h: u32, app: &App) {
  let panel_h = app.local_traces.len() as u32 + 1;
  let panel_y = h.saturating_sub(panel_h + 1);

  // Separator
  buf.fill_rect(0, panel_y, w, 1, &PANEL_BG);

  // Header
  let header_y = panel_y;
  buf.fill_rect(0, header_y, w, 1, &PANEL_BG);
  buf.draw_text("LOCAL TRACES", 1, header_y, &ORANGE, Some(&PANEL_BG), 0);
  let svc = "service · demo-api (3) · http://127.0.0.1:27686";
  draw_text_right(buf, svc, w - 1, header_y, &GRAY);

  // Traces
  for (i, trace) in app.local_traces.iter().enumerate() {
    let row_y = header_y + 1 + i as u32;

    let indicator = if trace.is_error { "!" } else { "·" };
    let ind_color = if trace.is_error { &RED } else { &GREEN };
    buf.draw_text(indicator, 1, row_y, ind_color, None, 0);

    let entry = format!("{} {} {}", trace.method, trace.path, trace.trace_id);
    buf.draw_text(&entry, 3, row_y, &WHITE, None, 0);

    let dur = fmt_dur(trace.duration_ms);
    let stats = format!("{}   {}sp   {}", dur, trace.span_count, trace.time_ago);
    draw_text_right(buf, &stats, w - 1, row_y, &GRAY);

    // Highlight duration in amber
    let dur_start = w - 1 - stats.len() as u32;
    buf.draw_text(&dur, dur_start, row_y, &AMBER, None, 0);
  }
}

fn render_frame(renderer: &mut Renderer, app: &App, w: u32, h: u32) {
  {
    let buf = renderer.next_buffer();
    buf.clear(&BG);

    draw_header(&buf, w);

    match app.view {
      View::Waterfall => {
        draw_trace_info(&buf, w, 2);
        draw_waterfall(&buf, w, 7, app);

        // Detail preview area
        let preview_y = 7 + 1 + app.spans.len() as u32 + 2;
        let panel_top = h.saturating_sub(app.local_traces.len() as u32 + 2);
        if preview_y + 2 < panel_top {
          draw_detail_preview(&buf, w, preview_y, app);
        }
      }
      View::SpanDetail => {
        draw_span_detail(&buf, w, 2, app);
      }
    }

    draw_local_traces(&buf, w, h, app);
  }
  renderer.render(false);
}

fn main() -> io::Result<()> {
  // Panic hook to restore terminal
  let default_hook = std::panic::take_hook();
  std::panic::set_hook(Box::new(move |info| {
    let _ = terminal::disable_raw_mode();
    let _ = crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show);
    default_hook(info);
  }));

  terminal::enable_raw_mode()?;
  crossterm::execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;

  let (cols, rows) = terminal::size()?;
  let (mut w, mut h) = (cols as u32, rows as u32);

  let mut renderer = Renderer::new(w, h).expect("failed to create renderer");
  let mut app = mock_app();

  // Initial render
  render_frame(&mut renderer, &app, w, h);

  loop {
    match event::read()? {
      Event::Key(key) => match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => break,
        KeyCode::Up | KeyCode::Char('k') => {
          if app.view == View::Waterfall && app.selected > 0 {
            app.selected -= 1;
            render_frame(&mut renderer, &app, w, h);
          }
        }
        KeyCode::Down | KeyCode::Char('j') => {
          if app.view == View::Waterfall && app.selected < app.spans.len() - 1 {
            app.selected += 1;
            render_frame(&mut renderer, &app, w, h);
          }
        }
        KeyCode::Enter => {
          if app.view == View::Waterfall {
            app.view = View::SpanDetail;
            render_frame(&mut renderer, &app, w, h);
          }
        }
        KeyCode::Esc | KeyCode::Backspace => {
          if app.view == View::SpanDetail {
            app.view = View::Waterfall;
            render_frame(&mut renderer, &app, w, h);
          }
        }
        _ => {}
      },
      Event::Resize(cols, rows) => {
        w = cols as u32;
        h = rows as u32;
        renderer.resize(w, h);
        // Force full redraw
        {
          let buf = renderer.next_buffer();
          buf.clear(&BG);
        }
        renderer.render(true);
        render_frame(&mut renderer, &app, w, h);
      }
      _ => {}
    }
  }

  crossterm::execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
