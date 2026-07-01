//! Optional profile and skin cache helpers for login-adjacent workflows.
//!
//! This module is intentionally inert by default. It provides a Valence-owned
//! functional core for provider response parsing, cache freshness decisions,
//! and request-budget admission, plus small adapter traits for HTTP providers
//! and storage shells.

use std::collections::HashMap;
use std::sync::Mutex;

use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;
use valence_protocol::profile::Property;

const PROFILE_ID_FIELD: &str = "id";
const PROFILE_NAME_FIELD: &str = "name";
const PROFILE_PROPERTIES_FIELD: &str = "properties";
const PROFILE_PROPERTY_NAME_FIELD: &str = "name";
const PROFILE_PROPERTY_VALUE_FIELD: &str = "value";
const PROFILE_PROPERTY_SIGNATURE_FIELD: &str = "signature";
const PROVIDER_ERROR_FIELD: &str = "error";
const PROFILE_ID_TEMPLATE: &str = "{profile_id}";
const DEFAULT_REQUESTS_PER_WINDOW: u32 = 1;
const DEFAULT_BUDGET_WINDOW_SECONDS: u64 = 60;
const DEFAULT_FRESH_TTL_SECONDS: u64 = 3_600;
const DEFAULT_STALE_TTL_SECONDS: u64 = 86_400;
const DEFAULT_RETENTION_SECONDS: u64 = 604_800;
const INITIAL_USED_REQUESTS: u32 = 0;

/// Optional profile-cache configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileCacheConfig {
    /// Enables cache/provider resolution when true.
    pub enabled: bool,
    /// Explicit provider configuration. Required when [`Self::enabled`] is
    /// true.
    pub provider: Option<ProfileProviderConfig>,
    /// Request budget used before provider calls.
    pub request_budget: RequestBudgetConfig,
    /// Storage backend selected by the application.
    pub cache_backend: CacheBackendConfig,
    /// Cache freshness and stale-serve policy.
    pub ttl: ProfileCacheTtl,
    /// Fallback behavior for offline providers and corrupted cache entries.
    pub fallback: OfflineFallbackPolicy,
    /// Privacy and retention policy for stored profile data.
    pub privacy: PrivacyRetentionConfig,
}

impl ProfileCacheConfig {
    /// Returns a disabled configuration with no provider selected.
    #[must_use]
    pub fn disabled() -> Self {
        Self::default()
    }

    /// Validates the configuration without performing I/O.
    pub fn validate(&self) -> Result<(), ProfileCacheConfigError> {
        validate_profile_cache_config(self)
    }
}

impl Default for ProfileCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: None,
            request_budget: RequestBudgetConfig::default(),
            cache_backend: CacheBackendConfig::Disabled,
            ttl: ProfileCacheTtl::default(),
            fallback: OfflineFallbackPolicy::FailClosed,
            privacy: PrivacyRetentionConfig::default(),
        }
    }
}

/// Provider endpoint configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileProviderConfig {
    /// Human-readable provider name used in diagnostics.
    pub name: String,
    /// URL template used for signed profile lookups by UUID.
    ///
    /// The template must contain `{profile_id}`. Applications choose the
    /// endpoint; this module does not install a default third-party provider.
    pub signed_profile_url_template: String,
}

impl ProfileProviderConfig {
    /// Builds a signed profile URL for a UUID.
    #[must_use]
    pub fn signed_profile_url(&self, profile_id: Uuid) -> String {
        self.signed_profile_url_template
            .replace(PROFILE_ID_TEMPLATE, &profile_id.to_string())
    }
}

/// Request-budget configuration for provider calls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestBudgetConfig {
    /// Maximum allowed requests in one window.
    pub max_requests: u32,
    /// Window length in seconds.
    pub window_seconds: u64,
}

impl Default for RequestBudgetConfig {
    fn default() -> Self {
        Self {
            max_requests: DEFAULT_REQUESTS_PER_WINDOW,
            window_seconds: DEFAULT_BUDGET_WINDOW_SECONDS,
        }
    }
}

/// Cache backend selected by the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CacheBackendConfig {
    /// Do not read or write a cache.
    Disabled,
    /// Use a process-local memory cache supplied by the application.
    Memory,
    /// Use an application-owned backend. Paths, names, and retention stay
    /// explicit so this module does not hard-code storage locations.
    Custom {
        /// Backend identifier for diagnostics.
        name: String,
        /// Optional application-owned storage path.
        storage_path: Option<std::path::PathBuf>,
    },
}

impl CacheBackendConfig {
    fn is_enabled(&self) -> bool {
        !matches!(self, Self::Disabled)
    }
}

/// Cache freshness policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProfileCacheTtl {
    /// Seconds after storage during which an entry is fresh.
    pub fresh_seconds: u64,
    /// Additional seconds during which stale entries may be used by fallback
    /// policy when provider lookup fails or is blocked.
    pub stale_seconds: u64,
}

