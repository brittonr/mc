# runtime-configuration Change Spec: Steel value decoder traits

## Requirements

### Requirement: Steel value decoder trait contract

r[runtime_configuration.steel_value_decoder_traits.contract] Runtime configuration code MUST define a bounded Steel value decoder trait contract before replacing repeated typed export helper functions.

#### Scenario: Decoder scope is limited to existing values

r[runtime_configuration.steel_value_decoder_traits.contract.scope]
- GIVEN the runtime loader decodes restricted Steel-compatible literal exports
- WHEN reviewers inspect the decoder contract
- THEN it lists the accepted target Rust types and existing `SteelValue` variants
- AND it does not add new Steel syntax, new sandbox capabilities, new exports, new policy hooks, or new snapshot schema fields.

### Requirement: Pure decoder core

r[runtime_configuration.steel_value_decoder_traits.core] Steel value decoder implementations MUST be pure deterministic conversions from `SteelValue` references to explicit Rust target types.

#### Scenario: Decoder conversion has no side effects

r[runtime_configuration.steel_value_decoder_traits.core.pure]
- GIVEN a decoder receives a `SteelValue` and an expected target type
- WHEN conversion runs for string, string-list, u32, or f64
- THEN it returns the converted value or a typed mismatch
- AND it does not read files, evaluate Steel code, spawn processes, inspect environment, use clocks, perform network access, or mutate runtime state.

### Requirement: Runtime config migration

r[runtime_configuration.steel_value_decoder_traits.migration] Runtime config normalization SHOULD use one generic typed required-export helper once decoder parity tests exist.

#### Scenario: Domain validation remains separate

r[runtime_configuration.steel_value_decoder_traits.migration.validation]
- GIVEN a required export decodes successfully
- WHEN runtime config normalization continues
- THEN schema-version checks, backend parsing, port bounds, timeout bounds, arrow damage range checks, mutability classification, sandbox checks, and snapshot provenance checks remain separate from the decoder trait.

### Requirement: Decoder tests

r[runtime_configuration.steel_value_decoder_traits.tests] The decoder refactor MUST include positive and negative tests for every supported decoded type and failure mode.

#### Scenario: Valid typed exports pass

r[runtime_configuration.steel_value_decoder_traits.tests.positive]
- GIVEN valid string, string-list, u32, and f64 exports are present in the parsed literal export map
- WHEN the generic required-export helper decodes them
- THEN each value is returned with the same value that the pre-refactor helper returned.

#### Scenario: Invalid typed exports fail closed

r[runtime_configuration.steel_value_decoder_traits.tests.negative]
- GIVEN an export is missing, has the wrong `SteelValue` variant, is malformed before decoding, is unsupported, or decodes to a value that later violates domain range checks
- WHEN loader tests run
- THEN diagnostics preserve the configured path and expected type
- AND no invalid candidate snapshot becomes active.

### Requirement: Runtime config regression coverage

r[runtime_configuration.steel_value_decoder_traits.regression] Existing sandbox, mutability, snapshot, and reload-planning behavior MUST remain covered after decoder migration.

#### Scenario: Existing invalid module behavior is preserved

r[runtime_configuration.steel_value_decoder_traits.regression.invalid]
- GIVEN a Steel module uses forbidden capabilities, unknown exports, missing required exports, malformed literals, wrong types, invalid ranges, fixed-protocol changes, or apply failures
- WHEN runtime config tests and checkers run
- THEN the previous snapshot or default policy remains authoritative
- AND diagnostics remain reviewable and non-secret.

### Requirement: Decoder validation

r[runtime_configuration.steel_value_decoder_traits.validation] The change MUST record runtime config tests, runtime Steel config checker output, and Cairn gates before archive.

#### Scenario: Decoder closeout is reviewable

r[runtime_configuration.steel_value_decoder_traits.validation.log]
- GIVEN Steel value decoder traits are implemented
- WHEN the change is archived
- THEN successful logs show decoder positive tests, decoder negative tests, runtime config regression tests, runtime Steel config checker output, Cairn proposal/design/tasks gates, and Cairn validation.