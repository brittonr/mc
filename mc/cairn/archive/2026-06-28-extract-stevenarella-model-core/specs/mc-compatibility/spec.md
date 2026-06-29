# mc-compatibility Change Spec: Stevenarella model core

## Requirements

### Requirement: Stevenarella model core

r[mc_compatibility.stevenarella_model.model_core] Stevenarella model code SHOULD expose pure cores for resource reference parsing, model path normalization, blockstate variant selection, multipart rule evaluation, model inheritance decisions, biome and light calculations, and vertex planning.

#### Scenario: Model decision is explicit

r[mc_compatibility.stevenarella_model.model_core.explicit]
- GIVEN resource identifiers, blockstate facts, model summaries, or lighting inputs
- WHEN model logic needs a path, selected variant, rule outcome, inherited model, or vertex plan
- THEN the decision is produced by a pure core over explicit inputs.

### Requirement: Stevenarella model shell boundary

r[mc_compatibility.stevenarella_model.model_shell_boundary] Model-core extraction MUST keep resource reads, JSON decoding, texture lookup, random source selection, renderer allocation, and logging outside pure model cores.

#### Scenario: Model side effects remain in shell

r[mc_compatibility.stevenarella_model.model_shell_boundary.effects]
- GIVEN the model core returns a reference, rule, geometry, or lighting plan
- WHEN the model shell applies that plan
- THEN only the shell performs resource access, decoding, texture lookup, random selection, renderer allocation, or logging.

### Requirement: Stevenarella model parity

r[mc_compatibility.stevenarella_model.parity] Model-core extraction MUST preserve current model/resource behavior, blockstate selection semantics, lighting and biome behavior, public model types, and evidence non-claims.

#### Scenario: Model behavior remains stable

r[mc_compatibility.stevenarella_model.parity.stable]
- GIVEN a supported pre-refactor model or blockstate input
- WHEN the extracted model core and shell process the same input
- THEN resource selection, model selection, geometry planning, light/biome behavior, and public type behavior remain equivalent.

### Requirement: Stevenarella model positive tests

r[mc_compatibility.stevenarella_model.positive_tests] The change MUST include positive tests for resource references, absolute and relative model paths, blockstate resources, parent resources, multipart matches, light and biome calculations, and vertex plans.

#### Scenario: Supported model paths pass

r[mc_compatibility.stevenarella_model.positive_tests.coverage]
- GIVEN representative supported model inputs
- WHEN extracted model cores process them
- THEN tests prove the expected references, paths, selections, calculations, or vertex plans are produced.

### Requirement: Stevenarella model negative tests

r[mc_compatibility.stevenarella_model.negative_tests] The change MUST include negative tests for malformed references, unsafe paths, missing parents, invalid multipart rules, unknown builtins, invalid face data, and unsupported tint or light inputs.

#### Scenario: Invalid model paths fail closed

r[mc_compatibility.stevenarella_model.negative_tests.fail_closed]
- GIVEN invalid model inputs
- WHEN extracted model cores process them
- THEN tests prove the inputs are rejected, defaulted, or contained according to current behavior.

### Requirement: Stevenarella model validation

r[mc_compatibility.stevenarella_model.validation] The change MUST record focused Stevenarella model/render tests, affected dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.stevenarella_model.validation.logs]
- GIVEN model-core extraction is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative model tests plus affected dry-runs and Cairn gates passing.
