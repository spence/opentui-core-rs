use std::marker::PhantomData;
use std::ptr::NonNull;

use opentui_core_sys as sys;

use crate::color::Rgba;
use crate::editor_view::EditorView;
use crate::error::{Error, Result};
use crate::renderer::WidthMethod;
use crate::text_buffer_view::TextBufferView;

/// A borrowed reference to a buffer managed by a [`Renderer`](crate::Renderer).
///
/// Does not free the buffer on drop — the renderer owns it.
pub struct BufferRef<'a> {
  ptr: NonNull<sys::OptimizedBuffer>,
  _marker: PhantomData<&'a ()>,
}

impl<'a> BufferRef<'a> {
  pub(crate) fn from_raw(ptr: NonNull<sys::OptimizedBuffer>) -> Self {
    Self {
      ptr,
      _marker: PhantomData,
    }
  }
}

impl BufferRef<'_> {
  fn as_ptr(&self) -> *mut sys::OptimizedBuffer {
    self.ptr.as_ptr()
  }
}

impl std::ops::Deref for BufferRef<'_> {
  type Target = BufferOps;

  fn deref(&self) -> &BufferOps {
    // SAFETY: BufferOps is a ZST trait-object-like shim; we reconstruct it
    // from the pointer in each method via the Sealed trait.
    //
    // In practice we never dereference this — all methods go through the
    // `BufferOps` impl on `BufferRef` directly. But Deref is required so
    // callers can use BufferRef and Buffer interchangeably.
    unreachable!("BufferOps methods are called directly on BufferRef/Buffer")
  }
}

/// An owned buffer created via [`Buffer::new`].
///
/// Freed on drop.
pub struct Buffer {
  ptr: NonNull<sys::OptimizedBuffer>,
}

impl Buffer {
  /// Create a standalone buffer.
  pub fn new(
    width: u32,
    height: u32,
    respect_alpha: bool,
    width_method: WidthMethod,
    id: &str,
  ) -> Result<Self> {
    // SAFETY: Returns null on failure.
    let ptr = unsafe {
      sys::createOptimizedBuffer(
        width,
        height,
        respect_alpha,
        width_method as u8,
        id.as_ptr(),
        id.len(),
      )
    };
    NonNull::new(ptr)
      .map(|ptr| Self { ptr })
      .ok_or(Error::CreationFailed("buffer"))
  }

  fn as_ptr(&self) -> *mut sys::OptimizedBuffer {
    self.ptr.as_ptr()
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    // SAFETY: We own this buffer.
    unsafe { sys::destroyOptimizedBuffer(self.ptr.as_ptr()) }
  }
}

// SAFETY: Buffer data is not shared across threads by the Zig core.
unsafe impl Send for Buffer {}

/// Drawing operations shared between [`Buffer`] and [`BufferRef`].
///
/// This is not a real type — it exists only so both buffer types share the same
/// method set without code duplication via a macro.
pub struct BufferOps {
  _private: (),
}

