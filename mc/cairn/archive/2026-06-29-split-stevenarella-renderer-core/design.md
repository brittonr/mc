# Design: Stevenarella renderer split

## Context

The renderer module contains both pure decisions and platform/OpenGL side effects. A split should isolate render planning and cache decisions while preserving the existing renderer public surface.

## Decisions

### 1. Split by renderer responsibility

**Choice:** Create modules for camera/view state, chunk buffers, texture manager, skin cache, upload queue, frame orchestration, and capture/readback integration.

**Rationale:** Renderer changes usually affect one of those responsibility families.

### 2. Keep GL and resource side effects isolated

**Choice:** OpenGL calls, texture uploads, resource locks, and external texture/cache access stay in shells.

**Rationale:** Pure render decisions should be testable without a GL context.

### 3. Preserve capture and cache behavior

**Choice:** Capture readback hooks, texture URL normalization, skin cache prefixing, pending upload behavior, and visible rendering behavior remain stable.

**Rationale:** MCP/capture evidence and manual client behavior depend on these surfaces.
