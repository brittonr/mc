# mc-compatibility Change Spec: Chat command containment live rail

## Requirements

### Requirement: Chat command live contract

r[mc_compatibility.chat_command_live_rail.contract] The `chat-command-containment` live rail MUST define a bounded owned-local contract before live promotion is attempted.

#### Scenario: Contract names one harmless payload

r[mc_compatibility.chat_command_live_rail.contract.scope]
- GIVEN the chat/command containment row is prepared for live promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, one harmless payload identity, packet row or rows, owned-local target scope, expected server receipt or rejection metric, redaction policy, backend/client path, and non-claims
- AND chat signing/security, command permissions, moderation, all commands, malicious-client resilience, public-server safety, production readiness, and full protocol 763 compatibility remain explicit non-claims.

### Requirement: Chat command baseline

r[mc_compatibility.chat_command_live_rail.baseline] The change MUST run existing targeted packet, matrix, current-bundle, and packet-inventory checks before modifying chat/command rail behavior.

#### Scenario: Fixture status is recorded first

r[mc_compatibility.chat_command_live_rail.baseline.recorded]
- GIVEN `chat-command-containment` has fixture-bounded evidence
- WHEN live-rail work begins
- THEN baseline logs record its existing evidence classification and non-claims before live evidence is introduced.

### Requirement: Chat command live rail

r[mc_compatibility.chat_command_live_rail.rail] The harness MUST expose an isolated owned-local chat/command rail or deterministic missing-driver blocker for the configured payload.

#### Scenario: Rail is isolated from public-server safety

r[mc_compatibility.chat_command_live_rail.rail.isolated]
- GIVEN existing public-server and network-safety rows have separate authorization contracts
- WHEN the chat/command rail is added
- THEN it uses only owned-local fixture targets
- AND existing CTF, survival, combat, inventory, network, public-server, and production-readiness claims remain unchanged.

### Requirement: Chat command live evidence

r[mc_compatibility.chat_command_live_rail.evidence] Chat/command live evidence MUST be durable and reviewable under `docs/evidence/` before promotion.

#### Scenario: Evidence includes containment and redaction fields

r[mc_compatibility.chat_command_live_rail.evidence.reviewable]
- GIVEN the configured chat/command payload is observed or blocked by a missing driver
- WHEN evidence is written
- THEN KV, receipt, and log artifacts name `chat-command-containment`, packet rows, payload identity, owned-local scope, server containment metric or blocker, redaction policy, backend/client path, revision metadata when available, and explicit non-claims.

### Requirement: Chat command live checker

r[mc_compatibility.chat_command_live_rail.checker] The targeted packet live-evidence checker MUST pass before `chat-command-containment` moves beyond fixture-bounded status.

#### Scenario: Weak chat evidence fails closed

r[mc_compatibility.chat_command_live_rail.checker.rejects]
- GIVEN chat evidence is missing, lacks owned-local scope, names the wrong payload or packet row, omits server correlation, reports a stale receipt digest, lacks redaction policy, or claims public-server/security/command breadth
- WHEN the checker evaluates the evidence
- THEN it fails with an explicit diagnostic and no docs are promoted.

### Requirement: Chat command narrow promotion

r[mc_compatibility.chat_command_live_rail.promotion] Matrix, current-bundle, and packet-inventory docs MUST promote only `chat-command-containment` after row-specific live evidence passes.

#### Scenario: Broader chat behavior remains non-claim

r[mc_compatibility.chat_command_live_rail.promotion.nonclaims]
- GIVEN chat/command live evidence passes
- WHEN docs are updated
- THEN only the configured owned-local containment row moves beyond fixture-bounded status
- AND chat signing/security, command permissions, moderation, all commands, public-server safety, full protocol coverage, and production readiness remain explicit non-claims.

### Requirement: Chat command validation

r[mc_compatibility.chat_command_live_rail.validation] The change MUST record rail checks, targeted packet checks, matrix/bundle/inventory checks, evidence checks, Cairn gates, sync/archive checks, and post-archive validation.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.chat_command_live_rail.validation.logs]
- GIVEN chat/command live rail work is complete
- WHEN the change is archived
- THEN reviewable logs show baseline checks, rail checks or documented blockers, checker positive/negative coverage, matrix/bundle/inventory checks, evidence-manifest checks, task-evidence checks, Cairn proposal/design/tasks gates, Cairn sync/archive checks, and Cairn validation passing.
