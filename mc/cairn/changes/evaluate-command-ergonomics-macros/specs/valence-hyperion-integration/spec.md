# valence-hyperion-integration Change Spec: Command ergonomics macros

## Requirements

### Requirement: Command ergonomics scope

r[valence_hyperion_integration.command_ergonomics.scope] The integration MUST review Hyperion command ergonomics and Valence command graph APIs before adding command helper macros or builders.

#### Scenario: Valence command ownership is preserved

r[valence_hyperion_integration.command_ergonomics.scope.ownership]
- GIVEN command ergonomics work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes state that Hyperion's command framework is reference-only
- AND Valence command graph, parser, scope, and handler internals remain authoritative.

### Requirement: Command helper contract

r[valence_hyperion_integration.command_ergonomics.contract] Optional command helpers SHOULD define semantics for literals, arguments, parsers, suggestions, scopes, handlers, diagnostics, and manual fallback.

#### Scenario: Duplicate literal is rejected

r[valence_hyperion_integration.command_ergonomics.contract.duplicate]
- GIVEN helper input defines duplicate command literals at the same graph level
- WHEN the helper validates or expands the command
- THEN it reports a deterministic diagnostic
- AND no ambiguous command graph is registered.

### Requirement: Inspectable command helper output

r[valence_hyperion_integration.command_ergonomics.prototype] Command helper output MUST be inspectable as Valence command graph data or equivalent testable structures.

#### Scenario: Helper graph matches manual graph

r[valence_hyperion_integration.command_ergonomics.prototype.parity]
- GIVEN a helper-defined command and an equivalent manually built command graph
- WHEN graph parity is checked
- THEN literals, arguments, parsers, executable nodes, scopes, and suggestions match the documented expected graph.

### Requirement: Command helper tests

r[valence_hyperion_integration.command_ergonomics.tests] Command ergonomics work MUST include positive and negative tests for generated graphs, manual parity, parser errors, duplicate literals, missing handlers, invalid scopes, suggestions, and disabled behavior.

#### Scenario: Missing handler fails clearly

r[valence_hyperion_integration.command_ergonomics.tests.missing_handler]
- GIVEN helper input defines an executable command without a valid handler
- WHEN the helper validates or expands it
- THEN it reports the missing handler with a deterministic diagnostic
- AND the incomplete command is not registered.

### Requirement: Command helper docs

r[valence_hyperion_integration.command_ergonomics.docs] Command helper documentation SHOULD explain usage, diagnostics, limitations, and when manual graph construction remains preferred.

#### Scenario: Docs avoid framework replacement claim

r[valence_hyperion_integration.command_ergonomics.docs.non_claim]
- GIVEN command helper docs are published
- WHEN reviewers inspect them
- THEN they describe helpers as optional ergonomics over Valence command APIs
- AND they do not claim replacement of the command graph internals.

### Requirement: Command ergonomics validation

r[valence_hyperion_integration.command_ergonomics.validation] Command ergonomics work MUST record macro or builder tests, graph parity tests, parser/suggestion fixtures, and Cairn gates before archive.

#### Scenario: Command ergonomics closeout is reviewable

r[valence_hyperion_integration.command_ergonomics.validation.log]
- GIVEN command ergonomics work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive helper tests, negative diagnostic tests, graph parity fixtures, parser and suggestion fixtures, plugin-disabled checks, and Cairn validation.
