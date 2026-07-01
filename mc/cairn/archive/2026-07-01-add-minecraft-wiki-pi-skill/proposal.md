# Proposal: Add a repo-level Minecraft Wiki Pi skill

## Why

Agents working in this repository will repeatedly consult `https://minecraft.wiki/` while designing composable Valence plugins and compatibility evidence. Without a repo-specific skill, wiki use can become inconsistent: agents may fetch broad pages unnecessarily, forget target-version scoping, treat mutable wiki text as authoritative evidence, or omit non-claims when translating behavior into Cairn tasks.

A project-level Pi skill can provide a narrow, repeatable retrieval workflow for wiki-guided plugin work while keeping fetched web content untrusted and evidence-backed.

## What Changes

- Add a repo-level Pi skill under `.pi/skills/minecraft-wiki/SKILL.md` or an equivalent project skill path documented by Pi.
- Define when agents should load the skill: Minecraft Wiki lookup, wiki-guided plugin design, behavior-card drafting, version scoping, or source-page evidence triage.
- Require narrow `crw_scrape` retrieval from known wiki URLs, starting with markdown and using links/CSS/XPath/JS only when needed.
- Require target edition/version/protocol notes, source-page citation, wiki-as-guide wording, and Paper/extracted-data parity before implementation claims.
- Add focused validation that the skill frontmatter, safety rules, version-scoping rules, and retrieval workflow are present.

## Impact

- **Files**: `.pi/skills/minecraft-wiki/SKILL.md`, optional skill validation helper/checker, docs/evidence validation logs, and this Cairn change.
- **Testing**: positive and negative skill-content validation, Pi skill-discovery/path checks where practical, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive.
- **Non-claims**: this does not vendor wiki content, guarantee wiki accuracy, implement plugins, change Pi global skills, or replace Paper/vanilla parity evidence.
