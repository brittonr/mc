# Design: MCP-controlled compatibility rail

## Context

The parent `mc` harness already launches Valence/Paper backends, Stevenarella under Xvfb when needed, records receipts, and promotes evidence under `docs/evidence/`. MCP adds a more precise client-control layer, but it must be integrated without broadening existing compatibility claims.

## Decisions

### 1. Runner owns orchestration, Stevenarella owns actions

**Choice:** The runner starts Stevenarella with MCP enabled, performs MCP initialize/tool calls, and records outcomes. Stevenarella remains responsible for applying commands and capturing frames.

**Rationale:** This keeps test orchestration in the parent harness while preserving client internals and GL ownership inside Stevenarella.

### 2. Dry-run first

**Choice:** Add deterministic dry-run receipt fixtures before any live MCP rail is promoted.

**Rationale:** The runner schema and checkers should fail closed without requiring a live GL/client environment.

### 3. Visual artifacts are evidence, not proof by themselves

**Choice:** Receipt fields describe screenshot/frame artifacts as observability outputs. Matrix/current-bundle promotion must keep semantic compatibility claims tied to protocol/server/client milestone checks.

**Rationale:** A screenshot can prove that a frame was captured, not that gameplay semantics are correct.

### 4. Stale child revisions fail closed

**Choice:** Receipts must record the Stevenarella child repo revision used for MCP/capture evidence or include an oracle checkpoint if the runner cannot machine-record it.

**Rationale:** Existing review history rejected evidence that cited a child client revision without a repo-local receipt/oracle.

### 5. Existing scenarios remain unchanged

**Choice:** Add a new named scenario such as `stevenarella-mcp-control-smoke` or `mcp-controlled-smoke`; do not retrofit existing scenario pass/fail semantics until the MCP rail is independently validated.

**Rationale:** Current evidence rows should stay stable and reviewable.

## Implementation notes

- Add scenario manifest entries for the MCP-controlled rail and generated Rust tables.
- Extend receipt DTOs with `mcp_control` and optional `frame_artifacts` blocks.
- Include fields: endpoint mode, handshake success, tool list digest, calls attempted, calls succeeded, first failure, stdout clean, command outcome ids, artifact paths, artifact digests, redaction status, client revision, and non-claims.
- Add checker fixtures: valid dry-run, missing handshake, stdout contamination, missing command outcome, missing frame digest, path outside evidence/capture root, stale/missing client revision, and overclaim wording.
- Promote artifacts to `docs/evidence/` only after live rail runs; do not cite transient `target/` outputs alone.

## Risks / Trade-offs

- Live MCP rail depends on Stevenarella MCP and capture Cairns landing first; tasks should encode that dependency rather than pretending the rail can pass alone.
- Frame artifact paths can stale evidence manifests if central docs change; manifests must be refreshed after promotion.
- If stdout-clean stdio is flaky, loopback TCP with token may be easier for live rails, but stdio still needs tests because MCP clients commonly use it.
