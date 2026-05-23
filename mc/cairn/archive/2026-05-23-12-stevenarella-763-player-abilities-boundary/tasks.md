# Tasks

- [x] [serial] Inspect Valence `PlayerAbilitiesS2CPacket` and Stevenarella packet DTO/parser shape for `play/clientbound/0x34`. r[mc_compatibility.protocol_763.player_abilities.shape_reviewed]
- [x] [serial] Implement the narrow protocol 763 translator/parser update for `PlayerAbilitiesS2CPacket` without broad aliasing. r[mc_compatibility.protocol_763.player_abilities.mapping_updated]
- [x] [serial] Add positive and negative protocol regression tests proving `0x34` maps correctly and no longer uses the inherited 758 fallback. r[mc_compatibility.protocol_763.player_abilities.tests_cover_mapping]
- [x] [serial] Run focused format/tests and the Valence `ctf` trace/probe to identify the next boundary. r[mc_compatibility.protocol_763.player_abilities.trace_advances]
- [x] [serial] Record parent `mc` evidence with a deterministic receipt/check and non-overclaiming contract flags. r[mc_compatibility.protocol_763.player_abilities.evidence_recorded]
