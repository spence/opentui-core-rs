#ifndef OPENTUI_H
#define OPENTUI_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Opaque types
typedef struct CliRenderer CliRenderer;
typedef struct OptimizedBuffer OptimizedBuffer;
typedef struct UnifiedTextBuffer UnifiedTextBuffer;
typedef struct UnifiedTextBufferView UnifiedTextBufferView;
typedef struct EditBuffer EditBuffer;
typedef struct EditorView EditorView;
typedef struct SyntaxStyle SyntaxStyle;
typedef struct NativeSpanFeedStream NativeSpanFeedStream;

// RGBA color: array of 4 floats [r, g, b, a]
typedef float RGBA[4];

// --- External structs ---

typedef struct {
  bool gpa_safe_stats;
  bool gpa_memory_limit_tracking;
} ExternalBuildOptions;

typedef struct {
  uint64_t total_requested_bytes;
  uint64_t active_allocations;
  uint64_t small_allocations;
  uint64_t large_allocations;
  bool requested_bytes_valid;
} ExternalAllocatorStats;

typedef struct {
  const uint8_t* ptr;
  size_t len;
} OutputSlice;

typedef struct {
  bool kitty_keyboard;
  bool kitty_graphics;
  bool rgb;
  uint8_t unicode;
  bool sgr_pixels;
  bool color_scheme_updates;
  bool explicit_width;
  bool scaled_text;
  bool sixel;
  bool focus_tracking;
  bool sync;
  bool bracketed_paste;
  bool hyperlinks;
  bool osc52;
  bool explicit_cursor_positioning;
  const uint8_t* term_name_ptr;
  size_t term_name_len;
  const uint8_t* term_version_ptr;
  size_t term_version_len;
  bool term_from_xtversion;
} ExternalCapabilities;

typedef struct {
  uint8_t style;
  uint8_t blinking;
  const float* color;
  uint8_t cursor;
} CursorStyleOptions;

typedef struct {
  uint32_t x;
  uint32_t y;
  bool visible;
  uint8_t style;
  bool blinking;
  float r;
  float g;
  float b;
  float a;
} ExternalCursorState;

typedef struct {
  bool draw_inner;
  bool draw_outer;
} ExternalGridDrawOptions;

typedef struct {
  uint32_t start;
  uint32_t end;
  uint32_t style_id;
  uint8_t priority;
  uint16_t hl_ref;
} ExternalHighlight;

typedef struct {
  const uint32_t* start_cols_ptr;
  uint32_t start_cols_len;
  const uint32_t* width_cols_ptr;
  uint32_t width_cols_len;
  const uint32_t* sources_ptr;
  uint32_t sources_len;
  const uint32_t* wraps_ptr;
  uint32_t wraps_len;
  uint32_t width_cols_max;
} ExternalLineInfo;

typedef struct {
  uint32_t line_count;
  uint32_t width_cols_max;
} ExternalMeasureResult;

typedef struct {
  uint32_t row;
  uint32_t col;
  uint32_t offset;
} ExternalLogicalCursor;

typedef struct {
  uint32_t visual_row;
  uint32_t visual_col;
  uint32_t logical_row;
  uint32_t logical_col;
  uint32_t offset;
} ExternalVisualCursor;

typedef struct {
  const uint8_t* text_ptr;
  size_t text_len;
  const float* fg_ptr;
  const float* bg_ptr;
  uint32_t attributes;
  const uint8_t* link_ptr;
  size_t link_len;
} StyledChunk;

typedef struct {
  uint8_t width;
  uint32_t char_code;
} EncodedChar;

typedef struct {
  uint32_t chunk_size;
  uint32_t initial_chunks;
  uint64_t max_bytes;
  uint8_t growth_policy;
  uint8_t auto_commit_on_full;
  uint32_t span_queue_capacity;
} NativeSpanFeedOptions;

// --- Callbacks ---

void setLogCallback(
    void (*callback)(uint8_t level, const uint8_t* msg_ptr, size_t msg_len));
void setEventCallback(
    void (*callback)(const uint8_t* name_ptr, size_t name_len,
                     const uint8_t* data_ptr, size_t data_len));

// --- Global state ---

size_t getArenaAllocatedBytes(void);
void getBuildOptions(ExternalBuildOptions* out);
void getAllocatorStats(ExternalAllocatorStats* out);

