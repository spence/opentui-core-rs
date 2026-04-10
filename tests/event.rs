use opentui_core::{allocator_stats, arena_allocated_bytes, build_options, LogLevel};

#[test]
fn arena_allocated_bytes_returns() {
  let bytes = arena_allocated_bytes();
  // may be 0 if nothing has been allocated yet, that's fine
  let _ = bytes;
}

#[test]
fn build_options_returns() {
  let opts = build_options();
  // we compile without gpa_safe_stats
  assert!(!opts.gpa_safe_stats);
}

#[test]
fn allocator_stats_returns() {
  let stats = allocator_stats();
  let _ = stats.active_allocations;
  let _ = stats.total_requested_bytes;
  let _ = stats.requested_bytes_valid;
}

#[test]
fn log_level_values() {
  assert_eq!(LogLevel::Debug as u8, 0);
  assert_eq!(LogLevel::Info as u8, 1);
  assert_eq!(LogLevel::Warn as u8, 2);
  assert_eq!(LogLevel::Error as u8, 3);
}
