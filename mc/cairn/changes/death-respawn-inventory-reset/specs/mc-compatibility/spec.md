# Delta: Death respawn inventory reset rail

## Requirements

### Requirement: Contract

r[mc_compatibility.death_respawn_inventory_reset.contract] The `death inventory reset` row MUST define a bounded deterministic evidence contract before producing or promoting receipts.

#### Scenario: Contract names exact scope

r[mc_compatibility.death_respawn_inventory_reset.contract.scope]
- GIVEN `death-respawn-inventory-reset` work starts
- WHEN the evidence contract is reviewed
- THEN it names one configured death event with pre-death inventory, death/drop or reset policy, respawn, and post-respawn inventory state
- AND it states that all death causes, all inventory policies, XP drops, item despawn timing, full death/respawn lifecycle, full CTF correctness, and production readiness remain non-claims.

### Requirement: Normalized checker

r[mc_compatibility.death_respawn_inventory_reset.checker] A deterministic checker MUST compare normalized metrics before the `death inventory reset` row is promoted.

#### Scenario: Missing or mismatched evidence fails closed

r[mc_compatibility.death_respawn_inventory_reset.checker.rejects]
- GIVEN evidence is missing or mismatches pre-death inventory slots, death cause, drop/reset policy, dropped item ids/counts, respawn inventory slots, and server correlation milestones
- WHEN the checker runs
- THEN it fails and names the missing or mismatched metric.

#### Scenario: Evidence standard is enforced

r[mc_compatibility.death_respawn_inventory_reset.checker.standard]
- GIVEN the row requires live receipt with explicit inventory policy and negative mismatch checks
- WHEN evidence lacks that standard
- THEN promotion fails before any matrix or current-bundle claim is recorded.

### Requirement: Runner or fixture rail

r[mc_compatibility.death_respawn_inventory_reset.rail] The harness MUST expose a `death-respawn-inventory-reset` rail or fixture set that records required client/server/protocol milestones without changing existing row semantics.

#### Scenario: Rail is isolated

r[mc_compatibility.death_respawn_inventory_reset.rail.isolated]
- GIVEN existing maintained scenarios and evidence rows
- WHEN `death-respawn-inventory-reset` is added
- THEN existing milestone requirements and claims remain unchanged
- AND the new row has explicit required milestones or fixture outputs.

### Requirement: Reviewable evidence

r[mc_compatibility.death_respawn_inventory_reset.evidence] `death inventory reset` evidence MUST be reviewable locally before promotion.

#### Scenario: Evidence artifacts are durable

r[mc_compatibility.death_respawn_inventory_reset.evidence.reviewable]
- GIVEN the `death inventory reset` row is proposed for promotion
- WHEN reviewers inspect the repo
- THEN required receipts, logs, checker output, and BLAKE3 manifests are present under `docs/evidence/`
- AND child revisions, target ownership, authorization, or oracle checkpoints are recorded when applicable.

### Requirement: Matrix promotion

r[mc_compatibility.death_respawn_inventory_reset.matrix] Acceptance matrix and current-bundle docs MUST promote only the `death inventory reset` row after required evidence passes.

#### Scenario: Broader claims remain false

r[mc_compatibility.death_respawn_inventory_reset.matrix.nonclaims]
- GIVEN `death inventory reset` evidence passes
- WHEN matrix/current-bundle docs are updated
- THEN only the configured `death inventory reset` row is marked covered
- AND all death causes, all inventory policies, XP drops, item despawn timing, full death/respawn lifecycle, full CTF correctness, and production readiness remain explicit non-claims.

### Requirement: Validation

r[mc_compatibility.death_respawn_inventory_reset.validation] The change MUST record checker, manifest, task gate, and Cairn validation output before archive.

#### Scenario: Validation output is reviewable

r[mc_compatibility.death_respawn_inventory_reset.validation.log]
- GIVEN the `death inventory reset` row is completed
- WHEN the change is archived
- THEN repo-local logs record the row checker, acceptance/current-bundle checks, evidence manifest check, task-evidence gate, and Cairn validation/gates.
