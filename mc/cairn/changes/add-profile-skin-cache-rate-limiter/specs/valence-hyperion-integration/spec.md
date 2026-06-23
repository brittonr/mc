# valence-hyperion-integration Change Spec: Profile and skin cache with rate limiting

## Requirements

### Requirement: Profile cache scope

r[valence_hyperion_integration.profile_cache.scope] The integration MUST review Hyperion profile/cache code and Valence login/profile surfaces before adding profile cache behavior.

#### Scenario: Authentication boundary is recorded

r[valence_hyperion_integration.profile_cache.scope.auth_boundary]
- GIVEN profile cache work is selected
- WHEN reviewers inspect the scope notes
- THEN the notes identify which data is cached, which login/authentication behavior remains unchanged, and which provider/cache policies are out of scope.

### Requirement: Typed profile cache configuration

r[valence_hyperion_integration.profile_cache.config] Profile caching MUST define typed configuration for providers, request budgets, cache backends, TTLs, offline fallback, and privacy retention.

#### Scenario: Missing provider config fails before requests

r[valence_hyperion_integration.profile_cache.config.missing_provider]
- GIVEN profile lookup is enabled without a valid provider configuration
- WHEN the configuration validator runs
- THEN it returns a deterministic diagnostic
- AND no HTTP request is attempted.

### Requirement: Pure profile cache core

r[valence_hyperion_integration.profile_cache.core] Profile response parsing, cache decisions, staleness decisions, and rate-limit admission MUST be pure deterministic logic over explicit inputs.

#### Scenario: Missing profile id is rejected

r[valence_hyperion_integration.profile_cache.core.missing_id]
- GIVEN a provider response omits the profile identifier field
- WHEN the parser evaluates the response
- THEN it returns the documented parse error
- AND no cache entry is created.

### Requirement: Optional HTTP and storage adapters

r[valence_hyperion_integration.profile_cache.adapters] HTTP clients and cache storage MUST be optional adapters with explicit configuration and no hard-coded provider or storage path assumptions.

#### Scenario: Corrupted cache entry fails safely

r[valence_hyperion_integration.profile_cache.adapters.corrupt]
- GIVEN the configured cache backend returns a corrupted profile entry
- WHEN the adapter decodes it
- THEN it reports a deterministic corruption diagnostic
- AND the configured fallback policy determines whether lookup continues or fails.

### Requirement: Profile cache tests

r[valence_hyperion_integration.profile_cache.tests] Profile cache work MUST include positive and negative tests for provider parsing, rate limiting, cache state, storage errors, and disabled behavior.

#### Scenario: Rate limit exhaustion blocks request

r[valence_hyperion_integration.profile_cache.tests.rate_limit]
- GIVEN the configured request budget is exhausted
- WHEN a new profile lookup is requested
- THEN the rate-limit core rejects or delays the request according to policy
- AND the HTTP shell does not issue an immediate request.

### Requirement: Profile cache validation

r[valence_hyperion_integration.profile_cache.validation] Profile cache work MUST record parser/cache tests, fake-provider tests, storage corruption tests, login/profile smoke tests, and Cairn gates before archive.

#### Scenario: Profile cache closeout is reviewable

r[valence_hyperion_integration.profile_cache.validation.log]
- GIVEN profile cache work is ready to archive
- WHEN reviewers inspect evidence logs
- THEN logs show positive parser tests, negative malformed-provider tests, rate-limit tests, cache corruption tests, plugin-disabled checks, login/profile smoke output, and Cairn validation.
