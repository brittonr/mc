# Next Cairn target project skill

## Decision

The project skill lives at `.pi/skills/next-cairn-target/SKILL.md` with frontmatter `name: next-cairn-target`.

This path follows Pi project skill discovery for trusted projects: `.pi/skills/` directories containing `SKILL.md` are discovered recursively, and the Agent Skills format requires YAML frontmatter with `name` and `description`.

The skill is repository-local. It is not installed as a global Pi skill unless separately copied to a global skill directory.

## Purpose

The skill guides agents when users ask to:

- decide "what next" in this workspace;
- hunt for the next bounded target;
- select the next plugin/evidence slice;
- write a native Cairn package for that target.

It defaults to creating one active Cairn package only. It does not implement, sync, archive, commit, or push unless the user explicitly asks for those steps.

## Required workflow

The skill requires agents to:

1. Preserve user changes and inspect repo status.
2. Run `nix run .#cairn -- change list --root .`.
3. Stop target selection if active Cairn changes already exist, unless the user explicitly chooses one.
4. Read the smallest relevant sources: `README.md`, `AGENTS.md`, roadmap docs, matching accepted specs, and recent archives only when needed.
5. Collect candidates from roadmap sequence rows, accepted specs, archived follow-up gaps, behavior cards, evidence gaps, and small checker/doc blockers.
6. Reject broad or risky scopes unless explicitly requested.
7. Choose one bounded target and write a native Cairn package with proposal, design, tasks, and a spec delta.
8. Run Cairn validate plus proposal/design/tasks gates.

## Candidate scoring

The preferred target has:

- bounded scope;
- a clear predecessor artifact;
- positive and negative testability;
- a reviewable evidence path under `docs/evidence/`;
- low schedule/default-plugin risk;
- explicit non-claims.

## Output shape

The skill asks the agent to record a short decision summary:

```markdown
## Candidate decision

Selected: <slug>
Why now: <bounded reason>
Prerequisites satisfied: <docs/specs/evidence>
Deferred alternatives: <candidate and reason>
Non-claims: <explicit boundaries>
```

The generated Cairn package must include:

- `proposal.md`
- `design.md`
- `tasks.md`
- `specs/<accepted-spec>/spec.md`

## Non-claims

This skill does not claim to:

- rank every possible project task;
- replace user priorities;
- implement selected targets;
- prove vanilla parity;
- change Valence default plugin membership;
- make public-server safety or production-readiness claims;
- create global Pi skills.

## Validation

`tools/check_next_cairn_target_skill.rs` validates:

- required `name` and `description` frontmatter;
- project skill path and discovery assumptions;
- target-hunt workflow steps;
- candidate scoring rules;
- Cairn package output shape;
- validation gates;
- explicit non-claims;
- positive and negative self-test fixtures.
