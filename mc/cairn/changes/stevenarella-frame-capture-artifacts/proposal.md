# Proposal: Stevenarella frame capture artifacts

## Why

MCP control is incomplete without observability. Current compatibility receipts rely mostly on logs and milestone strings; they cannot ask Stevenarella for a screenshot, retrieve the latest rendered frame, or record bounded visual artifacts tied to a receipt.

## What Changes

- Add a native Stevenarella frame capture surface that reads the rendered GL framebuffer after `Renderer::tick(...)` and before native `swap_buffers()`.
- Expose MCP capture tools/resources for single screenshots, latest frame, and bounded recording to a configured artifact directory.
- Encode PNG artifacts with dimensions, frame id, capture timing, and BLAKE3 digest metadata.
- Enforce path containment, rate limits, frame-count limits, and explicit non-claims around visual regression, semantic correctness, and headless EGL/OSMesa support.
- Add positive and negative tests for capture request validation, artifact metadata, rate limiting, and path traversal rejection.

## Impact

- **Files**: `stevenarella/src/gl/mod.rs`, `stevenarella/src/render/mod.rs` or `main.rs` capture hook, new capture module, MCP resource/tool wiring, tests, and parent `mc` evidence docs when promoted.
- **Validation**: unit tests for capture request validation, focused Xvfb smoke if available, artifact digest verification, Cairn gates, and parent validation.
- **Non-claims**: no visual-regression baseline approval, no semantic gameplay correctness from screenshots alone, no raw secret-safe guarantee without redaction review, and no headless EGL/OSMesa support in first implementation.
