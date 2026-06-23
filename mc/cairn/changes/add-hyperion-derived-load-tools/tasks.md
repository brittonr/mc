# Tasks

- [ ] [serial] Inventory Hyperion and Valence tool surfaces, classifying each concept as adopt, port, reference, or reject. r[valence_hyperion_integration.tools.inventory]
- [ ] [depends:inventory] Define typed config, target-safety rules, output contracts, redaction policy, and non-claim boundaries for load and packet tools. r[valence_hyperion_integration.tools.contract]
- [ ] [depends:contract] Add or adapt load-bot tooling with safe defaults, loopback smoke support, and structured failure reporting. r[valence_hyperion_integration.tools.load_bot]
- [ ] [depends:contract] Add or adapt packet-inspection tooling with bounded captures, malformed-capture handling, and redaction tests. r[valence_hyperion_integration.tools.packet_inspector]
- [ ] [depends:load_bot] [depends:packet_inspector] Document commands, evidence usage, and non-claims. r[valence_hyperion_integration.tools.docs]
- [ ] [depends:docs] Run config tests, malformed capture fixtures, loopback smoke tests, selected load dry runs, Cairn gates, and Cairn validation. r[valence_hyperion_integration.tools.validation]
