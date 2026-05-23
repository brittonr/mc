# MC compatibility boundary: Valence main vs Stevenarella

Date (UTC): 2026-05-23

## Summary

Current Valence main is **not** the same compatibility target as the preserved Stevenarella → Valence smoke proof.

- Current Valence checkout: `c5140b7`
- Valence source: `valence/crates/valence_protocol/src/lib.rs`
- Valence advertised Minecraft version: `1.20.1`
- Valence advertised protocol: `763`
- Stevenarella checkout: `815ac88`
- Stevenarella protocol source: `stevenarella/protocol/src/protocol/mod.rs`
- Stevenarella default/highest supported protocol: `758`
- Stevenarella default/highest supported version: `1.18.2`
- Leafish role: reference-only; not a live backend/client target for this workstream.

## Live probe

Ran Valence `parkour` live smoke and status-probed the listening server.

- Parkour receipt schema: `valence.parkour-smoke.receipt.v1`
- Parkour smoke status: `passed`
- Status probe for expected protocol `763`: passed
- Observed server status version: `1.20.1`
- Observed server status protocol: `763`
- Status probe for expected protocol `758`: failed as expected with `protocol mismatch in status response; expected 758`

Observed status body:

```json
{"description":{"text":"A Valence Server"},"players":{"max":20,"online":0,"sample":[]},"version":{"name":"1.20.1","protocol":763}}
```

## Boundary

The durable live compatibility proof remains Stevenarella → Valence protocol `758` (`1.18.2`). Current Valence main advertises protocol `763` (`1.20.1`). Therefore current Valence main compatibility should not be claimed until one of these happens:

1. Stevenarella is updated to support protocol `763`, or
2. Valence is pinned/translated back to protocol `758` for the compatibility harness.

This evidence intentionally makes **no** claim of current Valence client compatibility, Stevenarella protocol `763` support, or semantic correctness.

## Receipt

- Receipt: `docs/evidence/mc-compat-valence-boundary-2026-05-23.receipt.json`
- Receipt BLAKE3: `a811399eed3b9d3360367151e17a6f10db1ff655df5e07c839a7521895982836`
