# Tasks

- [x] [serial] Define the bounded `full CTF correctness aggregate` evidence contract and normalized metric names. r[mc_compatibility.full_ctf_correctness_gate.contract] Evidence: `docs/evidence/open-cairn-contracts-2026-06-01.run.log`, `docs/evidence/open-cairn-contracts-2026-06-01.b3`.
- [x] [depends:contract] Add deterministic checker positive and negative fixtures for `full CTF correctness aggregate`. r[mc_compatibility.full_ctf_correctness_gate.checker] Evidence: `docs/evidence/mc-compat-row-contract-checker-2026-06-01.run.log`, `docs/evidence/mc-compat-row-contract-checker-2026-06-01.b3`.
- [ ] [depends:checker] Enforce row-specific evidence standard before promotion. r[mc_compatibility.full_ctf_correctness_gate.evidence_standard]
- [ ] [depends:evidence_standard] Add `full-ctf-correctness-gate` rail/checker wiring without broadening existing scenarios. r[mc_compatibility.full_ctf_correctness_gate.rail]
  - Detail: Define required CTF rule-family set.
  - Detail: Add aggregate checker negatives.
  - Detail: Wire acceptance/current bundle claim blocks.
  - Detail: Promote aggregate only after all rule rows covered.
- [ ] [depends:rail] Copy reviewable row artifacts under `docs/evidence/`, including receipts/logs/check output, BLAKE3 manifests, and oracle checkpoints where required. r[mc_compatibility.full_ctf_correctness_gate.artifacts]
- [ ] [depends:artifacts] Promote only the `full CTF correctness aggregate` row in matrix/current-bundle docs and keep adjacent non-claims explicit. r[mc_compatibility.full_ctf_correctness_gate.matrix]
- [ ] [depends:matrix] Run row checker self-tests, row evidence checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation with output copied under `docs/evidence/`. r[mc_compatibility.full_ctf_correctness_gate.validation]
