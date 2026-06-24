//! Optional admin permission ergonomics for Valence commands.
//!
//! This module keeps permission decisions in a pure core over command metadata,
//! role bindings, explicit permission profiles, and command scopes. The
//! [`AdminPermissionPlugin`] is only a thin shell that maps a profile into
//! [`CommandScopes`], letting the existing command graph own visibility,
//! execution denial, and command-tree refresh.
//!
//! The storage boundary is intentionally just a typed row adapter and a trait.
//! Valence does not choose a database or moderation policy here.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{Component, Query, Res, Resource};

use crate::scopes::{CommandScopeRegistry, CommandScopes};

/// Optional plugin that maps [`AdminPermissionProfile`] into [`CommandScopes`].
///
/// Servers opt in by adding this plugin and attaching profiles to entities that
/// should be managed by the admin permission layer. If the plugin is absent, or
/// [`AdminPermissionSettings::state`] is disabled, existing command behavior is
/// preserved.
pub struct AdminPermissionPlugin;

impl Plugin for AdminPermissionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AdminPermissionSettings>()
            .init_resource::<AdminPermissionRoleBindings>()
            .add_systems(Update, sync_admin_permission_scopes);
    }
}

/// Runtime state for the optional admin permission shell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AdminPermissionPluginState {
    /// Apply admin profiles to command scopes.
    #[default]
    Enabled,
    /// Leave command scopes untouched.
    Disabled,
}

/// Settings for the optional admin permission shell.
#[derive(Clone, Debug, PartialEq, Eq, Resource)]
pub struct AdminPermissionSettings {
    /// Whether the shell updates command scopes.
    pub state: AdminPermissionPluginState,
}

impl Default for AdminPermissionSettings {
    fn default() -> Self {
        Self {
            state: AdminPermissionPluginState::Enabled,
        }
    }
}

/// Explicit permission profile for one command executor.
#[derive(Clone, Debug, Default, PartialEq, Eq, Component)]
pub struct AdminPermissionProfile {
    /// Moderation roles assigned by server policy.
    pub roles: BTreeSet<String>,
    /// Direct command scopes assigned without a role.
    pub scopes: BTreeSet<String>,
}

impl AdminPermissionProfile {
    /// Creates an empty profile.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a role to this profile.
    pub fn with_role<R: Into<String>>(mut self, role: R) -> Self {
        self.roles.insert(role.into());
        self
    }

    /// Adds a direct command scope to this profile.
    pub fn with_scope<S: Into<String>>(mut self, scope: S) -> Self {
        self.scopes.insert(scope.into());
        self
    }
}

/// Server-defined mapping from roles to command scopes.
#[derive(Clone, Debug, Default, PartialEq, Eq, Resource)]
pub struct AdminPermissionRoleBindings {
    role_scopes: BTreeMap<String, BTreeSet<String>>,
}

impl AdminPermissionRoleBindings {
    /// Assigns one scope to a role.
    pub fn bind_role_to_scope<R: Into<String>, S: Into<String>>(&mut self, role: R, scope: S) {
        self.role_scopes
            .entry(role.into())
            .or_default()
            .insert(scope.into());
    }

    /// Returns scopes assigned to a role, when the role is known.
    pub fn scopes_for_role(&self, role: &str) -> Option<&BTreeSet<String>> {
        self.role_scopes.get(role)
    }
}

/// Metadata needed to evaluate one command permission decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdminCommandMetadata {
    /// Human-readable command name used in diagnostics.
    pub name: String,
    /// Command scopes that can grant access. Matching uses Valence scope rules.
    pub required_scopes: Vec<String>,
}

impl AdminCommandMetadata {
    /// Creates metadata from a command name and required scopes.
    pub fn new<N, I, S>(name: N, required_scopes: I) -> Self
    where
        N: Into<String>,
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            name: name.into(),
            required_scopes: required_scopes.into_iter().map(Into::into).collect(),
        }
    }
}

