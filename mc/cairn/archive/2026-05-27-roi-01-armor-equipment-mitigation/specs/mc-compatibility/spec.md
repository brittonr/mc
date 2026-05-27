# Delta: Armor/equipment mitigation rail

## Requirements

### Requirement: Equipment State

r[mc_compatibility.roi_01_armor_equipment_mitigation.equipment_state] The repo MUST add a bounded protocol-763 rail that establishes deterministic armor or equipment state for a Valence CTF combat actor before mitigation is claimed.

#### Scenario: Equipment State evidence is required

r[mc_compatibility.roi_01_armor_equipment_mitigation.equipment_state.scenario]
- GIVEN `Armor/equipment mitigation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `equipment_state` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Mitigation Correlation

r[mc_compatibility.roi_01_armor_equipment_mitigation.mitigation_correlation] The rail MUST correlate Valence-observed armor/equipment mitigation with client-observed equipment or inventory evidence and damage/health outcomes.

#### Scenario: Mitigation Correlation evidence is required

r[mc_compatibility.roi_01_armor_equipment_mitigation.mitigation_correlation.scenario]
- GIVEN `Armor/equipment mitigation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `mitigation_correlation` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Live Receipt

r[mc_compatibility.roi_01_armor_equipment_mitigation.live_receipt] The rail MUST produce a live receipt with BLAKE3, scoped milestones, and explicit non-claims for full combat, enchantments, shields, projectiles, and vanilla-exact balancing.

#### Scenario: Live Receipt evidence is required

r[mc_compatibility.roi_01_armor_equipment_mitigation.live_receipt.scenario]
- GIVEN `Armor/equipment mitigation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `live_receipt` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant

### Requirement: Dry Run Check

r[mc_compatibility.roi_01_armor_equipment_mitigation.dry_run_check] A deterministic dry-run Nix check MUST validate the scenario shape, usernames/client count where relevant, required milestones, packet oracle, and non-claim contract.

#### Scenario: Dry Run Check evidence is required

r[mc_compatibility.roi_01_armor_equipment_mitigation.dry_run_check.scenario]
- GIVEN `Armor/equipment mitigation rail` is drained
- WHEN the maintained compatibility evidence is reviewed
- THEN `dry_run_check` is satisfied by tracked evidence or a deterministic check
- AND scoped non-claims are preserved where relevant
