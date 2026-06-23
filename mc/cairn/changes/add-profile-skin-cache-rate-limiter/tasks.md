# Tasks

- [ ] [serial] Review Hyperion Mojang/profile/cache code and Valence login/profile surfaces, then record profile-cache scope and non-goals. r[valence_hyperion_integration.profile_cache.scope]
- [ ] [depends:scope] Define typed provider, request-budget, cache-backend, TTL, offline fallback, and privacy-retention configuration. r[valence_hyperion_integration.profile_cache.config]
- [ ] [depends:config] Implement pure response parsing, cache hit/miss/staleness decisions, and rate-limit admission logic. r[valence_hyperion_integration.profile_cache.core]
- [ ] [depends:core] Wire HTTP and storage adapters as optional shells without hard-coded provider/path assumptions. r[valence_hyperion_integration.profile_cache.adapters]
- [ ] [depends:adapters] Add positive and negative fixtures for valid profiles, missing fields, provider errors, rate-limit exhaustion, cache hits/misses, stale entries, corrupted entries, and plugin-disabled behavior. r[valence_hyperion_integration.profile_cache.tests]
- [ ] [depends:tests] Run parser/cache tests, fake-provider tests, storage corruption tests, login/profile smoke tests, Cairn gates, and Cairn validation. r[valence_hyperion_integration.profile_cache.validation]
