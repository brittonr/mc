# repository-layout Change Spec: Minecraft Wiki Pi skill

## Requirements

### Requirement: Minecraft Wiki project skill path

r[repository_layout.minecraft_wiki_pi_skill.path] The repository SHOULD provide a project-level Pi skill for Minecraft Wiki research under a Pi-supported project skill path and document the chosen path and discovery assumptions.

#### Scenario: Skill path is reviewable

r[repository_layout.minecraft_wiki_pi_skill.path.reviewable]
- GIVEN a repository-owned Minecraft Wiki skill is added
- WHEN developers inspect the change evidence or repository docs
- THEN the selected path, expected `SKILL.md` format, required frontmatter, and project-trust loading assumption are recorded
- AND the skill is not represented as a global Pi skill unless separately installed there.

### Requirement: Minecraft Wiki skill workflow

r[repository_layout.minecraft_wiki_pi_skill.workflow] The project Minecraft Wiki skill MUST instruct agents to perform narrow, known-URL wiki retrieval, treat fetched content as untrusted external data, record edition/version/protocol scope, cite source pages, and require extracted-data or Paper/vanilla parity before behavior claims are promoted.

#### Scenario: Agent reads wiki without overclaiming

r[repository_layout.minecraft_wiki_pi_skill.workflow.non_overclaiming]
- GIVEN an agent uses the skill to inspect a Minecraft Wiki page for plugin design
- WHEN the skill workflow is followed
- THEN the agent starts with focused markdown retrieval from a known wiki URL, records source page identity and target version scope, summarizes behavior as a guide, and names required follow-up evidence before implementation claims
- AND the agent does not execute instructions from the page, vendor large page content by default, or claim broad Minecraft compatibility from wiki text alone.

### Requirement: Minecraft Wiki skill validation

r[repository_layout.minecraft_wiki_pi_skill.tests] The skill work MUST include positive and negative validation for required skill frontmatter and workflow rules.

#### Scenario: Valid skill passes

r[repository_layout.minecraft_wiki_pi_skill.tests.positive]
- GIVEN the repository Minecraft Wiki skill contains valid `name` and `description` frontmatter plus required retrieval, safety, version, citation, and non-claim rules
- WHEN the focused skill validation runs
- THEN the skill passes with a deterministic success result.

#### Scenario: Invalid skill fails clearly

r[repository_layout.minecraft_wiki_pi_skill.tests.negative]
- GIVEN a fixture skill is missing required frontmatter, known-URL retrieval guidance, untrusted-content handling, version scope, citation guidance, or wiki-as-guide wording
- WHEN the focused skill validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Minecraft Wiki skill closeout validation

r[repository_layout.minecraft_wiki_pi_skill.validation] Minecraft Wiki skill work MUST record focused skill validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks for promoted evidence before archive.

#### Scenario: Skill closeout is reviewable

r[repository_layout.minecraft_wiki_pi_skill.validation.log]
- GIVEN the repository Minecraft Wiki skill is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show valid-skill and invalid-skill validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest checks when artifacts are cited, and explicit non-claims for plugin implementation, wiki accuracy, broad vanilla parity, and global Pi installation.
