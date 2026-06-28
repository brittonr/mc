use super::{Direction, State};

mod v15w39c;
mod v18w50a;
mod v19w02a;
mod v1_10_2;
mod v1_11_2;
mod v1_12_2;
mod v1_13_2;
mod v1_14;
mod v1_14_1;
mod v1_14_2;
mod v1_14_3;
mod v1_14_4;
mod v1_15;
mod v1_16_1;
mod v1_16_4;
mod v1_17_1;
mod v1_18_1;
mod v1_18_2;
mod v1_20_1;
mod v1_7_10;
mod v1_8_9;
mod v1_9;
mod v1_9_2;

// https://wiki.vg/Protocol_History
// https://wiki.vg/Protocol_version_numbers#Versions_after_the_Netty_rewrite

const PROTOCOL_1_20_1: i32 = 763;
const PROTOCOL_1_18_2: i32 = 758;
const PROTOCOL_1_18_1: i32 = 757;
const PROTOCOL_1_17_1: i32 = 756;
const PROTOCOL_1_16_5: i32 = 754;
const PROTOCOL_1_16_3: i32 = 753;
const PROTOCOL_1_16_2: i32 = 751;
const PROTOCOL_1_16_1: i32 = 736;
const PROTOCOL_1_16: i32 = 735;
const PROTOCOL_1_15_2: i32 = 578;
const PROTOCOL_1_15_1: i32 = 575;
const PROTOCOL_1_14_4: i32 = 498;
const PROTOCOL_1_14_3: i32 = 490;
const PROTOCOL_1_14_2: i32 = 485;
const PROTOCOL_1_14_1: i32 = 480;
const PROTOCOL_1_14: i32 = 477;
const PROTOCOL_19W02A: i32 = 452;
const PROTOCOL_18W50A: i32 = 451;
const PROTOCOL_1_13_2: i32 = 404;
const PROTOCOL_1_12_2: i32 = 340;
const PROTOCOL_1_11_2: i32 = 316;
const PROTOCOL_1_11: i32 = 315;
const PROTOCOL_1_10_2: i32 = 210;
const PROTOCOL_1_9_2: i32 = 109;
const PROTOCOL_1_9: i32 = 107;
const PROTOCOL_15W39C: i32 = 74;
const PROTOCOL_1_8_9: i32 = 47;
const PROTOCOL_1_7_10: i32 = 5;

const NO_ALIASES: &[&str] = &[];
const ALIASES_1_16_5: &[&str] = &["1.16.4"];

