# Tasks

- [ ] [serial] Review Hyperion command ergonomics and Valence command graph APIs, then record reference-only scope and non-goals. r[valence_hyperion_integration.command_ergonomics.scope]
- [ ] [depends:scope] Define optional derive or builder semantics for literals, arguments, parsers, suggestions, scopes, handlers, diagnostics, and manual fallback. r[valence_hyperion_integration.command_ergonomics.contract]
- [ ] [depends:contract] Prototype helper output as inspectable Valence command graph data without replacing command internals. r[valence_hyperion_integration.command_ergonomics.prototype]
- [ ] [depends:prototype] Add positive and negative tests for generated graphs, manual graph parity, parser errors, duplicate literals, missing handlers, invalid scopes, suggestions, and plugin-disabled behavior. r[valence_hyperion_integration.command_ergonomics.tests]
- [ ] [depends:tests] Document helper usage, diagnostics, limitations, and when manual graph construction remains preferred. r[valence_hyperion_integration.command_ergonomics.docs]
- [ ] [depends:docs] Run macro/builder tests, command graph parity tests, parser/suggestion fixtures, Cairn gates, and Cairn validation. r[valence_hyperion_integration.command_ergonomics.validation]
