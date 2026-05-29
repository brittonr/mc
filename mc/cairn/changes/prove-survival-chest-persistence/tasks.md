# Tasks

- [x] [serial] Define the bounded chest persistence evidence contract and normalized metric names. r[mc_compatibility.survival_chest_persistence.contract]
  - Evidence: `docs/evidence/protocol-763-survival-chest-persistence-contract-2026-05-28.md` fixes the scenario envelope to one chest block, one `Dirt` stack, one chest slot, and one close/reconnect/reopen cycle, and lists 48 required normalized metrics plus non-claims.
- [x] [depends:contract] Add a deterministic checker with positive and negative fixtures for paired Paper/Valence chest evidence, missing metrics, mismatched slot/count/item, and Valence-only evidence. r[mc_compatibility.survival_chest_persistence.checker]
  - Evidence: `tools/check_survival_chest_persistence.py --self-test` covers a positive paired fixture and rejects missing reference/Valence-only evidence, missing open/store/close/reconnect/client persisted/server persisted metrics, mismatched slot/item/count between backends, paired evidence where both backends agree on the wrong fixed position/slot/item/count/reconnect session, and wrong backend.
- [ ] [depends:checker] Add the `survival-chest-persistence` client/runner rail and unit tests without broadening existing survival scenarios. r[mc_compatibility.survival_chest_persistence.runner]
- [ ] [depends:runner] Add Valence and Paper fixture instrumentation for chest open, store, close, reconnect/reopen, and persisted slot observation. r[mc_compatibility.survival_chest_persistence.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_chest_persistence.receipts]
- [ ] [depends:receipts] Promote only the `chest persistence` survival coverage matrix row and keep full survival, all-container, restart/world persistence, and broader vanilla parity as non-claims. r[mc_compatibility.survival_chest_persistence.matrix]
- [ ] [depends:matrix] Run validation and archive the change with checker, manifest, and Cairn evidence recorded. r[mc_compatibility.survival_chest_persistence.validation]
