# mc-compatibility Change Spec: Sign editor open/update live rail

## Requirements

### Requirement: Sign editor live contract

r[mc_compatibility.sign_editor_live_rail.contract] The `sign-editor-open-update` rail MUST define a bounded live sign-edit contract before promotion is attempted.

#### Scenario: Contract names one sign edit

r[mc_compatibility.sign_editor_live_rail.contract.scope]
- GIVEN the sign editor row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, packet rows `SignEditorOpenS2CPacket` and `UpdateSignC2SPacket`, backend/client path, expected accepted-update correlation, and non-claims
- AND all sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Sign editor baseline

r[mc_compatibility.sign_editor_live_rail.baseline] The change MUST run existing targeted packet, matrix, bundle, and packet-inventory checks before modifying sign editor rail behavior.

#### Scenario: Existing sign evidence is not reused as live sign-edit proof

r[mc_compatibility.sign_editor_live_rail.baseline.recorded]
- GIVEN sign block-entity persistence evidence exists separately
- WHEN sign editor live work begins
- THEN baseline logs record that `sign-editor-open-update` remains fixture-bounded until dedicated live open/update evidence passes.

### Requirement: Sign editor rail

r[mc_compatibility.sign_editor_live_rail.rail] The harness MUST expose an isolated sign editor open/update rail or deterministic fixture path.

#### Scenario: Sign editor and sign persistence remain separate

r[mc_compatibility.sign_editor_live_rail.rail.isolated]
- GIVEN existing sign block-entity persistence and packet-family rows are maintained separately
- WHEN the sign editor rail is added
- THEN existing sign persistence claims remain unchanged
- AND the sign editor row records separate open/update packet evidence and backend accepted-update correlation.

### Requirement: Sign editor evidence

r[mc_compatibility.sign_editor_live_rail.evidence] Sign editor live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes open and update correlation

r[mc_compatibility.sign_editor_live_rail.evidence.reviewable]
- GIVEN the configured sign edit is observed
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `sign-editor-open-update`, both packet rows, sign position, submitted payload, client open/update milestones, backend accepted-update correlation, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Sign editor checker

r[mc_compatibility.sign_editor_live_rail.checker] The targeted packet live-evidence checker MUST pass before `sign-editor-open-update` moves beyond fixture-bounded status.

#### Scenario: Weak sign editor evidence fails closed

r[mc_compatibility.sign_editor_live_rail.checker.rejects]
- GIVEN evidence lacks open or update correlation, names the wrong packet row, reports the wrong sign position or payload, has a stale receipt digest, or claims broad sign editing/block-entity coverage
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Sign editor narrow promotion

r[mc_compatibility.sign_editor_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `sign-editor-open-update` after row-specific live evidence passes.

#### Scenario: Broader sign behavior remains non-claim

r[mc_compatibility.sign_editor_live_rail.promotion.nonclaims]
- GIVEN sign editor live evidence passes
- WHEN docs are updated
- THEN only the configured sign editor row moves beyond fixture-bounded status
- AND sign persistence breadth, arbitrary sign text, all sign variants, arbitrary NBT, all block entities, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor validation

r[mc_compatibility.sign_editor_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.sign_editor_live_rail.validation.logs]
- GIVEN sign editor live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, sign editor rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.
