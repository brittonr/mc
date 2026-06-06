# Design: Introduce server backend runtime traits

## Context

Backend-specific logic is currently encoded as scattered enum matches. The refactor should improve locality without turning the runner into a dynamic plugin system or changing compatibility evidence. The enum can remain the serialized/configured identity; the trait owns behavior.

## Decisions

### 1. Keep `ServerBackend` as the stable config identity

**Choice:** Preserve the existing enum for parsing, receipts, config snapshots, and equality checks. Add a pure dispatch method that returns the appropriate runtime implementation.

**Rationale:** This avoids public config churn and keeps receipt comparison stable while still moving behavior behind a trait.

### 2. Use concrete zero-sized runtime implementations

**Choice:** Implement `ServerRuntime` for `ValenceRuntime` and `PaperRuntime` with explicit methods for `name`, `default_port`, `start`, `stop`, `force_stop`, `server_log_label`, and `read_log`.

**Rationale:** Concrete implementations keep control flow explicit and avoid heap allocation or object lifetime complexity.

### 3. Separate pure backend facts from imperative lifecycle operations

**Choice:** Put name/default-port/log-label construction in pure methods and keep process/container/file operations in imperative shell methods.

**Rationale:** Pure facts are easy to test with plain assertions. Lifecycle methods remain thin wrappers around existing command/file operations.

### 4. Prove no behavior drift with focused fixtures

**Choice:** Add tests that compare old expected backend names, default ports, parse failures, matrix config defaults, dry-run lifecycle outputs, and log-source selection.

**Rationale:** This is a refactor; validation must prove parity, not new semantics.

## Risks / Trade-offs

- Trait indirection can hide control flow if overused; keep dispatch explicit and local.
- Backend lifecycle still performs side effects; only the selector and pure facts should be pure core.
- Adding new backends remains out of scope until a separate evidence contract exists.