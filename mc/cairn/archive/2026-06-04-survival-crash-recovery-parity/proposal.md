# Proposal: Bounded survival crash-recovery parity

## Why

The current survival matrix promotes a bounded world-persistence restart row, but it explicitly leaves crash recovery and broader durability as non-claims. The next useful reduction is a narrow paired Paper/reference and Valence row that proves one configured block mutation remains observable after an ungraceful server stop and restart, without claiming broad durability or production readiness.

## What Changes

- Add `survival-crash-recovery-parity` as a distinct runner scenario, manifest row, receipt marker, and survival row-parity contract.
- Reuse the existing isolated world-persistence storage shape where possible, but record crash-specific client/server milestones for one configured mutation of `Dirt` at `24,64,0`.
- Orchestrate the restart with an ungraceful process/container stop marker instead of the existing graceful shutdown marker.
- Produce paired Paper-reference and Valence receipts/logs, normalized KV comparator inputs, evidence docs, and BLAKE3 manifests under `docs/evidence/`.
- Update the acceptance matrix/current bundle only for this one crash-recovery row while keeping long-term durability, crash consistency, multi-chunk persistence, all block entities, concurrent saves, backups, broad survival compatibility, broad vanilla parity, and production readiness as explicit non-claims.

## Impact

- **Area**: bounded survival persistence evidence.
- **Files**: `tools/mc-compat-runner`, `tools/check_survival_row_parity.rs`, scenario manifest/generated table, Paper survival fixture wiring if needed, Valence survival fixture wiring if needed, survival matrix/current bundle docs, evidence docs/manifests, Cairn artifacts.
- **Testing**: row-parity checker positive/negative fixtures, runner unit tests, scenario manifest check, paired receipt comparator, evidence manifest/task-evidence gates, Cairn gates, and Cairn validation.
