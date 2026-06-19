## Context

Sign block-entity persistence is already a separate survival/reference claim. Sign-editor open/update requires a dedicated live driver because the packet rows and evidence shape are different.

## Goals / Non-Goals

Goals:
- Add a typed Stevenarella path that observes one sign-editor open event and submits a configured four-line update through the protocol path.
- Preserve main-thread ownership and avoid host OS input synthesis.
- Integrate with the runner only enough to produce reviewable sign-editor live evidence.

Non-goals:
- Proving UI behavior, all sign variants, arbitrary text formats, arbitrary NBT, all block entities, public-server behavior, production readiness, or full protocol 763 compatibility.

## Design

1. Define a pure driver contract for sign position, initial state, four-line payload, line/length limits, expected open/update milestones, and protocol output.
2. Implement a thin Stevenarella shell that queues or applies the configured update at a deterministic client boundary while preserving winit, GL, `Game`, and `Server` ownership.
3. Add positive tests for the configured open/update flow and negative tests for missing open state, wrong position, malformed payload, line-count/length violations, path/control errors, and overclaim attempts.
4. Expose the driver to the runner as an isolated sign-editor path or capability update.
5. Emit targeted-packet live KV/receipt/log evidence only when the checker verifies open and update correlation.

## Risks

- Stevenarella may parse open-sign packets without enough state for automation. If so, preserve a blocker instead of promoting the row.
- The driver must not conflate sign persistence with sign-editor packet evidence.

## Validation

- Run focused Stevenarella tests through the mc devshell.
- Run runner dry-runs/unit tests for the sign-editor path.
- Run targeted-packet live-evidence checker positive and negative fixtures.
- Run evidence-manifest/task-evidence checks plus Cairn gates and validation.
