# mc-compatibility Change Spec: Stevenarella hotspot module reduction

## Requirements

### Requirement: Stevenarella hotspot module inventory

r[mc_compatibility.stevenarella_hotspot_modules.inventory] Stevenarella hotspot module reduction work MUST inventory selected `world`, `model`, `ui`, `ecs`, and `control` responsibilities, public APIs, internal consumers, and baseline tests before extraction.

#### Scenario: Hotspot ownership is reviewable

r[mc_compatibility.stevenarella_hotspot_modules.inventory.reviewable]
- GIVEN a Stevenarella hotspot migration wave is selected
- WHEN reviewers inspect the inventory
- THEN the selected modules, public items, responsibility groups, internal consumers, and baseline tests are named
- AND the first migration wave is scoped before core changes.

### Requirement: Stevenarella façade modules

r[mc_compatibility.stevenarella_hotspot_modules.facades] Large Stevenarella `mod.rs` or root modules SHOULD become thin façades over focused child modules while preserving public names where practical or documenting intentional local call-site updates.

#### Scenario: Hotspot modules expose focused children

r[mc_compatibility.stevenarella_hotspot_modules.facades.focused]
- GIVEN a selected hotspot module is reviewed
- WHEN maintainers inspect the module tree
- THEN the root module primarily declares or re-exports focused child modules
- AND unrelated data, rendering, ECS, parser, UI, and shell concerns are not reintroduced into one catch-all module.

### Requirement: Stevenarella hotspot core and shell boundary

r[mc_compatibility.stevenarella_hotspot_modules.core_shell] Deterministic parsing, normalization, layout, ECS planning, and state-transition logic SHOULD be pure over explicit inputs, while renderer, GL, filesystem, network, input, and global state effects remain in shells.

#### Scenario: Extracted hotspot logic is testable without the client runtime

r[mc_compatibility.stevenarella_hotspot_modules.core_shell.testable]
- GIVEN explicit hotspot module inputs such as model data, UI state, world state, ECS query facts, or control actions
- WHEN an extracted pure helper computes a decision
- THEN tests can verify the result without renderer state, GL context, filesystem access, network access, global console state, or event-loop startup
- AND shells own those side effects.

### Requirement: Stevenarella hotspot parity

r[mc_compatibility.stevenarella_hotspot_modules.parity] Hotspot module reduction MUST preserve default client behavior, compat instrumentation boundaries, module API compatibility where practical, protocol behavior, rendering behavior for touched paths, and non-claim boundaries.

#### Scenario: Existing client paths remain stable

r[mc_compatibility.stevenarella_hotspot_modules.parity.stable]
- GIVEN a supported pre-refactor client code path in a touched hotspot module
- WHEN the modularized code processes the same input
- THEN public API behavior, client-visible state, rendering-visible output for touched paths, and instrumentation boundaries remain compatible
- AND no new protocol, rendering correctness, compatibility, or production-readiness claim is introduced.

### Requirement: Stevenarella hotspot module tests

r[mc_compatibility.stevenarella_hotspot_modules.tests] The change MUST include positive tests for extracted pure logic and negative tests for invalid inputs, missing resources, empty collections, malformed state, unsupported layouts, and API drift where applicable.

#### Scenario: Valid hotspot inputs pass

r[mc_compatibility.stevenarella_hotspot_modules.tests.positive]
- GIVEN valid representative inputs for extracted hotspot helpers
- WHEN the helpers process them
- THEN tests prove expected parsing, layout, ECS planning, state transition, or control decisions are produced.

#### Scenario: Invalid hotspot inputs fail safely

r[mc_compatibility.stevenarella_hotspot_modules.tests.negative]
- GIVEN invalid inputs, missing resources, empty collections, malformed state, unsupported layouts, or API drift fixtures
- WHEN extracted hotspot helpers or façade tests process them
- THEN tests prove the code rejects, defaults, or diagnoses the input without panic or hidden state corruption.

### Requirement: Stevenarella hotspot module validation

r[mc_compatibility.stevenarella_hotspot_modules.validation] The change MUST record focused Stevenarella tests, affected mc-compat dry-runs if instrumentation behavior changes, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Hotspot modularization closeout is reviewable

r[mc_compatibility.stevenarella_hotspot_modules.validation.logs]
- GIVEN a Stevenarella hotspot migration wave is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change focused tests, affected dry-runs when applicable, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
