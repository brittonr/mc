# mc-compatibility Change Spec: Chat/command containment promotion

## Requirements

### Requirement: Chat/command containment contract

r[mc_compatibility.chat_command_containment_promotion.contract] The `chat-command-containment` row MUST define a bounded owned-local promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one harmless payload

r[mc_compatibility.chat_command_containment_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, packet row or rows, harmless payload, owned-local target scope, server receipt or rejection metric, redaction policy, child revisions, and checker metrics
- AND all chat signing/security, all commands, command permissions, moderation, public-server safety, adversarial resilience, full protocol-763 compatibility, and production readiness remain explicit non-claims.

### Requirement: Chat/command containment checker

r[mc_compatibility.chat_command_containment_promotion.checker] A deterministic Rust checker MUST validate normalized chat/command containment evidence before promotion.

#### Scenario: Valid chat/command containment evidence passes

r[mc_compatibility.chat_command_containment_promotion.checker.valid]
- GIVEN normalized evidence names `chat-command-containment`, clean child revisions, owned-local scope, harmless payload identity, server receipt or rejection metric, redaction policy, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak chat/command containment evidence fails closed

r[mc_compatibility.chat_command_containment_promotion.checker.rejects]
- GIVEN evidence is missing the row id, lacks owned-local scope, uses stale revisions, names the wrong payload or packet row, omits server correlation, lacks redaction policy, or claims public-server/security/command breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Chat/command containment rail

r[mc_compatibility.chat_command_containment_promotion.rail] The harness MUST expose an isolated chat/command containment rail without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Safety scope is isolated

r[mc_compatibility.chat_command_containment_promotion.rail.isolated]
- GIVEN existing network/public-server rows have separate safety contracts
- WHEN the chat/command containment rail is added
- THEN existing safety claims remain unchanged
- AND the new row records only owned-local fixture evidence.

### Requirement: Chat/command containment artifacts

r[mc_compatibility.chat_command_containment_promotion.artifacts] Review-critical chat/command containment artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and redaction policy

r[mc_compatibility.chat_command_containment_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, redaction policy, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow chat/command matrix promotion

r[mc_compatibility.chat_command_containment_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured chat/command containment row after checker and evidence gates pass.

#### Scenario: Broader chat/command safety remains a non-claim

r[mc_compatibility.chat_command_containment_promotion.matrix.nonclaims]
- GIVEN chat/command containment evidence passes
- WHEN docs are updated
- THEN only the configured owned-local row is marked covered
- AND public-server safety, security, all commands, chat signing, moderation, full protocol, and production claims remain explicit non-claims.

### Requirement: Chat/command containment validation evidence

r[mc_compatibility.chat_command_containment_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chat_command_containment_promotion.validation.log]
- GIVEN the chat/command containment row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