impl Default for ProfileCacheTtl {
    fn default() -> Self {
        Self {
            fresh_seconds: DEFAULT_FRESH_TTL_SECONDS,
            stale_seconds: DEFAULT_STALE_TTL_SECONDS,
        }
    }
}

/// Offline or corruption fallback policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfflineFallbackPolicy {
    /// Fail the lookup when fresh data is unavailable.
    FailClosed,
    /// Serve a stale cache entry within the configured stale window.
    UseStaleCache,
    /// Ignore corrupted cache data and attempt the provider when budget allows.
    RequestProviderOnCacheError,
}

/// Privacy and data-retention controls for cache entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrivacyRetentionConfig {
    /// Store profile UUIDs.
    pub store_profile_id: bool,
    /// Store current profile names.
    pub store_username: bool,
    /// Store signed texture properties when present.
    pub store_texture_properties: bool,
    /// Maximum retention duration for stored entries, in seconds.
    pub retention_seconds: u64,
    /// Delete cache entries when the feature is disabled by the application.
    pub delete_on_disable: bool,
}

impl Default for PrivacyRetentionConfig {
    fn default() -> Self {
        Self {
            store_profile_id: true,
            store_username: true,
            store_texture_properties: true,
            retention_seconds: DEFAULT_RETENTION_SECONDS,
            delete_on_disable: true,
        }
    }
}

/// Deterministic configuration diagnostics.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ProfileCacheConfigError {
    /// The feature is enabled without provider configuration.
    #[error("profile cache is enabled without provider configuration")]
    MissingProvider,
    /// The provider name is empty.
    #[error("profile provider name is empty")]
    EmptyProviderName,
    /// The provider URL template is empty.
    #[error("profile provider URL template is empty")]
    EmptyProviderTemplate,
    /// The provider URL template lacks the required profile-id placeholder.
    #[error("profile provider URL template must contain {PROFILE_ID_TEMPLATE}")]
    MissingProfileIdTemplate,
    /// The request limit is zero.
    #[error("profile request budget max_requests must be greater than zero")]
    EmptyRequestBudget,
    /// The request-budget window is zero seconds.
    #[error("profile request budget window_seconds must be greater than zero")]
    EmptyRequestWindow,
    /// The fresh and stale TTLs are both zero.
    #[error("profile cache TTL policy must keep fresh or stale entries for a positive duration")]
    EmptyTtl,
    /// The retention window is zero seconds while caching is enabled.
    #[error("profile cache retention_seconds must be greater than zero when storage is enabled")]
    EmptyRetention,
}

/// Parsed profile data safe to store in a cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CachedProfile {
    /// Profile UUID returned by the provider.
    pub profile_id: Uuid,
    /// Profile name returned by the provider.
    pub username: String,
    /// Profile properties, including `textures` when present.
    pub properties: Vec<Property>,
}

/// Cache entry with explicit storage time.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileCacheEntry {
    /// Cached profile data.
    pub profile: CachedProfile,
    /// Storage time as Unix seconds, supplied by the shell.
    pub stored_at_unix_seconds: u64,
}

/// Cache status for an entry at a given time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacheEntryStatus {
    /// The entry is inside the fresh TTL.
    Fresh,
    /// The entry is outside the fresh TTL but inside the fallback stale TTL.
    Stale,
    /// The entry is outside all configured TTL windows.
    Expired,
}

/// Provider response parsing errors.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum ProfileParseError {
    /// The response was not JSON.
    #[error("profile response is not valid JSON")]
    InvalidJson,
    /// The provider returned an error object.
    #[error("profile provider error: {0}")]
    ProviderError(String),
    /// The response omitted the profile identifier.
    #[error("profile response is missing id")]
    MissingProfileId,
    /// The profile identifier was not a string.
    #[error("profile response id is not a string")]
    ProfileIdNotString,
    /// The profile identifier was not a valid UUID.
    #[error("profile response id is not a valid UUID")]
    InvalidProfileId,
    /// The response omitted the profile name.
    #[error("profile response is missing name")]
    MissingProfileName,
    /// The profile name was not a string.
    #[error("profile response name is not a string")]
    ProfileNameNotString,
    /// The properties field was malformed.
    #[error("profile response properties are malformed")]
    MalformedProperties,
}

/// Rate-limit state for a fixed request-budget window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RateLimitWindow {
    /// Start of the active budget window as Unix seconds.
    pub started_at_unix_seconds: u64,
    /// Provider requests used during the active window.
    pub used_requests: u32,
}

impl RateLimitWindow {
    /// Creates an empty request window at the supplied time.
    #[must_use]
    pub const fn new(started_at_unix_seconds: u64) -> Self {
        Self {
            started_at_unix_seconds,
            used_requests: INITIAL_USED_REQUESTS,
        }
    }
}

