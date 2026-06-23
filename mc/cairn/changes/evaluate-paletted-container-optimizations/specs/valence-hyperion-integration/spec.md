# valence-hyperion-integration Change Spec: Paletted container optimizations

## Requirements

### Requirement: Paletted container inventory

r[valence_hyperion_integration.palette_optimization.inventory] The integration MUST compare Hyperion and Valence paletted container behavior before changing Valence chunk storage internals.

#### Scenario: Representation differences are recorded

r[valence_hyperion_integration.palette_optimization.inventory.recorded]
- GIVEN paletted container optimization is selected
- WHEN reviewers inspect the inventory
- THEN the inventory records representation states, encode paths, query helpers, mutation behavior, and unsafe or nightly dependencies.

### Requirement: Paletted container invariants

r[valence_hyperion_integration.palette_optimization.invariants] Paletted container work MUST define correctness invariants for indexing, representation transitions, unique queries, iteration, encoding, invalid indices, and mutation behavior.

#### Scenario: Representation transition preserves values

r[valence_hyperion_integration.palette_optimization.invariants.transition]
- GIVEN a section transitions from compact palette storage to direct storage
- WHEN all indices are read after the transition
- THEN every block state matches the pre-transition logical section state.

### Requirement: Benchmark and fixture baseline

r[valence_hyperion_integration.palette_optimization.baseline] Paletted container work MUST capture baseline correctness fixtures and benchmark results before modifying Valence internals.

#### Scenario: Baseline evidence names workloads

r[valence_hyperion_integration.palette_optimization.baseline.workloads]
- GIVEN baseline benchmarks are recorded
- WHEN reviewers inspect the evidence
- THEN each benchmark names the section distributions, mutation pattern, encode path, and command used to run it.

### Requirement: Stable-safe optimization port

r[valence_hyperion_integration.palette_optimization.port] Valence SHOULD port only measured stable-safe optimization concepts unless separate audit evidence approves unsafe or nightly-specific code.

#### Scenario: Unaudited unsafe implementation is rejected

r[valence_hyperion_integration.palette_optimization.port.reject_unsafe]
- GIVEN a proposed optimization depends on unaudited unsafe or nightly-only behavior
- WHEN the implementation plan is reviewed
- THEN the code is rejected, rewritten in stable safe Rust, or moved to a separate audit Cairn.

### Requirement: Paletted container tests

r[valence_hyperion_integration.palette_optimization.tests] Paletted container work MUST include positive and negative tests for storage states, transitions, invalid inputs, and encode parity.

#### Scenario: Invalid index fails correctly

r[valence_hyperion_integration.palette_optimization.tests.invalid_index]
- GIVEN a read or write uses an out-of-range section index
- WHEN the container API handles the request
- THEN it returns the documented error or panic boundary
- AND it does not corrupt any in-range block state.

### Requirement: Paletted container validation

r[valence_hyperion_integration.palette_optimization.validation] Paletted container work MUST record correctness tests, benchmark evidence, selected chunk compatibility checks when behavior changes, and Cairn gates before archive.

#### Scenario: Optimization closeout is reviewable

r[valence_hyperion_integration.palette_optimization.validation.log]
- GIVEN paletted container work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show baseline results, final benchmarks, positive correctness tests, negative invalid-input tests, encode parity checks, selected chunk dry runs if needed, and Cairn validation.