/// Explicit context for a pure admin permission decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdminPermissionContext {
    /// Optional shell state for this evaluation.
    pub state: AdminPermissionPluginState,
    /// Loaded profile. `None` means permission data is missing.
    pub profile: Option<AdminPermissionProfile>,
}

impl AdminPermissionContext {
    /// Builds a context from an enabled profile.
    pub fn enabled(profile: AdminPermissionProfile) -> Self {
        Self {
            state: AdminPermissionPluginState::Enabled,
            profile: Some(profile),
        }
    }

    /// Builds a context for missing permission data.
    pub fn missing_profile() -> Self {
        Self {
            state: AdminPermissionPluginState::Enabled,
            profile: None,
        }
    }

    /// Builds a context for a disabled admin permission shell.
    pub fn disabled() -> Self {
        Self {
            state: AdminPermissionPluginState::Disabled,
            profile: None,
        }
    }
}

/// Result of a permission decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminPermissionDecision {
    /// The command is allowed by the evaluated permission data.
    Allowed,
    /// The command is denied with a diagnostic reason.
    Denied(AdminPermissionDenial),
    /// The admin permission layer is not participating in this decision.
    NotManaged(AdminPermissionBypass),
}

impl AdminPermissionDecision {
    /// Returns true when the decision allows the command.
    pub fn is_allowed(&self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Returns true when the decision denies the command.
    pub fn is_denied(&self) -> bool {
        matches!(self, Self::Denied(_))
    }
}

/// Reason the admin layer did not evaluate a command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminPermissionBypass {
    /// The optional plugin is disabled or absent.
    PluginDisabled,
}

/// Diagnostic reason for a denied command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminPermissionDenial {
    /// No permission profile was available for a scoped command.
    MissingPermissionData,
    /// The profile's effective scopes do not grant any required scope.
    MissingRequiredScope {
        /// Command scopes that would have allowed the command.
        required_scopes: Vec<String>,
        /// Effective scopes produced from roles and direct profile scopes.
        effective_scopes: Vec<String>,
    },
}

impl AdminPermissionDenial {
    /// Builds a stable user-facing diagnostic for a denied command.
    pub fn diagnostic(&self, command_name: &str) -> String {
        match self {
            Self::MissingPermissionData => {
                format!("command `{command_name}` denied: permission data is missing")
            }
            Self::MissingRequiredScope {
                required_scopes,
                effective_scopes,
            } => format!(
                "command `{command_name}` denied: requires one of [{}], effective scopes [{}]",
                required_scopes.join(", "),
                effective_scopes.join(", ")
            ),
        }
    }
}

/// Planned command-scope mutation for the optional plugin shell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AdminScopeUpdate {
    /// Leave existing command scopes untouched.
    Unchanged,
    /// Replace command scopes with the effective admin scopes.
    Replace(BTreeSet<String>),
}

/// Typed storage row accepted by the optional storage boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StoredAdminPermissionRow<'a> {
    /// Subject key from caller-owned storage.
    pub subject: &'a str,
    /// Role labels loaded from storage.
    pub roles: &'a [&'a str],
    /// Direct command scopes loaded from storage.
    pub scopes: &'a [&'a str],
}

/// Errors returned by the storage row adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdminPermissionStorageError {
    /// The subject key was empty.
    EmptySubject,
    /// A role label was empty.
    EmptyRole,
    /// A scope label was empty.
    EmptyScope,
}

impl Display for AdminPermissionStorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySubject => write!(f, "stored admin permission row has an empty subject"),
            Self::EmptyRole => write!(f, "stored admin permission row has an empty role"),
            Self::EmptyScope => write!(f, "stored admin permission row has an empty scope"),
        }
    }
}

impl std::error::Error for AdminPermissionStorageError {}

/// Optional storage boundary for server-owned permission persistence.
pub trait AdminPermissionStorage {
    /// Storage implementation error.
    type Error;

    /// Loads a profile for a subject. `Ok(None)` means the row is missing.
    fn load_profile(&self, subject: &str) -> Result<Option<AdminPermissionProfile>, Self::Error>;
}

