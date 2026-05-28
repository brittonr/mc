# Protocol-763 vanilla combat parity checkpoint — 2026-05-27

## Scope

This checkpoint drains the vanilla combat parity Cairn by defining the reference-oracle contract, metrics, tolerances, and fail-closed comparison fixtures. It promotes no vanilla parity rows.

exact vanilla combat parity remains a non-claim.

## Reference oracle decision

- oracle_name: paper-1.20.1-reference-harness
- oracle_version: minecraft-1.20.1-protocol-763
- oracle_configuration: future local owned Paper/vanilla-compatible harness with deterministic PvP fixtures, fixed world seed, fixed player equipment, fixed latency/jitter/loss defaults, and per-row BLAKE3 manifests.
- evidence_path: none
- reference_receipt: none
- valence_reference_pair: none
- decision_owner: agent
- limitations: no reference harness was run in this checkpoint; Valence-only receipts are not accepted as parity evidence; Paper is itself a compatibility implementation and must be documented if used instead of Mojang vanilla.

This checkpoint rejects Valence-only evidence and blocks parity promotion until paired reference and Valence receipts exist.

## Metrics and tolerances

| Metric | Unit | Tolerance | Required reference value | Required Valence value | Rationale |
| --- | --- | --- | --- | --- | --- |
| damage_delta_half_hearts | half-hearts | exact unless row states otherwise | reference victim health before/after | Valence victim health before/after | Melee damage parity must compare health deltas. |
| knockback_velocity_vector | velocity components | row-defined vector tolerance | reference velocity update | Valence velocity update | Floating/vector drift needs explicit tolerance. |
| armor_mitigation_delta | half-hearts | row-defined numeric tolerance | reference mitigated damage | Valence mitigated damage | Armor math must compare base, mitigation, and final damage. |
| projectile_damage_delta | half-hearts | exact unless row states otherwise | reference projectile hit damage | Valence projectile hit damage | Projectile damage parity requires paired hit evidence. |

Metrics without a tolerance, reference value, Valence value, and paired receipts remain non-claims.

## Fixtures

`tools/check_vanilla_combat_parity.py --self-test` includes:

- positive equal-within-tolerance fixture;
- positive within-tolerance armor_mitigation_delta fixture;
- missing_reference negative fixture;
- wrong_reference_version negative fixture;
- out_of_tolerance negative fixture;
- valence_only_evidence negative fixture.

## Promotion gate

A vanilla parity row is promotable only when all are true:

- reference receipt exists and names `paper-1.20.1-reference-harness` or a more direct vanilla oracle;
- reference version is `minecraft-1.20.1-protocol-763`;
- Valence receipt exists for the same fixture;
- metric, tolerance, unit, reference value, Valence value, and rationale are present;
- both receipts and logs are copied under `docs/evidence/` with BLAKE3 manifests;
- acceptance matrix and current bundle add the row without weakening existing non-claims.

No row satisfies this gate today.

## Decision

- Question: Can any exact vanilla combat parity claim be promoted from the current Valence-only evidence set?
- Inspected evidence: acceptance matrix, current bundle, combat/knockback/armor/projectile receipts, and parity checker fixtures.
- Decision: No. Current receipts remain bounded Valence/Stevenarella compatibility evidence only; exact vanilla combat parity remains a non-claim.
- Owner: agent.
- Next action: add a paired reference harness and BLAKE3-backed reference receipts before proposing any parity row.

## Non-claims

No exact vanilla combat parity, vanilla damage parity, vanilla knockback parity, vanilla armor mitigation parity, vanilla projectile damage parity, production PvP readiness, full combat correctness, full CTF correctness, or broad Minecraft compatibility claim is made.
