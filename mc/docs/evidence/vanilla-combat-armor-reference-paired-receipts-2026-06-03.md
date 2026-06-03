# Vanilla combat armor reference paired receipts — 2026-06-03

## Covered row

`vanilla-combat-armor-reference-parity` is covered only for one bounded Paper-reference armor interaction:

- reference oracle: `paper-1.20.1-reference-harness`
- reference version: `minecraft-1.20.1-protocol-763`
- attacker/victim: `compatbota` hits `compatbotb`
- weapon/armor: `iron_sword`, armor `diamond_chestplate`
- enchantments/status effects: none / none
- expected first-hit health: `20.0 -> 15.3`, `damage_delta=4.7`
- normalized knockback metric: `0.00` with tolerance `0.05`
- Valence child revision: `f41e8649ba43`
- Stevenarella child revision: `75151ca3cd4e`

## Evidence

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Live wrapper summary | `docs/evidence/vanilla-combat-armor-reference-paired-live-2026-06-03.run.log` | `6cf142f231468bfa1a21e119375b15066f4dedb98a2485a7163eef74c554f7df` |
| Paper live log | `docs/evidence/vanilla-combat-armor-reference-paired-paper-live-2026-06-03.run.log` | `c5ef70bc62852593e81771d474041e869328263622449f58ed33c8643cb85e9a` |
| Valence live log | `docs/evidence/vanilla-combat-armor-reference-paired-valence-live-2026-06-03.run.log` | `1c731ec08e3e2509798922f43d461f699bd27ea89b70282b3b65feab961f70b2` |
| Paper receipt | `docs/evidence/vanilla-combat-armor-reference-paired-paper-receipt-2026-06-03.json` | `364e022f368e355ccfbe24207700cdb02d6b3df15a48e9fd7bf5166cb1997440` |
| Valence receipt | `docs/evidence/vanilla-combat-armor-reference-paired-valence-receipt-2026-06-03.json` | `f89fdfd939f7620f3c111502577642e43ec3518126a03dae2bd7a48dd627b91d` |
| Paper server log | `docs/evidence/vanilla-combat-armor-reference-paired-paper-server-2026-06-03.log` | `c03ffda1966968c0a8f02e6cd19153a97a5fabcd78aff4d33914159c4383af65` |
| Valence server log | `docs/evidence/vanilla-combat-armor-reference-paired-valence-server-2026-06-03.log` | `a6103d8b988002ffecfb8593a689cf5558104b0cf8bb91fe24d45f9592fc6029` |
| Paper typed events | `docs/evidence/vanilla-combat-armor-reference-paired-paper-typed-events-2026-06-03.log` | `0ef6b9a1b6b365c0a3fae011942818679592f4b7801cca7c736886f4ad9df647` |
| Valence typed events | `docs/evidence/vanilla-combat-armor-reference-paired-valence-typed-events-2026-06-03.log` | `6f8d14c5f606761878fc98f6c02b3208ab64d10d4db5ff983ff82120018b8ff8` |
| Paper KV input | `docs/evidence/vanilla-combat-armor-reference-paired-paper-reference-2026-06-03.kv` | `a182bf3edc4c96f6d49c672d1f766c78319cb856fb3ef322d8c6c2b2ffc9998f` |
| Valence KV input | `docs/evidence/vanilla-combat-armor-reference-paired-valence-reference-2026-06-03.kv` | `206d6ff6aac22010f69ca3b2a6ccd284722f50252df8bfa76ab8e161bf086105` |
| Comparator log | `docs/evidence/vanilla-combat-armor-reference-paired-compare-2026-06-03.run.log` | `b2b82048e4240d537b001d7badd1d2b05de3ae89c388d53d95c670057265b091` |
| Paper fixture jar | `docs/evidence/mc-compat-paper-vanilla-combat-armor-reference-fixture-2026-06-03.jar` | `88c62ab589fbad3f4cb0303d6ad282d6df98dfb11dfe3f9e54eeb8d3d2a5985c` |
| Paper fixture source snapshot | `docs/evidence/mc-compat-paper-vanilla-combat-armor-reference-fixture-2026-06-03.java` | `6b47ffa09612743da12f52f8c62423b696c856c6869929e95f96c3f9f813fd03` |
| Fixture build log | `docs/evidence/vanilla-combat-armor-reference-fixtures-2026-06-03.run.log` | `3ebe948e2b0c1b5179ac51a0c872a1ef6d5369a96ac9166805107ab6fa0064f1` |

The Rust comparator passed against normalized paired inputs:

- Paper reference KV: `docs/evidence/vanilla-combat-armor-reference-paired-paper-reference-2026-06-03.kv`
- Valence KV: `docs/evidence/vanilla-combat-armor-reference-paired-valence-reference-2026-06-03.kv`

## Promotion decision

Promote only `vanilla-combat-armor-reference-parity` for this configured Paper-reference row. Existing no-armor reference, Valence-only combat damage, knockback, armor mitigation, projectile, CTF, and survival rows keep their previous scope.

## Non-claims

This does not claim exact Mojang vanilla parity, all combat balancing, all weapons, all armor loadouts, enchantments/status effects, modifier stacking, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, public-server safety, production readiness, or behavior outside the named Paper-reference fixture.
