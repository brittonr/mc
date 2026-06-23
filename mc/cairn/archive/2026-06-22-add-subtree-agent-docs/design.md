# Design: Add subtree-local agent documentation

## Context

Agent guidance is strongest when close to the code it governs. Valence already has local notes. Stevenarella relies on root instructions, and other client/reference subtrees may need explicit boundaries once classified.

## Decisions

### 1. Owned components get local notes

**Choice:** Major owned editable components should have subtree-local agent docs naming commands, VCS boundary, evidence rules, and compatibility instrumentation boundaries.

**Rationale:** Editors often start inside subtrees; local notes prevent missed root context.

### 2. Stevenarella is the first required target

**Choice:** Add Stevenarella-local notes because it is a core component tree and has specific devshell/client instrumentation behavior.

**Rationale:** This is the largest documented gap in the current role layout.

### 3. Reference components get smaller boundary notes

**Choice:** If Leafish or another reference tree remains in the repo, add local notes that make default-gate participation and edit ownership explicit.

**Rationale:** Reference code should not be mistaken for active owned code without guidance.

### 4. Guard on major-owned missing docs

**Choice:** Add a layout guard or registry rule that reports major owned component roots without local agent docs or a documented waiver.

**Rationale:** The rule keeps future component additions from losing workflow guidance.

## Risks / Trade-offs

- Local docs can drift from root docs; mitigate by linking to root notes and keeping commands focused.
- Too many agent files can fragment guidance; mitigate by requiring them only for major editable subtrees.
- Guard strictness can block early experiments; mitigate with explicit waivers in the component registry.
