# Repository Layout Specification

## Purpose

Defines repository layout behavior for checkout retirement.

## Requirements

### Requirement: Retired checkout removed from registry

r[repository_layout.hyperion_checkout_retirement.registry] The component registry and layout checklist MUST NOT list the retired Hyperion checkout as a current component root or nested Git exception after physical removal.

#### Scenario: Hyperion is absent from active layout

r[repository_layout.hyperion_checkout_retirement.registry.absent]
- GIVEN the retired checkout has been backed up and removed locally
- WHEN layout validation inspects component registry entries and root directories
- THEN Hyperion is absent from current component-root and nested-Git-exception inventories
- AND historical evidence references do not reintroduce an active layout requirement.

### Requirement: Retired checkout deletion validation

r[repository_layout.hyperion_checkout_retirement.validation] Checkout retirement MUST record layout/config validation and local deletion evidence before archive.

#### Scenario: Layout closeout is reviewable

r[repository_layout.hyperion_checkout_retirement.validation.logs]
- GIVEN checkout retirement is ready to close
- WHEN reviewers inspect promoted evidence
- THEN logs show the checkout backup, local deletion, layout validation, Cairn validation, task-evidence validation, and evidence-manifest validation with `exit_status=0`.
