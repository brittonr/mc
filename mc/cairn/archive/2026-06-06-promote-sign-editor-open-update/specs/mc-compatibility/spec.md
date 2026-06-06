# mc-compatibility Change Spec: Sign editor open/update promotion

## Requirements

### Requirement: Sign editor open/update contract

r[mc_compatibility.sign_editor_open_update_promotion.contract] The `sign-editor-open-update` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one sign edit

r[mc_compatibility.sign_editor_open_update_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, sign position, initial sign state, submitted four-line payload, `SignEditorOpenS2CPacket`, `UpdateSignC2SPacket`, server acceptance metric, child revisions, and checker metrics
- AND all sign editing UI behavior, all sign variants, all text formats, arbitrary NBT semantics, all block entities, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor open/update checker

r[mc_compatibility.sign_editor_open_update_promotion.checker] A deterministic Rust checker MUST validate normalized sign editor open/update evidence before promotion.

#### Scenario: Valid sign edit evidence passes

r[mc_compatibility.sign_editor_open_update_promotion.checker.valid]
- GIVEN normalized evidence names `sign-editor-open-update`, clean child revisions, the configured sign position and payload, client open/update milestones, and Valence server accepted-update correlation
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak sign edit evidence fails closed

r[mc_compatibility.sign_editor_open_update_promotion.checker.rejects]
- GIVEN evidence is missing the row id, omits open or update correlation, uses stale or unknown child revisions, reports the wrong sign position or payload, lacks server acceptance, or claims broad sign editing or block-entity coverage
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Sign editor open/update rail

r[mc_compatibility.sign_editor_open_update_promotion.rail] The harness MUST expose an isolated sign editor open/update rail without changing existing sign persistence, survival, inventory, CTF, combat, network, or negative-live semantics.

#### Scenario: Existing sign persistence remains separate

r[mc_compatibility.sign_editor_open_update_promotion.rail.isolated]
- GIVEN existing sign block-entity persistence evidence is already promoted
- WHEN the sign editor rail is added
- THEN existing persistence claims remain unchanged
- AND the new row records separate open/update packet evidence.

### Requirement: Sign editor reviewable artifacts

r[mc_compatibility.sign_editor_open_update_promotion.artifacts] Review-critical sign editor artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.sign_editor_open_update_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow sign editor matrix promotion

r[mc_compatibility.sign_editor_open_update_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured sign editor row after checker and evidence gates pass.

#### Scenario: Broader sign editing remains a non-claim

r[mc_compatibility.sign_editor_open_update_promotion.matrix.nonclaims]
- GIVEN sign editor evidence passes
- WHEN docs are updated
- THEN only the configured sign editor open/update row is marked covered
- AND broad sign editing, arbitrary sign text, all sign variants, block-entity breadth, full protocol coverage, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Sign editor validation evidence

r[mc_compatibility.sign_editor_open_update_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.sign_editor_open_update_promotion.validation.log]
- GIVEN the sign editor row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
