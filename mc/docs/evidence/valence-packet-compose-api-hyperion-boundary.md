# Valence packet compose API Hyperion boundary

## Question
Can the Valence packet compose API use Hyperion's Compose-style routing concept without copying Hyperion core/runtime behavior or overclaiming compatibility?

## Inspected evidence
- `cairn/changes/add-valence-packet-compose-api/proposal.md`
- `cairn/changes/add-valence-packet-compose-api/design.md`
- `docs/hyperion-integration-boundaries.md`

## Inventory

| source_path | classification | owner | reason | valence_target | safety_notes | evidence | non_claims |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Change proposal/design description of Hyperion `Compose`-style packet routing; no Hyperion source code inspected or copied. | reference | add-valence-packet-compose-api | Routing vocabulary informs a Valence-owned API shape, while implementation uses Valence `Client`, `Entity`, and pure planner types. | `servers/valence/crates/valence_server/src/packet_compose.rs`, docs, and example | Stable Rust only; no unsafe, runtime replacement, scheduler replacement, networking runtime swap, or Bedwars/gameplay logic copied. | Planner/direct-flush unit tests, Valence example smoke, Cairn gates, and validation logs under `docs/evidence/`. | No Hyperion compatibility, proxy mode, production-scale performance, public-server safety, vanilla/reference parity, broad Minecraft compatibility, or default behavior change claim. |

## Decision
Reference the routing concept only. Implement a Valence-owned pure planner plus opt-in direct flush shell. Do not adopt Hyperion code or runtime behavior.

## Owner
add-valence-packet-compose-api

## Next action
Keep evidence and task citations scoped to the Valence-owned API, tests, docs, selected dry-runs, and Cairn archive gates.
