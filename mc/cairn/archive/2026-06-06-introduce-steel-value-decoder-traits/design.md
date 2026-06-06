# Design: Introduce Steel value decoder traits

## Context

The runtime configuration loader is already a Rust-owned typed boundary for a restricted Steel-compatible literal module. The refactor should improve decoding consistency without expanding the Steel language, changing sandbox behavior, or weakening diagnostics.

## Decisions

### 1. Decode through a small trait

**Choice:** Define a trait such as `FromSteelValue` with an expected-type label and a conversion method from `SteelValue`.

**Rationale:** A single generic required-export helper can report missing and wrong-type diagnostics consistently while keeping each accepted Rust type explicit.

### 2. Keep validation after decoding

**Choice:** The trait only converts the enum value to the requested Rust type. Range checks, schema-version checks, backend parsing, sandbox checks, and mutability classification remain separate validation steps.

**Rationale:** Decoding and domain validation are different responsibilities.

### 3. Preserve diagnostic paths

**Choice:** The generic helper receives the export name and diagnostic path exactly as current helpers do.

**Rationale:** Existing checker evidence and tests depend on meaningful paths such as `combat.arrow.max_damage`.

### 4. Do not generalize Steel evaluation

**Choice:** The trait supports only the existing `SteelValue` variants and exported literal subset.

**Rationale:** Full Steel evaluator semantics and new types are out of scope.

## Risks / Trade-offs

- A generic helper can obscure expected field types if call sites are not explicit; use type annotations where inference hurts readability.
- Keeping parsing separate from decoding means some duplication remains intentionally.
- Diagnostics must be checked carefully so evidence gates do not lose useful failure messages.