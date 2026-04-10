use opentui_core_sys as sys;

/// Log levels matching the Zig core's logger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogLevel {
  Debug = 0,
  Info = 1,
  Warn = 2,
  Error = 3,
}

/// Set a callback to receive log messages from the Zig core.
///
/// The callback receives the log level and message bytes. Pass `None` to clear.
///
/// # Safety
///
/// The callback must be safe to call from any thread. The message slice is only
/// valid for the duration of the callback.
pub fn set_log_callback(
  callback: Option<unsafe extern "C" fn(level: u8, msg_ptr: *const u8, msg_len: usize)>,
) {
  // SAFETY: The Zig core stores the function pointer globally and calls it
  // during log operations. The caller is responsible for thread safety.
  unsafe { sys::setLogCallback(callback) }
}

/// Set a callback to receive events from the Zig core.
///
/// # Safety
///
/// Same thread-safety requirements as [`set_log_callback`].
pub fn set_event_callback(
  callback: Option<
    unsafe extern "C" fn(
      name_ptr: *const u8,
      name_len: usize,
      data_ptr: *const u8,
      data_len: usize,
    ),
  >,
) {
  // SAFETY: Same as set_log_callback.
  unsafe { sys::setEventCallback(callback) }
}

/// Returns the total bytes allocated by the Zig arena allocator.
pub fn arena_allocated_bytes() -> usize {
  // SAFETY: Reads a global counter, no side effects.
  unsafe { sys::getArenaAllocatedBytes() }
}

/// Build options the Zig core was compiled with.
#[derive(Debug, Clone, Copy)]
pub struct BuildOptions {
  pub gpa_safe_stats: bool,
  pub gpa_memory_limit_tracking: bool,
}

/// Returns the build options the Zig core was compiled with.
pub fn build_options() -> BuildOptions {
  let mut out = sys::ExternalBuildOptions {
    gpa_safe_stats: false,
    gpa_memory_limit_tracking: false,
  };
  // SAFETY: Writes to a stack-allocated struct.
  unsafe { sys::getBuildOptions(&mut out) };
  BuildOptions {
    gpa_safe_stats: out.gpa_safe_stats,
    gpa_memory_limit_tracking: out.gpa_memory_limit_tracking,
  }
}

/// Allocator statistics from the Zig core.
#[derive(Debug, Clone, Copy)]
pub struct AllocatorStats {
  pub total_requested_bytes: u64,
  pub active_allocations: u64,
  pub small_allocations: u64,
  pub large_allocations: u64,
  pub requested_bytes_valid: bool,
}

/// Returns allocator statistics from the Zig core.
pub fn allocator_stats() -> AllocatorStats {
  let mut out = std::mem::MaybeUninit::<sys::ExternalAllocatorStats>::uninit();
  // SAFETY: Writes to a stack-allocated struct.
  let out = unsafe {
    sys::getAllocatorStats(out.as_mut_ptr());
    out.assume_init()
  };
  AllocatorStats {
    total_requested_bytes: out.total_requested_bytes,
    active_allocations: out.active_allocations,
    small_allocations: out.small_allocations,
    large_allocations: out.large_allocations,
    requested_bytes_valid: out.requested_bytes_valid,
  }
}
