# valence-hyperion-integration Change Spec: Hyperion packet inspector UI

## Requirements

### Requirement: Hyperion packet inspector scope

r[valence_hyperion_integration.hyperion_packet_inspector.scope] Hyperion packet-inspector UI modularity work MUST be scoped as Hyperion tool-owned nested-repo work unless a separate integration Cairn classifies specific code or concepts for Valence adoption, porting, or reference use.

#### Scenario: Hyperion packet-inspector ownership is explicit

r[valence_hyperion_integration.hyperion_packet_inspector.scope.owned]
- GIVEN Hyperion packet-inspector UI work is planned
- WHEN reviewers inspect the design
- THEN it states that implementation and validation are Hyperion-local tool work
- AND it does not claim Valence adoption or compatibility evidence.

### Requirement: Hyperion packet inspector boundaries

r[valence_hyperion_integration.hyperion_packet_inspector.boundaries] Hyperion packet-inspector UI code SHOULD expose cohesive boundaries for packet-list state, filters, selection, sorting/grouping, render models, and UI rendering shells.

#### Scenario: Packet-inspector responsibility has one owner

r[valence_hyperion_integration.hyperion_packet_inspector.boundaries.ownership]
- GIVEN a packet-inspector UI responsibility is reviewed
- WHEN maintainers inspect Hyperion packet-inspector modules
- THEN the responsibility is owned by a focused Hyperion tool module.

### Requirement: Hyperion packet inspector core

r[valence_hyperion_integration.hyperion_packet_inspector.core] Packet-inspector filtering, selection, sorting/grouping, viewport state, and render-model generation SHOULD be pure over explicit packet-list inputs.

#### Scenario: Packet-list decision is testable without terminal UI

r[valence_hyperion_integration.hyperion_packet_inspector.core.testable]
- GIVEN packet-list, filter, selection, sort, or viewport summaries
- WHEN the packet-inspector core processes them
- THEN the result can be tested without terminal rendering, event loop integration, IO, or logging.

### Requirement: Hyperion packet inspector parity

r[valence_hyperion_integration.hyperion_packet_inspector.parity] Packet-inspector UI modularization MUST preserve packet inspector UI behavior, keyboard/mouse interactions, filter semantics, public tool behavior, and non-claims.

#### Scenario: Packet-inspector behavior remains stable

r[valence_hyperion_integration.hyperion_packet_inspector.parity.stable]
- GIVEN a supported pre-refactor packet-inspector UI input
- WHEN the modularized packet-inspector processes the same input
- THEN selection, filtering, sorting, rendering model, public tool behavior, and non-claim boundaries remain equivalent.

### Requirement: Hyperion packet inspector tests

r[valence_hyperion_integration.hyperion_packet_inspector.tests] The change MUST include positive and negative tests for filter matches, empty filters, selection movement, sorting/grouping, empty packet lists, malformed packet summaries, and render models.

#### Scenario: Packet-inspector fixtures cover success and failure

r[valence_hyperion_integration.hyperion_packet_inspector.tests.coverage]
- GIVEN representative valid and invalid packet-inspector inputs
- WHEN packet-inspector tests run
- THEN supported UI-state decisions pass and malformed inputs fail closed.

### Requirement: Hyperion packet inspector validation

r[valence_hyperion_integration.hyperion_packet_inspector.validation] The change MUST record focused Hyperion packet-inspector tests run from the Hyperion root, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[valence_hyperion_integration.hyperion_packet_inspector.validation.logs]
- GIVEN Hyperion packet-inspector modularization is complete
- WHEN the change is closed
- THEN reviewable logs show Hyperion-local tool tests plus Cairn gates passing.
