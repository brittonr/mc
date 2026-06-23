# valence-hyperion-integration Change Spec: Hyperion integration boundaries

## Requirements

### Requirement: Hyperion integration inventory

r[valence_hyperion_integration.boundaries.inventory] Future Hyperion-to-Valence integration work MUST classify inspected Hyperion sources as adopt, port, reference, or reject before implementation.

#### Scenario: Source classification precedes code changes

r[valence_hyperion_integration.boundaries.inventory.precedes]
- GIVEN a future integration Cairn proposes using Hyperion code or concepts
- WHEN reviewers inspect its design
- THEN each relevant source is classified as adopt, port, reference, or reject
- AND the classification explains ownership, safety, and API impact.

### Requirement: Forbidden Valence core merges

r[valence_hyperion_integration.boundaries.forbidden_core] Integration work MUST NOT merge Bedwars-specific game logic, replace Valence's runtime with Hyperion's runtime wholesale, add custom combat as Valence core behavior, or import unaudited nightly/unsafe-heavy code directly into Valence core.

#### Scenario: Forbidden source is rejected

r[valence_hyperion_integration.boundaries.forbidden_core.rejected]
- GIVEN an inspected Hyperion source is Bedwars-specific, runtime-replacement scope, custom combat core behavior, or unaudited nightly/unsafe-heavy implementation
- WHEN the integration inventory is evaluated
- THEN the source is classified as reject or reference-only
- AND no Valence core task depends on copying it directly.

### Requirement: Optional gameplay plugin boundary

r[valence_hyperion_integration.boundaries.optional_plugins] Gameplay semantics inspired by Hyperion MAY be implemented only as optional plugins or examples unless separate accepted Valence scope and reference evidence justify core behavior.

#### Scenario: Combat remains optional without reference evidence

r[valence_hyperion_integration.boundaries.optional_plugins.combat]
- GIVEN Hyperion combat behavior is considered for Valence
- WHEN no separate vanilla/reference evidence proves the intended core behavior
- THEN the work is scoped as an optional plugin, example, or reference-only note
- AND Valence core behavior remains unchanged.

### Requirement: Integration review gate

r[valence_hyperion_integration.boundaries.review_gate] Future Hyperion integration Cairns SHOULD cite the boundary inventory and non-claim checklist before archive.

#### Scenario: Non-claim checklist is present

r[valence_hyperion_integration.boundaries.review_gate.non_claims]
- GIVEN an integration Cairn is ready to archive
- WHEN reviewers inspect its proposal and evidence
- THEN production-scale, vanilla-parity, Hyperion-compatibility, default-behavior, and safety claims are each either supported by evidence or explicitly left as non-claims.

### Requirement: Boundary fixtures

r[valence_hyperion_integration.boundaries.fixtures] Boundary work SHOULD include positive and negative checklist examples or fixtures for allowed reference use and forbidden direct imports.

#### Scenario: Reference-only use passes

r[valence_hyperion_integration.boundaries.fixtures.reference_only]
- GIVEN a future Cairn uses Hyperion code only as design reference
- WHEN the boundary checklist is evaluated
- THEN it passes if no copied code or unsupported behavior claim is present
- AND it records the referenced source and resulting Valence-owned design.

### Requirement: Boundary validation

r[valence_hyperion_integration.boundaries.validation] Boundary work MUST record inventory/checklist validation, negative forbidden-import examples, and Cairn gates before archive.

#### Scenario: Boundary closeout is reviewable

r[valence_hyperion_integration.boundaries.validation.log]
- GIVEN the boundary change is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show inventory/checklist validation, positive reference-only examples, negative forbidden-import examples, Cairn proposal/design/tasks gates, and Cairn validation.
