# Design: Repo-level Minecraft Wiki Pi skill

## Context

Pi loads project skills from `.pi/skills/` and `.agents/skills/` after a project is trusted. Skills are self-contained directories with `SKILL.md` frontmatter containing at least `name` and `description`. This repository currently has project Pi state under `.pi/` but no project skill for Minecraft Wiki retrieval.

The global `crw-web` skill covers safe fetching from known URLs, but it is not specific to this repository's compatibility and plugin evidence rules. The new skill should compose with `crw-web` rather than duplicate low-level scraping details.

## Decisions

### Use the project skill path

**Choice:** Implement the skill at `.pi/skills/minecraft-wiki/SKILL.md` unless implementation discovers a repo-specific reason to use an equivalent Pi-supported project skill path.

**Rationale:** Pi documentation lists `.pi/skills/` as a project skill location. The path keeps the workflow repository-owned and trust-scoped.

### Make the skill retrieval-only and evidence-aware

**Choice:** The skill will guide agents through narrow wiki retrieval, source summarization, version scoping, and Cairn/evidence handoff; it will not include vendored wiki tables or generated plugin code.

**Rationale:** Wiki content is mutable and external. Implementation claims should still come from target-version data extraction and Paper/vanilla receipts.

### Compose with existing web-fetch rules

**Choice:** The skill will instruct agents to use `crw_scrape` for known wiki URLs, start with markdown, use links for discovery, and narrow with CSS/XPath/JS only after markdown is insufficient.

**Rationale:** This follows Pi's existing `crw-web` operating rules while adding project-specific constraints.

### Require version and non-claim notes

**Choice:** Every skill-guided wiki read should record edition, version/protocol target, page URL/title, retrieval date when evidence matters, and explicit non-claims.

**Rationale:** This repository's compatibility work is row-based and non-overclaiming. Wiki pages can describe newer versions than Valence's current target.

### Validate the skill text directly

**Choice:** Add a focused checker or fixture test that validates required frontmatter and required workflow tokens, with positive and negative fixtures.

**Rationale:** Skill behavior is mostly prompt text. A deterministic text check catches accidental removal of source safety, version scoping, or retrieval rules.

## Risks / Trade-offs

- Project skills only load after trust, so docs should not assume the skill is globally available.
- A prompt-only skill cannot enforce behavior at runtime; validation and Cairn task evidence should catch drift.
- Overly broad scraping guidance can waste retrieval budget. The skill should require targeted page reads and stop once evidence is sufficient.
