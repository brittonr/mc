# Tasks

- [ ] [serial] Define the bounded chest persistence evidence contract and normalized metric names. r[mc_compatibility.survival_chest_persistence.contract]
- [ ] [depends:contract] Add a deterministic checker with positive and negative fixtures for paired Paper/Valence chest evidence, missing metrics, mismatched slot/count/item, and Valence-only evidence. r[mc_compatibility.survival_chest_persistence.checker]
- [ ] [depends:checker] Add the `survival-chest-persistence` client/runner rail and unit tests without broadening existing survival scenarios. r[mc_compatibility.survival_chest_persistence.runner]
- [ ] [depends:runner] Add Valence and Paper fixture instrumentation for chest open, store, close, reconnect/reopen, and persisted slot observation. r[mc_compatibility.survival_chest_persistence.fixtures]
- [ ] [depends:fixtures] Produce reviewable paired Paper and Valence receipts/logs under `docs/evidence/`, plus BLAKE3 manifests. r[mc_compatibility.survival_chest_persistence.receipts]
- [ ] [depends:receipts] Promote only the `chest persistence` survival coverage matrix row and keep full survival, all-container, restart/world persistence, and broader vanilla parity as non-claims. r[mc_compatibility.survival_chest_persistence.matrix]
- [ ] [depends:matrix] Run validation and archive the change with checker, manifest, and Cairn evidence recorded. r[mc_compatibility.survival_chest_persistence.validation]
