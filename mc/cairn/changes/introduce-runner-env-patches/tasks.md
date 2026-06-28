# Tasks

- [ ] [serial] Capture current client, Valence, and Paper env outputs for representative scenario families before extraction. r[mc_compatibility.runner_modularity.env_patch_core]
- [ ] [depends:env_patch_core] Introduce an `EnvPatch` core with deterministic composition, explicit source labels, and conflict diagnostics. r[mc_compatibility.runner_modularity.env_patch_core]
- [ ] [depends:env_patch_core] Refactor scenario client env, Valence server env, and Paper server env derivation to return patches while shell code applies patches to `Command`. r[mc_compatibility.runner_modularity.env_patch_shell]
- [ ] [depends:env_patch_shell] Add positive tests for patch composition and representative env output across inventory, survival, combat, projectile, CTF, reconnect, and MCP scenarios. r[mc_compatibility.runner_modularity.env_patch_positive_tests]
- [ ] [depends:env_patch_positive_tests] Add negative tests for conflicting keys, malformed keys, missing required session values, and backend-incompatible env fragments. r[mc_compatibility.runner_modularity.env_patch_negative_tests]
- [ ] [depends:env_patch_negative_tests] Run focused runner tests, env-patch checks, dry-run smoke checks, Cairn gates, and Cairn validation with reviewable logs. r[mc_compatibility.runner_modularity.env_patch_validation]
