# Proposal: Paired vanilla combat reference receipts

## Why

Exact vanilla combat parity is still a non-claim in the current evidence bundle because the existing combat, armor, knockback, and projectile rows are Valence-only compatibility receipts. The archived `vanilla-combat-reference-parity` checkpoint defined the rejection policy, but it did not produce paired reference receipts. A new bounded rail is needed before any combat reference-parity row can be promoted.

## What Changes

- Add a bounded `vanilla-combat-reference-parity` evidence row for one deterministic melee interaction with named attacker, victim, weapon, armor state, health delta, knockback/velocity metric, tolerance bounds, and reference implementation.
- Replace or retire the stale Python parity gate with a Rust checker that has pure comparison logic plus positive and negative fixtures.
- Add isolated Paper-reference and Valence fixture/runner support for the same interaction without changing existing CTF combat rows.
- Produce paired Paper-reference and Valence receipts/logs under `docs/evidence/` with BLAKE3 manifests and machine-recorded child revisions.
- Promote only the configured reference-parity row after the paired comparator passes; keep exact Mojang vanilla parity, broad combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, and production readiness as explicit non-claims.

## Impact

- **Area**: combat reference parity evidence.
- **Files**: runner/client probes, Paper and Valence fixtures, Rust parity checker, evidence docs/manifests, acceptance matrix/current bundle, Cairn specs/tasks.
- **Testing**: checker positive/negative fixtures, paired comparator, maintained dry-run where applicable, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
- **Non-claims**: exact Mojang vanilla parity unless a direct Mojang oracle is added, all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, public-server safety, and production readiness.
