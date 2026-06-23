# Tasks

- [ ] [serial] Audit Hyperion metadata diff tracking and Valence entity metadata update flow, then record gaps and non-goals. r[valence_hyperion_integration.metadata_diff.audit]
- [ ] [depends:audit] Define invariants for default metadata, changed metadata, same-tick changes, spawn/update ordering, despawn cleanup, and invalid metadata indices. r[valence_hyperion_integration.metadata_diff.invariants]
- [ ] [depends:invariants] Implement pure metadata diff helpers or document no-code conclusions if Valence already satisfies the audited behavior. r[valence_hyperion_integration.metadata_diff.core]
- [ ] [depends:core] Add positive and negative tests for unchanged values, changed values, multiple same-tick changes, default suppression, invalid indices, despawn cleanup, and packet ordering. r[valence_hyperion_integration.metadata_diff.tests]
- [ ] [depends:tests] Wire improvements into Valence entity update systems only where audit evidence justifies a change. r[valence_hyperion_integration.metadata_diff.wiring]
- [ ] [depends:wiring] Run metadata tests, packet fixtures, selected entity mc-compat scenarios, Cairn gates, and Cairn validation. r[valence_hyperion_integration.metadata_diff.validation]
