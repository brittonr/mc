# mc-compatibility Change Spec: Stevenarella resource manager

## Requirements

### Requirement: Stevenarella resource boundaries

r[mc_compatibility.stevenarella_resources.resource_boundaries] Stevenarella resource code SHOULD expose cohesive boundaries for resource identifiers and paths, pack discovery, lookup/indexing, cache policy, archive access, IO shells, and shared manager state.

#### Scenario: Resource responsibility has one owner

r[mc_compatibility.stevenarella_resources.resource_boundaries.ownership]
- GIVEN a resource manager responsibility is reviewed
- WHEN maintainers inspect resource modules
- THEN the responsibility is owned by a focused module
- AND unrelated path, pack, lookup, cache, archive, IO, and shared-state concerns are not reintroduced into one module.

### Requirement: Stevenarella resource core

r[mc_compatibility.stevenarella_resources.resource_core] Resource identifier parsing, lookup precedence, cache key derivation, path containment, and pack selection SHOULD be pure over explicit inputs.

#### Scenario: Resource decision is testable without IO

r[mc_compatibility.stevenarella_resources.resource_core.testable]
- GIVEN resource identifiers, pack metadata, cache facts, or path summaries
- WHEN the resource core processes them
- THEN the result can be tested without filesystem, archive, download, lock, or logging side effects.

### Requirement: Stevenarella resource parity

r[mc_compatibility.stevenarella_resources.parity] Resource manager splitting MUST preserve public resource APIs, lookup precedence, cache behavior, path safety, IO boundaries, and evidence non-claims.

#### Scenario: Resource behavior remains stable

r[mc_compatibility.stevenarella_resources.parity.stable]
- GIVEN a supported pre-refactor resource lookup or cache input
- WHEN the split resource modules process the same input
- THEN lookup result, cache behavior, path safety, public API behavior, and non-claim boundaries remain equivalent.

### Requirement: Stevenarella resource positive tests

r[mc_compatibility.stevenarella_resources.positive_tests] The change MUST include positive tests for resource identifiers, pack selection, lookup precedence, cache keys, archive entries, and contained paths.

#### Scenario: Supported resource paths pass

r[mc_compatibility.stevenarella_resources.positive_tests.coverage]
- GIVEN representative supported resource inputs
- WHEN extracted resource cores process them
- THEN tests prove the expected identifier, lookup, cache, archive, or path decisions are produced.

### Requirement: Stevenarella resource negative tests

r[mc_compatibility.stevenarella_resources.negative_tests] The change MUST include negative tests for unsafe paths, missing resources, duplicate pack entries, malformed archives, invalid identifiers, and failed IO adapters.

#### Scenario: Invalid resource paths fail closed

r[mc_compatibility.stevenarella_resources.negative_tests.fail_closed]
- GIVEN invalid or unsafe resource inputs
- WHEN extracted resource cores or shells process them
- THEN tests prove the inputs are rejected, missing, or contained according to current behavior.

### Requirement: Stevenarella resource validation

r[mc_compatibility.stevenarella_resources.validation] The change MUST record focused resource tests, affected model/render checks, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_resources.validation.logs]
- GIVEN resource manager splitting is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative resource tests plus affected checks and Cairn gates passing.
