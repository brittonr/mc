# Tasks

- [ ] [serial] Review Hyperion proxy contracts and Valence direct networking surfaces, then record proxy-mode scope and non-goals. r[valence_hyperion_integration.proxy_broadcast.scope]
- [ ] [depends:scope] Specify proxy messages, ordering, backpressure, stream lifecycle, subscription, and shutdown semantics. r[valence_hyperion_integration.proxy_broadcast.contract]
- [ ] [depends:contract] Implement or prototype a pure routing/planning core with positive and negative fixtures for global, local, channel, unicast, exclusion, stale state, and malformed-message cases. r[valence_hyperion_integration.proxy_broadcast.routing_core]
- [ ] [depends:routing_core] Wire an optional Valence proxy backend or plugin without changing direct-mode defaults. r[valence_hyperion_integration.proxy_broadcast.valence_backend]
- [ ] [depends:valence_backend] Run direct-mode regressions, proxy-mode smoke tests, malformed-message rejection, selected mc-compat dry runs, and record reviewable evidence. r[valence_hyperion_integration.proxy_broadcast.evidence]
- [ ] [depends:evidence] Run Cairn proposal/design/tasks gates and Cairn validation with reviewable logs. r[valence_hyperion_integration.proxy_broadcast.validation]
