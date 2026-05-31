# Design: Stevenarella frame capture artifacts

## Context

`Renderer::tick(...)` finishes drawing into the active framebuffer and the native event loop swaps buffers immediately afterward. That gives a stable capture seam: read pixels after render completion and before `glutin_window.swap_buffers()`.

## Decisions

### 1. Read from GL, not the host desktop

**Choice:** Capture the OpenGL framebuffer through a `gl::read_pixels_rgba(...)` helper rather than external screenshot tools.

**Rationale:** Desktop screenshots are environment-specific and can capture wrong windows. GL readback ties artifacts to Stevenarella's rendered frame.

### 2. Capture request validation is pure

**Choice:** Define capture request, output format, recording policy, and artifact metadata in a pure module. The imperative shell owns GL readback and file writes.

**Rationale:** Rate limits, dimensions, path containment, and format validation can be tested without OpenGL.

### 3. Store durable artifacts for large outputs

**Choice:** Single screenshots may return MCP image content when small enough, but recordings and repeated frames write files under `--capture-dir` and return paths plus BLAKE3 digests.

**Rationale:** MCP JSON responses are not a good transport for many large base64 images.

### 4. Vertical flip is part of capture core

**Choice:** Normalize RGBA buffers to top-left origin before PNG encoding and digesting.

**Rationale:** OpenGL readback starts at bottom-left. Consumers expect screenshots with top-left origin.

### 5. Bounded recording only

**Choice:** Recording requires explicit fps, frame count or duration, capture directory, and max bytes/frame guard.

**Rationale:** Unbounded capture can exhaust disk or stall rendering.

## Implementation notes

- Add `gl::read_pixels_rgba(width, height) -> Vec<u8>` using `glow::read_pixels` with `PixelPackData`.
- Add `capture::CaptureRequest`, `CapturePolicy`, `CaptureArtifact`, and pure validators.
- Add `Renderer` or main-loop hook that can satisfy pending capture requests after `game.renderer.tick(...)` and before swap.
- Use existing `image` crate for PNG encoding; add `blake3` for artifact IDs/digests unless parent policy already supplies a digest helper.
- Include metadata: width, height, frame id, monotonic capture sequence, format, file path, BLAKE3 digest, includes UI flag, and whether redaction was applied.
- For first implementation, `include_ui=false` may be rejected if the renderer cannot yet capture pre-UI frames; rejection must be explicit.

## Risks / Trade-offs

- GL readback can stall the render thread; bounded fps and one-shot capture keep this acceptable.
- Screenshot artifacts may contain usernames, chat, or server data; promoted evidence needs redaction metadata or a human checkpoint.
- WebAssembly capture may need browser canvas APIs; native-only first implementation keeps scope bounded.