/// Evaluates command metadata against explicit admin permission context.
pub fn evaluate_admin_command(
    metadata: &AdminCommandMetadata,
    context: &AdminPermissionContext,
    bindings: &AdminPermissionRoleBindings,
    registry: &CommandScopeRegistry,
) -> AdminPermissionDecision {
    if context.state == AdminPermissionPluginState::Disabled {
        return AdminPermissionDecision::NotManaged(AdminPermissionBypass::PluginDisabled);
    }

    if metadata.required_scopes.is_empty() {
        return AdminPermissionDecision::Allowed;
    }

    let Some(profile) = context.profile.as_ref() else {
        return AdminPermissionDecision::Denied(AdminPermissionDenial::MissingPermissionData);
    };

    let effective_scopes = effective_admin_scopes(profile, bindings);
    evaluate_command_scopes(registry, &effective_scopes, &metadata.required_scopes)
}

/// Evaluates required command scopes against already-effective command scopes.
pub fn evaluate_command_scopes(
    registry: &CommandScopeRegistry,
    effective_scopes: &BTreeSet<String>,
    required_scopes: &[String],
) -> AdminPermissionDecision {
    if required_scopes.is_empty() {
        return AdminPermissionDecision::Allowed;
    }

    for required_scope in required_scopes {
        for effective_scope in effective_scopes {
            if registry.grants(effective_scope, required_scope) {
                return AdminPermissionDecision::Allowed;
            }
        }
    }

    AdminPermissionDecision::Denied(AdminPermissionDenial::MissingRequiredScope {
        required_scopes: required_scopes.to_vec(),
        effective_scopes: effective_scopes.iter().cloned().collect(),
    })
}

/// Computes the effective command scopes for one admin permission profile.
pub fn effective_admin_scopes(
    profile: &AdminPermissionProfile,
    bindings: &AdminPermissionRoleBindings,
) -> BTreeSet<String> {
    let mut effective_scopes = profile.scopes.clone();

    for role in &profile.roles {
        if let Some(scopes) = bindings.scopes_for_role(role) {
            effective_scopes.extend(scopes.iter().cloned());
        }
    }

    effective_scopes
}

/// Plans whether the optional plugin shell should update command scopes.
pub fn plan_admin_scope_update(
    settings: &AdminPermissionSettings,
    profile: &AdminPermissionProfile,
    bindings: &AdminPermissionRoleBindings,
    existing_scopes: &CommandScopes,
) -> AdminScopeUpdate {
    if settings.state == AdminPermissionPluginState::Disabled {
        return AdminScopeUpdate::Unchanged;
    }

    let effective_scopes = effective_admin_scopes(profile, bindings);
    if existing_scopes.0 == effective_scopes {
        return AdminScopeUpdate::Unchanged;
    }

    AdminScopeUpdate::Replace(effective_scopes)
}

/// Converts an optional storage row into an explicit permission profile.
pub fn profile_from_storage_row(
    row: Option<StoredAdminPermissionRow<'_>>,
    default_profile: &AdminPermissionProfile,
) -> Result<AdminPermissionProfile, AdminPermissionStorageError> {
    let Some(row) = row else {
        return Ok(default_profile.clone());
    };

    if row.subject.trim().is_empty() {
        return Err(AdminPermissionStorageError::EmptySubject);
    }

    Ok(AdminPermissionProfile {
        roles: normalize_labels(row.roles, AdminPermissionStorageError::EmptyRole)?,
        scopes: normalize_labels(row.scopes, AdminPermissionStorageError::EmptyScope)?,
    })
}

fn sync_admin_permission_scopes(
    settings: Res<AdminPermissionSettings>,
    bindings: Res<AdminPermissionRoleBindings>,
    mut profiles: Query<(&AdminPermissionProfile, &mut CommandScopes)>,
) {
    for (profile, mut command_scopes) in &mut profiles {
        if let AdminScopeUpdate::Replace(scopes) =
            plan_admin_scope_update(&settings, profile, &bindings, &command_scopes)
        {
            command_scopes.0 = scopes;
        }
    }
}

