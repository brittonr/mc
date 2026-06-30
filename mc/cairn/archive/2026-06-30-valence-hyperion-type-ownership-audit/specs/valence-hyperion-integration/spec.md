# valence-hyperion-integration Change Spec: Type ownership audit

## Requirements

### Requirement: Valence type inventory

r[valence_hyperion_integration.type_ownership.valence_inventory] Type ownership work MUST inventory Valence protocol, networking, packet composition, layer/chunk, entity/player, command/chat, and optional proxy surfaces relevant to bridge boundaries before implementation depends on shared types.

#### Scenario: Valence ownership candidates are recorded

r[valence_hyperion_integration.type_ownership.valence_inventory.recorded]
- GIVEN type ownership work is selected
- WHEN reviewers inspect the Valence inventory
- THEN affected Valence crates, public APIs, internal-only APIs, compatibility-sensitive packet paths, and default-behavior assumptions are recorded.

### Requirement: Hyperion type inventory

r[valence_hyperion_integration.type_ownership.hyperion_inventory] Type ownership work MUST inventory Hyperion game-server, proxy, packet, join, movement, broadcast, chunk egress, command/chat, and game-mode surfaces relevant to bridge boundaries before implementation depends on Hyperion concepts.

#### Scenario: Hyperion ownership candidates are classified for review

r[valence_hyperion_integration.type_ownership.hyperion_inventory.recorded]
- GIVEN Hyperion sources are inspected for bridge work
- WHEN reviewers inspect the Hyperion inventory
- THEN runtime-local types, proxy-local types, game-mode-local types, reusable concepts, unsafe or nightly-sensitive sources, and forbidden core-merge candidates are identified.

### Requirement: Hyperion source classification

r[valence_hyperion_integration.type_ownership.classification] Type ownership work MUST classify inspected Hyperion sources as adopt, port, reference, or reject with owner, target, safety notes, required evidence, and non-claims.

#### Scenario: Unsafe or game-specific source is contained

r[valence_hyperion_integration.type_ownership.classification.rejects_unsafe]
- GIVEN an inspected source is Bedwars-specific, runtime-replacement scope, unaudited unsafe-heavy, nightly-only, or broad custom combat behavior
- WHEN the classification table is reviewed
- THEN the source is marked reject or reference-only
- AND no Valence public API depends on copying it directly.

### Requirement: Type ownership matrix

r[valence_hyperion_integration.type_ownership.matrix] Type ownership work MUST publish a matrix naming canonical Valence types, Hyperion-only types, adapter-owned DTOs, rejected shared abstractions, and source revision evidence for each mapped family.

#### Scenario: Shared abstraction requires justification

r[valence_hyperion_integration.type_ownership.matrix.shared_justified]
- GIVEN a type family is proposed as shared between Valence and Hyperion
- WHEN reviewers inspect the matrix
- THEN the matrix identifies why neither existing owner is sufficient, how compatibility is preserved, and which tests prove the shared abstraction does not leak runtime internals.

### Requirement: Conversion contract

r[valence_hyperion_integration.type_ownership.conversion_contract] Adapter conversions MUST define deterministic inputs, outputs, ownership, lossy-field policy, error diagnostics, and fail-closed behavior for each mapped type family.

#### Scenario: Ambiguous conversion fails closed

r[valence_hyperion_integration.type_ownership.conversion_contract.ambiguous]
- GIVEN a conversion lacks a required dimension, registry, session, entity, route, protocol, or packet field
- WHEN the adapter conversion runs
- THEN it returns a deterministic diagnostic
- AND no bridge shell mutation, packet delivery, or public compatibility claim is emitted for that input.

### Requirement: Ownership fixture coverage

r[valence_hyperion_integration.type_ownership.fixtures] Type ownership work MUST include positive and negative fixtures or fixture plans for valid mappings, stale sessions, missing dimensions, invalid routes, malformed packet bytes, unsupported protocol assumptions, and lossy mappings.

#### Scenario: Valid and invalid mappings are both covered

r[valence_hyperion_integration.type_ownership.fixtures.coverage]
- GIVEN representative bridge type mappings are documented
- WHEN fixture coverage is reviewed
- THEN supported mappings have positive examples and malformed, stale, unsupported, or lossy mappings have negative examples that fail closed.

### Requirement: Type ownership validation

r[valence_hyperion_integration.type_ownership.validation] Type ownership work MUST record Cairn proposal, design, tasks, repository validation, fixture checks if code is added, and evidence-manifest checks for promoted ownership evidence before archive.

#### Scenario: Ownership closeout is reviewable

r[valence_hyperion_integration.type_ownership.validation.log]
- GIVEN type ownership work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show Valence inventory, Hyperion inventory, source classifications, ownership matrix evidence, conversion fixture evidence or fixture plans, Cairn gates, Cairn validation, and explicit non-claims for bridge implementation, runtime replacement, default behavior, Hyperion compatibility, production scale, and vanilla parity.
