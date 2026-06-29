# Proposal: Split Stevenarella renderer core

## Why

`clients/stevenarella/src/render/mod.rs` combines frame rendering, chunk buffers, camera state, texture management, skin cache, pending texture uploads, framebuffer setup, and tests. Rendering changes are hard to review when resource loading, OpenGL side effects, and pure render planning share one owner.

## What Changes

- Split renderer code into focused modules for frame orchestration, camera/view state, chunk buffers, texture manager, skin/remote texture cache, pending uploads, framebuffer/readback helpers, and pure render planning.
- Keep OpenGL calls, resource manager locks, texture uploads, and filesystem/network cache access in shells or adapters.
- Extract pure decisions for texture URL normalization, cache paths, upload plans, chunk render visibility/order, and frame/capture planning.
- Preserve visible rendering behavior, capture interactions, texture cache semantics, and non-claims.

## Impact

- **Files**: `clients/stevenarella/src/render/mod.rs`, render submodules, capture integration tests, and Cairn artifacts.
- **Testing**: baseline render/capture tests, positive and negative pure render-planning tests, affected dry-runs, Cairn gates, and Cairn validation.
- **Non-claims**: renderer architecture only; no new rendering correctness or compatibility claim is promoted.