pub const SUPPORTED_PROTOCOL_COUNT: usize = 28;
pub const SUPPORTED_PROTOCOL_IDS: [i32; SUPPORTED_PROTOCOL_COUNT] = [
    PROTOCOL_1_20_1,
    PROTOCOL_1_18_2,
    PROTOCOL_1_18_1,
    PROTOCOL_1_17_1,
    PROTOCOL_1_16_5,
    PROTOCOL_1_16_3,
    PROTOCOL_1_16_2,
    PROTOCOL_1_16_1,
    PROTOCOL_1_16,
    PROTOCOL_1_15_2,
    PROTOCOL_1_15_1,
    PROTOCOL_1_14_4,
    PROTOCOL_1_14_3,
    PROTOCOL_1_14_2,
    PROTOCOL_1_14_1,
    PROTOCOL_1_14,
    PROTOCOL_19W02A,
    PROTOCOL_18W50A,
    PROTOCOL_1_13_2,
    PROTOCOL_1_12_2,
    PROTOCOL_1_11_2,
    PROTOCOL_1_11,
    PROTOCOL_1_10_2,
    PROTOCOL_1_9_2,
    PROTOCOL_1_9,
    PROTOCOL_15W39C,
    PROTOCOL_1_8_9,
    PROTOCOL_1_7_10,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranslationModule {
    Version1_20_1,
    Version1_18_2,
    Version1_18_1,
    Version1_17_1,
    Version1_16_4,
    Version1_16_1,
    Version1_15,
    Version1_14_4,
    Version1_14_3,
    Version1_14_2,
    Version1_14_1,
    Version1_14,
    Snapshot19w02a,
    Snapshot18w50a,
    Version1_13_2,
    Version1_12_2,
    Version1_11_2,
    Version1_10_2,
    Version1_9_2,
    Version1_9,
    Snapshot15w39c,
    Version1_8_9,
    Version1_7_10,
}

impl TranslationModule {
    pub const fn module_name(self) -> &'static str {
        match self {
            TranslationModule::Version1_20_1 => "v1_20_1",
            TranslationModule::Version1_18_2 => "v1_18_2",
            TranslationModule::Version1_18_1 => "v1_18_1",
            TranslationModule::Version1_17_1 => "v1_17_1",
            TranslationModule::Version1_16_4 => "v1_16_4",
            TranslationModule::Version1_16_1 => "v1_16_1",
            TranslationModule::Version1_15 => "v1_15",
            TranslationModule::Version1_14_4 => "v1_14_4",
            TranslationModule::Version1_14_3 => "v1_14_3",
            TranslationModule::Version1_14_2 => "v1_14_2",
            TranslationModule::Version1_14_1 => "v1_14_1",
            TranslationModule::Version1_14 => "v1_14",
            TranslationModule::Snapshot19w02a => "v19w02a",
            TranslationModule::Snapshot18w50a => "v18w50a",
            TranslationModule::Version1_13_2 => "v1_13_2",
            TranslationModule::Version1_12_2 => "v1_12_2",
            TranslationModule::Version1_11_2 => "v1_11_2",
            TranslationModule::Version1_10_2 => "v1_10_2",
            TranslationModule::Version1_9_2 => "v1_9_2",
            TranslationModule::Version1_9 => "v1_9",
            TranslationModule::Snapshot15w39c => "v15w39c",
            TranslationModule::Version1_8_9 => "v1_8_9",
            TranslationModule::Version1_7_10 => "v1_7_10",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackKind {
    PacketFallback,
    ReusesTranslationModule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FallbackRelationship {
    pub target_protocol_id: i32,
    pub kind: FallbackKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProtocolVersionRow {
    pub canonical_name: &'static str,
    pub aliases: &'static [&'static str],
    pub protocol_id: i32,
    pub translation_module: TranslationModule,
    pub fallback: Option<FallbackRelationship>,
}

impl ProtocolVersionRow {
    const fn own(
        canonical_name: &'static str,
        aliases: &'static [&'static str],
        protocol_id: i32,
        translation_module: TranslationModule,
    ) -> Self {
        Self {
            canonical_name,
            aliases,
            protocol_id,
            translation_module,
            fallback: None,
        }
    }

    const fn packet_fallback(
        canonical_name: &'static str,
        aliases: &'static [&'static str],
        protocol_id: i32,
        translation_module: TranslationModule,
        target_protocol_id: i32,
    ) -> Self {
        Self {
            canonical_name,
            aliases,
            protocol_id,
            translation_module,
            fallback: Some(FallbackRelationship {
                target_protocol_id,
                kind: FallbackKind::PacketFallback,
            }),
        }
    }

    const fn reuses_module(
        canonical_name: &'static str,
        aliases: &'static [&'static str],
        protocol_id: i32,
        translation_module: TranslationModule,
        target_protocol_id: i32,
    ) -> Self {
        Self {
            canonical_name,
            aliases,
            protocol_id,
            translation_module,
            fallback: Some(FallbackRelationship {
                target_protocol_id,
                kind: FallbackKind::ReusesTranslationModule,
            }),
        }
    }

    fn matches_name(&self, name: &str) -> bool {
        self.canonical_name == name || self.aliases.iter().any(|alias| *alias == name)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DispatchVersionRow {
    pub protocol_id: i32,
    pub translation_module: TranslationModule,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolVersionTableError {
    EmptyCanonicalName {
        protocol_id: i32,
    },
    EmptyAlias {
        protocol_id: i32,
    },
    DuplicateProtocolId {
        protocol_id: i32,
    },
    DuplicateName {
        name: &'static str,
    },
    MissingTranslationModule {
        protocol_id: i32,
        translation_module: TranslationModule,
    },
    UnknownFallbackTarget {
        protocol_id: i32,
        target_protocol_id: i32,
    },
    ReuseModuleMismatch {
        protocol_id: i32,
        target_protocol_id: i32,
        translation_module: TranslationModule,
        target_translation_module: TranslationModule,
    },
    SupportedProtocolLengthMismatch {
        expected: usize,
        actual: usize,
    },
    SupportedProtocolMismatch {
        index: usize,
        expected: i32,
        actual: i32,
    },
    DispatchLengthMismatch {
        expected: usize,
        actual: usize,
    },
    DispatchProtocolMismatch {
        index: usize,
        expected: i32,
        actual: i32,
    },
    DispatchModuleMismatch {
        protocol_id: i32,
        expected: TranslationModule,
        actual: TranslationModule,
    },
}

pub const PROTOCOL_VERSION_ROWS: [ProtocolVersionRow; SUPPORTED_PROTOCOL_COUNT] = [
    ProtocolVersionRow::packet_fallback(
        "1.20.1",
        NO_ALIASES,
        PROTOCOL_1_20_1,
        TranslationModule::Version1_20_1,
        PROTOCOL_1_18_2,
    ),
    ProtocolVersionRow::own(
        "1.18.2",
        NO_ALIASES,
        PROTOCOL_1_18_2,
        TranslationModule::Version1_18_2,
    ),
    ProtocolVersionRow::own(
        "1.18.1",
        NO_ALIASES,
        PROTOCOL_1_18_1,
        TranslationModule::Version1_18_1,
    ),
    ProtocolVersionRow::own(
        "1.17.1",
        NO_ALIASES,
        PROTOCOL_1_17_1,
        TranslationModule::Version1_17_1,
    ),
    ProtocolVersionRow::own(
        "1.16.5",
        ALIASES_1_16_5,
        PROTOCOL_1_16_5,
        TranslationModule::Version1_16_4,
    ),
    ProtocolVersionRow::reuses_module(
        "1.16.3",
        NO_ALIASES,
        PROTOCOL_1_16_3,
        TranslationModule::Version1_16_4,
        PROTOCOL_1_16_5,
    ),
    ProtocolVersionRow::reuses_module(
        "1.16.2",
        NO_ALIASES,
        PROTOCOL_1_16_2,
        TranslationModule::Version1_16_4,
        PROTOCOL_1_16_5,
    ),
    ProtocolVersionRow::own(
        "1.16.1",
        NO_ALIASES,
        PROTOCOL_1_16_1,
        TranslationModule::Version1_16_1,
    ),
    ProtocolVersionRow::reuses_module(
        "1.16",
        NO_ALIASES,
        PROTOCOL_1_16,
        TranslationModule::Version1_16_1,
        PROTOCOL_1_16_1,
    ),
    ProtocolVersionRow::own(
        "1.15.2",
        NO_ALIASES,
        PROTOCOL_1_15_2,
        TranslationModule::Version1_15,
    ),
    ProtocolVersionRow::reuses_module(
        "1.15.1",
        NO_ALIASES,
        PROTOCOL_1_15_1,
        TranslationModule::Version1_15,
        PROTOCOL_1_15_2,
    ),
    ProtocolVersionRow::own(
        "1.14.4",
        NO_ALIASES,
        PROTOCOL_1_14_4,
        TranslationModule::Version1_14_4,
    ),
    ProtocolVersionRow::own(
        "1.14.3",
        NO_ALIASES,
        PROTOCOL_1_14_3,
        TranslationModule::Version1_14_3,
    ),
    ProtocolVersionRow::own(
        "1.14.2",
        NO_ALIASES,
        PROTOCOL_1_14_2,
        TranslationModule::Version1_14_2,
    ),
    ProtocolVersionRow::own(
        "1.14.1",
        NO_ALIASES,
        PROTOCOL_1_14_1,
        TranslationModule::Version1_14_1,
    ),
    ProtocolVersionRow::own(
        "1.14",
        NO_ALIASES,
        PROTOCOL_1_14,
        TranslationModule::Version1_14,
    ),
    ProtocolVersionRow::own(
        "19w02a",
        NO_ALIASES,
        PROTOCOL_19W02A,
        TranslationModule::Snapshot19w02a,
    ),
    ProtocolVersionRow::own(
        "18w50a",
        NO_ALIASES,
        PROTOCOL_18W50A,
        TranslationModule::Snapshot18w50a,
    ),
    ProtocolVersionRow::own(
        "1.13.2",
        NO_ALIASES,
        PROTOCOL_1_13_2,
        TranslationModule::Version1_13_2,
    ),
    ProtocolVersionRow::own(
        "1.12.2",
        NO_ALIASES,
        PROTOCOL_1_12_2,
        TranslationModule::Version1_12_2,
    ),
    ProtocolVersionRow::own(
        "1.11.2",
        NO_ALIASES,
        PROTOCOL_1_11_2,
        TranslationModule::Version1_11_2,
    ),
    ProtocolVersionRow::reuses_module(
        "1.11",
        NO_ALIASES,
        PROTOCOL_1_11,
        TranslationModule::Version1_11_2,
        PROTOCOL_1_11_2,
    ),
    ProtocolVersionRow::own(
        "1.10.2",
        NO_ALIASES,
        PROTOCOL_1_10_2,
        TranslationModule::Version1_10_2,
    ),
    ProtocolVersionRow::own(
        "1.9.2",
        NO_ALIASES,
        PROTOCOL_1_9_2,
        TranslationModule::Version1_9_2,
    ),
    ProtocolVersionRow::own(
        "1.9",
        NO_ALIASES,
        PROTOCOL_1_9,
        TranslationModule::Version1_9,
    ),
    ProtocolVersionRow::own(
        "15w39c",
        NO_ALIASES,
        PROTOCOL_15W39C,
        TranslationModule::Snapshot15w39c,
    ),
    ProtocolVersionRow::own(
        "1.8.9",
        NO_ALIASES,
        PROTOCOL_1_8_9,
        TranslationModule::Version1_8_9,
    ),
    ProtocolVersionRow::own(
        "1.7.10",
        NO_ALIASES,
        PROTOCOL_1_7_10,
        TranslationModule::Version1_7_10,
    ),
];

const DISPATCH_VERSION_ROWS: [DispatchVersionRow; SUPPORTED_PROTOCOL_COUNT] = [
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_20_1,
        translation_module: TranslationModule::Version1_20_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_18_2,
        translation_module: TranslationModule::Version1_18_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_18_1,
        translation_module: TranslationModule::Version1_18_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_17_1,
        translation_module: TranslationModule::Version1_17_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_16_5,
        translation_module: TranslationModule::Version1_16_4,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_16_3,
        translation_module: TranslationModule::Version1_16_4,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_16_2,
        translation_module: TranslationModule::Version1_16_4,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_16_1,
        translation_module: TranslationModule::Version1_16_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_16,
        translation_module: TranslationModule::Version1_16_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_15_2,
        translation_module: TranslationModule::Version1_15,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_15_1,
        translation_module: TranslationModule::Version1_15,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_14_4,
        translation_module: TranslationModule::Version1_14_4,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_14_3,
        translation_module: TranslationModule::Version1_14_3,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_14_2,
        translation_module: TranslationModule::Version1_14_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_14_1,
        translation_module: TranslationModule::Version1_14_1,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_14,
        translation_module: TranslationModule::Version1_14,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_19W02A,
        translation_module: TranslationModule::Snapshot19w02a,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_18W50A,
        translation_module: TranslationModule::Snapshot18w50a,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_13_2,
        translation_module: TranslationModule::Version1_13_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_12_2,
        translation_module: TranslationModule::Version1_12_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_11_2,
        translation_module: TranslationModule::Version1_11_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_11,
        translation_module: TranslationModule::Version1_11_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_10_2,
        translation_module: TranslationModule::Version1_10_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_9_2,
        translation_module: TranslationModule::Version1_9_2,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_9,
        translation_module: TranslationModule::Version1_9,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_15W39C,
        translation_module: TranslationModule::Snapshot15w39c,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_8_9,
        translation_module: TranslationModule::Version1_8_9,
    },
    DispatchVersionRow {
        protocol_id: PROTOCOL_1_7_10,
        translation_module: TranslationModule::Version1_7_10,
    },
];

const AVAILABLE_TRANSLATION_MODULES: &[TranslationModule] = &[
    TranslationModule::Version1_20_1,
    TranslationModule::Version1_18_2,
    TranslationModule::Version1_18_1,
    TranslationModule::Version1_17_1,
    TranslationModule::Version1_16_4,
    TranslationModule::Version1_16_1,
    TranslationModule::Version1_15,
    TranslationModule::Version1_14_4,
    TranslationModule::Version1_14_3,
    TranslationModule::Version1_14_2,
    TranslationModule::Version1_14_1,
    TranslationModule::Version1_14,
    TranslationModule::Snapshot19w02a,
    TranslationModule::Snapshot18w50a,
    TranslationModule::Version1_13_2,
    TranslationModule::Version1_12_2,
    TranslationModule::Version1_11_2,
    TranslationModule::Version1_10_2,
    TranslationModule::Version1_9_2,
    TranslationModule::Version1_9,
    TranslationModule::Snapshot15w39c,
    TranslationModule::Version1_8_9,
    TranslationModule::Version1_7_10,
];

pub fn protocol_name_to_protocol_version(s: String) -> i32 {
    if s.is_empty() {
        return SUPPORTED_PROTOCOL_IDS[0];
    }

    if let Some(row) = protocol_row_for_name(s.as_ref()) {
        return row.protocol_id;
    }

    if let Ok(n) = s.parse::<i32>() {
        return n;
    }

    panic!("Unrecognized protocol name: {}", s)
}

pub fn translate_internal_packet_id_for_version(
    version: i32,
    state: State,
    dir: Direction,
    id: i32,
    to_internal: bool,
) -> i32 {
    if let Some(row) = dispatch_row_for_protocol_id(version) {
        return translate_with_module(row.translation_module, state, dir, id, to_internal);
    }

    panic!("unsupported protocol version: {}", version)
}

pub fn validate_protocol_version_tables() -> Result<(), ProtocolVersionTableError> {
    validate_protocol_version_manifest(
        &PROTOCOL_VERSION_ROWS,
        &SUPPORTED_PROTOCOL_IDS,
        &DISPATCH_VERSION_ROWS,
        AVAILABLE_TRANSLATION_MODULES,
    )
}

fn protocol_row_for_name(name: &str) -> Option<&'static ProtocolVersionRow> {
    PROTOCOL_VERSION_ROWS
        .iter()
        .find(|row| row.matches_name(name))
}

fn protocol_row_for_protocol_id<'a>(
    rows: &'a [ProtocolVersionRow],
    protocol_id: i32,
) -> Option<&'a ProtocolVersionRow> {
    rows.iter().find(|row| row.protocol_id == protocol_id)
}

