# Stevenarella MCP Cairn target oracle checkpoint

## Question

What did the user mean by `write cairns for this`, and are the three active Cairn changes the correct target?

## Inspected evidence

- Immediate prior user request: `how can we modify stevenarella so that we can control it via mcp and capture screenshots, frames, etc`
- Assistant analysis identified Stevenarella hook points for MCP control, frame capture, and parent-runner integration:
  - `stevenarella/src/main.rs` owns `tick_all(...)`, native render loop, and input mapping.
  - `stevenarella/src/server/mod.rs` owns movement, mouse, right-click, and chat packet seams.
  - `stevenarella/src/render/mod.rs` and `stevenarella/src/gl/mod.rs` own frame rendering/readback seams.
- Follow-up user request: `write cairns for this` appeared directly after that Stevenarella MCP/capture analysis and named no other active change.
- Current user request explicitly selects the ROI step to add this checkpoint resolving that `this` meant the `Stevenarella MCP/control/capture workstream`.
- The committed active Cairn changes split that workstream into the three bounded scopes:
  - `cairn/changes/stevenarella-mcp-control-plane`
  - `cairn/changes/stevenarella-frame-capture-artifacts`
  - `cairn/changes/mcp-controlled-compat-rail`

## Decision

Treat `write cairns for this` as referring to the Stevenarella MCP/control/capture workstream from the immediately preceding user request. The three active Cairn changes are the intended decomposition:

1. `stevenarella-mcp-control-plane` covers in-process MCP command/control.
2. `stevenarella-frame-capture-artifacts` covers screenshots, latest-frame, and bounded recording artifacts.
3. `mcp-controlled-compat-rail` covers parent `mc` runner/receipt integration after the Stevenarella control/capture prerequisites exist.

Do not replace these changes with unrelated `production-readiness-envelope` work. The earlier inspection of `production-readiness-envelope` was format/reference discovery only, not the selected target.

## Owner

- Decision owner: user / Britton, via the current ROI execution request.
- Implementation owner: agent for the checkpoint and validation rerun.

## Next action

Rerun repo-pinned Cairn validation and proposal/design/tasks gates for the three MCP-related changes, then keep future implementation aligned with `stevenarella-mcp-control-plane` before starting the dependent capture and runner rails.
