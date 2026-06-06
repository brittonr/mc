# Design: Introduce evidence matcher traits

## Context

Milestone evaluation is a pure decision over logs and context. It should be testable without spawning clients, reading files, or starting servers. The current implementation already has a pure core shape, but dynamic cases are hidden behind milestone-name string comparisons.

## Decisions

### 1. Model matching as pure values

**Choice:** Define `EvidenceMatcher` over an immutable evidence corpus and evaluation context. Implement concrete matchers for literal contains, lowercase contains, dynamic username, dynamic suffixed username, and any-of composition.

**Rationale:** Explicit matcher values make dynamic matching visible in scenario specs and remove hidden name-based control flow.

### 2. Keep milestone IDs separate from matcher behavior

**Choice:** A milestone rule contains a stable output ID plus a matcher. The matcher decides presence; the ID remains the receipt/checker-facing name.

**Rationale:** Compatibility evidence consumers depend on the milestone IDs, not on implementation details.

### 3. Use static dispatch where possible

**Choice:** Prefer an enum of matcher kinds or zero-allocation static matcher data unless dynamic composition requires trait objects.

**Rationale:** The runner should keep explicit, low-overhead control flow. Traits are a boundary, not a plugin surface.

### 4. Preserve evaluator output shape

**Choice:** `ScenarioEvidence` and `ServerScenarioEvidence` remain unchanged for this refactor.

**Rationale:** Receipts and existing checkers should not need schema changes.

## Risks / Trade-offs

- Matcher abstraction can become too generic; keep only matchers needed by current scenarios.
- Case normalization must be explicit so client and server evaluation do not accidentally diverge.
- Forbidden matchers must preserve current fail-closed behavior.