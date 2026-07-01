# Retired Hyperion Game Modes Specification

## Purpose

Records the retired status of the former local Hyperion game-mode capability. Historical archives and promoted evidence remain reviewable, but this workspace no longer maintains a live `hyperion/` checkout or accepts new Hyperion-local implementation tasks by default.

## Requirements

### Requirement: Hyperion game-mode checkout retired

r[hyperion_game_modes.retired_checkout.status] Hyperion game-mode work MUST be treated as retired from the local workspace unless a future Cairn explicitly restores a reviewable source snapshot or external checkout.

#### Scenario: No local checkout is required

r[hyperion_game_modes.retired_checkout.status.no_local_checkout]
- GIVEN workspace validation or evidence review runs after checkout retirement
- WHEN it evaluates Hyperion game-mode requirements
- THEN it does not require a live local Hyperion checkout
- AND historical archives and evidence remain citation-stable.

### Requirement: Historical game-mode evidence remains bounded

r[hyperion_game_modes.retired_checkout.historical_evidence] Historical Hyperion game-mode archives and evidence MAY be used only as archived reference material and MUST NOT imply active local maintenance, Valence adoption, vanilla parity, production readiness, public-server safety, or Hyperion compatibility.

#### Scenario: Historical evidence is cited safely

r[hyperion_game_modes.retired_checkout.historical_evidence.bounded]
- GIVEN a future Valence or compatibility change cites historical Hyperion game-mode evidence
- WHEN reviewers inspect the claim boundary
- THEN the evidence is identified as archived or snapshot-based reference material
- AND no local Hyperion command, live checkout state, or broad behavior claim is required.

### Requirement: Future Hyperion-derived work source boundary

r[hyperion_game_modes.retired_checkout.future_source_boundary] Future work that uses Hyperion-derived code or concepts MUST use a reviewable source snapshot, archived evidence, or an explicitly restored external checkout and MUST classify inspected sources as adopt, port, reference, or reject before implementation.

#### Scenario: Future source is reviewable

r[hyperion_game_modes.retired_checkout.future_source_boundary.reviewable]
- GIVEN a future change uses Hyperion-derived code or concepts
- WHEN the proposal and design are reviewed
- THEN they name the archived evidence, source snapshot, or restored external checkout used as input
- AND they record ownership, safety notes, non-claims, and positive and negative validation evidence before archive.

### Requirement: Retired checkout validation

r[hyperion_game_modes.retired_checkout.validation] Checkout retirement MUST record backup evidence for local nested work, layout/config updates, Cairn gates, Cairn validation, task-evidence validation, evidence-manifest validation, and the local deletion log.

#### Scenario: Retirement closeout is reviewable

r[hyperion_game_modes.retired_checkout.validation.logs]
- GIVEN the local checkout is retired
- WHEN reviewers inspect promoted evidence
- THEN logs show the nested work backup, layout/config validation, local deletion result, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest validation with `exit_status=0`
- AND BLAKE3 manifests cover the backup artifacts, changed specs, docs, and validation logs.
