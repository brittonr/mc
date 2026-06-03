# Vanilla combat reference paired receipts — 2026-06-02

## Covered row

`vanilla-combat-reference-parity` is covered only for one bounded Paper-reference interaction:

- reference oracle: `paper-1.20.1-reference-harness`
- reference version: `minecraft-1.20.1-protocol-763`
- attacker/victim: `compatbota` hits `compatbotb`
- weapon/armor: `iron_sword`, armor `none`
- expected first-hit health: `20.0 -> 14.0`, `damage_delta=6.0`
- normalized knockback metric: `0.00` with tolerance `0.05`
- Valence child revision: `be02847ea666`
- Stevenarella child revision: `75151ca3cd4e`

## Evidence

| Artifact | Path | BLAKE3 |
| --- | --- | --- |
| Live wrapper log | `docs/evidence/vanilla-combat-reference-paired-live-2026-06-02.run.log` | `30df23712403e171e16f280e7a802be1c818c38958ac132ddfd5f70c3976949b` |
| Paper receipt | `docs/evidence/vanilla-combat-reference-paired-paper-receipt-2026-06-02.json` | `cb87d9527c8149999af788e6dfb010cf1c7e4855781987cd711a55882066b1d0` |
| Valence receipt | `docs/evidence/vanilla-combat-reference-paired-valence-receipt-2026-06-02.json` | `42cecaabec0ad62d66f4cef83c03a0120843f91b5099d97e3426bbdca4e3a0e7` |
| Comparator log | `docs/evidence/vanilla-combat-reference-paired-compare-2026-06-02.run.log` | `5d5e6acf1675ce3af9cc5f81af753b39cd6fcdab71daa5ae385dfe6653582033` |
| Live manifest | `docs/evidence/vanilla-combat-reference-paired-live-2026-06-02.b3` | manifest includes Paper/Valence receipts, logs, typed events, and KV inputs |
| Comparator manifest | `docs/evidence/vanilla-combat-reference-paired-compare-2026-06-02.b3` | manifest includes comparator log, KV inputs, and Rust checker source |
| Fixture manifest | `docs/evidence/vanilla-combat-reference-paired-fixtures-2026-06-02.b3` | manifest includes source patches, final fixture validation log, Paper jar, and runner/manifest source |

The Rust comparator passed against normalized paired inputs:

- Paper reference KV: `docs/evidence/vanilla-combat-reference-paired-paper-reference-2026-06-02.kv`
- Valence KV: `docs/evidence/vanilla-combat-reference-paired-valence-reference-2026-06-02.kv`

## Promotion decision

Promote only `vanilla-combat-reference-parity` for this configured Paper-reference row. Existing Valence-only combat damage, knockback, armor, projectile, CTF, and survival rows keep their previous scope.

## Non-claims

This does not claim exact Mojang vanilla parity, all combat balancing, all weapons, all armor/enchantments/status effects, projectile physics, death/drop semantics, full CTF correctness, full Minecraft compatibility, public-server safety, production readiness, or behavior outside the named Paper-reference fixture.