// --- Renderer ---

CliRenderer* createRenderer(uint32_t width, uint32_t height, bool testing,
                            bool remote);
void destroyRenderer(CliRenderer* renderer);
bool setTerminalEnvVar(CliRenderer* renderer, const uint8_t* key_ptr,
                       size_t key_len, const uint8_t* value_ptr,
                       size_t value_len);
void setUseThread(CliRenderer* renderer, bool use_thread);
void resizeRenderer(CliRenderer* renderer, uint32_t width, uint32_t height);

OptimizedBuffer* getNextBuffer(CliRenderer* renderer);
OptimizedBuffer* getCurrentBuffer(CliRenderer* renderer);
void render(CliRenderer* renderer, bool force);
void getLastOutputForTest(CliRenderer* renderer, OutputSlice* out);

void updateStats(CliRenderer* renderer, double time, uint32_t fps,
                 double frame_callback_time);
void updateMemoryStats(CliRenderer* renderer, uint32_t heap_used,
                       uint32_t heap_total, uint32_t array_buffers);
void setBackgroundColor(CliRenderer* renderer, const float* color);
void setRenderOffset(CliRenderer* renderer, uint32_t offset);

// Terminal capabilities
void getTerminalCapabilities(CliRenderer* renderer,
                             ExternalCapabilities* out);
void processCapabilityResponse(CliRenderer* renderer,
                               const uint8_t* response_ptr,
                               size_t response_len);
void setHyperlinksCapability(CliRenderer* renderer, bool enabled);

// Cursor
void setCursorPosition(CliRenderer* renderer, int32_t x, int32_t y,
                       bool visible);
void setCursorColor(CliRenderer* renderer, const float* color);
void setCursorStyleOptions(CliRenderer* renderer,
                           const CursorStyleOptions* options);
void getCursorState(CliRenderer* renderer, ExternalCursorState* out);

// Terminal control
void clearTerminal(CliRenderer* renderer);
void setTerminalTitle(CliRenderer* renderer, const uint8_t* title_ptr,
                      size_t title_len);
bool copyToClipboardOSC52(CliRenderer* renderer, uint8_t target,
                          const uint8_t* payload_ptr, size_t payload_len);
bool clearClipboardOSC52(CliRenderer* renderer, uint8_t target);
void restoreTerminalModes(CliRenderer* renderer);
void setupTerminal(CliRenderer* renderer, bool use_alternate_screen);
void suspendRenderer(CliRenderer* renderer);
void resumeRenderer(CliRenderer* renderer);

// Debug
void setDebugOverlay(CliRenderer* renderer, bool enabled, uint8_t corner);
void dumpBuffers(CliRenderer* renderer, int64_t timestamp);
void dumpStdoutBuffer(CliRenderer* renderer, int64_t timestamp);

// Mouse
void enableMouse(CliRenderer* renderer, bool enable_movement);
void disableMouse(CliRenderer* renderer);

// Keyboard
void enableKittyKeyboard(CliRenderer* renderer, uint8_t flags);
void disableKittyKeyboard(CliRenderer* renderer);
void setKittyKeyboardFlags(CliRenderer* renderer, uint8_t flags);
uint8_t getKittyKeyboardFlags(CliRenderer* renderer);

// Output
void writeOut(CliRenderer* renderer, const uint8_t* data_ptr, size_t data_len);
void queryPixelResolution(CliRenderer* renderer);

// --- Hit grid ---

void addToHitGrid(CliRenderer* renderer, int32_t x, int32_t y, uint32_t width,
                  uint32_t height, uint32_t id);
void clearCurrentHitGrid(CliRenderer* renderer);
void hitGridPushScissorRect(CliRenderer* renderer, int32_t x, int32_t y,
                            uint32_t width, uint32_t height);
void hitGridPopScissorRect(CliRenderer* renderer);
void hitGridClearScissorRects(CliRenderer* renderer);
void addToCurrentHitGridClipped(CliRenderer* renderer, int32_t x, int32_t y,
                                uint32_t width, uint32_t height, uint32_t id);
uint32_t checkHit(CliRenderer* renderer, uint32_t x, uint32_t y);
bool getHitGridDirty(CliRenderer* renderer);
void dumpHitGrid(CliRenderer* renderer);

// --- OptimizedBuffer ---

