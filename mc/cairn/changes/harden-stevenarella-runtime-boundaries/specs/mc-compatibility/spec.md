# mc-compatibility Change Spec: Stevenarella runtime boundaries

## Requirements

### Requirement: Stevenarella unsafe boundary audit

r[mc_compatibility.stevenarella_runtime_boundaries.audit] Stevenarella runtime-boundary work MUST audit unsafe code, global mutable state, synchronization assumptions, caller invariants, and failure modes before refactoring GL, resources, or ECS internals.

#### Scenario: Unsafe assumptions are explicit

r[mc_compatibility.stevenarella_runtime_boundaries.audit.explicit]
- GIVEN a GL, resource, ECS, render, model, or chunk-builder boundary is selected for hardening
- WHEN reviewers inspect the audit
- THEN each unsafe block, unsafe impl, global mutable state item, and relevant unwrap/panic assumption is classified with ownership, invariant, failure mode, and planned mitigation
- AND unsupported full memory-safety proof claims remain non-claims.

### Requirement: Explicit GL context boundary

r[mc_compatibility.stevenarella_runtime_boundaries.gl_context] Stevenarella rendering SHOULD replace or quarantine the global mutable GL context behind an explicit initialization and context-access boundary.

#### Scenario: GL use is initialization-checked

r[mc_compatibility.stevenarella_runtime_boundaries.gl_context.initialized]
- GIVEN rendering code needs GL access
- WHEN it requests the context through the hardened boundary
- THEN the boundary proves initialization or returns a deterministic diagnostic/panic policy documented for startup-only failure
- AND raw global mutable pointer access is isolated to the smallest possible module or removed.

### Requirement: Resource manager sharing boundary

r[mc_compatibility.stevenarella_runtime_boundaries.resources] Resource manager sharing MUST separate pack IO, reload/version state, progress reporting, and worker-thread access behind safe ownership or synchronization contracts.

#### Scenario: Resource access is synchronized by design

r[mc_compatibility.stevenarella_runtime_boundaries.resources.synchronized]
- GIVEN render, model, UI, or chunk-builder code reads resources while reload/progress work may occur
- WHEN the resource boundary is used
- THEN immutable reads, mutable pack updates, progress updates, and worker communication are represented by explicit synchronized types or ownership handoffs
- AND unsafe `Sync` assumptions are removed or documented as temporary shims with tests and retirement tasks.

### Requirement: ECS unsafe storage boundary

r[mc_compatibility.stevenarella_runtime_boundaries.ecs] ECS raw storage and lifetime-sensitive access MUST be encapsulated behind safe APIs that enforce entity generation, component membership, aliasing, and drop invariants.

#### Scenario: Invalid ECS access fails closed

r[mc_compatibility.stevenarella_runtime_boundaries.ecs.invalid]
- GIVEN an entity key is stale, a component is absent, storage has been removed, or an invalid borrow pattern is attempted through safe APIs
- WHEN ECS accessors are called
- THEN they return `None`, a deterministic diagnostic, or a compile-time rejection according to the API contract
- AND safe callers cannot observe unchecked transmute or raw pointer access.

### Requirement: Stevenarella runtime compatibility preservation

r[mc_compatibility.stevenarella_runtime_boundaries.compatibility] Runtime-boundary hardening MUST preserve observable render/resource/probe behavior and existing mc-compat evidence boundaries unless another Cairn changes them.

#### Scenario: Client rails remain comparable

r[mc_compatibility.stevenarella_runtime_boundaries.compatibility.stable]
- GIVEN a hardened runtime boundary is used by existing render, resource, ECS, or probe paths
- WHEN focused Stevenarella checks or selected mc-compat rails run
- THEN existing user-visible behavior, capture output shape, probe milestones, and receipt fields remain compatible
- AND the change does not claim full client safety, renderer portability, or full protocol compatibility.

### Requirement: Stevenarella runtime-boundary tests

r[mc_compatibility.stevenarella_runtime_boundaries.tests] Runtime-boundary hardening MUST include positive invariant tests and negative fail-closed tests for each hardened boundary.

#### Scenario: Valid runtime invariants pass

r[mc_compatibility.stevenarella_runtime_boundaries.tests.positive]
- GIVEN GL initialization, resource reads, progress updates, entity generation, component membership, and component drop cases satisfy the documented invariants
- WHEN focused tests run
- THEN the hardened boundaries allow existing behavior and preserve data correctly.

#### Scenario: Invalid runtime invariants fail closed

r[mc_compatibility.stevenarella_runtime_boundaries.tests.negative]
- GIVEN GL access is uninitialized, a resource is missing, progress state is invalid, an entity generation is stale, a component is absent, removal/drop ordering is invalid, or a borrow pattern is unsupported
- WHEN boundary tests run
- THEN deterministic diagnostics, `None`, or documented startup failure behavior occurs
- AND no undefined behavior is exposed through safe APIs.

### Requirement: Stevenarella runtime-boundary validation

r[mc_compatibility.stevenarella_runtime_boundaries.validation] Runtime-boundary hardening MUST record focused Stevenarella tests/checks, selected render/capture or mc-compat checks, Cairn gates, and task-evidence checks before archive.

#### Scenario: Runtime-boundary closeout is reviewable

r[mc_compatibility.stevenarella_runtime_boundaries.validation.log]
- GIVEN runtime-boundary hardening is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative boundary tests, focused Stevenarella checks through the mc devshell, selected render/capture or mc-compat evidence as scoped, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
