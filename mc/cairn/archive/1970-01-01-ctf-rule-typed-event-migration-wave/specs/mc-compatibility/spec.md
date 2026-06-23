# mc-compatibility Change Spec: CTF rule typed-event migration wave

## Requirements

### Requirement: CTF typed-event migration inventory

r[mc_compatibility.ctf_rule_typed_event_migration_wave.readiness] The system MUST define a row-family inventory for maintained CTF rows before marking any CTF rule row `typed-event-ready`.

#### Scenario: CTF row families are inventoried

r[mc_compatibility.ctf_rule_typed_event_migration_wave.readiness.inventory]
- GIVEN maintained CTF rows are selected for typed-event migration
- WHEN reviewers inspect the migration inventory
- THEN each selected row is assigned to a family with required client milestones, Valence server milestones, forbidden surfaces, actor or state correlation, and non-claim scope.

### Requirement: CTF typed-event family gates

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate] The runner MUST include migrated CTF rows in typed-event pass/fail gates so missing, misordered, or incorrectly correlated structured CTF events fail before substring fallback can satisfy a row.

#### Scenario: Missing CTF typed evidence fails closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.missing]
- GIVEN a CTF fixture contains legacy substring-compatible milestones but omits a row-required typed event
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with a structured diagnostic naming the missing event and row.

#### Scenario: Incorrect CTF correlation fails closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.correlation]
- GIVEN a CTF fixture contains typed events with the wrong actor, victim, flag, team, or state correlation for the selected row
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with a correlation diagnostic instead of passing through substring fallback.

#### Scenario: Misordered CTF phases fail closed

r[mc_compatibility.ctf_rule_typed_event_migration_wave.gate.order]
- GIVEN a CTF fixture contains all required typed events but puts a postcondition before the required action or server transition
- WHEN typed-event validation evaluates that CTF row
- THEN the fixture fails with an ordering diagnostic instead of passing through substring fallback.

### Requirement: CTF typed-event documentation

r[mc_compatibility.ctf_rule_typed_event_migration_wave.docs] The migration MUST update generated and human documentation to name migrated CTF rows as typed-event-ready while preserving full CTF correctness and broad compatibility non-claims.

#### Scenario: Documentation stays non-overclaiming

r[mc_compatibility.ctf_rule_typed_event_migration_wave.docs.non_claiming]
- GIVEN CTF rows are migrated to typed-event-ready
- WHEN README and evidence docs are inspected
- THEN they state that typed-event migration changes observability and pass/fail only and does not claim full CTF correctness, all races, all invalid actions, adversarial security, public-server safety, production readiness, or vanilla/reference parity.

### Requirement: CTF typed-event migration validation

r[mc_compatibility.ctf_rule_typed_event_migration_wave.validation] The migration MUST record reviewable evidence for row-family fixtures, scenario-manifest checks, generated-surface freshness, dry-run wrappers, evidence manifests, Cairn gates, and Cairn validation before archive.

#### Scenario: Validation evidence is complete

r[mc_compatibility.ctf_rule_typed_event_migration_wave.validation.log]
- GIVEN CTF row families are migrated to typed-event-ready
- WHEN reviewers inspect the task evidence
- THEN logs show positive and negative typed-event fixtures, typed-event-ready manifest accounting, generated-surface freshness, CTF dry-run wrapper checks, evidence manifest validation, Cairn proposal/design/tasks gates, and Cairn validation.
