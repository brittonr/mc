# Valence Hyperion Integration Specification

## Purpose

Defines the retained Valence-owned integration boundary after retiring the local Hyperion checkout. Historical Hyperion-derived work remains available as archived evidence or explicit source snapshots, but this workspace no longer treats a live local Hyperion repository as a component root or validation input.

## Requirements

### Requirement: Hyperion integration source boundary

r[valence_hyperion_integration.boundaries.inventory] Future Hyperion-to-Valence integration work MUST classify inspected archived evidence, source snapshots, or external checkout sources as adopt, port, reference, or reject before implementation.

#### Scenario: Source classification precedes code changes

r[valence_hyperion_integration.boundaries.inventory.precedes]
- GIVEN a future integration Cairn proposes using Hyperion-derived code or concepts
- WHEN reviewers inspect its design
- THEN each relevant source is classified as adopt, port, reference, or reject
- AND the classification explains ownership, safety, source availability, and API impact.

### Requirement: Forbidden Valence core merges

r[valence_hyperion_integration.boundaries.forbidden_core] Integration work MUST NOT merge Bedwars-specific game logic, replace Valence's runtime wholesale, add custom combat as Valence core behavior, or import unaudited nightly or unsafe-heavy code directly into Valence core.

#### Scenario: Forbidden source is rejected

r[valence_hyperion_integration.boundaries.forbidden_core.rejected]
- GIVEN an inspected source is Bedwars-specific, runtime-replacement scope, custom combat core behavior, or unaudited nightly or unsafe-heavy implementation
- WHEN the integration inventory is evaluated
- THEN the source is classified as reject or reference-only
- AND no Valence core task depends on copying it directly.

### Requirement: Optional gameplay plugin boundary

r[valence_hyperion_integration.boundaries.optional_plugins] Gameplay semantics inspired by historical Hyperion evidence MAY be implemented only as optional plugins or examples unless separate accepted Valence scope and reference evidence justify core behavior.

#### Scenario: Combat remains optional without reference evidence

r[valence_hyperion_integration.boundaries.optional_plugins.combat]
- GIVEN historical Hyperion combat behavior is considered for Valence
- WHEN no separate vanilla or reference evidence proves the intended core behavior
- THEN the work is scoped as an optional plugin, example, or reference-only note
- AND Valence core behavior remains unchanged.

### Requirement: Retired local checkout

r[valence_hyperion_integration.retired_checkout.status] The mc workspace MUST NOT require a live local Hyperion checkout for default layout, validation, evidence, or Cairn lifecycle checks after checkout retirement.

#### Scenario: Layout no longer needs Hyperion checkout

r[valence_hyperion_integration.retired_checkout.status.layout]
- GIVEN workspace layout validation runs after checkout retirement
- WHEN it inspects root component directories and nested Git exceptions
- THEN Hyperion is not required as a local component root
- AND remaining historical Hyperion references are treated as archives, evidence, non-claims, or source-snapshot references.

### Requirement: Historical evidence boundary

r[valence_hyperion_integration.retired_checkout.historical_evidence] Historical Hyperion archives and evidence MUST remain citation-stable but MUST NOT imply active local maintenance, Valence adoption, production readiness, public-server safety, vanilla parity, or Hyperion compatibility.

#### Scenario: Historical evidence stays bounded

r[valence_hyperion_integration.retired_checkout.historical_evidence.bounded]
- GIVEN a future change cites historical Hyperion-derived evidence
- WHEN reviewers inspect the claim boundary
- THEN the evidence is identified as archived or snapshot-based reference material
- AND no live local checkout command or broad behavior claim is required.

### Requirement: Future source restoration

r[valence_hyperion_integration.retired_checkout.future_source] A future change that needs fresh Hyperion source inspection MUST use a reviewable source snapshot or explicitly restored external checkout and MUST record the source revision and ownership boundary before implementation.

#### Scenario: Fresh source dependency is explicit

r[valence_hyperion_integration.retired_checkout.future_source.reviewable]
- GIVEN a future change needs Hyperion source beyond archived evidence
- WHEN the proposal is reviewed
- THEN it names the source snapshot or external checkout revision
- AND it states whether that source is local scratch, external dependency, or parent-tracked input.

### Requirement: Checkout retirement validation

r[valence_hyperion_integration.retired_checkout.validation] Checkout retirement MUST record nested-work backup evidence, layout/config updates, deletion evidence, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest validation before archive.

#### Scenario: Retirement closeout is reviewable

r[valence_hyperion_integration.retired_checkout.validation.logs]
- GIVEN the local checkout is retired
- WHEN reviewers inspect promoted evidence
- THEN logs show nested-work backup, layout/config validation, local deletion, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest validation with `exit_status=0`
- AND BLAKE3 manifests cover the backup artifacts, docs, accepted specs, task file, and validation logs.
