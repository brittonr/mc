# Tasks

- [x] [serial] Inventory major owned/reference subtrees and whether each has local agent/workflow notes or a waiver. r[repository_layout.subtree_agent_docs.inventory]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
- [x] [depends:inventory] Add Stevenarella-local agent docs covering devshell Cargo commands, protocol tests, compat probes, VCS boundary, and evidence rules. r[repository_layout.subtree_agent_docs.stevenarella]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
- [x] [depends:inventory] Add Leafish/reference local notes or a documented waiver after Leafish classification decides ownership. r[repository_layout.subtree_agent_docs.references]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
- [x] [depends:stevenarella] Update root docs to point to subtree-local notes without duplicating every command. r[repository_layout.subtree_agent_docs.root_links]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
- [x] [depends:root_links] Add or update a layout guard/checklist for missing local agent docs on major owned components. r[repository_layout.subtree_agent_docs.guard]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
- [x] [depends:guard] Run docs/layout checks, documented command dry-runs if added, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.subtree_agent_docs.validation]
  Evidence: docs/evidence/subtree-agent-docs-2026-06-22.run.log; docs/evidence/subtree-agent-docs-2026-06-22.b3
