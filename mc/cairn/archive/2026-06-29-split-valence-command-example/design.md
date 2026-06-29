# Design: Valence command example split

## Context

The command example is both a runnable example and a source of regression fixtures. Splitting it should improve readability without changing the command framework contract.

## Decisions

### 1. Split example concerns

**Choice:** Extract app setup, command graph definitions, handler logic, fixtures/tests, and explanatory docs into focused modules or sections.

**Rationale:** Example readers can find API usage without scanning test and shell code.

### 2. Keep handlers testable

**Choice:** Non-trivial handler decisions should return explicit outcomes that Bevy shells apply.

**Rationale:** Example behavior can be tested without full app setup.

### 3. Preserve teaching value

**Choice:** Public example flow and comments remain clear after the split.

**Rationale:** Examples are documentation as much as tests.
