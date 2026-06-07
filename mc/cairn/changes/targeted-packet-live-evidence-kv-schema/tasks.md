# Tasks

- [ ] [serial] Define the common targeted packet live evidence KV schema and distinguish promotion evidence from blocker/non-promotion documentation. r[mc_compatibility.targeted_packet_live_kv_schema.contract]
- [ ] [depends:contract] Implement pure schema validation helpers over parsed key/value records, keeping file I/O and CLI reporting in a thin shell. r[mc_compatibility.targeted_packet_live_kv_schema.core]
- [ ] [depends:core] Add row-extension validation hooks for creative inventory, resource-pack status, sign editor open/update, and future targeted packet rows. r[mc_compatibility.targeted_packet_live_kv_schema.extensions]
- [ ] [depends:extensions] Add positive and negative checker fixtures for valid common evidence, missing keys, wrong packet rows, stale receipt digests, weak revision metadata, malformed extension fields, and broad overclaims. r[mc_compatibility.targeted_packet_live_kv_schema.tests]
- [ ] [depends:tests] Document the KV schema and future live-rail workflow in repo-local docs or evidence notes. r[mc_compatibility.targeted_packet_live_kv_schema.docs]
- [ ] [depends:docs] Run targeted packet checks, evidence-manifest/task-evidence checks, Cairn gates, sync, archive, and post-archive validation. r[mc_compatibility.targeted_packet_live_kv_schema.validation]
