# mc-compatibility Change Spec: Creative inventory action live rail

## Requirements

### Requirement: Creative inventory live contract

r[mc_compatibility.creative_inventory_live_rail.contract] The `creative-inventory-action` live rail MUST define a bounded owned-local contract before live promotion is attempted.

#### Scenario: Contract names one creative mutation

r[mc_compatibility.creative_inventory_live_rail.contract.scope]
- GIVEN the creative inventory row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, creative-mode precondition, semantic slot, wire slot, item id, item count, packet row `CreativeInventoryActionC2SPacket`, backend/client path, expected server correlation, and non-claims
- AND all creative inventory semantics, all slots, all items, all game-mode transitions, pick-block behavior, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Creative inventory baseline

r[mc_compatibility.creative_inventory_live_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before changing the runner.

#### Scenario: Baseline preserves fixture status

r[mc_compatibility.creative_inventory_live_rail.baseline.recorded]
- GIVEN `creative-inventory-action` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record the existing fixture status and current non-claims before live evidence is introduced.

### Requirement: Creative inventory live rail

r[mc_compatibility.creative_inventory_live_rail.rail] The harness MUST expose an isolated owned-local creative inventory rail or deterministic fixture path for the configured mutation.

#### Scenario: Existing inventory rows remain separate

r[mc_compatibility.creative_inventory_live_rail.rail.isolated]
- GIVEN survival/player-inventory rows and targeted packet fixture rows already exist
- WHEN the creative rail is added
- THEN existing survival/player-inventory scenario semantics remain unchanged
- AND the creative row records its own game-mode, packet, slot-state, and backend-correlation evidence.

### Requirement: Creative inventory live evidence

r[mc_compatibility.creative_inventory_live_rail.evidence] Creative inventory live evidence MUST be reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence is row-specific

r[mc_compatibility.creative_inventory_live_rail.evidence.reviewable]
- GIVEN the configured creative mutation is observed
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `creative-inventory-action`, the packet row, scenario, backend/client path, revision metadata when available, slot/item/count metrics, server correlation, and explicit non-claims.

### Requirement: Creative inventory live checker

r[mc_compatibility.creative_inventory_live_rail.checker] The targeted packet live-evidence checker MUST pass before `creative-inventory-action` moves beyond fixture-bounded status.

#### Scenario: Weak creative evidence fails closed

r[mc_compatibility.creative_inventory_live_rail.checker.rejects]
- GIVEN creative evidence is missing, names the wrong packet row, omits the game-mode precondition, omits server correlation, reports a stale receipt digest, or claims broad creative inventory semantics
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no matrix or bundle row is promoted.

### Requirement: Creative inventory narrow promotion

r[mc_compatibility.creative_inventory_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `creative-inventory-action` after row-specific live evidence passes.

#### Scenario: Other targeted rows stay fixture-bounded

r[mc_compatibility.creative_inventory_live_rail.promotion.narrow]
- GIVEN creative live evidence passes and other targeted rows lack live evidence
- WHEN docs are updated
- THEN only `creative-inventory-action` moves beyond fixture-bounded status
- AND every unproven targeted row retains its prior evidence classification and non-claim notes.

### Requirement: Creative inventory validation

r[mc_compatibility.creative_inventory_live_rail.validation] The change MUST record runner checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.creative_inventory_live_rail.validation.logs]
- GIVEN the creative inventory live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.
