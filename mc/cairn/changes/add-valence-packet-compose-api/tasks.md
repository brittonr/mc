# Tasks

- [ ] [serial] Define packet compose API scope, route modes, ordering guarantees, error model, and direct-write migration guidance. r[valence_hyperion_integration.packet_compose.contract]
- [ ] [depends:contract] Implement a pure packet delivery planner for unicast, global, local, channel-like groups, exclusions, and invalid route targets. r[valence_hyperion_integration.packet_compose.planner]
- [ ] [depends:planner] Add direct-mode flush wiring that resolves plans to Valence clients without changing default packet-write behavior. r[valence_hyperion_integration.packet_compose.direct_flush]
- [ ] [depends:direct_flush] Add positive and negative tests for bundle ordering, route resolution, exclusions, closed clients, encode failures, and partial flush errors. r[valence_hyperion_integration.packet_compose.tests]
- [ ] [depends:tests] Add docs/examples showing compose API use and when direct client writes remain appropriate. r[valence_hyperion_integration.packet_compose.docs]
- [ ] [depends:docs] Run planner tests, direct flush regressions, selected examples or playground smoke tests, selected mc-compat dry runs, Cairn gates, and Cairn validation. r[valence_hyperion_integration.packet_compose.validation]
