//! Bounded labels and redaction decisions for observability records.

/// Label value emitted when a sensitive input must not leave the process.
pub const REDACTED_LABEL_VALUE: &str = "<redacted>";

const LABEL_VALUE_LIST_SEPARATOR: char = ',';
const LABEL_VALUE_KEY_SEPARATOR: char = '=';

/// Bounded subsystem labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilitySubsystem {
    /// Tick or schedule phase work.
    Tick,
    /// Network ingress or egress work.
    Network,
    /// Chunk egress or world data work.
    Chunk,
    /// Entity update work.
    Entity,
    /// Plugin adapter work.
    Plugin,
    /// Optional exporter adapter work.
    Exporter,
}

impl ObservabilitySubsystem {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tick => "tick",
            Self::Network => "network",
            Self::Chunk => "chunk",
            Self::Entity => "entity",
            Self::Plugin => "plugin",
            Self::Exporter => "exporter",
        }
    }
}

/// Bounded Valence phase labels selected for initial hooks.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityPhase {
    /// Bevy `PreUpdate` phase.
    PreUpdate,
    /// Valence event loop pre-update phase.
    EventLoopPreUpdate,
    /// Valence event loop update phase.
    EventLoopUpdate,
    /// Valence event loop post-update phase.
    EventLoopPostUpdate,
    /// Bevy `PostUpdate` phase.
    PostUpdate,
}

impl ObservabilityPhase {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreUpdate => "pre_update",
            Self::EventLoopPreUpdate => "event_loop_pre_update",
            Self::EventLoopUpdate => "event_loop_update",
            Self::EventLoopPostUpdate => "event_loop_post_update",
            Self::PostUpdate => "post_update",
        }
    }
}

/// Bounded packet direction label.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityDirection {
    /// Client-to-server packet flow.
    Serverbound,
}

impl ObservabilityDirection {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Serverbound => "serverbound",
        }
    }
}

/// Bounded packet ID class.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum PacketIdClass {
    /// Packet ID is non-negative and protocol-shaped.
    Known,
    /// Packet ID is outside the protocol-shaped non-negative range.
    Unknown,
}

impl PacketIdClass {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Known => "known",
            Self::Unknown => "unknown",
        }
    }
}

/// Redaction policy applied to a record.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum ObservabilityRedaction {
    /// No sensitive inputs participated in the record.
    None,
    /// Sensitive inputs were present but omitted from labels.
    OmittedSensitiveInput,
    /// A sensitive label value was replaced with the redaction marker.
    RedactedSensitiveInput,
}

impl ObservabilityRedaction {
    /// Returns the stable label value.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OmittedSensitiveInput => "omitted_sensitive_input",
            Self::RedactedSensitiveInput => "redacted_sensitive_input",
        }
    }
}

/// Bounded label set for observability records.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ObservabilityLabels {
    /// Subsystem that produced the record.
    pub subsystem: ObservabilitySubsystem,
    /// Optional phase label for span records.
    pub phase: Option<ObservabilityPhase>,
    /// Optional packet direction label for network records.
    pub direction: Option<ObservabilityDirection>,
    /// Optional packet ID class; raw IDs are intentionally not labels.
    pub packet_id_class: Option<PacketIdClass>,
    /// Redaction policy applied to sensitive inputs.
    pub redaction: ObservabilityRedaction,
}

impl ObservabilityLabels {
    pub(crate) const fn tick(phase: ObservabilityPhase) -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Tick,
            phase: Some(phase),
            direction: None,
            packet_id_class: None,
            redaction: ObservabilityRedaction::None,
        }
    }

    pub(crate) const fn network(packet_id_class: PacketIdClass) -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Network,
            phase: None,
            direction: Some(ObservabilityDirection::Serverbound),
            packet_id_class: Some(packet_id_class),
            redaction: ObservabilityRedaction::OmittedSensitiveInput,
        }
    }

    pub(crate) const fn exporter_failure() -> Self {
        Self {
            subsystem: ObservabilitySubsystem::Exporter,
            phase: None,
            direction: None,
            packet_id_class: None,
            redaction: ObservabilityRedaction::OmittedSensitiveInput,
        }
    }
}

/// Sensitive input categories never copied into public labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SensitiveObservabilityField {
    /// Player usernames, UUIDs, or account identifiers.
    PlayerIdentifier,
    /// Socket addresses or connection endpoints.
    SocketAddress,
    /// Raw packet bytes or decoded packet payloads.
    PacketPayload,
    /// Chat, sign, command, or other user-provided text.
    UserText,
}

/// Label value produced after applying redaction policy.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityLabelValue {
    /// Bounded static label value.
    Static(&'static str),
    /// Redacted label value.
    Redacted,
}

impl ObservabilityLabelValue {
    /// Creates an exportable public label value after rejecting unsafe values.
    pub fn public_static(value: &'static str) -> Result<Self, ObservabilityLabelError> {
        validate_public_label_value(value)?;
        Ok(Self::Static(value))
    }

    /// Returns the string that may be exported.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static(value) => value,
            Self::Redacted => REDACTED_LABEL_VALUE,
        }
    }
}

/// Label validation diagnostics.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityLabelError {
    /// Label values must not be empty.
    Empty,
    /// Public static labels must not equal the redaction marker.
    ReservedRedactionMarker,
    /// Public static labels must not contain list or key separators.
    UnsafeDelimiter,
}

impl ObservabilityLabelError {
    /// Stable diagnostic text suitable for fixtures and docs.
    pub const fn diagnostic(self) -> &'static str {
        match self {
            Self::Empty => "empty observability label value",
            Self::ReservedRedactionMarker => "reserved observability redaction marker",
            Self::UnsafeDelimiter => "unsafe observability label delimiter",
        }
    }
}

/// Purely redacts a sensitive input category into an export-safe label value.
pub const fn redact_sensitive_field(
    _field: SensitiveObservabilityField,
) -> ObservabilityLabelValue {
    ObservabilityLabelValue::Redacted
}

fn validate_public_label_value(value: &str) -> Result<(), ObservabilityLabelError> {
    if value.is_empty() {
        return Err(ObservabilityLabelError::Empty);
    }

    if value == REDACTED_LABEL_VALUE {
        return Err(ObservabilityLabelError::ReservedRedactionMarker);
    }

    if value.contains(LABEL_VALUE_LIST_SEPARATOR) || value.contains(LABEL_VALUE_KEY_SEPARATOR) {
        return Err(ObservabilityLabelError::UnsafeDelimiter);
    }

    Ok(())
}
