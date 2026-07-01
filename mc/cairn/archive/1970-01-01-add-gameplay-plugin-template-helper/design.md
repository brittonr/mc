# Design: Add gameplay plugin template helper

## Context

Many Valence examples define local phase enums, contract resources, `Plugin` implementations, schedule ordering, setup resources, and tests. The shared `gameplay_contracts` module helps larger fixtures, but smaller examples still duplicate patterns and can skip negative disabled-plugin checks.

## Decisions

### 1. Prefer helper functions/types before macros

**Choice:** Start with explicit helper types/functions for descriptors, contract registration, phase checks, and disabled-plugin assertions. Use macros only if the helper remains too verbose after implementation.

**Rationale:** Explicit helpers are easier to review and keep control flow visible.

### 2. Keep plugin logic outside the template

**Choice:** The helper owns boilerplate registration and contract/test wiring. Gameplay decisions remain in pure cores and shell systems owned by each plugin.

**Rationale:** Templates should not hide gameplay behavior or make ECS mutation implicit.

### 3. Require positive and negative plugin tests

**Choice:** The template includes fixtures or assertions for installed metadata/resources and for absent behavior when the plugin is not added.

**Rationale:** New plugins should prove both install and disabled behavior consistently.

### 4. Migrate selected examples incrementally

**Choice:** Apply the helper to a small set of representative examples first, keeping compatibility milestones and behavior unchanged.

**Rationale:** This validates the helper without large churn across all examples.

## Risks / Trade-offs

- A too-generic helper can obscure simple examples; keep opt-in and allow local code when the helper adds no value.
- Macros can hide control flow; avoid them unless tests and docs prove the generated shape clearly.
- Migration can touch many examples; start with selected examples and document remaining gaps.
