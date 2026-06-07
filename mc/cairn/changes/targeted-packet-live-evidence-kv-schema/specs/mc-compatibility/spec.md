# mc-compatibility Change Spec: Targeted packet live evidence KV schema

## Requirements

### Requirement: Targeted packet live KV contract

r[mc_compatibility.targeted_packet_live_kv_schema.contract] Targeted packet live promotion evidence MUST use a shared KV schema before rows move beyond fixture-bounded status.

#### Scenario: Common live promotion keys are explicit

r[mc_compatibility.targeted_packet_live_kv_schema.contract.common_keys]
- GIVEN a targeted packet row is proposed for live promotion
- WHEN reviewers inspect the evidence KV
- THEN it includes row id, live promotion status, evidence mode, packet row, scenario, backend/client path, receipt path, receipt BLAKE3 digest, digest currentness, revision metadata when available, and explicit non-claims
- AND blocker or selection notes are not accepted as live promotion evidence.

### Requirement: Pure KV schema core

r[mc_compatibility.targeted_packet_live_kv_schema.core] Live evidence schema validation MUST be pure deterministic logic over parsed key/value records.

#### Scenario: Schema validation has no side effects

r[mc_compatibility.targeted_packet_live_kv_schema.core.pure]
- GIVEN parsed key/value evidence and a row contract
- WHEN schema validation runs
- THEN it returns success or diagnostics without reading files, writing files, spawning commands, inspecting environment, using clocks, or performing network access.

### Requirement: Row extension hooks

r[mc_compatibility.targeted_packet_live_kv_schema.extensions] The schema MUST support row-specific validation extensions without weakening common live-promotion requirements.

#### Scenario: Extensions keep row metrics explicit

r[mc_compatibility.targeted_packet_live_kv_schema.extensions.row_metrics]
- GIVEN creative inventory, resource-pack status, sign editor, or future targeted packet rows need row-specific metrics
- WHEN their live evidence is validated
- THEN common keys are checked first
- AND extension diagnostics name missing or mismatched row-specific metrics such as slot/item/count, local resource-pack offer/status, sign position/payload, or backend correlation.

### Requirement: KV schema tests

r[mc_compatibility.targeted_packet_live_kv_schema.tests] The change MUST include positive and negative tests for common and row-specific live evidence validation.

#### Scenario: Invalid live evidence fails closed

r[mc_compatibility.targeted_packet_live_kv_schema.tests.negative]
- GIVEN evidence is missing required keys, names the wrong packet row, reports a stale receipt digest, lacks required revision metadata, has malformed row-specific fields, or claims broad protocol/gameplay/public-server coverage
- WHEN the checker validates the evidence
- THEN it fails with explicit diagnostics and no promotion is accepted.

### Requirement: KV schema documentation

r[mc_compatibility.targeted_packet_live_kv_schema.docs] The repository SHOULD document the live evidence KV schema and future live-rail workflow.

#### Scenario: Future live rails can follow the schema

r[mc_compatibility.targeted_packet_live_kv_schema.docs.workflow]
- GIVEN a future targeted packet live rail is implemented
- WHEN maintainers inspect the workflow docs
- THEN they can identify required common keys, row-extension fields, non-claim requirements, checker command shape, and evidence-manifest expectations.

### Requirement: KV schema validation

r[mc_compatibility.targeted_packet_live_kv_schema.validation] The change MUST record targeted packet checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.targeted_packet_live_kv_schema.validation.logs]
- GIVEN the KV schema work is complete
- WHEN the change is archived
- THEN reviewable logs show checker positive/negative tests, targeted packet checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync, Cairn archive, and Cairn validation passing.
