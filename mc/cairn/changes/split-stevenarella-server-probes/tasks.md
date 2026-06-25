# Tasks

- [ ] [serial] Inventory `clients/stevenarella/src/server/mod.rs` responsibilities, probe families, shared helpers, env reads, packet writes, logs, and state mutations. r[mc_compatibility.stevenarella_server_probe_split.inventory]
- [ ] [depends:inventory] Define module boundaries for general server state, probe cores, probe shells, inventory/window helpers, sign/block-entity helpers, and environment/config inputs. r[mc_compatibility.stevenarella_server_probe_split.boundaries]
- [ ] [depends:boundaries] Extract pure probe state machines for selected CTF, survival, inventory, combat, and biome/dimension rails. r[mc_compatibility.stevenarella_server_probe_split.pure_probes]
- [ ] [depends:pure_probes] Wire packet-handler shells to call probe cores and emit existing actions/milestones without changing protocol behavior. r[mc_compatibility.stevenarella_server_probe_split.shell_wiring]
- [ ] [depends:shell_wiring] Preserve env var names, milestone text, fixture constants, packet action order, and non-claim boundaries. r[mc_compatibility.stevenarella_server_probe_split.compatibility]
- [ ] [depends:compatibility] Add positive probe-action tests and negative malformed env, missing fixture, out-of-order packet, invalid window, stale sign/block-entity, and rejected-action tests. r[mc_compatibility.stevenarella_server_probe_split.tests]
- [ ] [depends:tests] Run focused Stevenarella tests through the mc devshell, selected mc-compat dry-runs, Cairn gates, Cairn validation, and task-evidence checks before archive. r[mc_compatibility.stevenarella_server_probe_split.validation]
