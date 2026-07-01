## ADDED Requirements

### Requirement: Next Cairn target project skill

r[repository_layout.next_cairn_target_skill] The repository SHOULD provide a project-level Pi skill for hunting the next bounded work target and writing one native Cairn package.

#### Scenario: Skill selects one bounded target

r[repository_layout.next_cairn_target_skill.bounded]
- GIVEN a user asks what to do next, asks to hunt or find the next target, or asks to write a Cairn for the next target
- WHEN the skill workflow runs
- THEN it checks repository status and active Cairn changes, reads the smallest relevant roadmap/spec/archive sources, collects candidate targets, scores boundedness and evidence readiness, and selects one target
- AND it rejects broad vanilla parity, all recipes, all block entities, broad redstone, broad mob AI, world generation, default plugin membership changes, public-server safety, or production-readiness scope unless explicitly requested.

#### Scenario: Skill writes a native Cairn package

r[repository_layout.next_cairn_target_skill.package]
- GIVEN a bounded next target is selected
- WHEN the skill writes the Cairn
- THEN it creates proposal, design, tasks, and spec delta files under `cairn/changes/<slug>/`
- AND it runs Cairn validation plus proposal/design/tasks gates
- AND it does not implement, sync, archive, commit, push, or claim broad compatibility unless the user explicitly asks for those next steps.

### Requirement: Next Cairn target skill documentation

r[repository_layout.next_cairn_target_skill.docs] The next-target skill work MUST document the selected project skill path, Pi discovery assumptions, workflow, scoring rubric, Cairn output shape, and non-claims.

#### Scenario: Skill docs are reviewable

r[repository_layout.next_cairn_target_skill.docs.reviewable]
- GIVEN reviewers inspect the repository documentation
- WHEN they compare it with the project skill
- THEN they can identify the `.pi/skills/next-cairn-target/SKILL.md` path, frontmatter expectations, project-trust loading assumption, candidate sources, scoring criteria, output shape, validation steps, and explicit non-claims.

### Requirement: Next Cairn target skill validation

r[repository_layout.next_cairn_target_skill.validation] The next-target skill work MUST include focused validation with positive and negative self-tests for required skill and documentation rules.

#### Scenario: Complete target-hunt skill passes

r[repository_layout.next_cairn_target_skill.validation.positive]
- GIVEN the skill and docs contain valid frontmatter, preflight steps, candidate sources, scoring rubric, Cairn package output shape, validation gates, and non-claims
- WHEN focused validation runs
- THEN it passes with a deterministic success result.

#### Scenario: Incomplete target-hunt skill fails clearly

r[repository_layout.next_cairn_target_skill.validation.negative]
- GIVEN a fixture skill or documentation set is missing frontmatter, status/change-list preflight, candidate sources, scoring rubric, Cairn package output shape, validation gates, or non-claims
- WHEN focused validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Next Cairn target skill closeout

r[repository_layout.next_cairn_target_skill.closeout] The next-target skill work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Target skill closeout is reviewable

r[repository_layout.next_cairn_target_skill.closeout.log]
- GIVEN the next-target project skill change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative validation, Cairn gates, Cairn validation, task-evidence validation, accepted-spec requirement IDs, evidence-manifest freshness, flake checks, and archive receipts
- AND the evidence preserves non-claims for target implementation, global Pi installation, broad project prioritization, broad Minecraft compatibility, vanilla parity, public-server safety, and production readiness.