OptimizedBuffer* createOptimizedBuffer(uint32_t width, uint32_t height,
                                       bool respect_alpha,
                                       uint8_t width_method,
                                       const uint8_t* id_ptr, size_t id_len);
void destroyOptimizedBuffer(OptimizedBuffer* buffer);
void destroyFrameBuffer(OptimizedBuffer* buffer);
uint32_t getBufferWidth(OptimizedBuffer* buffer);
uint32_t getBufferHeight(OptimizedBuffer* buffer);

// Drawing
void bufferClear(OptimizedBuffer* buffer, const float* bg);
void bufferDrawText(OptimizedBuffer* buffer, const uint8_t* text,
                    size_t text_len, uint32_t x, uint32_t y, const float* fg,
                    const float* bg, uint32_t attributes);
void bufferDrawChar(OptimizedBuffer* buffer, uint32_t char_code, uint32_t x,
                    uint32_t y, const float* fg, const float* bg,
                    uint32_t attributes);
void bufferSetCell(OptimizedBuffer* buffer, uint32_t x, uint32_t y,
                   uint32_t char_code, const float* fg, const float* bg,
                   uint32_t attributes);
void bufferSetCellWithAlphaBlending(OptimizedBuffer* buffer, uint32_t x,
                                    uint32_t y, uint32_t char_code,
                                    const float* fg, const float* bg,
                                    uint32_t attributes);
void drawFrameBuffer(OptimizedBuffer* target, int32_t dest_x, int32_t dest_y,
                     OptimizedBuffer* frame_buffer, uint32_t source_x,
                     uint32_t source_y, uint32_t source_width,
                     uint32_t source_height);
void bufferFillRect(OptimizedBuffer* buffer, uint32_t x, uint32_t y,
                    uint32_t width, uint32_t height, const float* bg);
void bufferDrawGrid(OptimizedBuffer* buffer, const uint32_t* border_chars,
                    const float* border_fg, const float* border_bg,
                    const int32_t* column_offsets, uint32_t column_count,
                    const int32_t* row_offsets, uint32_t row_count,
                    const ExternalGridDrawOptions* options);
void bufferDrawBox(OptimizedBuffer* buffer, int32_t x, int32_t y,
                   uint32_t width, uint32_t height,
                   const uint32_t* border_chars, uint32_t packed_options,
                   const float* border_color, const float* background_color,
                   const uint8_t* title, uint32_t title_len,
                   const uint8_t* bottom_title, uint32_t bottom_title_len);
void bufferDrawPackedBuffer(OptimizedBuffer* buffer, const uint8_t* data,
                            size_t data_len, uint32_t pos_x, uint32_t pos_y,
                            uint32_t terminal_width_cells,
                            uint32_t terminal_height_cells);
void bufferDrawGrayscaleBuffer(OptimizedBuffer* buffer, int32_t pos_x,
                               int32_t pos_y, const float* intensities,
                               uint32_t src_width, uint32_t src_height,
                               const float* fg, const float* bg);
void bufferDrawGrayscaleBufferSupersampled(
    OptimizedBuffer* buffer, int32_t pos_x, int32_t pos_y,
    const float* intensities, uint32_t src_width, uint32_t src_height,
    const float* fg, const float* bg);
void bufferDrawSuperSampleBuffer(OptimizedBuffer* buffer, uint32_t x,
                                 uint32_t y, const uint8_t* pixel_data,
                                 size_t len, uint8_t format,
                                 uint32_t aligned_bytes_per_row);

// Color matrix
void bufferColorMatrix(OptimizedBuffer* buffer, const float* matrix,
                       const float* cell_mask, size_t cell_mask_count,
                       float strength, uint8_t target);
void bufferColorMatrixUniform(OptimizedBuffer* buffer, const float* matrix,
                              float strength, uint8_t target);

// Scissor
void bufferPushScissorRect(OptimizedBuffer* buffer, int32_t x, int32_t y,
                           uint32_t width, uint32_t height);
void bufferPopScissorRect(OptimizedBuffer* buffer);
void bufferClearScissorRects(OptimizedBuffer* buffer);

// Opacity
void bufferPushOpacity(OptimizedBuffer* buffer, float opacity);
void bufferPopOpacity(OptimizedBuffer* buffer);
float bufferGetCurrentOpacity(OptimizedBuffer* buffer);
void bufferClearOpacity(OptimizedBuffer* buffer);

