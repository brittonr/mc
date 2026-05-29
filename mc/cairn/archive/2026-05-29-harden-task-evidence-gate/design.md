## Context

The mc workspace already has deterministic checks for evidence manifests and promotion plans, but task closeout still depends on reviewer discipline. A checked task can cite a command in prose while the command output or BLAKE3 manifest is absent from `docs/evidence/`, which lets archive candidates pass normal Cairn validation and fail later review.

## Decisions

### 1. Gate active changes, not historical archives

**Choice:** The checker scans `cairn/changes/*/tasks.md` only.

**Rationale:** Historical archives predate the new policy and would create noisy retroactive failures. Active changes are the point where evidence can still be added before archive.

### 2. Keep validation as a pure core with a thin shell

**Choice:** The core takes in-memory task files and a set of copied evidence paths, then returns diagnostics. The CLI shell only discovers active task files and `docs/evidence/` files.

**Rationale:** This keeps parsing and policy decisions testable without filesystem fixtures while preserving simple local execution through Rust and Nix.

### 3. Require durable verification output and hash evidence per checked task

**Choice:** A completed task must include an evidence-labeled line, at least one existing `docs/evidence/` artifact, a `docs/evidence/*.run.log` verification-output path, and either a `docs/evidence/*.b3` path or inline BLAKE3 digest.

**Rationale:** The repeated failure mode is not that evidence text is missing entirely; it is that claims cite non-durable logs or omit hash/manifest proof. The gate checks the exact artifact shape reviewers need.

## Risks / Trade-offs

- The gate can feel strict for small code-only tasks, but a shared validation `.run.log` and `.b3` can satisfy multiple completed tasks.
- The gate does not parse the `.run.log` contents. Evidence manifest checks remain responsible for BLAKE3 sidecar integrity; reviewers inspect command output semantics.
- Archived changes remain best-effort unless reopened as active changes.