fn dispatch_row_for_protocol_id(protocol_id: i32) -> Option<&'static DispatchVersionRow> {
    DISPATCH_VERSION_ROWS
        .iter()
        .find(|row| row.protocol_id == protocol_id)
}

fn translate_with_module(
    module: TranslationModule,
    state: State,
    dir: Direction,
    id: i32,
    to_internal: bool,
) -> i32 {
    match module {
        TranslationModule::Version1_20_1 => {
            v1_20_1::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_18_2 => {
            v1_18_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_18_1 => {
            v1_18_1::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_17_1 => {
            v1_17_1::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_16_4 => {
            v1_16_4::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_16_1 => {
            v1_16_1::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_15 => {
            v1_15::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_14_4 => {
            v1_14_4::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_14_3 => {
            v1_14_3::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_14_2 => {
            v1_14_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_14_1 => {
            v1_14_1::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_14 => {
            v1_14::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Snapshot19w02a => {
            v19w02a::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Snapshot18w50a => {
            v18w50a::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_13_2 => {
            v1_13_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_12_2 => {
            v1_12_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_11_2 => {
            v1_11_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_10_2 => {
            v1_10_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_9_2 => {
            v1_9_2::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_9 => {
            v1_9::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Snapshot15w39c => {
            v15w39c::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_8_9 => {
            v1_8_9::translate_internal_packet_id(state, dir, id, to_internal)
        }
        TranslationModule::Version1_7_10 => {
            v1_7_10::translate_internal_packet_id(state, dir, id, to_internal)
        }
    }
}

fn validate_protocol_version_manifest(
    rows: &[ProtocolVersionRow],
    supported_protocol_ids: &[i32],
    dispatch_rows: &[DispatchVersionRow],
    available_modules: &[TranslationModule],
) -> Result<(), ProtocolVersionTableError> {
    validate_supported_protocol_ids(rows, supported_protocol_ids)?;
    validate_unique_protocol_ids(rows)?;
    validate_names(rows)?;
    validate_available_modules(rows, available_modules)?;
    validate_fallbacks(rows)?;
    validate_dispatch_rows(rows, dispatch_rows)?;

    Ok(())
}

fn validate_supported_protocol_ids(
    rows: &[ProtocolVersionRow],
    supported_protocol_ids: &[i32],
) -> Result<(), ProtocolVersionTableError> {
    if rows.len() != supported_protocol_ids.len() {
        return Err(ProtocolVersionTableError::SupportedProtocolLengthMismatch {
            expected: rows.len(),
            actual: supported_protocol_ids.len(),
        });
    }

    for (index, row) in rows.iter().enumerate() {
        let actual = supported_protocol_ids[index];
        if row.protocol_id != actual {
            return Err(ProtocolVersionTableError::SupportedProtocolMismatch {
                index,
                expected: row.protocol_id,
                actual,
            });
        }
    }

    Ok(())
}

fn validate_unique_protocol_ids(
    rows: &[ProtocolVersionRow],
) -> Result<(), ProtocolVersionTableError> {
    for (left_index, left) in rows.iter().enumerate() {
        for right in rows.iter().skip(left_index + 1) {
            if left.protocol_id == right.protocol_id {
                return Err(ProtocolVersionTableError::DuplicateProtocolId {
                    protocol_id: left.protocol_id,
                });
            }
        }
    }

    Ok(())
}

fn validate_names(rows: &[ProtocolVersionRow]) -> Result<(), ProtocolVersionTableError> {
    for (left_index, left) in rows.iter().enumerate() {
        validate_row_names(left)?;
        validate_names_against_following_rows(left_index, left, rows)?;
    }

    Ok(())
}

fn validate_row_names(row: &ProtocolVersionRow) -> Result<(), ProtocolVersionTableError> {
    if row.canonical_name.is_empty() {
        return Err(ProtocolVersionTableError::EmptyCanonicalName {
            protocol_id: row.protocol_id,
        });
    }

    for (left_index, left_alias) in row.aliases.iter().enumerate() {
        if left_alias.is_empty() {
            return Err(ProtocolVersionTableError::EmptyAlias {
                protocol_id: row.protocol_id,
            });
        }

        if *left_alias == row.canonical_name {
            return Err(ProtocolVersionTableError::DuplicateName { name: left_alias });
        }

        for right_alias in row.aliases.iter().skip(left_index + 1) {
            if left_alias == right_alias {
                return Err(ProtocolVersionTableError::DuplicateName { name: left_alias });
            }
        }
    }

    Ok(())
}

fn validate_names_against_following_rows(
    left_index: usize,
    left: &ProtocolVersionRow,
    rows: &[ProtocolVersionRow],
) -> Result<(), ProtocolVersionTableError> {
    for right in rows.iter().skip(left_index + 1) {
        if left.canonical_name == right.canonical_name {
            return Err(ProtocolVersionTableError::DuplicateName {
                name: left.canonical_name,
            });
        }

        for right_alias in right.aliases {
            if left.canonical_name == *right_alias {
                return Err(ProtocolVersionTableError::DuplicateName {
                    name: left.canonical_name,
                });
            }
        }

        for left_alias in left.aliases {
            if *left_alias == right.canonical_name {
                return Err(ProtocolVersionTableError::DuplicateName { name: left_alias });
            }

            for right_alias in right.aliases {
                if left_alias == right_alias {
                    return Err(ProtocolVersionTableError::DuplicateName { name: left_alias });
                }
            }
        }
    }

    Ok(())
}

fn validate_available_modules(
    rows: &[ProtocolVersionRow],
    available_modules: &[TranslationModule],
) -> Result<(), ProtocolVersionTableError> {
    for row in rows {
        if !available_modules
            .iter()
            .any(|module| *module == row.translation_module)
        {
            return Err(ProtocolVersionTableError::MissingTranslationModule {
                protocol_id: row.protocol_id,
                translation_module: row.translation_module,
            });
        }
    }

    Ok(())
}

fn validate_fallbacks(rows: &[ProtocolVersionRow]) -> Result<(), ProtocolVersionTableError> {
    for row in rows {
        if let Some(fallback) = row.fallback {
            let target = match protocol_row_for_protocol_id(rows, fallback.target_protocol_id) {
                Some(target) => target,
                None => {
                    return Err(ProtocolVersionTableError::UnknownFallbackTarget {
                        protocol_id: row.protocol_id,
                        target_protocol_id: fallback.target_protocol_id,
                    });
                }
            };

            if fallback.kind == FallbackKind::ReusesTranslationModule
                && row.translation_module != target.translation_module
            {
                return Err(ProtocolVersionTableError::ReuseModuleMismatch {
                    protocol_id: row.protocol_id,
                    target_protocol_id: fallback.target_protocol_id,
                    translation_module: row.translation_module,
                    target_translation_module: target.translation_module,
                });
            }
        }
    }

    Ok(())
}

fn validate_dispatch_rows(
    rows: &[ProtocolVersionRow],
    dispatch_rows: &[DispatchVersionRow],
) -> Result<(), ProtocolVersionTableError> {
    if rows.len() != dispatch_rows.len() {
        return Err(ProtocolVersionTableError::DispatchLengthMismatch {
            expected: rows.len(),
            actual: dispatch_rows.len(),
        });
    }

    for (index, row) in rows.iter().enumerate() {
        let dispatch_row = dispatch_rows[index];
        if row.protocol_id != dispatch_row.protocol_id {
            return Err(ProtocolVersionTableError::DispatchProtocolMismatch {
                index,
                expected: row.protocol_id,
                actual: dispatch_row.protocol_id,
            });
        }

        if row.translation_module != dispatch_row.translation_module {
            return Err(ProtocolVersionTableError::DispatchModuleMismatch {
                protocol_id: row.protocol_id,
                expected: row.translation_module,
                actual: dispatch_row.translation_module,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYTES_PER_KIB: usize = 1024;
    const KIB_PER_MIB: usize = 1024;
    const TEST_PACKET_PARSE_STACK_MIB: usize = 8;
    const TEST_PACKET_PARSE_STACK_BYTES: usize =
        TEST_PACKET_PARSE_STACK_MIB * KIB_PER_MIB * BYTES_PER_KIB;

    fn run_high_stack_packet_parse_fixture(parse: impl FnOnce() + Send + 'static) {
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(parse)
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    const EMPTY_PROTOCOL_NAME: &str = "";
    const NUMERIC_PROTOCOL_INPUT_1_20_1: &str = "763";
    const UNKNOWN_PROTOCOL_NAME: &str = "definitely-not-a-protocol";
    const UNSUPPORTED_PROTOCOL_ID: i32 = 999_999;
    const PROTOCOL_1_18_2_INDEX: usize = 1;
    const PROTOCOL_1_16_3_INDEX: usize = 5;
    const PROTOCOL_1_16_INDEX: usize = 8;
    const HANDSHAKE_WIRE_ID: i32 = 0x00;

    #[test]
    fn current_protocol_version_tables_validate_against_metadata() {
        assert_eq!(validate_protocol_version_tables(), Ok(()));
        assert_eq!(crate::protocol::SUPPORTED_PROTOCOLS, SUPPORTED_PROTOCOL_IDS);
    }

    #[test]
    fn metadata_resolves_supported_names_aliases_and_numeric_inputs() {
        assert_eq!(
            protocol_name_to_protocol_version(EMPTY_PROTOCOL_NAME.to_string()),
            PROTOCOL_1_20_1,
        );
        assert_eq!(
            protocol_name_to_protocol_version(NUMERIC_PROTOCOL_INPUT_1_20_1.to_string()),
            PROTOCOL_1_20_1,
        );

        for row in PROTOCOL_VERSION_ROWS {
            assert_eq!(
                protocol_name_to_protocol_version(row.canonical_name.to_string()),
                row.protocol_id,
                "canonical name {} should resolve to protocol {}",
                row.canonical_name,
                row.protocol_id,
            );

            for alias in row.aliases {
                assert_eq!(
                    protocol_name_to_protocol_version(alias.to_string()),
                    row.protocol_id,
                    "alias {alias} should resolve to protocol {}",
                    row.protocol_id,
                );
            }
        }
    }

    #[test]
    fn metadata_records_explicit_fallback_and_reuse_relationships() {
        let row_1_20_1 = protocol_row_for_protocol_id(&PROTOCOL_VERSION_ROWS, PROTOCOL_1_20_1)
            .expect("1.20.1 metadata is present");
        assert_eq!(
            row_1_20_1.fallback,
            Some(FallbackRelationship {
                target_protocol_id: PROTOCOL_1_18_2,
                kind: FallbackKind::PacketFallback,
            }),
        );

        let row_1_16_3 = protocol_row_for_protocol_id(&PROTOCOL_VERSION_ROWS, PROTOCOL_1_16_3)
            .expect("1.16.3 metadata is present");
        assert_eq!(
            row_1_16_3.fallback,
            Some(FallbackRelationship {
                target_protocol_id: PROTOCOL_1_16_5,
                kind: FallbackKind::ReusesTranslationModule,
            }),
        );
        assert_eq!(row_1_16_3.translation_module.module_name(), "v1_16_4");
    }

    #[test]
    fn dispatch_surface_matches_metadata_modules() {
        for row in PROTOCOL_VERSION_ROWS {
            let dispatch_row = dispatch_row_for_protocol_id(row.protocol_id)
                .expect("dispatch row exists for metadata row");
            assert_eq!(dispatch_row.translation_module, row.translation_module);
        }

        assert_eq!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_16_3,
                State::Handshaking,
                Direction::Serverbound,
                HANDSHAKE_WIRE_ID,
                true,
            ),
            translate_internal_packet_id_for_version(
                PROTOCOL_1_16_5,
                State::Handshaking,
                Direction::Serverbound,
                HANDSHAKE_WIRE_ID,
                true,
            ),
        );
    }

    #[test]
    fn duplicate_alias_is_rejected() {
        const DUPLICATE_ALIAS: &[&str] = &["1.18.2"];
        let mut rows = PROTOCOL_VERSION_ROWS;
        rows[PROTOCOL_1_16_3_INDEX].aliases = DUPLICATE_ALIAS;

        assert_eq!(
            validate_protocol_version_manifest(
                &rows,
                &SUPPORTED_PROTOCOL_IDS,
                &DISPATCH_VERSION_ROWS,
                AVAILABLE_TRANSLATION_MODULES,
            ),
            Err(ProtocolVersionTableError::DuplicateName { name: "1.18.2" }),
        );
    }

    #[test]
    fn missing_translation_module_is_rejected() {
        const MODULES_WITHOUT_1_20_1: &[TranslationModule] = &[TranslationModule::Version1_18_2];

        assert_eq!(
            validate_protocol_version_manifest(
                &PROTOCOL_VERSION_ROWS,
                &SUPPORTED_PROTOCOL_IDS,
                &DISPATCH_VERSION_ROWS,
                MODULES_WITHOUT_1_20_1,
            ),
            Err(ProtocolVersionTableError::MissingTranslationModule {
                protocol_id: PROTOCOL_1_20_1,
                translation_module: TranslationModule::Version1_20_1,
            }),
        );
    }

    #[test]
    fn unknown_fallback_target_is_rejected() {
        const UNKNOWN_FALLBACK_PROTOCOL_ID: i32 = 1_000_001;
        let mut rows = PROTOCOL_VERSION_ROWS;
        rows[PROTOCOL_1_16_INDEX].fallback = Some(FallbackRelationship {
            target_protocol_id: UNKNOWN_FALLBACK_PROTOCOL_ID,
            kind: FallbackKind::ReusesTranslationModule,
        });

        assert_eq!(
            validate_protocol_version_manifest(
                &rows,
                &SUPPORTED_PROTOCOL_IDS,
                &DISPATCH_VERSION_ROWS,
                AVAILABLE_TRANSLATION_MODULES,
            ),
            Err(ProtocolVersionTableError::UnknownFallbackTarget {
                protocol_id: PROTOCOL_1_16,
                target_protocol_id: UNKNOWN_FALLBACK_PROTOCOL_ID,
            }),
        );
    }

    #[test]
    fn protocol_number_mismatch_is_rejected() {
        let mut supported_protocol_ids = SUPPORTED_PROTOCOL_IDS;
        supported_protocol_ids[PROTOCOL_1_18_2_INDEX] = PROTOCOL_1_18_1;

        assert_eq!(
            validate_protocol_version_manifest(
                &PROTOCOL_VERSION_ROWS,
                &supported_protocol_ids,
                &DISPATCH_VERSION_ROWS,
                AVAILABLE_TRANSLATION_MODULES,
            ),
            Err(ProtocolVersionTableError::SupportedProtocolMismatch {
                index: PROTOCOL_1_18_2_INDEX,
                expected: PROTOCOL_1_18_2,
                actual: PROTOCOL_1_18_1,
            }),
        );
    }

    #[test]
    fn stale_dispatch_surface_is_rejected() {
        let mut dispatch_rows = DISPATCH_VERSION_ROWS;
        dispatch_rows[PROTOCOL_1_18_2_INDEX].translation_module = TranslationModule::Version1_18_1;

        assert_eq!(
            validate_protocol_version_manifest(
                &PROTOCOL_VERSION_ROWS,
                &SUPPORTED_PROTOCOL_IDS,
                &dispatch_rows,
                AVAILABLE_TRANSLATION_MODULES,
            ),
            Err(ProtocolVersionTableError::DispatchModuleMismatch {
                protocol_id: PROTOCOL_1_18_2,
                expected: TranslationModule::Version1_18_2,
                actual: TranslationModule::Version1_18_1,
            }),
        );
    }

    #[test]
    #[should_panic(expected = "Unrecognized protocol name: definitely-not-a-protocol")]
    fn unsupported_protocol_name_still_panics() {
        protocol_name_to_protocol_version(UNKNOWN_PROTOCOL_NAME.to_string());
    }

    #[test]
    #[should_panic(expected = "unsupported protocol version: 999999")]
    fn unsupported_translation_version_still_panics() {
        translate_internal_packet_id_for_version(
            UNSUPPORTED_PROTOCOL_ID,
            State::Handshaking,
            Direction::Serverbound,
            HANDSHAKE_WIRE_ID,
            true,
        );
    }

    #[test]
    fn protocol_name_accepts_valence_current_1_20_1() {
        assert_eq!(protocol_name_to_protocol_version("1.20.1".to_string()), 763);
    }

    #[test]
    fn protocol_763_reuses_1_18_2_handshake_translation() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Handshaking,
                Direction::Serverbound,
                0,
                true,
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Handshaking,
                Direction::Serverbound,
                0,
                true,
            )
        );
    }

    #[test]
    fn protocol_763_uses_optional_uuid_login_start() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Login,
                Direction::Serverbound,
                0x00,
                true
            ),
            crate::protocol::packet::login::serverbound::internal_ids::LoginStart_WithOptionalUuid,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Login,
                Direction::Serverbound,
                crate::protocol::packet::login::serverbound::internal_ids::LoginStart_WithOptionalUuid,
                false,
            ),
            0x00,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                763,
                State::Login,
                Direction::Serverbound,
                0x00,
                true
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Login,
                Direction::Serverbound,
                0x00,
                true
            ),
        );
    }

    #[test]
    fn protocol_763_uses_login_success_properties() {
        assert_eq!(
            translate_internal_packet_id_for_version(763, State::Login, Direction::Clientbound, 0x02, true),
            crate::protocol::packet::login::clientbound::internal_ids::LoginSuccess_UUID_WithProperties,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Login,
                Direction::Clientbound,
                crate::protocol::packet::login::clientbound::internal_ids::LoginSuccess_UUID_WithProperties,
                false,
            ),
            0x02,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                763,
                State::Login,
                Direction::Clientbound,
                0x02,
                true
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Login,
                Direction::Clientbound,
                0x02,
                true
            ),
        );
    }

    #[test]
    fn protocol_763_maps_valence_game_join_boundary() {
        assert_eq!(
            translate_internal_packet_id_for_version(763, State::Play, Direction::Clientbound, 0x28, true),
            crate::protocol::packet::play::clientbound::internal_ids::JoinGame_WorldNames_IsHard_SimDist_LastDeath_PortalCooldown,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                crate::protocol::packet::play::clientbound::internal_ids::JoinGame_WorldNames_IsHard_SimDist_LastDeath_PortalCooldown,
                false,
            ),
            0x28,
        );
    }

    #[test]
    fn protocol_763_no_longer_treats_play_0x28_as_trade_list() {
        assert_ne!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                0x28,
                true
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Play,
                Direction::Clientbound,
                0x28,
                true
            ),
        );
    }

    #[test]
    fn protocol_763_maps_paper_entity_damage_boundary() {
        const PROTOCOL_1_20_1: i32 = 763;
        const WIRE_ENTITY_DAMAGE: i32 = 0x18;
        const ENTITY_DAMAGE_PAYLOAD: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        assert_eq!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_ENTITY_DAMAGE,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::EntityDamageRaw,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_ENTITY_DAMAGE,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::PluginMessageClientbound,
        );

        run_high_stack_packet_parse_fixture(move || {
            let mut cursor = std::io::Cursor::new(ENTITY_DAMAGE_PAYLOAD.to_vec());
            let packet = crate::protocol::packet::packet_by_id(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_ENTITY_DAMAGE,
                &mut cursor,
            )
            .expect("entity damage parses")
            .expect("entity damage packet exists");
            let crate::protocol::packet::Packet::EntityDamageRaw(packet) = packet else {
                panic!("expected EntityDamageRaw packet");
            };
            assert_eq!(packet.data, ENTITY_DAMAGE_PAYLOAD);
        });
    }

    #[test]
    fn protocol_763_maps_paper_damage_tilt_boundary() {
        const PROTOCOL_1_20_1: i32 = 763;
        const WIRE_DAMAGE_TILT: i32 = 0x21;
        const DAMAGE_TILT_PAYLOAD: &[u8] = &[0x01, 0x02, 0x03];
        assert_eq!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_DAMAGE_TILT,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::DamageTiltRaw,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_DAMAGE_TILT,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::KeepAliveClientbound_i64,
        );

        run_high_stack_packet_parse_fixture(move || {
            let mut cursor = std::io::Cursor::new(DAMAGE_TILT_PAYLOAD.to_vec());
            let packet = crate::protocol::packet::packet_by_id(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_DAMAGE_TILT,
                &mut cursor,
            )
            .expect("damage tilt parses")
            .expect("damage tilt packet exists");
            let crate::protocol::packet::Packet::DamageTiltRaw(packet) = packet else {
                panic!("expected DamageTiltRaw packet");
            };
            assert_eq!(packet.data, DAMAGE_TILT_PAYLOAD);
        });
    }

    #[test]
    fn protocol_763_maps_paper_combat_event_boundaries() {
        const PROTOCOL_1_20_1: i32 = 763;
        const WIRE_COMBAT_EVENT_END: i32 = 0x36;
        const WIRE_COMBAT_EVENT_ENTER: i32 = 0x37;
        assert_eq!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_END,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::CombatEventEndRaw,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_END,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::CombatEventEnd,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_ENTER,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::CombatEventEnter,
        );
        assert_ne!(
            translate_internal_packet_id_for_version(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_ENTER,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::FacePlayer,
        );

        run_high_stack_packet_parse_fixture(move || {
            let end_payload: Vec<u8> = Vec::new();
            let mut end_cursor = std::io::Cursor::new(end_payload);
            let end_packet = crate::protocol::packet::packet_by_id(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_END,
                &mut end_cursor,
            )
            .expect("end combat parses")
            .expect("end combat packet exists");
            let crate::protocol::packet::Packet::CombatEventEndRaw(packet) = end_packet else {
                panic!("expected CombatEventEndRaw packet");
            };
            assert!(packet.data.is_empty());

            let enter_payload: Vec<u8> = Vec::new();
            let mut enter_cursor = std::io::Cursor::new(enter_payload);
            let enter_packet = crate::protocol::packet::packet_by_id(
                PROTOCOL_1_20_1,
                State::Play,
                Direction::Clientbound,
                WIRE_COMBAT_EVENT_ENTER,
                &mut enter_cursor,
            )
            .expect("enter combat parses")
            .expect("enter combat packet exists");
            let crate::protocol::packet::Packet::CombatEventEnter(packet) = enter_packet else {
                panic!("expected CombatEventEnter packet");
            };
            assert!(packet.empty.is_empty());
        });
    }

    #[test]
    fn protocol_763_maps_valence_command_tree_boundary() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                0x10,
                true
            ),
            crate::protocol::packet::play::clientbound::internal_ids::DeclareCommandsRaw,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                crate::protocol::packet::play::clientbound::internal_ids::DeclareCommandsRaw,
                false,
            ),
            0x10,
        );
    }

    #[test]
    fn protocol_763_no_longer_treats_play_0x10_as_clear_titles() {
        assert_ne!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                0x10,
                true
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Play,
                Direction::Clientbound,
                0x10,
                true
            ),
        );
    }

    #[test]
    fn protocol_763_maps_valence_game_message_boundary() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                0x64,
                true
            ),
            crate::protocol::packet::play::clientbound::internal_ids::ServerMessage_Position,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                crate::protocol::packet::play::clientbound::internal_ids::ServerMessage_Position,
                false,
            ),
            0x64,
        );
    }

    #[test]
    fn protocol_763_no_longer_treats_play_0x64_as_entity_properties() {
        assert_ne!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                0x64,
                true
            ),
            translate_internal_packet_id_for_version(
                758,
                State::Play,
                Direction::Clientbound,
                0x64,
                true
            ),
        );
    }

    #[test]
    fn protocol_763_maps_paper_feature_flags_boundary() {
        const PAPER_FEATURE_FLAGS_WIRE_ID: i32 = 0x6b;
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                PAPER_FEATURE_FLAGS_WIRE_ID,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::FeatureFlags,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                crate::protocol::packet::play::clientbound::internal_ids::FeatureFlags,
                false,
            ),
            PAPER_FEATURE_FLAGS_WIRE_ID,
        );
    }

    #[test]
    fn feature_flags_consumes_1_20_payload() {
        const PAPER_FEATURE_FLAGS_WIRE_ID: i32 = 0x6b;
        const FEATURE_COUNT: usize = 2;
        const FEATURE_COUNT_VARINT: u8 = FEATURE_COUNT as u8;
        const VANILLA_FEATURE_NAME_BYTES: u8 = 17;
        const TRIAL_FEATURE_NAME_BYTES: u8 = 15;
        let payload = vec![
            FEATURE_COUNT_VARINT,
            VANILLA_FEATURE_NAME_BYTES,
            b'm',
            b'i',
            b'n',
            b'e',
            b'c',
            b'r',
            b'a',
            b'f',
            b't',
            b':',
            b'v',
            b'a',
            b'n',
            b'i',
            b'l',
            b'l',
            b'a',
            TRIAL_FEATURE_NAME_BYTES,
            b'm',
            b'i',
            b'n',
            b'e',
            b'c',
            b'r',
            b'a',
            b'f',
            b't',
            b':',
            b't',
            b'r',
            b'i',
            b'a',
            b'l',
        ];
        const TEST_PACKET_PARSE_STACK_BYTES: usize = 8 * 1024 * 1024;
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(move || {
                let mut payload = std::io::Cursor::new(payload);
                let packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    PAPER_FEATURE_FLAGS_WIRE_ID,
                    &mut payload,
                )
                .expect("feature flags parse")
                .expect("feature flags packet exists");
                let crate::protocol::packet::Packet::FeatureFlags(packet) = packet else {
                    panic!("expected FeatureFlags packet");
                };
                assert_eq!(packet.features.data.len(), FEATURE_COUNT);
                assert_eq!(packet.features.data[0], "minecraft:vanilla");
            })
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    #[test]
    fn protocol_763_maps_paper_entity_effect_boundary() {
        const PAPER_ENTITY_EFFECT_WIRE_ID: i32 = 0x6c;
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                PAPER_ENTITY_EFFECT_WIRE_ID,
                true,
            ),
            crate::protocol::packet::play::clientbound::internal_ids::EntityEffect_VarInt,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Clientbound,
                crate::protocol::packet::play::clientbound::internal_ids::EntityEffect_VarInt,
                false,
            ),
            PAPER_ENTITY_EFFECT_WIRE_ID,
        );
    }

    #[test]
    fn entity_effect_varint_consumes_1_20_factor_tail() {
        const PAPER_ENTITY_EFFECT_WIRE_ID: i32 = 0x6c;
        const FACTOR_TAIL_BYTES: usize = 14;
        const TEST_PACKET_PARSE_STACK_BYTES: usize = 8 * 1024 * 1024;
        let payload = vec![
            0x01, 0x02, 0x00, 0x14, 0x01, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x10, 0x20, 0x30,
            0x40, 0x50, 0x60, 0x70, 0x71,
        ];
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(move || {
                let mut payload = std::io::Cursor::new(payload);
                let packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    PAPER_ENTITY_EFFECT_WIRE_ID,
                    &mut payload,
                )
                .expect("entity effect parses")
                .expect("entity effect packet exists");
                let crate::protocol::packet::Packet::EntityEffect_VarInt(packet) = packet else {
                    panic!("expected EntityEffect_VarInt packet");
                };
                assert_eq!(packet.factor_data.len(), FACTOR_TAIL_BYTES);
                assert_eq!(packet.factor_data[0], 0xaa);
            })
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    #[test]
    fn protocol_763_window_property_consumes_exact_payload() {
        const PROTOCOL_1_20_1: i32 = 763;
        const WINDOW_PROPERTY_WIRE_ID: i32 = 0x13;
        const WINDOW_ID: u8 = 1;
        const PROPERTY_ID: i16 = 0;
        const PROPERTY_VALUE: i16 = 1;
        const ZERO_HIGH_BYTE: u8 = 0;
        const TRUNCATED_VALUE_BYTE: u8 = 0;
        let valid_payload = vec![
            WINDOW_ID,
            ZERO_HIGH_BYTE,
            PROPERTY_ID as u8,
            ZERO_HIGH_BYTE,
            PROPERTY_VALUE as u8,
        ];
        let mut valid_cursor = std::io::Cursor::new(valid_payload);
        let packet = crate::protocol::packet::packet_by_id(
            PROTOCOL_1_20_1,
            State::Play,
            Direction::Clientbound,
            WINDOW_PROPERTY_WIRE_ID,
            &mut valid_cursor,
        )
        .expect("window property parse")
        .expect("window property packet exists");
        let crate::protocol::packet::Packet::WindowProperty(packet) = packet else {
            panic!("expected WindowProperty packet");
        };
        assert_eq!(packet.id, WINDOW_ID);
        assert_eq!(packet.property, PROPERTY_ID);
        assert_eq!(packet.value, PROPERTY_VALUE);

        let truncated_payload = vec![
            WINDOW_ID,
            ZERO_HIGH_BYTE,
            PROPERTY_ID as u8,
            TRUNCATED_VALUE_BYTE,
        ];
        let mut truncated_cursor = std::io::Cursor::new(truncated_payload);
        assert!(crate::protocol::packet::packet_by_id(
            PROTOCOL_1_20_1,
            State::Play,
            Direction::Clientbound,
            WINDOW_PROPERTY_WIRE_ID,
            &mut truncated_cursor,
        )
        .is_err());
    }

    #[test]
    fn protocol_763_maps_remaining_observed_valence_boundaries() {
        let boundaries = [
            (0x00, crate::protocol::packet::play::clientbound::internal_ids::BundleDelimiterRaw),
            (0x01, crate::protocol::packet::play::clientbound::internal_ids::SpawnObject_VarInt_HeadYaw),
            (0x02, crate::protocol::packet::play::clientbound::internal_ids::SpawnExperienceOrb),
            (0x03, crate::protocol::packet::play::clientbound::internal_ids::SpawnPlayer_f64_NoMeta),
            (0x04, crate::protocol::packet::play::clientbound::internal_ids::Animation),
            (0x0a, crate::protocol::packet::play::clientbound::internal_ids::BlockChange_VarInt),
            (0x0b, crate::protocol::packet::play::clientbound::internal_ids::BossBar),
            (0x0c, crate::protocol::packet::play::clientbound::internal_ids::ServerDifficulty_Locked),
            (0x10, crate::protocol::packet::play::clientbound::internal_ids::DeclareCommandsRaw),
            (0x12, crate::protocol::packet::play::clientbound::internal_ids::WindowItems_StateCarry),
            (0x13, crate::protocol::packet::play::clientbound::internal_ids::WindowProperty),
            (0x14, crate::protocol::packet::play::clientbound::internal_ids::WindowSetSlot_State),
            (0x17, crate::protocol::packet::play::clientbound::internal_ids::PluginMessageClientbound),
            (0x1c, crate::protocol::packet::play::clientbound::internal_ids::EntityStatus),
            (0x1e, crate::protocol::packet::play::clientbound::internal_ids::ChunkUnload),
            (0x1f, crate::protocol::packet::play::clientbound::internal_ids::ChangeGameState),
            (0x22, crate::protocol::packet::play::clientbound::internal_ids::WorldBorderInit),
            (0x23, crate::protocol::packet::play::clientbound::internal_ids::KeepAliveClientbound_i64),
            (0x24, crate::protocol::packet::play::clientbound::internal_ids::ChunkData_AndLight_NoTrustEdges),
            (0x25, crate::protocol::packet::play::clientbound::internal_ids::WorldEventRaw),
            (0x26, crate::protocol::packet::play::clientbound::internal_ids::ParticleRaw),
            (0x27, crate::protocol::packet::play::clientbound::internal_ids::UpdateLightRaw),
            (0x2b, crate::protocol::packet::play::clientbound::internal_ids::EntityMove_i16),
            (0x2c, crate::protocol::packet::play::clientbound::internal_ids::EntityLookAndMove_i16),
            (0x2d, crate::protocol::packet::play::clientbound::internal_ids::EntityLook_VarInt),
            (0x2e, crate::protocol::packet::play::clientbound::internal_ids::VehicleTeleport),
            (0x34, crate::protocol::packet::play::clientbound::internal_ids::PlayerAbilities),
            (0x38, crate::protocol::packet::play::clientbound::internal_ids::DeathMessage_VarInt),
            (0x39, crate::protocol::packet::play::clientbound::internal_ids::PlayerRemove_UUIDs),
            (0x3a, crate::protocol::packet::play::clientbound::internal_ids::PlayerInfo_BitSet),
            (0x3c, crate::protocol::packet::play::clientbound::internal_ids::TeleportPlayer_WithConfirm),
            (0x3d, crate::protocol::packet::play::clientbound::internal_ids::UnlockRecipesRaw),
            (0x3e, crate::protocol::packet::play::clientbound::internal_ids::EntityDestroy),
            (0x41, crate::protocol::packet::play::clientbound::internal_ids::Respawn_WorldNames_LastDeath_PortalCooldown),
            (0x42, crate::protocol::packet::play::clientbound::internal_ids::EntityHeadLook),
            (0x43, crate::protocol::packet::play::clientbound::internal_ids::ChunkDeltaUpdateRaw),
            (0x45, crate::protocol::packet::play::clientbound::internal_ids::ServerMetadataRaw),
            (0x4d, crate::protocol::packet::play::clientbound::internal_ids::SetCurrentHotbarSlot),
            (0x4e, crate::protocol::packet::play::clientbound::internal_ids::UpdateViewPosition),
            (0x4f, crate::protocol::packet::play::clientbound::internal_ids::UpdateViewDistance),
            (0x50, crate::protocol::packet::play::clientbound::internal_ids::SpawnPosition_Angle),
            (0x51, crate::protocol::packet::play::clientbound::internal_ids::ScoreboardDisplay),
            (0x52, crate::protocol::packet::play::clientbound::internal_ids::EntityMetadata),
            (0x53, crate::protocol::packet::play::clientbound::internal_ids::EntityAttach),
            (0x54, crate::protocol::packet::play::clientbound::internal_ids::EntityVelocity),
            (0x55, crate::protocol::packet::play::clientbound::internal_ids::EntityEquipment_Array),
            (0x56, crate::protocol::packet::play::clientbound::internal_ids::SetExperience),
            (0x57, crate::protocol::packet::play::clientbound::internal_ids::UpdateHealth),
            (0x58, crate::protocol::packet::play::clientbound::internal_ids::ScoreboardObjective),
            (0x59, crate::protocol::packet::play::clientbound::internal_ids::SetPassengers),
            (0x5a, crate::protocol::packet::play::clientbound::internal_ids::Teams_VarInt),
            (0x5b, crate::protocol::packet::play::clientbound::internal_ids::UpdateScore_VarInt),
            (0x5c, crate::protocol::packet::play::clientbound::internal_ids::SimulationDistanceRaw),
            (0x5e, crate::protocol::packet::play::clientbound::internal_ids::TimeUpdate),
            (0x62, crate::protocol::packet::play::clientbound::internal_ids::PlaySoundRaw),
            (0x64, crate::protocol::packet::play::clientbound::internal_ids::ServerMessage_Position),
            (0x67, crate::protocol::packet::play::clientbound::internal_ids::CollectItem),
            (0x68, crate::protocol::packet::play::clientbound::internal_ids::EntityTeleport_f64),
            (0x69, crate::protocol::packet::play::clientbound::internal_ids::Advancements),
            (0x6a, crate::protocol::packet::play::clientbound::internal_ids::EntityProperties_VarIntVarInt),
            (0x6b, crate::protocol::packet::play::clientbound::internal_ids::FeatureFlags),
            (0x6c, crate::protocol::packet::play::clientbound::internal_ids::EntityEffect_VarInt),
            (0x6d, crate::protocol::packet::play::clientbound::internal_ids::SynchronizeRecipesRaw),
            (0x6e, crate::protocol::packet::play::clientbound::internal_ids::Tags_Nested),
        ];

        for (wire_id, internal_id) in boundaries {
            assert_eq!(
                translate_internal_packet_id_for_version(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    wire_id,
                    true,
                ),
                internal_id,
                "wire id 0x{wire_id:02x} should map to the expected Stevenarella internal id",
            );
            assert_eq!(
                translate_internal_packet_id_for_version(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    internal_id,
                    false,
                ),
                wire_id,
                "internal id {internal_id} should map back to wire id 0x{wire_id:02x}",
            );
        }
    }

    #[test]
    fn protocol_763_high_risk_raw_parser_fixtures_accept_payloads() {
        const TEST_PACKET_PARSE_STACK_BYTES: usize = 8 * 1024 * 1024;
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(move || {
                let command_payload = [0xde, 0xad, 0xbe, 0xef];
                let mut command_cursor = &command_payload[..];
                let command_packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    0x10,
                    &mut command_cursor,
                )
                .expect("command raw packet parses")
                .expect("command raw packet is known");
                let crate::protocol::packet::Packet::DeclareCommandsRaw(command_packet) =
                    command_packet
                else {
                    panic!("expected DeclareCommandsRaw packet");
                };
                assert_eq!(command_packet.data, command_payload);

                let chunk_delta_payload = [0xca, 0xfe, 0xba, 0xbe];
                let mut chunk_delta_cursor = &chunk_delta_payload[..];
                let chunk_delta_packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    0x43,
                    &mut chunk_delta_cursor,
                )
                .expect("chunk delta raw packet parses")
                .expect("chunk delta raw packet is known");
                let crate::protocol::packet::Packet::ChunkDeltaUpdateRaw(chunk_delta_packet) =
                    chunk_delta_packet
                else {
                    panic!("expected ChunkDeltaUpdateRaw packet");
                };
                assert_eq!(chunk_delta_packet.data, chunk_delta_payload);

                let particle_payload = [0x2a, 0x01, 0xff, 0x00];
                let mut particle_cursor = &particle_payload[..];
                let particle_packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    0x26,
                    &mut particle_cursor,
                )
                .expect("particle raw packet parses")
                .expect("particle raw packet is known");
                let crate::protocol::packet::Packet::ParticleRaw(particle_packet) = particle_packet
                else {
                    panic!("expected ParticleRaw packet");
                };
                assert_eq!(particle_packet.data, particle_payload);

                let recipe_payload = [0x13, 0x37, 0x00, 0x01];
                let mut recipe_cursor = &recipe_payload[..];
                let recipe_packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    0x6d,
                    &mut recipe_cursor,
                )
                .expect("recipe raw packet parses")
                .expect("recipe raw packet is known");
                let crate::protocol::packet::Packet::SynchronizeRecipesRaw(recipe_packet) =
                    recipe_packet
                else {
                    panic!("expected SynchronizeRecipesRaw packet");
                };
                assert_eq!(recipe_packet.data, recipe_payload);
            })
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    #[test]
    fn protocol_763_custom_payload_parser_fixture_accepts_brand_payload() {
        const TEST_PACKET_PARSE_STACK_BYTES: usize = 8 * 1024 * 1024;
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(move || {
                let payload = [
                    0x0f, b'm', b'i', b'n', b'e', b'c', b'r', b'a', b'f', b't', b':', b'b', b'r',
                    b'a', b'n', b'd', 0x05, b'P', b'a', b'p', b'e', b'r',
                ];
                let mut cursor = &payload[..];
                let packet = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Serverbound,
                    0x0d,
                    &mut cursor,
                )
                .expect("custom payload packet parses")
                .expect("custom payload packet is known");
                let crate::protocol::packet::Packet::PluginMessageServerbound(packet) = packet
                else {
                    panic!("expected PluginMessageServerbound packet");
                };
                assert_eq!(packet.channel, "minecraft:brand");
                assert_eq!(packet.data, [0x05, b'P', b'a', b'p', b'e', b'r']);
            })
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    #[test]
    fn protocol_763_custom_payload_parser_fixture_rejects_malformed_channel() {
        const TEST_PACKET_PARSE_STACK_BYTES: usize = 8 * 1024 * 1024;
        std::thread::Builder::new()
            .stack_size(TEST_PACKET_PARSE_STACK_BYTES)
            .spawn(move || {
                let invalid_utf8_channel = [0x01, 0xff, 0x00];
                let mut invalid_utf8_cursor = &invalid_utf8_channel[..];
                let invalid_utf8 = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Serverbound,
                    0x0d,
                    &mut invalid_utf8_cursor,
                )
                .expect_err("invalid UTF-8 channel is rejected");
                assert!(
                    invalid_utf8.to_string().contains("Invalid UTF-8 string"),
                    "unexpected error: {invalid_utf8}"
                );

                let oversized_channel_len = [0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
                let mut oversized_cursor = &oversized_channel_len[..];
                let oversized = crate::protocol::packet::packet_by_id(
                    763,
                    State::Play,
                    Direction::Serverbound,
                    0x0d,
                    &mut oversized_cursor,
                )
                .expect_err("oversized channel length is rejected");
                assert!(
                    oversized.to_string().contains("VarInt too big"),
                    "unexpected error: {oversized}"
                );
            })
            .expect("spawn packet parse test")
            .join()
            .expect("packet parse test passes");
    }

    #[test]
    fn protocol_763_maps_play_keep_alive_response() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Serverbound,
                crate::protocol::packet::play::serverbound::internal_ids::KeepAliveServerbound_i64,
                false,
            ),
            0x12,
        );
    }

    #[test]
    fn protocol_763_maps_play_position_updates() {
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Serverbound,
                crate::protocol::packet::play::serverbound::internal_ids::PlayerPosition,
                false,
            ),
            0x14,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Serverbound,
                0x14,
                true,
            ),
            crate::protocol::packet::play::serverbound::internal_ids::PlayerPosition,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Serverbound,
                crate::protocol::packet::play::serverbound::internal_ids::PlayerPositionLook,
                false,
            ),
            0x15,
        );
        assert_eq!(
            translate_internal_packet_id_for_version(
                763,
                State::Play,
                Direction::Serverbound,
                0x15,
                true,
            ),
            crate::protocol::packet::play::serverbound::internal_ids::PlayerPositionLook,
        );
    }

    #[test]
    fn protocol_763_maps_play_interaction_packets() {
        let boundaries = [
            (
                0x07,
                crate::protocol::packet::play::serverbound::internal_ids::ClientStatus,
            ),
            (
                0x0c,
                crate::protocol::packet::play::serverbound::internal_ids::CloseWindow,
            ),
            (
                0x0d,
                crate::protocol::packet::play::serverbound::internal_ids::PluginMessageServerbound,
            ),
            (
                0x10,
                crate::protocol::packet::play::serverbound::internal_ids::UseEntity_Sneakflag,
            ),
            (
                0x1d,
                crate::protocol::packet::play::serverbound::internal_ids::PlayerDigging_WithSequence,
            ),
            (
                0x28,
                crate::protocol::packet::play::serverbound::internal_ids::HeldItemChange,
            ),
            (
                0x31,
                crate::protocol::packet::play::serverbound::internal_ids::PlayerBlockPlacement_insideblock_sequence,
            ),
            (
                0x32,
                crate::protocol::packet::play::serverbound::internal_ids::UseItem_WithSequence,
            ),
        ];

        for (wire_id, internal_id) in boundaries {
            assert_eq!(
                translate_internal_packet_id_for_version(
                    763,
                    State::Play,
                    Direction::Serverbound,
                    internal_id,
                    false,
                ),
                wire_id,
            );
            assert_eq!(
                translate_internal_packet_id_for_version(
                    763,
                    State::Play,
                    Direction::Serverbound,
                    wire_id,
                    true,
                ),
                internal_id,
            );
        }
    }

    #[test]
    fn protocol_763_no_longer_uses_758_fallback_for_remaining_observed_boundaries() {
        for wire_id in [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x06, 0x0a, 0x0b, 0x0c, 0x14, 0x17, 0x1c, 0x1e, 0x1f,
            0x22, 0x24, 0x25, 0x27, 0x2b, 0x2c, 0x2d, 0x2e, 0x34, 0x38, 0x39, 0x3a, 0x3d, 0x3e,
            0x42, 0x43, 0x45, 0x4d, 0x4e, 0x4f, 0x51, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a,
            0x5b, 0x5c, 0x5e, 0x62, 0x67,
        ] {
            assert_ne!(
                translate_internal_packet_id_for_version(
                    763,
                    State::Play,
                    Direction::Clientbound,
                    wire_id,
                    true,
                ),
                translate_internal_packet_id_for_version(
                    758,
                    State::Play,
                    Direction::Clientbound,
                    wire_id,
                    true,
                ),
                "wire id 0x{wire_id:02x} should not inherit the protocol 758 mapping",
            );
        }
    }
}
