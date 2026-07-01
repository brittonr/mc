# Valence Hyperion Integration Specification

## Purpose

Defines the `valence-hyperion-integration` capability.

## Requirements

### Requirement: Hyperion checkout retirement audit

r[valence_hyperion_integration.hyperion_checkout_retirement.audit] Work that proposes deleting or retiring the local `hyperion/` checkout MUST first audit parent tracking, nested repo state, accepted spec references, docs/config references, and promoted evidence references.

#### Scenario: Retirement inputs are explicit

r[valence_hyperion_integration.hyperion_checkout_retirement.audit.explicit]
- GIVEN a change proposes deleting or retiring `hyperion/`
- WHEN reviewers inspect its evidence
- THEN the evidence lists whether `mc/hyperion` is tracked by the parent repo
- AND it lists whether the nested checkout has uncommitted `jj` or Git work
- AND it lists accepted specs, docs/config, and promoted evidence that still depend on Hyperion paths or ownership.

### Requirement: Hyperion checkout deletion blockers

r[valence_hyperion_integration.hyperion_checkout_retirement.blockers] The local `hyperion/` checkout MUST NOT be deleted while accepted specs, docs/config, promoted evidence, or active nested repo state still require Hyperion-local ownership or while deletion would discard uncommitted nested work.

#### Scenario: Live references block deletion

r[valence_hyperion_integration.hyperion_checkout_retirement.blockers.live_refs]
- GIVEN a retirement audit finds accepted specs, docs/config, promoted evidence, or active nested repo state still referencing Hyperion-local ownership
- WHEN the change decides whether to remove the checkout
- THEN deletion is blocked
- AND the checkout remains present until a future change classifies, migrates, rejects, or retires each blocking dependency.

### Requirement: Hyperion checkout deletion mechanism

r[valence_hyperion_integration.hyperion_checkout_retirement.mechanism] A future physical deletion of `hyperion/` MUST state whether deletion is a parent-tracked diff, a local workspace cleanup, or a separate nested-repo archival action, and MUST preserve or intentionally discard nested repo state through reviewable evidence.

#### Scenario: Deletion scope is reviewable

r[valence_hyperion_integration.hyperion_checkout_retirement.mechanism.reviewable]
- GIVEN a future change clears all retirement blockers
- WHEN it performs or instructs physical deletion
- THEN reviewers can tell whether the deletion is represented in parent version control, local workspace cleanup, or nested-repo archival
- AND uncommitted nested work is either absent, committed, backed up, or intentionally discarded with explicit evidence.

### Requirement: Hyperion checkout retirement validation

r[valence_hyperion_integration.hyperion_checkout_retirement.validation] Hyperion checkout retirement work MUST record the audit log, Cairn proposal/design/tasks gates, Cairn validation, and evidence-manifest validation before archive.

#### Scenario: Retirement decision is reviewable

r[valence_hyperion_integration.hyperion_checkout_retirement.validation.logs]
- GIVEN a checkout retirement audit is ready to close
- WHEN reviewers inspect task evidence
- THEN logs show the retirement audit and Cairn validation passed
- AND a BLAKE3 manifest covers the audit evidence, task file, spec delta, and validation logs.