/// Rate-limit admission decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateLimitAdmission {
    /// The request may proceed with the returned updated window.
    Allowed(RateLimitWindow),
    /// The request is blocked until the retry delay elapses.
    Exhausted {
        /// Unchanged active window.
        window: RateLimitWindow,
        /// Seconds until a new provider request can be attempted.
        retry_after_seconds: u64,
    },
}

/// Profile lookup request.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProfileLookupRequest {
    /// UUID to fetch from the configured provider.
    pub profile_id: Uuid,
}

/// Provider fetch request passed to HTTP shells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileFetchRequest {
    /// UUID being fetched.
    pub profile_id: Uuid,
    /// Fully formatted provider URL.
    pub url: String,
}

/// Provider adapter error.
#[derive(Debug, Error)]
pub enum ProfileProviderError {
    /// HTTP or transport failure.
    #[error("profile provider request failed: {0}")]
    Request(String),
}

/// Cache store read outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileCacheLoad {
    /// A valid entry exists.
    Hit(ProfileCacheEntry),
    /// No entry exists for the profile.
    Miss,
    /// Stored data could not be decoded.
    Corrupted(String),
}

/// Cache store adapter error.
#[derive(Debug, Error)]
pub enum ProfileCacheStoreError {
    /// Storage failed while loading data.
    #[error("profile cache load failed: {0}")]
    Load(String),
    /// Storage failed while saving data.
    #[error("profile cache store failed: {0}")]
    Store(String),
}

/// End-to-end profile lookup result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProfileLookupOutcome {
    /// The optional cache feature is disabled.
    Disabled,
    /// Configuration failed before any provider request was attempted.
    ConfigRejected(ProfileCacheConfigError),
    /// A fresh cache entry satisfied the lookup.
    CacheHit(CachedProfile),
    /// A stale cache entry satisfied the lookup according to fallback policy.
    StaleCacheHit(CachedProfile),
    /// A provider response supplied the profile.
    Fetched(CachedProfile),
    /// Request budget blocked the provider request.
    RateLimited { retry_after_seconds: u64 },
    /// The cache entry was corrupted and fallback policy did not allow lookup.
    CacheCorrupted { diagnostic: String },
    /// The provider request failed.
    ProviderFailed { diagnostic: String },
    /// The provider response could not be parsed.
    ParseFailed(ProfileParseError),
    /// The cache store failed.
    StoreFailed { diagnostic: String },
}

/// Profile lookup result plus updated request-budget window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileLookupReport {
    /// Lookup outcome.
    pub outcome: ProfileLookupOutcome,
    /// Updated or unchanged rate-limit window.
    pub rate_limit_window: RateLimitWindow,
}

/// Adapter trait for profile providers.
#[async_trait]
pub trait ProfileProvider: Send + Sync {
    /// Fetches a raw provider response for a profile.
    async fn fetch_profile(
        &self,
        request: &ProfileFetchRequest,
    ) -> Result<String, ProfileProviderError>;
}

/// Adapter trait for cache storage.
#[async_trait]
pub trait ProfileCacheStore: Send + Sync {
    /// Loads a cached profile entry.
    async fn load_profile(
        &self,
        profile_id: Uuid,
    ) -> Result<ProfileCacheLoad, ProfileCacheStoreError>;

    /// Stores a profile entry.
    async fn store_profile(&self, entry: ProfileCacheEntry) -> Result<(), ProfileCacheStoreError>;
}

/// Reqwest-backed provider shell.
#[derive(Clone, Debug)]
pub struct HttpProfileProvider {
    client: reqwest::Client,
}

