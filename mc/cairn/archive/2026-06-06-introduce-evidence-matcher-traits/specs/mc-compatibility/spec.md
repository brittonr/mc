# mc-compatibility Change Spec: Evidence matcher traits

## Requirements

### Requirement: Evidence matcher contract

r[mc_compatibility.evidence_matcher_traits.contract] Scenario evaluation MUST use an explicit evidence matcher contract before dynamic milestone matching is moved out of ad hoc string-name branches.

#### Scenario: Matcher contract separates ID from behavior

r[mc_compatibility.evidence_matcher_traits.contract.identity]
- GIVEN a milestone rule is evaluated
- WHEN the rule is reported as observed, missing, or forbidden
- THEN the stable milestone ID remains separate from the matcher behavior
- AND receipt-facing `ScenarioEvidence` and `ServerScenarioEvidence` output shapes remain unchanged.

### Requirement: Pure matcher core

r[mc_compatibility.evidence_matcher_traits.core] Evidence matchers MUST be pure deterministic functions over in-memory evidence text and explicit context.

#### Scenario: Matcher core has no side effects

r[mc_compatibility.evidence_matcher_traits.core.pure]
- GIVEN matcher evaluation receives client output, server output, normalized text, username context, and scenario context
- WHEN a literal, case-insensitive, dynamic username, dynamic client-suffix, or any-of matcher runs
- THEN it returns only a boolean presence decision
- AND it does not read files, spawn commands, inspect environment, use clocks, perform network access, or mutate external state.

### Requirement: Explicit milestone rules

r[mc_compatibility.evidence_matcher_traits.rules] Client, server, and forbidden milestone tables SHOULD attach explicit matcher values rather than relying on milestone-name string comparisons.

#### Scenario: Dynamic server checks are visible

r[mc_compatibility.evidence_matcher_traits.rules.dynamic]
- GIVEN a server milestone checks the configured username, client A username, client B username, or flag-or-score fallback
- WHEN reviewers inspect the milestone rule
- THEN the dynamic matcher kind is visible in the rule definition
- AND the stable milestone ID remains the same as the pre-refactor output ID.

### Requirement: Matcher migration

r[mc_compatibility.evidence_matcher_traits.migration] Existing scenario evaluation MUST migrate to matcher evaluation without changing receipt schemas or milestone pass/fail semantics.

#### Scenario: Evaluation parity is preserved

r[mc_compatibility.evidence_matcher_traits.migration.parity]
- GIVEN existing client output and server log fixtures
- WHEN old and new evaluation expectations are compared
- THEN observed milestones, missing milestones, forbidden matches, and pass/fail booleans match the documented pre-refactor behavior.

### Requirement: Matcher tests

r[mc_compatibility.evidence_matcher_traits.tests] The matcher core MUST include positive and negative tests for each supported matcher kind and scenario parity.

#### Scenario: Supported matchers pass valid fixtures

r[mc_compatibility.evidence_matcher_traits.tests.positive]
- GIVEN fixtures contain literal, lowercase-normalized, dynamic username, dynamic client-suffix, and any-of evidence
- WHEN matcher tests run
- THEN each matcher reports the expected observed milestone without requiring a live client or server.

#### Scenario: Missing or forbidden evidence fails closed

r[mc_compatibility.evidence_matcher_traits.tests.negative]
- GIVEN fixtures omit required evidence, contain only differently cased text where case-sensitive matching is required, use the wrong dynamic username, or contain forbidden patterns
- WHEN matcher and scenario evaluation tests run
- THEN missing and forbidden IDs are reported explicitly
- AND pass/fail booleans remain fail-closed.

### Requirement: Matcher validation

r[mc_compatibility.evidence_matcher_traits.validation] The change MUST record focused matcher tests, scenario evaluation tests, relevant evidence checker output, and Cairn gates before archive.

#### Scenario: Matcher closeout is reviewable

r[mc_compatibility.evidence_matcher_traits.validation.log]
- GIVEN evidence matcher traits are implemented
- WHEN the change is archived
- THEN successful logs show matcher positive tests, matcher negative tests, scenario evaluation parity, relevant checker execution, Cairn proposal/design/tasks gates, and Cairn validation.