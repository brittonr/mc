# Design: Automate evidence manifest refresh

## Context

The evidence manifest checker verifies BLAKE3 rows, but it does not provide a safe fixpoint refresh path. Operators currently regenerate selected manifests with one-off loops. That works for a single file, but archive and accepted-spec updates often change rows referenced by other manifests, requiring repeated passes until the graph stabilizes.

## Decisions

### 1. Use a small Rust helper with pure manifest logic

**Choice:** Implement parsing, digest comparison, stale-row reporting, and fixpoint planning as deterministic pure functions. Keep filesystem reads, writes, and CLI exit handling in a thin shell.

**Rationale:** Manifest refresh is safety-sensitive and benefits from positive and negative tests over in-memory fixtures. A Rust helper also fits the repo guidance for non-trivial standalone automation.

### 2. Separate check and refresh modes

**Choice:** Provide a check-only mode that exits non-zero on stale or malformed rows without writing, plus an explicit refresh mode that rewrites only changed digest fields.

**Rationale:** CI needs fail-fast diagnostics; local evidence preparation needs a controlled mutation path.

### 3. Preserve reviewable unresolved rows

**Choice:** If a row points to a missing file, the helper reports it and leaves the row unchanged unless a future explicit cleanup mode is added.

**Rationale:** Missing evidence references should stay visible for reviewers. Silent removal could hide an evidence gap.

### 4. Keep scope bounded to reviewable evidence manifests

**Choice:** By default, the helper operates on `docs/evidence/*.b3` rows that resolve inside the repository and rejects path traversal or outside-root references.

**Rationale:** Cairn evidence manifests must be reviewable source-closure artifacts, not arbitrary host files.

## Risks / Trade-offs

- The helper may initially duplicate some checker parsing logic; keep shared behavior explicit in tests until a broader refactor is justified.
- A refresh mode can mutate many manifests in one run; require clear diagnostics and a check mode before wiring it into routine workflows.
- Preserving missing rows means the helper is not a cleanup tool; unresolved evidence references still require a separate human decision.