impl HttpProfileProvider {
    /// Creates an HTTP provider shell from an application-owned client.
    #[must_use]
    pub const fn new(client: reqwest::Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProfileProvider for HttpProfileProvider {
    async fn fetch_profile(
        &self,
        request: &ProfileFetchRequest,
    ) -> Result<String, ProfileProviderError> {
        self.client
            .get(&request.url)
            .send()
            .await
            .map_err(|err| ProfileProviderError::Request(err.to_string()))?
            .text()
            .await
            .map_err(|err| ProfileProviderError::Request(err.to_string()))
    }
}

/// In-memory cache store shell for applications and tests.
#[derive(Default, Debug)]
pub struct MemoryProfileCacheStore {
    entries: Mutex<HashMap<Uuid, MemoryProfileCacheSlot>>,
}

impl MemoryProfileCacheStore {
    /// Inserts a corrupted entry for fail-closed testing or diagnostics.
    pub fn insert_corrupted<D>(&self, profile_id: Uuid, diagnostic: D)
    where
        D: Into<String>,
    {
        let mut entries = self
            .entries
            .lock()
            .expect("profile cache mutex not poisoned");
        entries.insert(
            profile_id,
            MemoryProfileCacheSlot::Corrupted(diagnostic.into()),
        );
    }

    /// Returns true when the profile exists in the in-memory store.
    #[must_use]
    pub fn contains_profile(&self, profile_id: Uuid) -> bool {
        let entries = self
            .entries
            .lock()
            .expect("profile cache mutex not poisoned");
        entries.contains_key(&profile_id)
    }
}

#[async_trait]
impl ProfileCacheStore for MemoryProfileCacheStore {
    async fn load_profile(
        &self,
        profile_id: Uuid,
    ) -> Result<ProfileCacheLoad, ProfileCacheStoreError> {
        let entries = self
            .entries
            .lock()
            .expect("profile cache mutex not poisoned");
        Ok(match entries.get(&profile_id) {
            Some(MemoryProfileCacheSlot::Entry(entry)) => ProfileCacheLoad::Hit(entry.clone()),
            Some(MemoryProfileCacheSlot::Corrupted(diagnostic)) => {
                ProfileCacheLoad::Corrupted(diagnostic.clone())
            }
            None => ProfileCacheLoad::Miss,
        })
    }

    async fn store_profile(&self, entry: ProfileCacheEntry) -> Result<(), ProfileCacheStoreError> {
        let mut entries = self
            .entries
            .lock()
            .expect("profile cache mutex not poisoned");
        entries.insert(
            entry.profile.profile_id,
            MemoryProfileCacheSlot::Entry(entry.clone()),
        );
        Ok(())
    }
}

#[derive(Clone, Debug)]
enum MemoryProfileCacheSlot {
    Entry(ProfileCacheEntry),
    Corrupted(String),
}

/// Validates profile-cache configuration without I/O.
pub fn validate_profile_cache_config(
    config: &ProfileCacheConfig,
) -> Result<(), ProfileCacheConfigError> {
    if !config.enabled {
        return Ok(());
    }
    if config.request_budget.max_requests == INITIAL_USED_REQUESTS {
        return Err(ProfileCacheConfigError::EmptyRequestBudget);
    }
    if config.request_budget.window_seconds == u64::from(INITIAL_USED_REQUESTS) {
        return Err(ProfileCacheConfigError::EmptyRequestWindow);
    }
    if config.ttl.fresh_seconds == u64::from(INITIAL_USED_REQUESTS)
        && config.ttl.stale_seconds == u64::from(INITIAL_USED_REQUESTS)
    {
        return Err(ProfileCacheConfigError::EmptyTtl);
    }
    if config.cache_backend.is_enabled()
        && config.privacy.retention_seconds == u64::from(INITIAL_USED_REQUESTS)
    {
        return Err(ProfileCacheConfigError::EmptyRetention);
    }

    let provider = config
        .provider
        .as_ref()
        .ok_or(ProfileCacheConfigError::MissingProvider)?;
    if provider.name.is_empty() {
        return Err(ProfileCacheConfigError::EmptyProviderName);
    }
    if provider.signed_profile_url_template.is_empty() {
        return Err(ProfileCacheConfigError::EmptyProviderTemplate);
    }
    if !provider
        .signed_profile_url_template
        .contains(PROFILE_ID_TEMPLATE)
    {
        return Err(ProfileCacheConfigError::MissingProfileIdTemplate);
    }

    Ok(())
}

/// Parses a provider profile response into cache-safe data.
pub fn parse_profile_response(body: &str) -> Result<CachedProfile, ProfileParseError> {
    let value: Value = serde_json::from_str(body).map_err(|_| ProfileParseError::InvalidJson)?;

    if let Some(error) = value.get(PROVIDER_ERROR_FIELD) {
        return Err(ProfileParseError::ProviderError(provider_error_text(error)));
    }

    let profile_id = required_string(
        &value,
        PROFILE_ID_FIELD,
        ProfileParseError::MissingProfileId,
    )
    .and_then(parse_profile_id)?;
    let username = required_string(
        &value,
        PROFILE_NAME_FIELD,
        ProfileParseError::MissingProfileName,
    )?
    .to_owned();
    let properties = parse_profile_properties(value.get(PROFILE_PROPERTIES_FIELD))?;

    Ok(CachedProfile {
        profile_id,
        username,
        properties,
    })
}

/// Returns the freshness status for a cache entry.
#[must_use]
pub fn cache_entry_status(
    now_unix_seconds: u64,
    entry: &ProfileCacheEntry,
    ttl: ProfileCacheTtl,
) -> CacheEntryStatus {
    let age_seconds = now_unix_seconds.saturating_sub(entry.stored_at_unix_seconds);
    if age_seconds <= ttl.fresh_seconds {
        return CacheEntryStatus::Fresh;
    }

    let stale_deadline = ttl.fresh_seconds.saturating_add(ttl.stale_seconds);
    if age_seconds <= stale_deadline {
        return CacheEntryStatus::Stale;
    }

    CacheEntryStatus::Expired
}

/// Applies a fixed-window request budget to a provider request.
#[must_use]
pub fn admit_profile_request(
    budget: RequestBudgetConfig,
    now_unix_seconds: u64,
    window: RateLimitWindow,
) -> RateLimitAdmission {
    let window_deadline = window
        .started_at_unix_seconds
        .saturating_add(budget.window_seconds);
    let active_window = if now_unix_seconds >= window_deadline {
        RateLimitWindow::new(now_unix_seconds)
    } else {
        window
    };

    if active_window.used_requests < budget.max_requests {
        return RateLimitAdmission::Allowed(RateLimitWindow {
            started_at_unix_seconds: active_window.started_at_unix_seconds,
            used_requests: active_window.used_requests.saturating_add(1),
        });
    }

    RateLimitAdmission::Exhausted {
        window: active_window,
        retry_after_seconds: window_deadline.saturating_sub(now_unix_seconds),
    }
}

/// Resolves a profile through cache and provider adapters.
pub async fn resolve_profile_with_cache<P, S>(
    config: &ProfileCacheConfig,
    request: ProfileLookupRequest,
    now_unix_seconds: u64,
    rate_limit_window: RateLimitWindow,
    provider: &P,
    store: &S,
) -> ProfileLookupReport
where
    P: ProfileProvider,
    S: ProfileCacheStore,
{
    if !config.enabled {
        return ProfileLookupReport {
            outcome: ProfileLookupOutcome::Disabled,
            rate_limit_window,
        };
    }
    if let Err(err) = config.validate() {
        return ProfileLookupReport {
            outcome: ProfileLookupOutcome::ConfigRejected(err),
            rate_limit_window,
        };
    }

    let stale_entry =
        match load_cache_entry(config, request.profile_id, now_unix_seconds, store).await {
            CacheProbe::Fresh(profile) => {
                return ProfileLookupReport {
                    outcome: ProfileLookupOutcome::CacheHit(profile),
                    rate_limit_window,
                };
            }
            CacheProbe::Stale(entry) => Some(entry),
            CacheProbe::Miss => None,
            CacheProbe::Corrupted(diagnostic) => {
                if config.fallback != OfflineFallbackPolicy::RequestProviderOnCacheError {
                    return ProfileLookupReport {
                        outcome: ProfileLookupOutcome::CacheCorrupted { diagnostic },
                        rate_limit_window,
                    };
                }
                None
            }
            CacheProbe::StoreFailed(diagnostic) => {
                return ProfileLookupReport {
                    outcome: ProfileLookupOutcome::StoreFailed { diagnostic },
                    rate_limit_window,
                };
            }
        };

    let admission =
        admit_profile_request(config.request_budget, now_unix_seconds, rate_limit_window);
    let updated_window = match admission {
        RateLimitAdmission::Allowed(window) => window,
        RateLimitAdmission::Exhausted {
            retry_after_seconds,
            window,
        } => {
            return ProfileLookupReport {
                outcome: stale_entry
                    .filter(|_| config.fallback == OfflineFallbackPolicy::UseStaleCache)
                    .map_or(
                        ProfileLookupOutcome::RateLimited {
                            retry_after_seconds,
                        },
                        |entry| ProfileLookupOutcome::StaleCacheHit(entry.profile),
                    ),
                rate_limit_window: window,
            };
        }
    };

    let provider_config = config
        .provider
        .as_ref()
        .expect("validated config contains provider");
    let fetch_request = ProfileFetchRequest {
        profile_id: request.profile_id,
        url: provider_config.signed_profile_url(request.profile_id),
    };
    let response = match provider.fetch_profile(&fetch_request).await {
        Ok(response) => response,
        Err(err) => {
            return ProfileLookupReport {
                outcome: stale_entry
                    .filter(|_| config.fallback == OfflineFallbackPolicy::UseStaleCache)
                    .map_or(
                        ProfileLookupOutcome::ProviderFailed {
                            diagnostic: err.to_string(),
                        },
                        |entry| ProfileLookupOutcome::StaleCacheHit(entry.profile),
                    ),
                rate_limit_window: updated_window,
            };
        }
    };

    let profile = match parse_profile_response(&response) {
        Ok(profile) => profile,
        Err(err) => {
            return ProfileLookupReport {
                outcome: ProfileLookupOutcome::ParseFailed(err),
                rate_limit_window: updated_window,
            };
        }
    };

    if config.cache_backend.is_enabled() {
        let entry = ProfileCacheEntry {
            profile: profile.clone(),
            stored_at_unix_seconds: now_unix_seconds,
        };
        if let Err(err) = store.store_profile(entry).await {
            return ProfileLookupReport {
                outcome: ProfileLookupOutcome::StoreFailed {
                    diagnostic: err.to_string(),
                },
                rate_limit_window: updated_window,
            };
        }
    }

    ProfileLookupReport {
        outcome: ProfileLookupOutcome::Fetched(profile),
        rate_limit_window: updated_window,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum CacheProbe {
    Fresh(CachedProfile),
    Stale(ProfileCacheEntry),
    Miss,
    Corrupted(String),
    StoreFailed(String),
}

async fn load_cache_entry<S>(
    config: &ProfileCacheConfig,
    profile_id: Uuid,
    now_unix_seconds: u64,
    store: &S,
) -> CacheProbe
where
    S: ProfileCacheStore,
{
    if !config.cache_backend.is_enabled() {
        return CacheProbe::Miss;
    }

    match store.load_profile(profile_id).await {
        Ok(ProfileCacheLoad::Hit(entry)) => {
            match cache_entry_status(now_unix_seconds, &entry, config.ttl) {
                CacheEntryStatus::Fresh => CacheProbe::Fresh(entry.profile),
                CacheEntryStatus::Stale => CacheProbe::Stale(entry),
                CacheEntryStatus::Expired => CacheProbe::Miss,
            }
        }
        Ok(ProfileCacheLoad::Miss) => CacheProbe::Miss,
        Ok(ProfileCacheLoad::Corrupted(diagnostic)) => CacheProbe::Corrupted(diagnostic),
        Err(err) => CacheProbe::StoreFailed(err.to_string()),
    }
}

fn parse_profile_id(id: &str) -> Result<Uuid, ProfileParseError> {
    Uuid::parse_str(id).map_err(|_| ProfileParseError::InvalidProfileId)
}

fn required_string<'a>(
    value: &'a Value,
    field: &str,
    missing_error: ProfileParseError,
) -> Result<&'a str, ProfileParseError> {
    let Some(field_value) = value.get(field) else {
        return Err(missing_error);
    };

    field_value.as_str().ok_or(match field {
        PROFILE_ID_FIELD => ProfileParseError::ProfileIdNotString,
        PROFILE_NAME_FIELD => ProfileParseError::ProfileNameNotString,
        _ => ProfileParseError::MalformedProperties,
    })
}

fn parse_profile_properties(
    properties: Option<&Value>,
) -> Result<Vec<Property>, ProfileParseError> {
    let Some(properties) = properties else {
        return Ok(Vec::new());
    };
    let Some(properties) = properties.as_array() else {
        return Err(ProfileParseError::MalformedProperties);
    };

    let mut parsed = Vec::with_capacity(properties.len());
    for property in properties {
        let name = property
            .get(PROFILE_PROPERTY_NAME_FIELD)
            .and_then(Value::as_str)
            .ok_or(ProfileParseError::MalformedProperties)?;
        let value = property
            .get(PROFILE_PROPERTY_VALUE_FIELD)
            .and_then(Value::as_str)
            .ok_or(ProfileParseError::MalformedProperties)?;
        let signature = property
            .get(PROFILE_PROPERTY_SIGNATURE_FIELD)
            .map(|signature| {
                signature
                    .as_str()
                    .map(str::to_owned)
                    .ok_or(ProfileParseError::MalformedProperties)
            })
            .transpose()?;

        parsed.push(Property {
            name: name.to_owned(),
            value: value.to_owned(),
            signature,
        });
    }

    Ok(parsed)
}

fn provider_error_text(error: &Value) -> String {
    error
        .as_str()
        .map_or_else(|| error.to_string(), str::to_owned)
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "tests should panic with source locations"
)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use serde_json::json;

    use super::*;

    const PROFILE_UUID_HYPHENATED: &str = "86271406-1188-44a5-8496-7af10c906204";
    const PROFILE_UUID_COMPACT: &str = "86271406118844a584967af10c906204";
    const PROFILE_NAME: &str = "Emerald_Explorer";
    const PROVIDER_NAME: &str = "owned-fixture";
    const PROVIDER_TEMPLATE: &str = "https://profiles.invalid/session/{profile_id}?unsigned=false";
    const TEXTURE_PROPERTY_NAME: &str = "textures";
    const TEXTURE_VALUE: &str = "texture-payload";
    const TEXTURE_SIGNATURE: &str = "texture-signature";
    const NOW_SECONDS: u64 = 1_000;
    const FRESH_STORED_AT_SECONDS: u64 = 990;
    const STALE_STORED_AT_SECONDS: u64 = 900;
    const EXPIRED_STORED_AT_SECONDS: u64 = 500;
    const FRESH_TTL_SECONDS: u64 = 20;
    const STALE_TTL_SECONDS: u64 = 200;
    const BUDGET_WINDOW_SECONDS: u64 = 60;
    const ONE_REQUEST: u32 = 1;
    const USED_ONE_REQUEST: u32 = 1;
    const EXPECTED_RETRY_SECONDS: u64 = 60;

    #[test]
    fn parse_profile_response_accepts_valid_profile() {
        let profile = parse_profile_response(&valid_profile_body()).unwrap();
        let expected_id = Uuid::parse_str(PROFILE_UUID_HYPHENATED).unwrap();

        assert_eq!(profile.profile_id, expected_id);
        assert_eq!(profile.username, PROFILE_NAME);
        assert_eq!(profile.properties.len(), 1);
        assert_eq!(profile.properties[0].name, TEXTURE_PROPERTY_NAME);
        assert_eq!(profile.properties[0].value, TEXTURE_VALUE);
        assert_eq!(
            profile.properties[0].signature.as_deref(),
            Some(TEXTURE_SIGNATURE)
        );
    }

    #[test]
    fn parse_profile_response_rejects_missing_id() {
        let body = json!({ PROFILE_NAME_FIELD: PROFILE_NAME }).to_string();
        let err = parse_profile_response(&body).unwrap_err();

        assert_eq!(err, ProfileParseError::MissingProfileId);
    }

    #[test]
    fn parse_profile_response_rejects_provider_errors() {
        let body = json!({ PROVIDER_ERROR_FIELD: "TooManyRequests" }).to_string();
        let err = parse_profile_response(&body).unwrap_err();

        assert_eq!(
            err,
            ProfileParseError::ProviderError("TooManyRequests".to_owned())
        );
    }

    #[test]
    fn validate_profile_cache_config_requires_provider_when_enabled() {
        let config = ProfileCacheConfig {
            enabled: true,
            ..ProfileCacheConfig::default()
        };

        assert_eq!(
            config.validate().unwrap_err(),
            ProfileCacheConfigError::MissingProvider
        );
    }

    #[test]
    fn cache_entry_status_classifies_fresh_stale_and_expired_entries() {
        let ttl = test_ttl();
        let fresh = cache_entry(FRESH_STORED_AT_SECONDS);
        let stale = cache_entry(STALE_STORED_AT_SECONDS);
        let expired = cache_entry(EXPIRED_STORED_AT_SECONDS);

        assert_eq!(
            cache_entry_status(NOW_SECONDS, &fresh, ttl),
            CacheEntryStatus::Fresh
        );
        assert_eq!(
            cache_entry_status(NOW_SECONDS, &stale, ttl),
            CacheEntryStatus::Stale
        );
        assert_eq!(
            cache_entry_status(NOW_SECONDS, &expired, ttl),
            CacheEntryStatus::Expired
        );
    }

    #[test]
    fn admit_profile_request_rejects_exhausted_window() {
        let budget = RequestBudgetConfig {
            max_requests: ONE_REQUEST,
            window_seconds: BUDGET_WINDOW_SECONDS,
        };
        let window = RateLimitWindow {
            started_at_unix_seconds: NOW_SECONDS,
            used_requests: USED_ONE_REQUEST,
        };

        let admission = admit_profile_request(budget, NOW_SECONDS, window);

        assert_eq!(
            admission,
            RateLimitAdmission::Exhausted {
                window,
                retry_after_seconds: EXPECTED_RETRY_SECONDS,
            }
        );
    }

    #[tokio::test]
    async fn resolve_profile_returns_disabled_without_provider_request() {
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();
        let report = resolve_profile_with_cache(
            &ProfileCacheConfig::disabled(),
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(report.outcome, ProfileLookupOutcome::Disabled);
        assert_eq!(provider.call_count(), 0);
    }

    #[tokio::test]
    async fn resolve_profile_uses_fresh_cache_without_provider_request() {
        let config = enabled_config(OfflineFallbackPolicy::FailClosed);
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();
        store
            .store_profile(cache_entry(FRESH_STORED_AT_SECONDS))
            .await
            .unwrap();

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::CacheHit(test_profile())
        );
        assert_eq!(provider.call_count(), 0);
    }

    #[tokio::test]
    async fn resolve_profile_fetches_and_stores_cache_miss() {
        let config = enabled_config(OfflineFallbackPolicy::FailClosed);
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::Fetched(test_profile())
        );
        assert_eq!(provider.call_count(), 1);
        assert!(store.contains_profile(test_profile_id()));
    }

