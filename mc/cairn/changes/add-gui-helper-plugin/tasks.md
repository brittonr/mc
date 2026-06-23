# Tasks

- [ ] [serial] Review Hyperion GUI helper code and Valence inventory/window behavior, then define GUI helper scope and non-goals. r[valence_hyperion_integration.gui_helper.scope]
- [ ] [depends:scope] Define a GUI model for windows, slots, readonly behavior, click outcomes, close events, and lifecycle cleanup. r[valence_hyperion_integration.gui_helper.model]
- [ ] [depends:model] Implement pure GUI transition helpers with thin ECS/event shells over `valence_inventory`. r[valence_hyperion_integration.gui_helper.core]
- [ ] [depends:core] Add positive and negative tests for open, click, readonly slots, stale window IDs, invalid slots, close events, disconnect cleanup, and plugin-disabled behavior. r[valence_hyperion_integration.gui_helper.tests]
- [ ] [depends:tests] Add examples/docs for common menus without claiming full vanilla container parity. r[valence_hyperion_integration.gui_helper.docs]
- [ ] [depends:docs] Run GUI tests, inventory integration tests, examples/playground smoke tests, selected inventory mc-compat dry runs, Cairn gates, and Cairn validation. r[valence_hyperion_integration.gui_helper.validation]