fn normalize_labels(
    labels: &[&str],
    empty_error: AdminPermissionStorageError,
) -> Result<BTreeSet<String>, AdminPermissionStorageError> {
    let mut normalized = BTreeSet::new();

    for label in labels {
        let trimmed = label.trim();
        if trimmed.is_empty() {
            return Err(empty_error);
        }
        normalized.insert(trimmed.to_owned());
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADMIN_ROLE: &str = "admin";
    const MODERATOR_ROLE: &str = "moderator";
    const COMMAND_NAME: &str = "kick";
    const COMMAND_SCOPE: &str = "valence.command";
    const KICK_SCOPE: &str = "valence.command.kick";
    const DIRECT_SCOPE: &str = "valence.command.say";
    const SUBJECT: &str = "player-uuid";

    #[test]
    fn role_binding_allows_command_scope() {
        let mut bindings = AdminPermissionRoleBindings::default();
        bindings.bind_role_to_scope(ADMIN_ROLE, COMMAND_SCOPE);

        let mut registry = CommandScopeRegistry::new();
        registry.add_scope(COMMAND_SCOPE);
        registry.add_scope(KICK_SCOPE);

        let metadata = AdminCommandMetadata::new(COMMAND_NAME, [KICK_SCOPE]);
        let context =
            AdminPermissionContext::enabled(AdminPermissionProfile::new().with_role(ADMIN_ROLE));

        assert_eq!(
            evaluate_admin_command(&metadata, &context, &bindings, &registry),
            AdminPermissionDecision::Allowed
        );
    }

    #[test]
    fn missing_required_scope_denies_command() {
        let bindings = AdminPermissionRoleBindings::default();
        let mut registry = CommandScopeRegistry::new();
        registry.add_scope(KICK_SCOPE);

        let metadata = AdminCommandMetadata::new(COMMAND_NAME, [KICK_SCOPE]);
        let context = AdminPermissionContext::enabled(
            AdminPermissionProfile::new().with_role(MODERATOR_ROLE),
        );

        let decision = evaluate_admin_command(&metadata, &context, &bindings, &registry);
        let AdminPermissionDecision::Denied(AdminPermissionDenial::MissingRequiredScope {
            required_scopes,
            effective_scopes,
        }) = decision
        else {
            panic!("expected missing required scope denial");
        };

        assert_eq!(required_scopes, vec![KICK_SCOPE.to_owned()]);
        assert!(effective_scopes.is_empty());
    }

    #[test]
    fn missing_permission_data_denies_scoped_command() {
        let bindings = AdminPermissionRoleBindings::default();
        let registry = CommandScopeRegistry::new();
        let metadata = AdminCommandMetadata::new(COMMAND_NAME, [KICK_SCOPE]);
        let context = AdminPermissionContext::missing_profile();

        assert_eq!(
            evaluate_admin_command(&metadata, &context, &bindings, &registry),
            AdminPermissionDecision::Denied(AdminPermissionDenial::MissingPermissionData)
        );
    }

    #[test]
    fn storage_missing_row_uses_default_profile() {
        let default_profile = AdminPermissionProfile::new().with_scope(DIRECT_SCOPE);

        assert_eq!(
            profile_from_storage_row(None, &default_profile).unwrap(),
            default_profile
        );
    }

    #[test]
    fn storage_rejects_invalid_rows() {
        let default_profile = AdminPermissionProfile::new();
        let empty_subject = StoredAdminPermissionRow {
            subject: " ",
            roles: &[],
            scopes: &[],
        };
        let empty_role = StoredAdminPermissionRow {
            subject: SUBJECT,
            roles: &[""],
            scopes: &[],
        };
        let empty_scope = StoredAdminPermissionRow {
            subject: SUBJECT,
            roles: &[],
            scopes: &[""],
        };

        assert_eq!(
            profile_from_storage_row(Some(empty_subject), &default_profile),
            Err(AdminPermissionStorageError::EmptySubject)
        );
        assert_eq!(
            profile_from_storage_row(Some(empty_role), &default_profile),
            Err(AdminPermissionStorageError::EmptyRole)
        );
        assert_eq!(
            profile_from_storage_row(Some(empty_scope), &default_profile),
            Err(AdminPermissionStorageError::EmptyScope)
        );
    }

    #[test]
    fn stale_command_tree_scope_plan_replaces_old_scopes() {
        let settings = AdminPermissionSettings::default();
        let mut bindings = AdminPermissionRoleBindings::default();
        bindings.bind_role_to_scope(ADMIN_ROLE, COMMAND_SCOPE);
        let existing_scopes = CommandScopes(BTreeSet::from([DIRECT_SCOPE.to_owned()]));
        let profile = AdminPermissionProfile::new().with_role(ADMIN_ROLE);

        assert_eq!(
            plan_admin_scope_update(&settings, &profile, &bindings, &existing_scopes),
            AdminScopeUpdate::Replace(BTreeSet::from([COMMAND_SCOPE.to_owned()]))
        );
    }

    #[test]
    fn disabled_plugin_preserves_command_scopes() {
        let settings = AdminPermissionSettings {
            state: AdminPermissionPluginState::Disabled,
        };
        let mut bindings = AdminPermissionRoleBindings::default();
        bindings.bind_role_to_scope(ADMIN_ROLE, COMMAND_SCOPE);
        let existing_scopes = CommandScopes(BTreeSet::from([DIRECT_SCOPE.to_owned()]));
        let profile = AdminPermissionProfile::new().with_role(ADMIN_ROLE);

        assert_eq!(
            plan_admin_scope_update(&settings, &profile, &bindings, &existing_scopes),
            AdminScopeUpdate::Unchanged
        );
    }

    #[test]
    fn plugin_shell_updates_command_scopes_from_profile() {
        let mut app = App::new();
        app.add_plugins(AdminPermissionPlugin);
        app.world_mut()
            .resource_mut::<AdminPermissionRoleBindings>()
            .bind_role_to_scope(ADMIN_ROLE, COMMAND_SCOPE);
        let entity = app
            .world_mut()
            .spawn((
                AdminPermissionProfile::new().with_role(ADMIN_ROLE),
                CommandScopes::new(),
            ))
            .id();

        app.update();

        assert_eq!(
            app.world().entity(entity).get::<CommandScopes>().unwrap().0,
            BTreeSet::from([COMMAND_SCOPE.to_owned()])
        );
    }

    #[test]
    fn plugin_shell_disabled_preserves_existing_scopes() {
        let mut app = App::new();
        app.add_plugins(AdminPermissionPlugin);
        app.world_mut()
            .resource_mut::<AdminPermissionSettings>()
            .state = AdminPermissionPluginState::Disabled;
        app.world_mut()
            .resource_mut::<AdminPermissionRoleBindings>()
            .bind_role_to_scope(ADMIN_ROLE, COMMAND_SCOPE);
        let entity = app
            .world_mut()
            .spawn((
                AdminPermissionProfile::new().with_role(ADMIN_ROLE),
                CommandScopes(BTreeSet::from([DIRECT_SCOPE.to_owned()])),
            ))
            .id();

        app.update();

        assert_eq!(
            app.world().entity(entity).get::<CommandScopes>().unwrap().0,
            BTreeSet::from([DIRECT_SCOPE.to_owned()])
        );
    }

    #[test]
    fn denial_diagnostic_names_command_and_scopes() {
        let denial = AdminPermissionDenial::MissingRequiredScope {
            required_scopes: vec![KICK_SCOPE.to_owned()],
            effective_scopes: vec![DIRECT_SCOPE.to_owned()],
        };

        assert_eq!(
            denial.diagnostic(COMMAND_NAME),
            "command `kick` denied: requires one of [valence.command.kick], effective scopes [valence.command.say]"
        );
    }
}
