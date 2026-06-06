# mc-compatibility Change Spec: Movement packet-family promotion

## Requirements

### Requirement: Movement packet-family contract

r[mc_compatibility.movement_packet_family_promotion.contract] The `movement-packet-family` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names one movement transition

r[mc_compatibility.movement_packet_family_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one actor, start position, target position, look fields if applicable, on-ground state, packet row or rows, server correlation, tolerance if any, child revisions, and checker metrics
- AND movement physics, collision, anti-cheat, latency tolerance, malicious-client resilience, all movement packet variants, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Movement packet-family checker

r[mc_compatibility.movement_packet_family_promotion.checker] A deterministic Rust checker MUST validate normalized movement packet evidence before promotion.

#### Scenario: Valid movement evidence passes

r[mc_compatibility.movement_packet_family_promotion.checker.valid]
- GIVEN normalized evidence names `movement-packet-family`, clean child revisions, the configured movement fields, client action milestone, Valence server correlation, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and within configured tolerance.

#### Scenario: Weak movement evidence fails closed

r[mc_compatibility.movement_packet_family_promotion.checker.rejects]
- GIVEN evidence is missing the row id, uses stale revisions, names the wrong packet variant, omits movement fields, exceeds tolerance, lacks server correlation, or claims physics/anti-cheat/security breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Movement packet-family rail

r[mc_compatibility.movement_packet_family_promotion.rail] The harness MUST expose an isolated movement packet rail without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Existing gameplay movement stays implicit

r[mc_compatibility.movement_packet_family_promotion.rail.isolated]
- GIVEN existing gameplay rows may move clients incidentally
- WHEN the movement packet rail is added
- THEN existing rows remain unchanged
- AND the new row records explicit normalized movement metrics.

### Requirement: Movement packet artifacts

r[mc_compatibility.movement_packet_family_promotion.artifacts] Review-critical movement packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and revisions

r[mc_compatibility.movement_packet_family_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts, logs, normalized inputs, checker output, BLAKE3 manifests, child revisions, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow movement packet matrix promotion

r[mc_compatibility.movement_packet_family_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured movement row after checker and evidence gates pass.

#### Scenario: Broader movement correctness remains a non-claim

r[mc_compatibility.movement_packet_family_promotion.matrix.nonclaims]
- GIVEN movement packet evidence passes
- WHEN docs are updated
- THEN only the configured movement row is marked covered
- AND physics, anti-cheat, collision, latency, malicious-client resilience, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Movement packet validation evidence

r[mc_compatibility.movement_packet_family_promotion.validation] The change MUST record checker, runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.movement_packet_family_promotion.validation.log]
- GIVEN the movement packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, runner/fixture checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
