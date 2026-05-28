# Tasks

- [x] [serial] Define lifecycle states and invariants. r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_model]
- [x] [serial] Add positive death/respawn scenarios. r[mc_compatibility.prove_death_respawn_lifecycle.positive_lifecycle_scenarios]
- [x] [serial] Add negative duplicate/missing/out-of-order lifecycle scenarios. r[mc_compatibility.prove_death_respawn_lifecycle.negative_lifecycle_scenarios]
- [x] [serial] Promote only receipt-backed lifecycle rows. r[mc_compatibility.prove_death_respawn_lifecycle.lifecycle_promotion_gate]

## Progress

- Lifecycle states and invariants are documented in `docs/evidence/protocol-763-death-respawn-lifecycle-2026-05-27.md`.
- Positive validation uses the protocol-763 live flag-carrier death/return receipt at `docs/evidence/protocol-763-flag-carrier-death-return.matrix.receipt.json` with BLAKE3 `d4202d7f04245dd385f16f9a174b84fa59a837fd75a8f9ba7db3cc7adaf692a4`.
- Negative fixtures in `tools/check_death_respawn_lifecycle.py --self-test` reject missing respawn, duplicate death, out-of-order death/respawn, forbidden score/capture, protocol mismatch, and missing model text.
- `tools/check_death_respawn_lifecycle.py` promotes only the bounded receipt-backed row and keeps full death/respawn lifecycle correctness as a non-claim.
