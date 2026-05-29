## Context

`tools/check_evidence_manifests.py` validates repo-relative paths from the live working tree. That is enough locally, but Nix builds evaluate only files in the parent repo source closure. Nested git repo files (`stevenarella/...`, `valence/...`) and generated `target/...` jars are therefore invisible to the Nix check even when their local BLAKE3 digests match.

## Decisions

### 1. Copy immutable evidence artifacts into `docs/evidence/`

**Choice:** Keep historical manifest semantics by copying the referenced child/source/generated files into `docs/evidence/` and updating manifest paths to those copies.

**Rationale:** This preserves BLAKE3 content identity while making every promoted manifest entry reviewable from the parent repo and from Nix.

### 2. Do not weaken historical manifests into prose-only references

**Choice:** Do not replace source/jar entries with only oracle markdown. Source snapshots and the Paper fixture jar remain hashed artifacts.

**Rationale:** The previous manifests used BLAKE3 to bind exact bytes. Durable copies keep that stronger guarantee and avoid making reviews depend only on narrative checkpoints.

### 3. Keep checker behavior unchanged

**Choice:** Fix manifests and evidence artifacts without modifying the Python checker.

**Rationale:** Workspace policy prefers Rust/Steel for new or touched gates. The existing checker already enforces the needed invariant once all referenced paths are in the source closure.

## Risks / Trade-offs

- Full source snapshots add about a few hundred KiB of evidence material, but avoid unreviewable nested-repo dependencies.
- Historical manifests still include mutable accepted spec hashes; this change refreshes those digests but does not redesign historical/current provenance splitting.
