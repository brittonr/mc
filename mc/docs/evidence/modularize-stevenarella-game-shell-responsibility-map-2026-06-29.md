# Stevenarella game shell responsibility map

## Question

Which `clients/stevenarella/src/main.rs` responsibilities were owned by the root game shell before closing Cairn change `modularize-stevenarella-game-shell`, and where are they owned after the refactor?

## Inspected evidence

- Change scope: `cairn/changes/modularize-stevenarella-game-shell/{proposal.md,design.md,tasks.md}`.
- Affected subtree workflow: `clients/stevenarella/AGENTS.md` and `clients/stevenarella/README.md`.
- Baseline logs: `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-game-shell.baseline-mcp-control-tests.run.log` and `docs/evidence/run-logs/2026-06-29/modularize-stevenarella-game-shell.baseline-compat-instrumentation-tests.run.log`.

## Responsibility owners

| Responsibility | Owner after refactor | Side-effect boundary |
| --- | --- | --- |
| CLI startup flags and option shape | `clients/stevenarella/src/game_shell/startup.rs` | StructOpt parsing remains in `main2`; options are plain data. |
| Startup capture policy and recording plan | `clients/stevenarella/src/game_shell/capture_startup.rs` | Pure request planning validates options before `main2` starts recording or exits. |
| Runtime capture servicing | `clients/stevenarella/src/game_shell/capture_startup.rs` | Framebuffer readback and recording mutation stay in `Game` shell methods. |
| Connection orchestration | `clients/stevenarella/src/game_shell/connection.rs` | Ping, thread spawn, and server replacement remain side effects behind `Game::connect_to`. |
| Per-frame game lifecycle | `clients/stevenarella/src/game_shell/lifecycle.rs` | `Game::tick` owns connection reply/drain state updates. |
| MCP control planning and application | `clients/stevenarella/src/game_shell/mcp_control.rs` | `plan_mcp_control_command` is pure over `ControlShellState`; packet writes, input mutation, and capture enqueue stay in `Game` shell methods. |
| Whole-frame tick orchestration | `clients/stevenarella/src/game_shell/ticking.rs` | Renderer, server tick, capture servicing, and FPS sleep remain shell effects. |
| Window and input events | `clients/stevenarella/src/game_shell/window_events.rs` | Winit cursor/window operations and UI/server input mutation stay in the event shell. |
| Executable wiring | `clients/stevenarella/src/main.rs` | Main now wires config, logging, window/context construction, instrumentation, game construction, and event-loop dispatch. |

## Decision

The refactor keeps the executable shell behavior in `main.rs` but moves cohesive game-shell responsibilities into focused modules. Pure startup and MCP control decisions are testable without constructing a renderer, window, network session, or live `Game`.

## Non-claims

This evidence is architecture and focused-test evidence only. It does not claim broad Minecraft compatibility, gameplay correctness, production readiness, public-server safety, semantic equivalence, full CTF correctness, or full survival correctness.