// Data access
uint32_t* bufferGetCharPtr(OptimizedBuffer* buffer);
float* bufferGetFgPtr(OptimizedBuffer* buffer);
float* bufferGetBgPtr(OptimizedBuffer* buffer);
uint32_t* bufferGetAttributesPtr(OptimizedBuffer* buffer);
uint32_t bufferGetRealCharSize(OptimizedBuffer* buffer);
bool bufferGetRespectAlpha(OptimizedBuffer* buffer);
void bufferSetRespectAlpha(OptimizedBuffer* buffer, bool respect_alpha);
size_t bufferGetId(OptimizedBuffer* buffer, uint8_t* out_ptr, size_t max_len);
void bufferResize(OptimizedBuffer* buffer, uint32_t width, uint32_t height);
uint32_t bufferWriteResolvedChars(OptimizedBuffer* buffer, uint8_t* output_ptr,
                                  size_t output_len, bool add_line_breaks);

// Composite drawing
void bufferDrawEditorView(OptimizedBuffer* buffer, EditorView* view,
                          int32_t x, int32_t y);
void bufferDrawTextBufferView(OptimizedBuffer* buffer,
                              UnifiedTextBufferView* view, int32_t x,
                              int32_t y);

// --- Hyperlinks ---

void clearGlobalLinkPool(void);
uint32_t linkAlloc(const uint8_t* url_ptr, size_t url_len);
size_t linkGetUrl(uint32_t id, uint8_t* out_ptr, size_t max_len);
uint32_t attributesWithLink(uint32_t base_attributes, uint32_t link_id);
uint32_t attributesGetLinkId(uint32_t attributes);

// --- TextBuffer (UnifiedTextBuffer) ---

UnifiedTextBuffer* createTextBuffer(uint8_t width_method);
void destroyTextBuffer(UnifiedTextBuffer* tb);

// Properties
uint32_t textBufferGetLength(UnifiedTextBuffer* tb);
uint32_t textBufferGetByteSize(UnifiedTextBuffer* tb);
uint32_t textBufferGetLineCount(UnifiedTextBuffer* tb);

// Content
void textBufferReset(UnifiedTextBuffer* tb);
void textBufferClear(UnifiedTextBuffer* tb);
void textBufferAppend(UnifiedTextBuffer* tb, const uint8_t* data_ptr,
                      size_t data_len);
bool textBufferLoadFile(UnifiedTextBuffer* tb, const uint8_t* path_ptr,
                        size_t path_len);
void textBufferSetStyledText(UnifiedTextBuffer* tb,
                             const StyledChunk* chunks_ptr,
                             size_t chunk_count);

// Memory buffer registry
uint16_t textBufferRegisterMemBuffer(UnifiedTextBuffer* tb,
                                     const uint8_t* data_ptr, size_t data_len,
                                     bool owned);
bool textBufferReplaceMemBuffer(UnifiedTextBuffer* tb, uint8_t id,
                                const uint8_t* data_ptr, size_t data_len,
                                bool owned);
void textBufferClearMemRegistry(UnifiedTextBuffer* tb);
void textBufferSetTextFromMem(UnifiedTextBuffer* tb, uint8_t id);
void textBufferAppendFromMemId(UnifiedTextBuffer* tb, uint8_t id);

// Style defaults
void textBufferSetDefaultFg(UnifiedTextBuffer* tb, const float* fg);
void textBufferSetDefaultBg(UnifiedTextBuffer* tb, const float* bg);
void textBufferSetDefaultAttributes(UnifiedTextBuffer* tb,
                                    const uint32_t* attr);
void textBufferResetDefaults(UnifiedTextBuffer* tb);

// Tabs
uint8_t textBufferGetTabWidth(UnifiedTextBuffer* tb);
void textBufferSetTabWidth(UnifiedTextBuffer* tb, uint8_t width);

// Text extraction
size_t textBufferGetPlainText(UnifiedTextBuffer* tb, uint8_t* out_ptr,
                              size_t max_len);
size_t textBufferGetTextRange(UnifiedTextBuffer* tb, uint32_t start_offset,
                              uint32_t end_offset, uint8_t* out_ptr,
                              size_t max_len);
