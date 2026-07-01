# Minecraft Wiki Pi skill path decision

## Decision

The repository-owned Minecraft Wiki skill lives at `.pi/skills/minecraft-wiki/SKILL.md`.

## Source basis

Pi `docs/skills.md` says Pi loads project skills only after the project is trusted, including `.pi/skills/` and `.agents/skills/` in the current directory or ancestors. The same doc says direct root `.md` files are discovered in `.pi/skills/`, and directories containing `SKILL.md` are discovered recursively. It also documents the required `SKILL.md` frontmatter fields `name` and `description`, and warns that skills with a missing description are not loaded.

The Agent Skills specification likewise describes a skill as a directory containing `SKILL.md`, with YAML frontmatter followed by Markdown body content. Its required frontmatter fields are `name` and `description`; names are lowercase letters, numbers, and hyphens, and descriptions should describe what the skill does and when to use it.

## Rationale

`.pi/skills/minecraft-wiki/SKILL.md` is selected because it is a Pi-supported project skill path and keeps the retrieval workflow repository-owned and project-trust scoped. The skill is not represented as a global Pi skill. Agents outside this trusted repository should not assume it is loaded unless it is separately installed or passed through an explicit skill path.

The parent directory name matches the skill `name` (`minecraft-wiki`) even though Pi is lenient about directory/name mismatches. Matching the name keeps the package compatible with stricter Agent Skills tooling.

## Discovery assumptions

- Pi project skill discovery is available only after this project is trusted.
- The skill body is loaded on demand when a task mentions Minecraft Wiki lookup, wiki-guided plugin design, behavior-card drafting, target-version scoping, or source-page evidence triage.
- Relative references in the skill, if any are added later, should be resolved from `.pi/skills/minecraft-wiki/`.
- The skill is retrieval-only guidance. It does not vendor wiki content, install global Pi state, implement plugins, or prove wiki accuracy.

## Validation

`tools/check_minecraft_wiki_guidance.rs` validates the selected path, required frontmatter, known-URL retrieval workflow, untrusted-source safety language, target edition/version/protocol scoping, citation requirements, wiki-as-guide/non-authority wording, behavior-card handoff, and non-overclaiming evidence guidance.
