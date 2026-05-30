# WAN tolerance final manifest checkpoint — 2026-05-29

## Question

How is the final repo-wide evidence-manifest pass proven after refreshing the `.b3` digest for `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-archive-validation-2026-05-29.run.log`?

## Inspected evidence

- `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-2026-05-29.b3` records the refreshed archive-validation log digest and all bounded WAN telemetry evidence artifacts.
- `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-archive-validation-2026-05-29.run.log` now captures runnable checker output, Cairn validation output, Nix checker output, the store path printed by `nix build --print-out-paths`, and `nix_build_exit_status=0`.
- `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-final-manifest-2026-05-29.run.log` is the separate final manifest rerun log generated after the `.b3` refresh. Its BLAKE3 digest at inspection time is `d0b0a0b6b609f21d0004d57ad745a783c2966b95aeef163286e4f694386f9989`, and it records `evidence manifests ok: 73 manifests, 359 entries, 61 receipts scanned`, `manifest_exit_status=0`, and `final manifest PASS`. It is intentionally not listed inside a `.b3` manifest because doing so would require another post-log digest update and reintroduce the self-referential proof loop this checkpoint resolves.

## Decision

The final evidence-manifest PASS must be claimed only from `docs/evidence/protocol-763-wan-tolerance-bounded-telemetry-final-manifest-2026-05-29.run.log`, not from prose inside the archive-validation log. The archive-validation log proves the Nix checker exit status; the final-manifest log proves the repo-wide `.b3` state after the archive log digest refresh.

## Owner

agent

## Next action

If any WAN telemetry `.b3` row or cited artifact changes, rerun `nix develop --no-update-lock-file -c python3 tools/check_evidence_manifests.py` and replace the final manifest log instead of appending prose-only PASS claims.
