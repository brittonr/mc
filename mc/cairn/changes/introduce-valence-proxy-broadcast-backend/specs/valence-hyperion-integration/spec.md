# valence-hyperion-integration Change Spec: Valence proxy broadcast backend

## Requirements

### Requirement: Proxy broadcast scope

r[valence_hyperion_integration.proxy_broadcast.scope] The integration MUST record the Hyperion proxy surfaces, Valence direct-networking surfaces, and proxy-mode non-goals before implementation.

#### Scenario: Proxy scope is reviewable

r[valence_hyperion_integration.proxy_broadcast.scope.reviewed]
- GIVEN proxy backend work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify the Hyperion source files/docs inspected, the Valence crates affected, the preserved direct-mode behavior, and the out-of-scope Hyperion runtime pieces.

### Requirement: Proxy message contract

r[valence_hyperion_integration.proxy_broadcast.contract] Proxy mode MUST define a stable server-to-proxy and proxy-to-server message contract for unicast, global broadcast, local broadcast, channel broadcast, player position updates, subscriptions, stream lifecycle, backpressure, and shutdown.

#### Scenario: Contract rejects invalid visibility state

r[valence_hyperion_integration.proxy_broadcast.contract.invalid_state]
- GIVEN a proxy message references an unknown stream, unknown channel, stale subscription, malformed payload, or invalid player position
- WHEN the proxy contract validator evaluates it
- THEN the message is rejected with a deterministic diagnostic
- AND no delivery plan is produced for unintended clients.

### Requirement: Pure proxy routing core

r[valence_hyperion_integration.proxy_broadcast.routing_core] Proxy route selection MUST be implemented as a pure deterministic core over player positions, subscriptions, exclusions, and broadcast intents.

#### Scenario: Local broadcast excludes sender

r[valence_hyperion_integration.proxy_broadcast.routing_core.local_exclude]
- GIVEN a local broadcast intent with a center chunk, visibility radius, and excluded stream
- WHEN the routing core evaluates active player positions
- THEN only matching in-range streams are returned
- AND the excluded stream is absent from the delivery plan.

### Requirement: Optional Valence backend

r[valence_hyperion_integration.proxy_broadcast.valence_backend] Valence SHOULD expose proxy mode as an optional backend or plugin without changing the default direct-networking mode.

#### Scenario: Direct mode remains stable

r[valence_hyperion_integration.proxy_broadcast.valence_backend.direct_stable]
- GIVEN proxy mode is not enabled
- WHEN existing Valence networking tests and selected mc-compat dry runs execute
- THEN direct client connection, login, packet flush, and disconnect behavior remain unchanged.

### Requirement: Proxy backend evidence

r[valence_hyperion_integration.proxy_broadcast.evidence] Proxy backend work MUST record direct-mode regression evidence, proxy-mode smoke evidence, malformed-message rejection, and non-overclaiming compatibility notes before archive.

#### Scenario: Proxy evidence is non-overclaiming

r[valence_hyperion_integration.proxy_broadcast.evidence.non_overclaiming]
- GIVEN proxy mode has smoke evidence
- WHEN the evidence is promoted
- THEN it claims only the exercised proxy routing/backend behavior
- AND it does not claim full Hyperion compatibility, full production-scale readiness, or default Valence behavior changes.

### Requirement: Proxy backend validation

r[valence_hyperion_integration.proxy_broadcast.validation] The change MUST pass Cairn proposal, design, tasks, and repository validation gates before archive.

#### Scenario: Proxy Cairn closeout is reviewable

r[valence_hyperion_integration.proxy_broadcast.validation.log]
- GIVEN the proxy backend change is ready to archive
- WHEN reviewers inspect the evidence logs
- THEN logs show routing fixtures, negative proxy-state fixtures, direct-mode regressions, proxy-mode smoke tests, selected mc-compat dry runs, and Cairn validation.
