# Mc Compatibility Delta: Valence proxy compatibility seam

## Requirements

### Requirement: Proxy compatibility work

r[mc_compatibility.valence_proxy_compat.boundary_documented] Valence proxy compatibility work MUST define direct, proxied, and deferred architecture boundaries before implementation.

#### Scenario: Boundary is explicit

r[mc_compatibility.valence_proxy_compat.boundary_documented.scenario]

- GIVEN the proxy compatibility seam is planned

- WHEN the proposal/design is reviewed

- THEN the plan records direct-vs-proxy claims and defers full Hyperion-style multi-proxy/mTLS unless separately requested

### Requirement: Proxy-path compatibility receipts

r[mc_compatibility.valence_proxy_compat.receipt_fields] Proxy-path compatibility receipts MUST record the route, forwarding mode, observed protocol/version, and non-claims.

#### Scenario: Proxy receipt identifies route

r[mc_compatibility.valence_proxy_compat.receipt_fields.scenario]

- GIVEN a proxy-path smoke or dry-run executes

- WHEN the receipt is written

- THEN the receipt records direct or proxied route, forwarding mode if known, observed protocol/version, and non-claims

### Requirement: Proxy compatibility

r[mc_compatibility.valence_proxy_compat.local_gate] Proxy compatibility MUST have a deterministic local gate before live proxy evidence is treated as durable.

#### Scenario: Dry-run proxy gate validates shape

r[mc_compatibility.valence_proxy_compat.local_gate.scenario]

- GIVEN the proxy compatibility receipt schema exists

- WHEN the dry-run gate runs

- THEN the gate validates route, forwarding-mode, protocol, and non-claim fields
