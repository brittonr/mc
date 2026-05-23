# Tasks

- [x] [serial] Inspect Valence `EntityStatusS2CPacket` and Stevenarella packet DTO/parser shape for `play/clientbound/0x1c`. r[mc_compatibility.protocol_763.entity_status.shape_reviewed]
- [x] [serial] Implement the narrow protocol 763 translator/parser update for `EntityStatusS2CPacket` without broad aliasing. r[mc_compatibility.protocol_763.entity_status.mapping_updated]
- [x] [serial] Add positive and negative protocol regression tests proving `0x1c` maps correctly and no longer uses the inherited 758 fallback. r[mc_compatibility.protocol_763.entity_status.tests_cover_mapping]
- [x] [serial] Run focused format/tests and the Valence `ctf` trace/probe to identify the next boundary. r[mc_compatibility.protocol_763.entity_status.trace_advances]
- [x] [serial] Record parent `mc` evidence with a deterministic receipt/check and non-overclaiming contract flags. r[mc_compatibility.protocol_763.entity_status.evidence_recorded]