size_t textBufferGetTextRangeByCoords(UnifiedTextBuffer* tb, uint32_t start_row,
                                      uint32_t start_col, uint32_t end_row,
                                      uint32_t end_col, uint8_t* out_ptr,
                                      size_t max_len);

// Highlights
void textBufferAddHighlightByCharRange(UnifiedTextBuffer* tb,
                                       const ExternalHighlight* hl);
void textBufferAddHighlight(UnifiedTextBuffer* tb, uint32_t line_idx,
                            const ExternalHighlight* hl);
void textBufferRemoveHighlightsByRef(UnifiedTextBuffer* tb, uint16_t hl_ref);
void textBufferClearLineHighlights(UnifiedTextBuffer* tb, uint32_t line_idx);
void textBufferClearAllHighlights(UnifiedTextBuffer* tb);
const ExternalHighlight* textBufferGetLineHighlightsPtr(UnifiedTextBuffer* tb,
                                                        uint32_t line_idx,
                                                        size_t* out_count);
void textBufferFreeLineHighlights(const ExternalHighlight* ptr, size_t count);
uint32_t textBufferGetHighlightCount(UnifiedTextBuffer* tb);

// Syntax
void textBufferSetSyntaxStyle(UnifiedTextBuffer* tb, SyntaxStyle* style);

// --- TextBufferView ---

UnifiedTextBufferView* createTextBufferView(UnifiedTextBuffer* tb);
void destroyTextBufferView(UnifiedTextBufferView* view);

// Selection
void textBufferViewSetSelection(UnifiedTextBufferView* view, uint32_t start,
                                uint32_t end, const float* bg_color,
                                const float* fg_color);
void textBufferViewResetSelection(UnifiedTextBufferView* view);
uint64_t textBufferViewGetSelectionInfo(UnifiedTextBufferView* view);
bool textBufferViewSetLocalSelection(UnifiedTextBufferView* view,
                                     int32_t anchor_x, int32_t anchor_y,
                                     int32_t focus_x, int32_t focus_y,
                                     const float* bg_color,
                                     const float* fg_color);
void textBufferViewUpdateSelection(UnifiedTextBufferView* view, uint32_t end,
                                   const float* bg_color,
                                   const float* fg_color);
bool textBufferViewUpdateLocalSelection(UnifiedTextBufferView* view,
                                        int32_t anchor_x, int32_t anchor_y,
                                        int32_t focus_x, int32_t focus_y,
                                        const float* bg_color,
                                        const float* fg_color);
void textBufferViewResetLocalSelection(UnifiedTextBufferView* view);

// Layout
void textBufferViewSetWrapWidth(UnifiedTextBufferView* view, uint32_t width);
void textBufferViewSetWrapMode(UnifiedTextBufferView* view, uint8_t mode);
void textBufferViewSetViewportSize(UnifiedTextBufferView* view, uint32_t width,
                                   uint32_t height);
void textBufferViewSetViewport(UnifiedTextBufferView* view, uint32_t x,
                               uint32_t y, uint32_t width, uint32_t height);
uint32_t textBufferViewGetVirtualLineCount(UnifiedTextBufferView* view);

// Line info
void textBufferViewGetLineInfoDirect(UnifiedTextBufferView* view,
                                     ExternalLineInfo* out);
void textBufferViewGetLogicalLineInfoDirect(UnifiedTextBufferView* view,
                                            ExternalLineInfo* out);

// Text extraction
size_t textBufferViewGetSelectedText(UnifiedTextBufferView* view,
                                     uint8_t* out_ptr, size_t max_len);
size_t textBufferViewGetPlainText(UnifiedTextBufferView* view,
                                  uint8_t* out_ptr, size_t max_len);

// Display
void textBufferViewSetTabIndicator(UnifiedTextBufferView* view,
                                   uint32_t indicator);
void textBufferViewSetTabIndicatorColor(UnifiedTextBufferView* view,
                                        const float* color);
void textBufferViewSetTruncate(UnifiedTextBufferView* view, bool truncate);

// Measurement
bool textBufferViewMeasureForDimensions(UnifiedTextBufferView* view,
                                        uint32_t width, uint32_t height,
                                        ExternalMeasureResult* out);

// --- EditBuffer ---

