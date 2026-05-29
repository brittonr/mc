# Oracle checkpoint: survival chest runner review blockers — 2026-05-29

## Question

Can the runner task cite the Stevenarella chest probe implementation when the parent-repo review diff does not include the nested `stevenarella/src/server/mod.rs` diff?

## Inspected evidence

- Nested repo: `mc/stevenarella`
- Commit: `ee67a4c7fb511421b79fa551452350503b9671af`
- Short commit: `ee67a4c7fb51`
- Path: `stevenarella/src/server/mod.rs`
- BLAKE3: `f710fe46036d65d149eb1e19d400a27bf34e384d693a4dd620804c66e1a59c84`
- Working-tree status bytes for `src/server/mod.rs`: `0` (0 means clean)

```text
ee67a4c add survival chest persistence probe
 src/server/mod.rs | 259 +++++++++++++++++++++++++++++++++++++++++++++++++++++-
 1 file changed, 256 insertions(+), 3 deletions(-)
```

The committed probe adds `MC_COMPAT_SURVIVAL_CHEST_PROBE` handling and emits the bounded client milestones documented by the runner task: open, store, close, reconnect, reopen, and persisted observation for chest position `8,64,0`, chest slot `0`, item `Dirt`, and count `1`.

## Decision

Treat the nested Stevenarella commit `ee67a4c7fb51` plus the BLAKE3 hash above as the reviewable oracle for the client-probe implementation until the receipt schema records nested child-repo revisions directly. Do not claim paired Paper/Valence chest persistence from this checkpoint; it only supports the runner/client rail task.

## Owner

MC compatibility maintainer.

## Next action

When producing live receipts, copy receipt logs under `docs/evidence/` and include this Stevenarella commit (or a newer committed child revision) in the receipt metadata or a fresh oracle checkpoint.

## Question

Should `docs/evidence/protocol-763-survival-reference-paper-fixture-gate-2026-05-28.b3` keep the updated `tools/mc-compat-runner/src/main.rs` hash even though the manifest is named for 2026-05-28 evidence?

## Inspected evidence

- Repo-wide manifest checker failed when the old hash remained after `tools/mc-compat-runner/src/main.rs` changed.
- The 2026-05-28 manifest contains a mutable source-path row for `tools/mc-compat-runner/src/main.rs` in addition to immutable evidence logs/receipts.
- Final validation command `nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py` passes only when that mutable source row matches current checked-in source.

## Decision

Keep the updated source hash in the 2026-05-28 manifest for now so the repo-wide manifest checker remains green. Interpret that row as current checker/source provenance, not as a claim that the 2026-05-28 runtime evidence was regenerated. The immutable evidence log/receipt hashes in the same manifest remain unchanged.

## Owner

MC compatibility maintainer.

## Next action

Split future evidence manifests so historical receipt bundles do not include mutable source files, or teach the manifest checker to distinguish historical evidence hashes from current-source provenance hashes.
