# Design: Add mc-compat failure evidence bundles

## Context

Successful evidence promotion already expects durable receipts and BLAKE3 manifests. Failure triage is less standardized. A failure bundle should be deterministic enough for review, but it must never be mistaken for successful compatibility evidence.

## Decisions

### 1. Use a fail-only bundle schema

**Choice:** The bundle records `outcome = failed` or a named blocked state and rejects success-labeled bundles.

**Rationale:** Failure diagnostics should not become accidental success evidence.

### 2. Hash artifact contents with BLAKE3

**Choice:** Bundle rows record BLAKE3 digests for receipt, client log, server log, typed events, stderr, and any additional failure artifacts that exist.

**Rationale:** BLAKE3 is the repo default for content identity and matches evidence-manifest practice.

### 3. Keep paths reviewable and bounded

**Choice:** Bundle paths must be repo-relative or explicitly copied under `docs/evidence/` before citation. Path escapes and target-only critical artifacts fail validation.

**Rationale:** Reviewers and Nix checks need source-closure-visible evidence.

### 4. Separate pure bundle validation from artifact collection

**Choice:** A pure validator checks bundle shape, digests, nonclaims, and path policy from in-memory data. The runner shell collects files and writes the bundle.

**Rationale:** Failure policy can be tested without inducing live failures.

## Risks / Trade-offs

- Failure bundles can create noise if every transient failure is promoted; mitigate by documenting when to copy into `docs/evidence/`.
- Capturing logs can expose noisy local paths; mitigate with existing redaction/path policy and explicit path validation.
- Bundle emission on process failure can be fragile; mitigate by writing incrementally and validating after collection.
