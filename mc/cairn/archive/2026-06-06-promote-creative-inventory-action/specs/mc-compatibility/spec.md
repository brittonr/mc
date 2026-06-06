# mc-compatibility Change Spec: Creative inventory action promotion

## Requirements

### Requirement: Creative inventory action contract

r[mc_compatibility.creative_inventory_action_promotion.contract] The `creative-inventory-action` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one creative slot mutation

r[mc_compatibility.creative_inventory_action_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, creative game-mode precondition, packet row `CreativeInventoryActionC2SPacket`, semantic slot, wire slot, item, count, server acceptance metric, final slot state, child revisions, and checker metrics
- AND all creative inventory semantics, all slots, all items, all game-mode transitions, all pick-block behavior, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Creative inventory action checker

r[mc_compatibility.creative_inventory_action_promotion.checker] A deterministic Rust checker MUST validate normalized creative inventory evidence before promotion.

#### Scenario: Valid creative action evidence passes

r[mc_compatibility.creative_inventory_action_promotion.checker.valid]
- GIVEN normalized evidence names `creative-inventory-action`, clean child revisions, creative-mode precondition, configured slot/item/count, client action milestone, Valence acceptance, and final slot state
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak creative action evidence fails closed

r[mc_compatibility.creative_inventory_action_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks creative game mode, uses stale revisions, names the wrong slot/item/count, omits server acceptance, mismatches final state, or claims broad creative inventory coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Creative inventory action rail

r[mc_compatibility.creative_inventory_action_promotion.rail] The harness MUST expose an isolated creative inventory rail without changing existing survival/player-inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Survival inventory rows stay separate

r[mc_compatibility.creative_inventory_action_promotion.rail.isolated]
- GIVEN existing inventory rows cover survival/player-inventory actions
- WHEN the creative rail is added
- THEN existing inventory scenario milestones and non-claims remain unchanged
- AND the creative row records its own game-mode, packet, and slot-state evidence.

### Requirement: Creative inventory reviewable artifacts

r[mc_compatibility.creative_inventory_action_promotion.artifacts] Review-critical creative inventory artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.creative_inventory_action_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow creative inventory matrix promotion

r[mc_compatibility.creative_inventory_action_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured creative inventory action row after checker and evidence gates pass.

#### Scenario: Broader creative inventory remains a non-claim

r[mc_compatibility.creative_inventory_action_promotion.matrix.nonclaims]
- GIVEN creative inventory action evidence passes
- WHEN docs are updated
- THEN only the configured creative row is marked covered
- AND all broader creative inventory, all slots/items, game-mode breadth, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Creative inventory validation evidence

r[mc_compatibility.creative_inventory_action_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.creative_inventory_action_promotion.validation.log]
- GIVEN the creative inventory action row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
