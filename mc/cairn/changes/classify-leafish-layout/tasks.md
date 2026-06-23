# Tasks

- [ ] [serial] Record the Leafish ownership decision: owned client role, reference/vendor input, or external checkout. r[repository_layout.leafish_classification.ownership]
- [ ] [depends:ownership] If owned, move or transition Leafish under a role path and document its commands, test boundary, and VCS boundary. r[repository_layout.leafish_classification.owned_role_path]
- [ ] [depends:ownership] If reference-only or external, document the reference path/removal policy and keep it out of default compatibility gates unless explicitly selected. r[repository_layout.leafish_classification.reference_boundary]
- [ ] [depends:ownership] Update root layout docs, `AGENTS.md`, README, and architecture notes so nested Git exceptions and component roles are reviewable. r[repository_layout.leafish_classification.docs]
- [ ] [depends:docs] Add or update layout checks/tests so undocumented root-level nested Git checkouts fail or require an explicit documented exception. r[repository_layout.leafish_classification.layout_guard]
- [ ] [depends:layout_guard] Run layout checks, affected component checks if any files move, Cairn gates, and Cairn validation with reviewable logs. r[repository_layout.leafish_classification.validation]