macro_rules! impl_buffer_ops {
  ($ty:ty) => {
    impl $ty {
      pub fn width(&self) -> u32 {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::getBufferWidth(self.as_ptr()) }
      }

      pub fn height(&self) -> u32 {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::getBufferHeight(self.as_ptr()) }
      }

      pub fn clear(&self, bg: &Rgba) {
        // SAFETY: Pointers valid for the call.
        unsafe { sys::bufferClear(self.as_ptr(), bg.as_ptr()) }
      }

      pub fn draw_text(
        &self,
        text: &str,
        x: u32,
        y: u32,
        fg: &Rgba,
        bg: Option<&Rgba>,
        attributes: u32,
      ) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawText(
            self.as_ptr(),
            text.as_ptr(),
            text.len(),
            x,
            y,
            fg.as_ptr(),
            bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
            attributes,
          );
        }
      }

      pub fn draw_char(&self, ch: u32, x: u32, y: u32, fg: &Rgba, bg: &Rgba, attributes: u32) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawChar(
            self.as_ptr(),
            ch,
            x,
            y,
            fg.as_ptr(),
            bg.as_ptr(),
            attributes,
          );
        }
      }

      pub fn set_cell(&self, x: u32, y: u32, ch: u32, fg: &Rgba, bg: &Rgba, attributes: u32) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferSetCell(
            self.as_ptr(),
            x,
            y,
            ch,
            fg.as_ptr(),
            bg.as_ptr(),
            attributes,
          );
        }
      }

      pub fn set_cell_blended(
        &self,
        x: u32,
        y: u32,
        ch: u32,
        fg: &Rgba,
        bg: &Rgba,
        attributes: u32,
      ) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferSetCellWithAlphaBlending(
            self.as_ptr(),
            x,
            y,
            ch,
            fg.as_ptr(),
            bg.as_ptr(),
            attributes,
          );
        }
      }

      pub fn fill_rect(&self, x: u32, y: u32, width: u32, height: u32, bg: &Rgba) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferFillRect(self.as_ptr(), x, y, width, height, bg.as_ptr());
        }
      }

      pub fn draw_box(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_chars: &[u32],
        packed_options: u32,
        border_color: &Rgba,
        bg_color: &Rgba,
        title: Option<&str>,
        bottom_title: Option<&str>,
      ) {
        let (title_ptr, title_len) = title
          .map(|t| (t.as_ptr(), t.len() as u32))
          .unwrap_or((std::ptr::null(), 0));
        let (btitle_ptr, btitle_len) = bottom_title
          .map(|t| (t.as_ptr(), t.len() as u32))
          .unwrap_or((std::ptr::null(), 0));
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawBox(
            self.as_ptr(),
            x,
            y,
            width,
            height,
            border_chars.as_ptr(),
            packed_options,
            border_color.as_ptr(),
            bg_color.as_ptr(),
            title_ptr,
            title_len,
            btitle_ptr,
            btitle_len,
          );
        }
      }

      pub fn draw_grid(
        &self,
        border_chars: &[u32],
        border_fg: &Rgba,
        border_bg: &Rgba,
        column_offsets: &[i32],
        row_offsets: &[i32],
        draw_inner: bool,
        draw_outer: bool,
      ) {
        let options = sys::ExternalGridDrawOptions {
          draw_inner,
          draw_outer,
        };
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawGrid(
            self.as_ptr(),
            border_chars.as_ptr(),
            border_fg.as_ptr(),
            border_bg.as_ptr(),
            column_offsets.as_ptr(),
            column_offsets.len() as u32,
            row_offsets.as_ptr(),
            row_offsets.len() as u32,
            &options,
          );
        }
      }

      pub fn draw_frame_buffer(
        &self,
        dest_x: i32,
        dest_y: i32,
        source: &Buffer,
        source_x: u32,
        source_y: u32,
        source_width: u32,
        source_height: u32,
      ) {
        // SAFETY: Both buffer pointers are valid.
        unsafe {
          sys::drawFrameBuffer(
            self.as_ptr(),
            dest_x,
            dest_y,
            source.as_ptr(),
            source_x,
            source_y,
            source_width,
            source_height,
          );
        }
      }

      pub fn draw_packed_buffer(
        &self,
        data: &[u8],
        pos_x: u32,
        pos_y: u32,
        terminal_width: u32,
        terminal_height: u32,
      ) {
        // SAFETY: Pointer valid for the call.
        unsafe {
          sys::bufferDrawPackedBuffer(
            self.as_ptr(),
            data.as_ptr(),
            data.len(),
            pos_x,
            pos_y,
            terminal_width,
            terminal_height,
          );
        }
      }

      pub fn draw_grayscale_buffer(
        &self,
        pos_x: i32,
        pos_y: i32,
        intensities: &[f32],
        src_width: u32,
        src_height: u32,
        fg: Option<&Rgba>,
        bg: Option<&Rgba>,
      ) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawGrayscaleBuffer(
            self.as_ptr(),
            pos_x,
            pos_y,
            intensities.as_ptr(),
            src_width,
            src_height,
            fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
            bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
          );
        }
      }

      pub fn draw_grayscale_buffer_supersampled(
        &self,
        pos_x: i32,
        pos_y: i32,
        intensities: &[f32],
        src_width: u32,
        src_height: u32,
        fg: Option<&Rgba>,
        bg: Option<&Rgba>,
      ) {
        // SAFETY: Pointers valid for the call.
        unsafe {
          sys::bufferDrawGrayscaleBufferSupersampled(
            self.as_ptr(),
            pos_x,
            pos_y,
            intensities.as_ptr(),
            src_width,
            src_height,
            fg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
            bg.map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
          );
        }
      }

      pub fn draw_supersample_buffer(
        &self,
        x: u32,
        y: u32,
        pixel_data: &[u8],
        format: u8,
        aligned_bytes_per_row: u32,
      ) {
        // SAFETY: Pointer valid for the call.
        unsafe {
          sys::bufferDrawSuperSampleBuffer(
            self.as_ptr(),
            x,
            y,
            pixel_data.as_ptr(),
            pixel_data.len(),
            format,
            aligned_bytes_per_row,
          );
        }
      }

      // Color matrix

      pub fn color_matrix(&self, matrix: &[f32; 16], cell_mask: &[f32], strength: f32, target: u8) {
        // SAFETY: Pointers valid for the call. cell_mask length is count * 3.
        unsafe {
          sys::bufferColorMatrix(
            self.as_ptr(),
            matrix.as_ptr(),
            cell_mask.as_ptr(),
            cell_mask.len() / 3,
            strength,
            target,
          );
        }
      }

      pub fn color_matrix_uniform(&self, matrix: &[f32; 16], strength: f32, target: u8) {
        // SAFETY: Pointer valid for the call.
        unsafe {
          sys::bufferColorMatrixUniform(self.as_ptr(), matrix.as_ptr(), strength, target);
        }
      }

      // Scissor

      pub fn push_scissor_rect(&self, x: i32, y: i32, width: u32, height: u32) {
        // SAFETY: Buffer pointer is valid.
        unsafe {
          sys::bufferPushScissorRect(self.as_ptr(), x, y, width, height);
        }
      }

      pub fn pop_scissor_rect(&self) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferPopScissorRect(self.as_ptr()) }
      }

      pub fn clear_scissor_rects(&self) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferClearScissorRects(self.as_ptr()) }
      }

      // Opacity

      pub fn push_opacity(&self, opacity: f32) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferPushOpacity(self.as_ptr(), opacity) }
      }

      pub fn pop_opacity(&self) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferPopOpacity(self.as_ptr()) }
      }

      pub fn current_opacity(&self) -> f32 {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferGetCurrentOpacity(self.as_ptr()) }
      }

      pub fn clear_opacity(&self) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferClearOpacity(self.as_ptr()) }
      }

      // Data access

      pub fn respect_alpha(&self) -> bool {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferGetRespectAlpha(self.as_ptr()) }
      }

      pub fn set_respect_alpha(&self, respect_alpha: bool) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferSetRespectAlpha(self.as_ptr(), respect_alpha) }
      }

      pub fn real_char_size(&self) -> u32 {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferGetRealCharSize(self.as_ptr()) }
      }

      pub fn id(&self) -> String {
        let mut buf = vec![0u8; 256];
        // SAFETY: Buffer and output pointer are valid.
        let len = unsafe { sys::bufferGetId(self.as_ptr(), buf.as_mut_ptr(), buf.len()) };
        buf.truncate(len);
        String::from_utf8_lossy(&buf).into_owned()
      }

      pub fn resize(&self, width: u32, height: u32) {
        // SAFETY: Buffer pointer is valid.
        unsafe { sys::bufferResize(self.as_ptr(), width, height) }
      }

      pub fn write_resolved_chars(&self, output: &mut [u8], add_line_breaks: bool) -> u32 {
        // SAFETY: Output pointer is valid.
        unsafe {
          sys::bufferWriteResolvedChars(
            self.as_ptr(),
            output.as_mut_ptr(),
            output.len(),
            add_line_breaks,
          )
        }
      }

      // Composite drawing

      pub fn draw_editor_view(&self, view: &mut EditorView, x: i32, y: i32) {
        // SAFETY: Both pointers are valid.
        unsafe {
          sys::bufferDrawEditorView(self.as_ptr(), view.as_ptr(), x, y);
        }
      }

      pub fn draw_text_buffer_view(&self, view: &mut TextBufferView, x: i32, y: i32) {
        // SAFETY: Both pointers are valid.
        unsafe {
          sys::bufferDrawTextBufferView(self.as_ptr(), view.as_ptr(), x, y);
        }
      }
    }
  };
}

impl_buffer_ops!(BufferRef<'_>);
impl_buffer_ops!(Buffer);
