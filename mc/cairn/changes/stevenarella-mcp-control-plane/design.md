# Design: Stevenarella MCP control plane

## Context

The useful seam is already present: `src/main.rs` owns the winit event loop, `tick_all(...)` owns per-frame orchestration, `handle_window_event(...)` maps physical input into internal calls, and `src/server/mod.rs` exposes movement/mouse methods such as `key_press`, `on_left_mouse_button`, `on_right_mouse_button`, and `on_right_click`.

The control plane should reuse those internal methods instead of synthesizing host OS input.

## Decisions

### 1. Main-thread ownership remains strict

**Choice:** Run MCP transport on a native-only worker, but send `ControlCommand` values over a channel to the winit/render thread. Drain commands at a deterministic point before `game.server.tick(...)`.

**Rationale:** Winit, glutin, GL, `Game`, and `Server` state are not safe to mutate from an MCP worker. A queue keeps the imperative shell boring and makes command handling testable as a pure command-to-action core.

### 2. Stable command model before transport details

**Choice:** Define pure command/request/response types separately from stdio or TCP transport. The first commands are status, connect, disconnect, key, look, mouse button, use item, attack, and chat.

**Rationale:** Tests can validate command parsing and state transitions without launching MCP or OpenGL.

### 3. stdout cleanliness is mandatory for stdio MCP

**Choice:** `--mcp-stdio` switches terminal logging away from stdout. Logs remain available through `client.log`, stderr where safe, and later MCP resources.

**Rationale:** JSON-RPC over stdio breaks if ordinary client log lines share stdout.

### 4. Network exposure is default-deny

**Choice:** stdio and loopback TCP are allowed. Non-loopback listening requires an explicit flag plus token environment variable. Public-server automation remains out of scope.

**Rationale:** An MCP-controlled game client is a remote-control surface. It must fail closed by default.

### 5. Existing probes remain compatible

**Choice:** Existing `MC_COMPAT_*` hard-coded probes are left intact. MCP control is an additional generic path, not a rewrite of current evidence rails.

**Rationale:** Current promoted compatibility evidence should not be invalidated by the new automation surface.

## Implementation notes

- Add `src/control.rs` for pure command enums, validation, and bounded response shapes.
- Add native-only `src/mcp.rs` for MCP JSON-RPC transport and tool registration.
- Add an `Option<ControlPlane>` or command receiver to `Game` initialization.
- Drain commands in `tick_all(...)` before server tick so key/look/click commands affect the same frame/tick boundary consistently.
- Implement look by mutating player `Rotation` when connected; mutate only renderer camera for disconnected/free-camera status if intentionally supported.
- Implement chat through a new `Server::send_chat(&str)` wrapper over `packet::play::serverbound::ChatMessage`.
- Return structured errors for invalid key names, invalid button names, disconnected-only operations, missing auth token, and non-loopback bind attempts.

## Risks / Trade-offs

- MCP stdio and terminal logs compete for stdout; this change must be validated before any runner depends on stdio.
- `Server::send_chat` packet shape may need version-specific handling in future protocols; first implementation should stay within supported Stevenarella packet abstractions.
- TCP MCP can become a security footgun if allowed on non-loopback interfaces without token enforcement.
