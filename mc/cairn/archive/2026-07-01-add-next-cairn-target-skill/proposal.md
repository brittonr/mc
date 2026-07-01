## Why

The repository now has a chain of Minecraft Wiki, behavior-card, and selected-row core work. Users are asking "what next" and need a repeatable project workflow that hunts for the next bounded target and writes one native Cairn package without overclaiming or accidentally implementing broad vanilla scope.

## What Changes

- Add project skill `.pi/skills/next-cairn-target/SKILL.md` for target hunting and Cairn package authoring.
- Add `docs/next-cairn-target-skill.md` documenting the path, trigger scope, workflow, scoring rubric, output shape, and non-claims.
- Add focused validation in `tools/check_next_cairn_target_skill.rs` with positive and negative self-tests.
- Extend the repository-layout spec with project-skill requirements for next-target selection and Cairn authoring.

## Impact

- **Files**: `.pi/skills/next-cairn-target/SKILL.md`, `docs/next-cairn-target-skill.md`, `tools/check_next_cairn_target_skill.rs`, accepted repository-layout spec, archived Cairn package, and evidence logs.
- **Testing**: Focused skill validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest checks, and flake evidence checks.
- **Non-claims**: The skill does not implement selected targets, rank every project task, replace user priorities, prove vanilla parity, change default plugin membership, create public-server safety claims, create production-readiness claims, or install a global Pi skill.
