# Delta: Survival break/place/pickup rail

## Requirements

### Requirement: Survival Fixture

r[mc_compatibility.survival_break_place_pickup.survival_fixture] The repo MUST provide a dedicated local Valence survival fixture for the break/place/pickup rail instead of reusing the CTF fixture as survival evidence.

#### Scenario: Survival fixture is selected

r[mc_compatibility.survival_break_place_pickup.survival_fixture.scenario]
- GIVEN the `survival-break-place-pickup` scenario is selected
- WHEN the maintained Nix app or runner dry-run is evaluated
- THEN the selected Valence example is `survival_compat`
- AND the receipt scope remains owned-local and bounded.

### Requirement: Client Probe

r[mc_compatibility.survival_break_place_pickup.client_probe] The Stevenarella probe MUST emit deterministic client milestones for survival join/render, fixed-coordinate block break, pickup/inventory observation, and block placement.

#### Scenario: Client probe evidence is required

r[mc_compatibility.survival_break_place_pickup.client_probe.scenario]
- GIVEN a live `survival-break-place-pickup` run
- WHEN the client log is evaluated
- THEN required milestones include protocol detection, join game, render tick, `survival_probe_break_block_sent`, `survival_probe_pickup_seen`, and `survival_probe_place_block_sent`
- AND panic, EOF, protocol mismatch, and decode errors reject the scenario.

### Requirement: Server Correlation

r[mc_compatibility.survival_break_place_pickup.server_correlation] The Valence survival fixture MUST correlate the same username and fixed block coordinates with server-observed break, pickup, and place milestones.

#### Scenario: Server correlation is required

r[mc_compatibility.survival_break_place_pickup.server_correlation.scenario]
- GIVEN a live `survival-break-place-pickup` run
- WHEN the server log is evaluated
- THEN required server milestones include `survival_join`, `survival_block_break`, `survival_pickup_item`, and `survival_block_place`
- AND each milestone is scoped to the configured client username or fixed survival rail coordinate.

### Requirement: Receipt Nonclaims

r[mc_compatibility.survival_break_place_pickup.receipt_nonclaims] The scenario receipt MUST preserve non-claims for vanilla parity, full survival compatibility, broad protocol coverage, production readiness, public load safety, and semantic equivalence.

#### Scenario: Receipt stays non-overclaiming

r[mc_compatibility.survival_break_place_pickup.receipt_nonclaims.scenario]
- GIVEN the survival rail emits a receipt
- WHEN the receipt is reviewed
- THEN `claims_correctness` and `claims_semantic_equivalence` remain false
- AND the evidence text does not promote vanilla parity without paired reference and Valence receipts.

### Requirement: Dry Run Check

r[mc_compatibility.survival_break_place_pickup.dry_run_check] A deterministic Nix dry-run check MUST validate the survival scenario shape, required milestones, selected fixture, and receipt non-claims before live evidence is promoted.

#### Scenario: Dry-run check validates shape

r[mc_compatibility.survival_break_place_pickup.dry_run_check.scenario]
- GIVEN no live Minecraft client is started
- WHEN the survival dry-run check is built
- THEN it produces a passing `survival-break-place-pickup` receipt shape
- AND the check fails if the scenario name or selected fixture regresses.
