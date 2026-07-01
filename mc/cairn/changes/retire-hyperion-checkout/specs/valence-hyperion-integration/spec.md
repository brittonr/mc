# Valence Hyperion Integration Specification

## Purpose

Defines checkout-retirement behavior for Valence-owned Hyperion-derived integration work.

## Requirements

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
