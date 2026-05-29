# Delta: Negative live rails

## Requirements

### Requirement: Negative live-rail envelope

r[mc_compatibility.negative_live_rails.envelope] Negative live rails MUST run inside a bounded owned-local safety envelope before injecting invalid client actions or malformed packet fixtures.

#### Scenario: Unsafe negative rail is blocked

r[mc_compatibility.negative_live_rails.envelope.safe]
- GIVEN a negative rail targets a public or unowned server, lacks explicit authorization, lacks bounds, or lacks an expected containment outcome
- WHEN the runner preflight executes
- THEN it fails before sending traffic
- AND the receipt records that production/public/adversarial safety remains a non-claim.

### Requirement: Negative dry-run receipts

r[mc_compatibility.negative_live_rails.dry_run] Each negative live rail MUST have a deterministic dry-run receipt shape check before live evidence is promoted.

#### Scenario: Dry-run records expected outcome

r[mc_compatibility.negative_live_rails.dry_run.shape]
- GIVEN a negative rail is dry-run
- WHEN its receipt is inspected
- THEN the receipt names the invalid action, bounds, expected outcome, evidence fields, and explicit non-claims.

### Requirement: Stale inventory state negative rail

r[mc_compatibility.negative_live_rails.inventory_stale_state] The harness SHOULD add a live rail for one stale inventory state-id action.

#### Scenario: Stale state is contained

r[mc_compatibility.negative_live_rails.inventory_stale_state.contained]
- GIVEN the client has an open inventory/container state
- WHEN it sends one stale state-id action under the bounded rail
- THEN the receipt records rejection, restoration, disconnect-as-expected, or another declared containment outcome
- AND no full inventory transaction coverage is claimed.

### Requirement: Invalid inventory click negative rail

r[mc_compatibility.negative_live_rails.inventory_invalid_click] The harness SHOULD add a live rail for one invalid slot/window click action.

#### Scenario: Invalid click is contained

r[mc_compatibility.negative_live_rails.inventory_invalid_click.contained]
- GIVEN the client has a known inventory/window state
- WHEN it sends one invalid slot or window click
- THEN server/client evidence records the expected containment outcome and postcondition.

### Requirement: Malformed custom payload negative rail

r[mc_compatibility.negative_live_rails.custom_payload] The harness SHOULD add a live rail for one malformed custom-payload action.

#### Scenario: Malformed payload is rejected

r[mc_compatibility.negative_live_rails.custom_payload.rejected]
- GIVEN the client sends the configured malformed custom payload
- WHEN the server handles the packet
- THEN the receipt classifies rejection, disconnect, or containment without claiming broad plugin-message semantics.

### Requirement: Reconnect race negative rail

r[mc_compatibility.negative_live_rails.reconnect_race] The harness SHOULD add a live rail for one reconnect flag-state race or duplicate transition.

#### Scenario: Reconnect race does not corrupt score

r[mc_compatibility.negative_live_rails.reconnect_race.contained]
- GIVEN a player reconnects while a flag-state transition is pending
- WHEN the bounded invalid transition is attempted
- THEN the receipt records coherent post-reconnect flag state and no unexpected score milestone.

### Requirement: Wrong score path negative rail

r[mc_compatibility.negative_live_rails.ctf_wrong_score] The harness SHOULD add a live rail for one wrong team or wrong portal scoring attempt.

#### Scenario: Wrong score path is not promoted

r[mc_compatibility.negative_live_rails.ctf_wrong_score.rejected]
- GIVEN the client attempts the configured wrong-team or wrong-portal score path
- WHEN scenario evaluation runs
- THEN no promoted score milestone is recorded and the receipt names the containment evidence.

### Requirement: Negative live-rail validation

r[mc_compatibility.negative_live_rails.validation] Negative live-rail evidence MUST be copied under `docs/evidence/` and must preserve explicit non-claims.

#### Scenario: Promotion is narrow

r[mc_compatibility.negative_live_rails.validation.narrow]
- GIVEN a negative rail is promoted
- WHEN the acceptance matrix and current bundle are updated
- THEN only the exact invalid behavior row is claimed
- AND unrelated invalid actions, adversarial safety, public-server safety, and production readiness remain non-claims.