EditBuffer* createEditBuffer(uint8_t width_method);
void destroyEditBuffer(EditBuffer* eb);
UnifiedTextBuffer* editBufferGetTextBuffer(EditBuffer* eb);

// Editing
void editBufferInsertText(EditBuffer* eb, const uint8_t* text_ptr,
                          size_t text_len);
void editBufferDeleteRange(EditBuffer* eb, uint32_t start_row,
                           uint32_t start_col, uint32_t end_row,
                           uint32_t end_col);
void editBufferDeleteCharBackward(EditBuffer* eb);
void editBufferDeleteChar(EditBuffer* eb);
void editBufferInsertChar(EditBuffer* eb, const uint8_t* char_ptr,
                          size_t char_len);
void editBufferNewLine(EditBuffer* eb);
void editBufferDeleteLine(EditBuffer* eb);
void editBufferSetText(EditBuffer* eb, const uint8_t* text_ptr,
                       size_t text_len);
void editBufferSetTextFromMem(EditBuffer* eb, uint8_t mem_id);
void editBufferReplaceText(EditBuffer* eb, const uint8_t* text_ptr,
                           size_t text_len);
void editBufferReplaceTextFromMem(EditBuffer* eb, uint8_t mem_id);
void editBufferClear(EditBuffer* eb);

// Cursor movement
void editBufferMoveCursorLeft(EditBuffer* eb);
void editBufferMoveCursorRight(EditBuffer* eb);
void editBufferMoveCursorUp(EditBuffer* eb);
void editBufferMoveCursorDown(EditBuffer* eb);

// Cursor position
void editBufferGetCursor(EditBuffer* eb, uint32_t* out_row,
                         uint32_t* out_col);
void editBufferSetCursor(EditBuffer* eb, uint32_t row, uint32_t col);
void editBufferSetCursorToLineCol(EditBuffer* eb, uint32_t row, uint32_t col);
void editBufferSetCursorByOffset(EditBuffer* eb, uint32_t offset);
void editBufferGotoLine(EditBuffer* eb, uint32_t line);
void editBufferGetCursorPosition(EditBuffer* eb, ExternalLogicalCursor* out);

// Word/line navigation
void editBufferGetNextWordBoundary(EditBuffer* eb,
                                   ExternalLogicalCursor* out);
void editBufferGetPrevWordBoundary(EditBuffer* eb,
                                   ExternalLogicalCursor* out);
void editBufferGetEOL(EditBuffer* eb, ExternalLogicalCursor* out);

// Offset conversion
bool editBufferOffsetToPosition(EditBuffer* eb, uint32_t offset,
                                ExternalLogicalCursor* out);
uint32_t editBufferPositionToOffset(EditBuffer* eb, uint32_t row,
                                    uint32_t col);
uint32_t editBufferGetLineStartOffset(EditBuffer* eb, uint32_t row);

// Text extraction
size_t editBufferGetText(EditBuffer* eb, uint8_t* out_ptr, size_t max_len);
size_t editBufferGetTextRange(EditBuffer* eb, uint32_t start_offset,
                              uint32_t end_offset, uint8_t* out_ptr,
                              size_t max_len);
size_t editBufferGetTextRangeByCoords(EditBuffer* eb, uint32_t start_row,
                                      uint32_t start_col, uint32_t end_row,
                                      uint32_t end_col, uint8_t* out_ptr,
                                      size_t max_len);

// Undo/redo
size_t editBufferUndo(EditBuffer* eb, uint8_t* out_ptr, size_t max_len);
size_t editBufferRedo(EditBuffer* eb, uint8_t* out_ptr, size_t max_len);
bool editBufferCanUndo(EditBuffer* eb);
bool editBufferCanRedo(EditBuffer* eb);
void editBufferClearHistory(EditBuffer* eb);

// Metadata
uint16_t editBufferGetId(EditBuffer* eb);
void editBufferDebugLogRope(EditBuffer* eb);

// --- EditorView ---

EditorView* createEditorView(EditBuffer* eb, uint32_t viewport_width,
                             uint32_t viewport_height);
void destroyEditorView(EditorView* view);

// Viewport
void editorViewSetViewport(EditorView* view, uint32_t x, uint32_t y,
                           uint32_t width, uint32_t height, bool move_cursor);
void editorViewClearViewport(EditorView* view);
bool editorViewGetViewport(EditorView* view, uint32_t* out_x, uint32_t* out_y,
                           uint32_t* out_width, uint32_t* out_height);