    #[tokio::test]
    async fn resolve_profile_blocks_request_when_rate_limit_exhausted() {
        let config = enabled_config(OfflineFallbackPolicy::FailClosed);
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();
        let exhausted = RateLimitWindow {
            started_at_unix_seconds: NOW_SECONDS,
            used_requests: USED_ONE_REQUEST,
        };

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            exhausted,
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::RateLimited {
                retry_after_seconds: EXPECTED_RETRY_SECONDS,
            }
        );
        assert_eq!(provider.call_count(), 0);
    }

    #[tokio::test]
    async fn resolve_profile_serves_stale_cache_when_provider_fails_and_policy_allows() {
        let config = enabled_config(OfflineFallbackPolicy::UseStaleCache);
        let provider = FakeProvider::err("offline");
        let store = MemoryProfileCacheStore::default();
        store
            .store_profile(cache_entry(STALE_STORED_AT_SECONDS))
            .await
            .unwrap();

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::StaleCacheHit(test_profile())
        );
        assert_eq!(provider.call_count(), 1);
    }

    #[tokio::test]
    async fn resolve_profile_fails_closed_on_corrupted_cache_by_default() {
        let config = enabled_config(OfflineFallbackPolicy::FailClosed);
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();
        store.insert_corrupted(test_profile_id(), "invalid serialized profile");

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::CacheCorrupted {
                diagnostic: "invalid serialized profile".to_owned(),
            }
        );
        assert_eq!(provider.call_count(), 0);
    }

    #[tokio::test]
    async fn resolve_profile_uses_provider_after_corruption_when_policy_allows() {
        let config = enabled_config(OfflineFallbackPolicy::RequestProviderOnCacheError);
        let provider = FakeProvider::ok(valid_profile_body());
        let store = MemoryProfileCacheStore::default();
        store.insert_corrupted(test_profile_id(), "invalid serialized profile");

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::Fetched(test_profile())
        );
        assert_eq!(provider.call_count(), 1);
    }

    #[tokio::test]
    async fn resolve_profile_does_not_store_malformed_provider_response() {
        let config = enabled_config(OfflineFallbackPolicy::FailClosed);
        let provider = FakeProvider::ok(json!({ PROFILE_NAME_FIELD: PROFILE_NAME }).to_string());
        let store = MemoryProfileCacheStore::default();

        let report = resolve_profile_with_cache(
            &config,
            test_request(),
            NOW_SECONDS,
            RateLimitWindow::new(NOW_SECONDS),
            &provider,
            &store,
        )
        .await;

        assert_eq!(
            report.outcome,
            ProfileLookupOutcome::ParseFailed(ProfileParseError::MissingProfileId)
        );
        assert!(!store.contains_profile(test_profile_id()));
    }

    fn valid_profile_body() -> String {
        json!({
            PROFILE_ID_FIELD: PROFILE_UUID_COMPACT,
            PROFILE_NAME_FIELD: PROFILE_NAME,
            PROFILE_PROPERTIES_FIELD: [{
                PROFILE_PROPERTY_NAME_FIELD: TEXTURE_PROPERTY_NAME,
                PROFILE_PROPERTY_VALUE_FIELD: TEXTURE_VALUE,
                PROFILE_PROPERTY_SIGNATURE_FIELD: TEXTURE_SIGNATURE,
            }],
        })
        .to_string()
    }

    fn enabled_config(fallback: OfflineFallbackPolicy) -> ProfileCacheConfig {
        ProfileCacheConfig {
            enabled: true,
            provider: Some(ProfileProviderConfig {
                name: PROVIDER_NAME.to_owned(),
                signed_profile_url_template: PROVIDER_TEMPLATE.to_owned(),
            }),
            request_budget: RequestBudgetConfig {
                max_requests: ONE_REQUEST,
                window_seconds: BUDGET_WINDOW_SECONDS,
            },
            cache_backend: CacheBackendConfig::Memory,
            ttl: test_ttl(),
            fallback,
            privacy: PrivacyRetentionConfig::default(),
        }
    }

    fn test_ttl() -> ProfileCacheTtl {
        ProfileCacheTtl {
            fresh_seconds: FRESH_TTL_SECONDS,
            stale_seconds: STALE_TTL_SECONDS,
        }
    }

    fn cache_entry(stored_at_unix_seconds: u64) -> ProfileCacheEntry {
        ProfileCacheEntry {
            profile: test_profile(),
            stored_at_unix_seconds,
        }
    }

    fn test_profile() -> CachedProfile {
        CachedProfile {
            profile_id: test_profile_id(),
            username: PROFILE_NAME.to_owned(),
            properties: vec![Property {
                name: TEXTURE_PROPERTY_NAME.to_owned(),
                value: TEXTURE_VALUE.to_owned(),
                signature: Some(TEXTURE_SIGNATURE.to_owned()),
            }],
        }
    }

    fn test_profile_id() -> Uuid {
        Uuid::parse_str(PROFILE_UUID_HYPHENATED).unwrap()
    }

    fn test_request() -> ProfileLookupRequest {
        ProfileLookupRequest {
            profile_id: test_profile_id(),
        }
    }

    struct FakeProvider {
        response: Result<String, String>,
        calls: AtomicUsize,
    }

    impl FakeProvider {
        fn ok(response: String) -> Self {
            Self {
                response: Ok(response),
                calls: AtomicUsize::new(0),
            }
        }

        fn err(diagnostic: impl Into<String>) -> Self {
            Self {
                response: Err(diagnostic.into()),
                calls: AtomicUsize::new(0),
            }
        }

        fn call_count(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl ProfileProvider for FakeProvider {
        async fn fetch_profile(
            &self,
            _request: &ProfileFetchRequest,
        ) -> Result<String, ProfileProviderError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.response.clone().map_err(ProfileProviderError::Request)
        }
    }
}
