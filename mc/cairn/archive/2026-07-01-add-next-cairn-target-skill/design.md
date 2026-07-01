## Context

The mc workspace uses native Cairn packages under `cairn/changes/`, accepted specs under `cairn/specs/`, and archived changes under `cairn/archive/`. Recent work added a Minecraft Wiki research skill, a composable plugin roadmap, a furnace smelting behavior card, and a selected-row pure core. The next useful automation is a project skill that turns "what next" requests into one bounded Cairn package.

Pi discovers project skills under `.pi/skills/` after a project is trusted. Agent Skills require `SKILL.md` frontmatter with a lowercase hyphenated `name` and a non-empty `description` that states when to use the skill.

## Decisions

### 1. Use a repository-local project skill

**Choice:** Add `.pi/skills/next-cairn-target/SKILL.md` and document it in `docs/next-cairn-target-skill.md`.

**Rationale:** The workflow is specific to this workspace: native Cairn commands, repo evidence rules, roadmap/spec sources, and Minecraft compatibility non-claims. A project skill keeps the guidance local and does not imply global installation.

### 2. Stop at one active Cairn by default

**Choice:** The skill's completion criteria are one active Cairn package with proposal, design, tasks, spec delta, and passing Cairn gates. Implementation, sync, archive, commit, and push are explicit next steps only.

**Rationale:** Target selection and implementation are different risk levels. Stopping after one Cairn preserves reviewability and avoids accidental broad changes.

### 3. Validate skill content with a focused Rust checker

**Choice:** Add `tools/check_next_cairn_target_skill.rs` with pure validation logic over in-memory skill/doc strings and a thin CLI/file-reading shell.

**Rationale:** The checker enforces required frontmatter, workflow steps, candidate scoring, output shape, Cairn gates, and non-claims. Positive and negative fixtures prevent silent drift.

## Risks / Trade-offs

- The skill can guide target choice, but it cannot replace user priority or domain judgment.
- Candidate scoring is intentionally conservative and may defer high-impact broad work until prerequisites exist.
- The skill writes a Cairn package by default; users must explicitly ask to implement or drain it afterward.