void editorViewSetViewportSize(EditorView* view, uint32_t width,
                               uint32_t height);

// Scroll
void editorViewSetScrollMargin(EditorView* view, float margin);

// Line info
uint32_t editorViewGetVirtualLineCount(EditorView* view);
uint32_t editorViewGetTotalVirtualLineCount(EditorView* view);
void editorViewGetLineInfoDirect(EditorView* view, ExternalLineInfo* out);
void editorViewGetLogicalLineInfoDirect(EditorView* view,
                                        ExternalLineInfo* out);

// Display
void editorViewSetWrapMode(EditorView* view, uint8_t mode);
void editorViewSetTabIndicator(EditorView* view, uint32_t indicator);
void editorViewSetTabIndicatorColor(EditorView* view, const float* color);
void editorViewSetPlaceholderStyledText(EditorView* view,
                                       const StyledChunk* chunks_ptr,
                                       size_t chunk_count);

// View access
UnifiedTextBufferView* editorViewGetTextBufferView(EditorView* view);

// Selection
void editorViewSetSelection(EditorView* view, uint32_t start, uint32_t end,
                            const float* bg_color, const float* fg_color);
void editorViewResetSelection(EditorView* view);
uint64_t editorViewGetSelection(EditorView* view);
bool editorViewSetLocalSelection(EditorView* view, int32_t anchor_x,
                                 int32_t anchor_y, int32_t focus_x,
                                 int32_t focus_y, const float* bg_color,
                                 const float* fg_color, bool update_cursor,
                                 bool follow_cursor);
void editorViewUpdateSelection(EditorView* view, uint32_t end,
                               const float* bg_color, const float* fg_color);
bool editorViewUpdateLocalSelection(EditorView* view, int32_t anchor_x,
                                    int32_t anchor_y, int32_t focus_x,
                                    int32_t focus_y, const float* bg_color,
                                    const float* fg_color, bool update_cursor,
                                    bool follow_cursor);
void editorViewResetLocalSelection(EditorView* view);
size_t editorViewGetSelectedTextBytes(EditorView* view, uint8_t* out_ptr,
                                      size_t max_len);

// Cursor
void editorViewGetCursor(EditorView* view, uint32_t* out_row,
                         uint32_t* out_col);
void editorViewGetVisualCursor(EditorView* view, ExternalVisualCursor* out);
void editorViewMoveUpVisual(EditorView* view);
void editorViewMoveDownVisual(EditorView* view);
void editorViewSetCursorByOffset(EditorView* view, uint32_t offset);

// Word/line navigation
void editorViewGetNextWordBoundary(EditorView* view,
                                   ExternalVisualCursor* out);
void editorViewGetPrevWordBoundary(EditorView* view,
                                   ExternalVisualCursor* out);
void editorViewGetEOL(EditorView* view, ExternalVisualCursor* out);
void editorViewGetVisualSOL(EditorView* view, ExternalVisualCursor* out);
void editorViewGetVisualEOL(EditorView* view, ExternalVisualCursor* out);

// Text operations
void editorViewDeleteSelectedText(EditorView* view);
size_t editorViewGetText(EditorView* view, uint8_t* out_ptr, size_t max_len);

// --- SyntaxStyle ---

SyntaxStyle* createSyntaxStyle(void);
void destroySyntaxStyle(SyntaxStyle* style);
uint32_t syntaxStyleRegister(SyntaxStyle* style, const uint8_t* name_ptr,
                             size_t name_len, const float* fg, const float* bg,
                             uint32_t attributes);
uint32_t syntaxStyleResolveByName(SyntaxStyle* style, const uint8_t* name_ptr,
                                  size_t name_len);
size_t syntaxStyleGetStyleCount(SyntaxStyle* style);

// --- Unicode ---

bool encodeUnicode(const uint8_t* text_ptr, size_t text_len,
                   EncodedChar** out_ptr, size_t* out_len,
                   uint8_t width_method);
void freeUnicode(const EncodedChar* chars_ptr, size_t chars_len);

// --- Native span feed ---

NativeSpanFeedStream* createNativeSpanFeed(
    const NativeSpanFeedOptions* options);

#ifdef __cplusplus
}
#endif

#endif // OPENTUI_H
