# Tasks

- [ ] [serial] Inspect Valence `HealthUpdateS2CPacket` and Stevenarella packet DTO/parser shape for `play/clientbound/0x57`. r[mc_compatibility.protocol_763.health_update.shape_reviewed]
- [ ] [serial] Implement the narrow protocol 763 translator/parser update for `HealthUpdateS2CPacket` without broad aliasing. r[mc_compatibility.protocol_763.health_update.mapping_updated]
- [ ] [serial] Add positive and negative protocol regression tests proving `0x57` maps correctly and no longer uses the inherited 758 fallback. r[mc_compatibility.protocol_763.health_update.tests_cover_mapping]
- [ ] [serial] Run focused format/tests and the Valence `ctf` trace/probe to identify the next boundary. r[mc_compatibility.protocol_763.health_update.trace_advances]
- [ ] [serial] Record parent `mc` evidence with a deterministic receipt/check and non-overclaiming contract flags. r[mc_compatibility.protocol_763.health_update.evidence_recorded]
