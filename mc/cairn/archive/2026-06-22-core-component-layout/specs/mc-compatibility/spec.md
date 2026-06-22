# mc-compatibility Change Spec: Core component layout

## Requirements

### Requirement: Role-based core component boundaries

r[mc_compatibility.core_component_layout.boundaries] Core Minecraft source trees SHOULD be organized by product role rather than upstream provenance when the project owns ongoing changes to that source tree.

#### Scenario: Core components are named by role

r[mc_compatibility.core_component_layout.boundaries.roles]
- GIVEN Stevenarella and Valence are parent-owned core source trees
- WHEN the project layout is inspected
- THEN the client implementation is discoverable under a client role boundary
- AND the server implementation is discoverable under a server role boundary
- AND historical upstream ancestry is documented without labeling those trees as passive vendors.

### Requirement: Central source layout resolver

r[mc_compatibility.core_component_layout.resolver] The compatibility harness MUST resolve core component source roots through a single typed layout resolver instead of scattering path probes across runner, wrapper, and documentation code.

#### Scenario: Resolver accepts valid layouts and rejects unsafe layouts

r[mc_compatibility.core_component_layout.resolver.fixtures]
- GIVEN the repository is in either the approved transition layout or the final role-based layout
- WHEN the resolver locates client, server, and compatibility roots
- THEN it returns canonical component paths for downstream callers
- AND missing roots, ambiguous duplicate roots, or nested Git directories under core components fail with deterministic diagnostics.

### Requirement: Core source moves preserve evidence semantics

r[mc_compatibility.core_component_layout.core_moves] Moving Valence or Stevenarella into role-based component roots MUST preserve parent-owned source-tree semantics and path-scoped revision evidence.

#### Scenario: Revision evidence stays path-scoped after moves

r[mc_compatibility.core_component_layout.core_moves.revision_scope]
- GIVEN a core component has moved to a role-based path
- WHEN the runner records client or server revision evidence
- THEN the recorded revision and dirty-state checks are scoped to the resolved component path
- AND they do not rely on nested Git repositories inside the component tree.

### Requirement: Compatibility harness boundary

r[mc_compatibility.core_component_layout.compat_boundary] Compatibility runner source, scenario manifests, generated harness surfaces, and Paper/reference fixtures SHOULD live under a dedicated compatibility boundary when moving them does not weaken generated-surface freshness or evidence checks.

#### Scenario: Harness paths remain generated and checkable

r[mc_compatibility.core_component_layout.compat_boundary.generated]
- GIVEN compatibility harness files are moved under the compatibility boundary
- WHEN generated-surface and scenario-manifest checks run
- THEN generated paths, wrapper names, dry-run metadata, and scenario indexes remain current
- AND the move does not change scenario behavior or pass/fail semantics.

### Requirement: Core component documentation

r[mc_compatibility.core_component_layout.docs] Documentation MUST describe the current role-based ownership model for clients, servers, and compatibility harnesses, including upstream ancestry where relevant.

#### Scenario: Documentation avoids vendor/fork terminology for core components

r[mc_compatibility.core_component_layout.docs.terminology]
- GIVEN a reviewer reads README, AGENTS, or architecture notes after the migration
- WHEN those docs refer to Stevenarella or Valence
- THEN they describe the trees as core client and server components
- AND upstream ancestry is presented as provenance metadata rather than the active ownership boundary.

### Requirement: Layout migration validation

r[mc_compatibility.core_component_layout.validation] The layout migration MUST NOT be marked complete until tests and reviewable evidence prove the resolver, moved paths, docs, generated surfaces, and Cairn lifecycle are consistent.

#### Scenario: Validation evidence covers positive and negative cases

r[mc_compatibility.core_component_layout.validation.evidence]
- GIVEN tasks claim the role-based layout migration is complete
- WHEN reviewers inspect promoted evidence
- THEN logs show positive and negative resolver fixtures, no nested Git directories under core components, runner dry-run path discovery, scenario-manifest checks, generated-surface freshness checks, Cairn gates, and Cairn validation
- AND the evidence explicitly states that compatibility semantics and live parity claims are unchanged.
