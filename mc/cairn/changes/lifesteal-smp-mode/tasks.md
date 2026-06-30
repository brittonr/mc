# Tasks

- [ ] [serial] Define LifeSteal SMP mode ownership, heart-rule non-claims, named configuration, final-death policy options, and module layout. r[hyperion_game_modes.lifesteal.scope]
- [ ] [depends:scope] Implement mode-local heart-capacity state and projection boundaries so default survival, Bedwars, CTF, and other modes are unaffected. r[hyperion_game_modes.lifesteal.heart_state]
- [ ] [depends:heart_state] Implement pure death-attribution and heart-transfer cores for PvP kills, stale combat tags, self-kills, environmental deaths, duplicate events, and overflow rejection. r[hyperion_game_modes.lifesteal.death_transfer]
- [ ] [depends:death_transfer] Implement configurable final-death/exclusion, recovery, grace, craft/trade-heart hooks where scoped, and player feedback decisions. r[hyperion_game_modes.lifesteal.final_death]
- [ ] [depends:final_death] Add snapshot persistence, restore validation, audit summaries, and bounded admin repair actions for heart state. r[hyperion_game_modes.lifesteal.persistence_admin]
- [ ] [depends:persistence_admin] Add positive tests for valid PvP transfers, final-death policy, recovery, snapshot restore, bounded admin repair, and mode isolation. r[hyperion_game_modes.lifesteal.tests]
- [ ] [depends:tests] Add negative tests for self-kills, environmental deaths, stale tags, duplicate death events, overflow, unauthorized admin actions, corrupt snapshots, and cross-mode heart mutation. r[hyperion_game_modes.lifesteal.tests]
- [ ] [depends:tests] Run focused Hyperion checks, transfer core tests, shell/plugin tests, persistence fixtures, admin command tests, Cairn gates, Cairn validation, task-evidence validation, and evidence-manifest checks before archive. r[hyperion_game_modes.lifesteal.validation]
