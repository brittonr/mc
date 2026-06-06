# mc-compatibility Change Spec: Scoreboard/team packet-family promotion

## Requirements

### Requirement: Scoreboard/team packet-family contract

r[mc_compatibility.scoreboard_team_packet_family_promotion.contract] The `scoreboard-team-packet-family` row MUST define a bounded promotion contract before packet inventory, matrix, or current-bundle coverage is claimed.

#### Scenario: Contract names exact scoreboard or team packet rows

r[mc_compatibility.scoreboard_team_packet_family_promotion.contract.scope]
- GIVEN the row is prepared for promotion
- WHEN reviewers inspect the contract
- THEN it names one scenario context, exact packet row or rows, normalized team/objective/display/score fields, client observation or fixture evidence, server correlation, child revisions if live, and checker metrics
- AND scoreboard UI parity, all team rules, all objective/display/score variants, full CTF correctness, full protocol-763 compatibility, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Scoreboard/team packet-family checker

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker] A deterministic Rust checker MUST validate normalized scoreboard/team packet evidence before promotion.

#### Scenario: Valid scoreboard/team evidence passes

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker.valid]
- GIVEN normalized evidence names `scoreboard-team-packet-family`, the configured packet row or rows, normalized fields, client observation or fixture evidence, server correlation, and required non-claims
- WHEN the checker evaluates the record
- THEN it passes only if every configured metric is present and internally consistent.

#### Scenario: Weak scoreboard/team evidence fails closed

r[mc_compatibility.scoreboard_team_packet_family_promotion.checker.rejects]
- GIVEN evidence is missing the row id, names an unsupported packet row, omits normalized fields, lacks client/fixture or server correlation, uses stale revisions, or claims UI/full-CTF/scoreboard breadth
- WHEN the checker evaluates the record
- THEN it fails and names the missing, stale, unexpected, or mismatched metric.

### Requirement: Scoreboard/team packet-family rail

r[mc_compatibility.scoreboard_team_packet_family_promotion.rail] The harness MUST expose or select isolated scoreboard/team packet evidence without changing existing CTF, survival, inventory, combat, network, or negative-live semantics.

#### Scenario: Gameplay and packet-family claims stay separate

r[mc_compatibility.scoreboard_team_packet_family_promotion.rail.isolated]
- GIVEN existing CTF rows cover bounded gameplay transitions
- WHEN scoreboard/team packet evidence is added
- THEN existing CTF claims remain unchanged
- AND the packet-family row records separate packet metrics and checker output.

### Requirement: Scoreboard/team packet artifacts

r[mc_compatibility.scoreboard_team_packet_family_promotion.artifacts] Review-critical scoreboard/team packet artifacts MUST be copied under `docs/evidence/` before promotion.

#### Scenario: Artifacts include checks and exact packet rows

r[mc_compatibility.scoreboard_team_packet_family_promotion.artifacts.reviewable]
- GIVEN the row is ready for promotion
- WHEN reviewers inspect the repository
- THEN receipts or fixtures, logs, normalized inputs, checker output, BLAKE3 manifests, packet inventory updates, child revisions if live, and any oracle checkpoint are present under `docs/evidence/`.

### Requirement: Narrow scoreboard/team packet matrix promotion

r[mc_compatibility.scoreboard_team_packet_family_promotion.matrix] Packet inventory, acceptance matrix, and current bundle docs MUST promote only the configured scoreboard/team packet row after checker and evidence gates pass.

#### Scenario: Broader scoreboard/team remains a non-claim

r[mc_compatibility.scoreboard_team_packet_family_promotion.matrix.nonclaims]
- GIVEN scoreboard/team packet evidence passes
- WHEN docs are updated
- THEN only the configured packet row or rows are marked covered
- AND UI parity, all scoreboards, all team rules, full CTF correctness, full protocol, public-server safety, and production readiness remain explicit non-claims.

### Requirement: Scoreboard/team packet validation evidence

r[mc_compatibility.scoreboard_team_packet_family_promotion.validation] The change MUST record checker, fixture or runner, packet inventory, manifest, task gate, Cairn gates, and Cairn validation output before archive.

#### Scenario: Closeout evidence is complete

r[mc_compatibility.scoreboard_team_packet_family_promotion.validation.log]
- GIVEN the scoreboard/team packet row is completed
- WHEN the change is archived
- THEN repo-local evidence logs record checker tests, fixture/runner checks, packet inventory checks, evidence manifest check, task-evidence gate, Cairn gates, and Cairn validation.
