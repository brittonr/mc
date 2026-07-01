---
name: next-cairn-target
description: Use this skill when the user asks what to do next in this mc workspace, asks to hunt/find/select the next implementation target, or asks to write a Cairn/change package for the next bounded task. It produces one reviewable native Cairn proposal/design/tasks/spec delta with evidence and non-claims, not an implementation unless explicitly requested.
---

# Next Cairn Target

Goal: find the next bounded, evidence-ready target in `/home/brittonr/git/mc` and write one native Cairn change package for it.

## Trigger phrases

Use this skill for prompts like:

- "what next?"
- "find the next target"
- "hunt for a target"
- "write a Cairn"
- "create the next change"
- "pick the next plugin slice"

Do not use it for draining or implementing existing active changes; load the Cairn drain workflow for those.

## Before hunting

1. Check status and preserve user changes.
2. Run `nix run .#cairn -- change list --root .`.
3. If active changes exist, stop target selection and report that the repo should drain or explicitly choose one active change first.
4. Read the smallest relevant sources:
   - `README.md`
   - `AGENTS.md`
   - `docs/vanilla-composable-plugins-roadmap.md`
   - accepted specs under `cairn/specs/` that match the likely domain
   - recent archived Cairns under `cairn/archive/` only when needed to avoid repeating work

## Candidate hunt

Collect candidates from:

- Roadmap sequence rows and stop conditions.
- Accepted specs with requirement IDs that imply follow-on work.
- Behavior cards and docs that say "next required evidence" or "stop before".
- Evidence gaps where a previous Cairn created a card or local core but not data extraction, Paper parity, or Valence shell integration.
- Small checker/doc gaps that unblock a later implementation.

Reject candidates that require broad vanilla parity, all recipes, all block entities, broad redstone, broad mob AI, world generation, default plugin membership changes, public-server safety, or production readiness unless the user explicitly asks for that high-risk scope.

## Scoring rubric

Prefer the candidate with the best combination of:

- Bounded scope: one feature seam or one evidence rail.
- Clear predecessor: a behavior card, accepted requirement, or archived Cairn already names it.
- Testability: can be validated with positive and negative tests without starting the world.
- Evidence path: the required check or receipt can be copied under `docs/evidence/`.
- Low schedule risk: no new default plugin membership or fragile tick-order dependency.
- Non-claims are clear.

## Output decision before writing

Write a short decision summary in the response or change proposal:

```markdown
## Candidate decision

Selected: <slug>
Why now: <bounded reason>
Prerequisites satisfied: <docs/specs/evidence>
Deferred alternatives: <candidate and reason>
Non-claims: <explicit boundaries>
```

## Write one native Cairn

Create one package under `cairn/changes/<slug>/` with:

- `proposal.md`: why, what changes, impact, non-claims.
- `design.md`: current state, decisions, risks/tradeoffs.
- `tasks.md`: serial/dependent tasks with requirement IDs.
- `specs/<accepted-spec>/spec.md`: ADDED/MODIFIED requirements using `r[...]` IDs.

Use native Cairn commands from repo root:

```sh
nix run .#cairn -- change create <slug> --root .
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal <slug> --root .
nix run .#cairn -- gate design <slug> --root .
nix run .#cairn -- gate tasks <slug> --root .
```

## Completion criteria

A target-hunt Cairn is complete when:

- Exactly one active Cairn package exists for the selected next target.
- The package has proposal, design, tasks, and a spec delta.
- Tasks include positive and negative validation requirements when implementation or checker work is planned.
- Proposal/design record rejected alternatives and explicit non-claims.
- Cairn validate and proposal/design/tasks gates pass.
- No implementation, sync, archive, or push happens unless the user explicitly asks for those next steps.

## Gotchas

- This workspace is git-only under the parent `/home/brittonr/git`; use git only because no `.jj/` is present.
- `.pi/` is ignored by default; force-add project skills when committing.
- Nix source-closure checks only see tracked/staged files.
- Do not cite evidence left only under `target/`; copy review-critical receipts to `docs/evidence/`.
- Do not overclaim from Minecraft Wiki text. Wiki pages can guide candidates, but behavior claims need extracted data, Paper/vanilla receipts, or another accepted reference artifact.
