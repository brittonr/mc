# Design: Introduce shared gameplay plugin composition contracts

## Context

The current plugin seam is useful but uneven: several examples expose local `SystemSet` phases and contract resources, while each plugin names and tests those concepts independently. Composition across game modes needs one small shared vocabulary for phase-level ordering and one repeatable way to state what a plugin installs.

## Decisions

### 1. Share phase intent, not every internal set

**Choice:** Define a shared gameplay phase taxonomy for input, rule evaluation, world mutation, presentation, and cleanup. Existing plugins may retain private local subphases when public ordering does not need them.

**Rationale:** Downstream users need stable phase-level ordering. One set per internal system would freeze implementation details and make examples harder to evolve.

### 2. Keep contract data explicit and inspectable

**Choice:** Each converted gameplay plugin exposes or records a contract describing schedule labels, phase order, owned resources/events, expected scope model, installation mode, and non-claim boundaries.

**Rationale:** Tests and reviewers should not reverse-engineer plugin behavior from schedule graphs alone.

### 3. Provide reusable test helpers

**Choice:** Add helper assertions for installed phases, absent disabled-plugin state, resource/event ownership, and ordering-regression failures.

**Rationale:** CTF, survival compatibility, terrain, and smaller example plugins should not drift into bespoke testing patterns.

### 4. Preserve example support boundaries

**Choice:** Shared contracts document composition shape but do not promote examples into stable production gameplay crates.

**Rationale:** The current examples remain opt-in documentation and compatibility fixtures until separate work extracts supported gameplay crates.

## Risks / Trade-offs

- A shared phase vocabulary can imply more stability than intended; private subphases and non-claims must stay explicit.
- Migrating local phases can introduce schedule regressions; schedule hygiene and disabled-plugin tests are required.
- If the contract surface grows too large, it becomes another API to maintain. Keep it minimal and review-driven.
