use std::error::Error as StdError;
use std::fmt;
use std::ops::Range;

use bevy_ecs::prelude::*;
use tracing::warn;
use valence_protocol::Encode;

const TRACKED_DATA_TERMINATOR_INDEX: u8 = 0xff;
const TRACKED_DATA_ENTRY_HEADER_LEN: usize = 2;

/// Error returned when tracked entity data cannot be encoded safely.
#[derive(Debug)]
pub enum TrackedDataError {
    /// The metadata index is reserved for the packet terminator.
    ReservedTerminatorIndex { index: u8 },
    /// Encoding the typed metadata value failed.
    EncodeFailed { source: anyhow::Error },
}

impl fmt::Display for TrackedDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReservedTerminatorIndex { index } => {
                write!(
                    f,
                    "tracked data index {index:#04x} is reserved for the terminator"
                )
            }
            Self::EncodeFailed { source } => {
                write!(f, "failed to encode tracked data value: {source:#}")
            }
        }
    }
}

impl StdError for TrackedDataError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::ReservedTerminatorIndex { .. } => None,
            Self::EncodeFailed { source } => Some(source.as_ref()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct TrackedDataEntry {
    index: u8,
    byte_len: usize,
}

/// Cache for all the tracked data of an entity. Used for the
/// [`EntityTrackerUpdateS2c`][packet] packet.
///
/// [packet]: valence_protocol::packets::play::EntityTrackerUpdateS2c
#[derive(Component, Default, Debug)]
pub struct TrackedData {
    init_data: Vec<u8>,
    /// A map of tracked data indices to the byte length of the entry in
    /// `init_data`.
    init_entries: Vec<TrackedDataEntry>,
    update_data: Vec<u8>,
    /// A map of tracked data indices to the byte length of the entry in
    /// `update_data`.
    update_entries: Vec<TrackedDataEntry>,
}

impl TrackedData {
    /// Returns initial tracked data for the entity, ready to be sent in the
    /// [`EntityTrackerUpdateS2c`][packet] packet. This is used when the entity
    /// enters the view of a client.
    ///
    /// [packet]: valence_protocol::packets::play::EntityTrackerUpdateS2c
    pub fn init_data(&self) -> Option<&[u8]> {
        (self.init_data.len() > 1).then_some(&self.init_data)
    }

    /// Contains updated tracked data for the entity, ready to be sent in the
    /// [`EntityTrackerUpdateS2c`][packet] packet. This is used when tracked
    /// data is changed and the client is already in view of the entity.
    ///
    /// [packet]: valence_protocol::packets::play::EntityTrackerUpdateS2c
    pub fn update_data(&self) -> Option<&[u8]> {
        (self.update_data.len() > 1).then_some(&self.update_data)
    }

    /// Inserts or replaces an initial tracked value.
    ///
    /// Invalid metadata indices or encoding failures are logged and leave the
    /// tracked-data cache unchanged.
    pub fn insert_init_value<V: Encode>(&mut self, index: u8, type_id: u8, value: V) {
        if let Err(error) = self.try_insert_init_value(index, type_id, value) {
            warn!("failed to insert initial tracked data: {error:#}");
        }
    }

    /// Inserts or replaces an initial tracked value, returning a deterministic
    /// error when the value cannot be encoded safely.
    pub fn try_insert_init_value<V: Encode>(
        &mut self,
        index: u8,
        type_id: u8,
        value: V,
    ) -> Result<(), TrackedDataError> {
        let encoded_entry = encode_tracked_data_entry(index, type_id, value)?;

        remove_tracked_entry(&mut self.init_data, &mut self.init_entries, index);
        remove_packet_terminator(&mut self.init_data);
        append_tracked_entry(
            &mut self.init_data,
            &mut self.init_entries,
            index,
            &encoded_entry,
        );
        self.init_data.push(TRACKED_DATA_TERMINATOR_INDEX);

        Ok(())
    }

    /// Removes an initial tracked value.
    pub fn remove_init_value(&mut self, index: u8) -> bool {
        if let Err(error) = validate_tracked_data_index(index) {
            warn!("failed to remove initial tracked data: {error:#}");
            return false;
        }

        remove_tracked_entry(&mut self.init_data, &mut self.init_entries, index)
    }

    /// Appends an updated tracked value.
    ///
    /// Repeated updates for the same metadata index during one flush window are
    /// collapsed to the final encoded value for that index. Invalid metadata
    /// indices or encoding failures are logged and leave the update cache
    /// unchanged.
    pub fn append_update_value<V: Encode>(&mut self, index: u8, type_id: u8, value: V) {
        if let Err(error) = self.try_append_update_value(index, type_id, value) {
            warn!("failed to append updated tracked data: {error:#}");
        }
    }

    /// Appends an updated tracked value, returning a deterministic error when
    /// the value cannot be encoded safely.
    pub fn try_append_update_value<V: Encode>(
        &mut self,
        index: u8,
        type_id: u8,
        value: V,
    ) -> Result<(), TrackedDataError> {
        let encoded_entry = encode_tracked_data_entry(index, type_id, value)?;

        remove_tracked_entry(&mut self.update_data, &mut self.update_entries, index);
        remove_packet_terminator(&mut self.update_data);
        append_tracked_entry(
            &mut self.update_data,
            &mut self.update_entries,
            index,
            &encoded_entry,
        );
        self.update_data.push(TRACKED_DATA_TERMINATOR_INDEX);

        Ok(())
    }

    /// Clears all updated tracked values queued for already-visible clients.
    pub fn clear_update_values(&mut self) {
        self.update_data.clear();
        self.update_entries.clear();
    }
}

fn validate_tracked_data_index(index: u8) -> Result<(), TrackedDataError> {
    if index == TRACKED_DATA_TERMINATOR_INDEX {
        return Err(TrackedDataError::ReservedTerminatorIndex { index });
    }

    Ok(())
}

fn encode_tracked_data_entry<V: Encode>(
    index: u8,
    type_id: u8,
    value: V,
) -> Result<Vec<u8>, TrackedDataError> {
    validate_tracked_data_index(index)?;

    let mut encoded_entry = Vec::with_capacity(TRACKED_DATA_ENTRY_HEADER_LEN);
    encoded_entry.extend_from_slice(&[index, type_id]);
    value
        .encode(&mut encoded_entry)
        .map_err(|source| TrackedDataError::EncodeFailed { source })?;

    Ok(encoded_entry)
}

fn find_tracked_entry_range(
    entries: &[TrackedDataEntry],
    index: u8,
) -> Option<(usize, Range<usize>)> {
    let mut start = 0;

    for (position, entry) in entries.iter().enumerate() {
        let end = start + entry.byte_len;

        if entry.index == index {
            return Some((position, start..end));
        }

        start = end;
    }

    None
}

fn remove_tracked_entry(
    data: &mut Vec<u8>,
    entries: &mut Vec<TrackedDataEntry>,
    index: u8,
) -> bool {
    if let Some((position, range)) = find_tracked_entry_range(entries, index) {
        data.drain(range);
        entries.remove(position);
        return true;
    }

    false
}

fn append_tracked_entry(
    data: &mut Vec<u8>,
    entries: &mut Vec<TrackedDataEntry>,
    index: u8,
    encoded_entry: &[u8],
) {
    data.extend_from_slice(encoded_entry);
    entries.push(TrackedDataEntry {
        index,
        byte_len: encoded_entry.len(),
    });
}

fn remove_packet_terminator(data: &mut Vec<u8>) {
    if data.last() == Some(&TRACKED_DATA_TERMINATOR_INDEX) {
        data.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLAGS_INDEX: u8 = 0;
    const CUSTOM_NAME_INDEX: u8 = 2;
    const BYTE_TYPE_ID: u8 = 0;
    const STRING_TYPE_ID: u8 = 4;
    const FIRST_FLAGS_VALUE: u8 = 0b0000_0001;
    const SECOND_FLAGS_VALUE: u8 = 0b0000_0010;
    const DEFAULT_FLAGS_VALUE: u8 = 0;
    const CUSTOM_NAME_VALUE: &str = "name";
    const REPLACEMENT_CUSTOM_NAME_VALUE: &str = "replacement";

    #[derive(Debug)]
    struct FailingEncode;

    impl Encode for FailingEncode {
        fn encode(&self, _w: impl std::io::Write) -> anyhow::Result<()> {
            anyhow::bail!("intentional tracked-data fixture failure")
        }
    }

    #[test]
    fn insert_remove_init_tracked_data() {
        let mut tracked_data = TrackedData::default();

        tracked_data.insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE);
        tracked_data.insert_init_value(CUSTOM_NAME_INDEX, STRING_TYPE_ID, CUSTOM_NAME_VALUE);

        assert!(tracked_data.remove_init_value(CUSTOM_NAME_INDEX));
        assert!(!tracked_data.remove_init_value(CUSTOM_NAME_INDEX));

        // Insertion overwrites value at the same index.
        tracked_data.insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, SECOND_FLAGS_VALUE);

        assert!(tracked_data.remove_init_value(FLAGS_INDEX));

        assert!(
            tracked_data.init_data.as_slice().is_empty()
                || tracked_data.init_data.as_slice() == [TRACKED_DATA_TERMINATOR_INDEX]
        );
        assert!(tracked_data.init_data().is_none());

        assert!(tracked_data.update_data.is_empty());
    }

    #[test]
    fn unchanged_tracked_data_emits_no_packets() {
        let tracked_data = TrackedData::default();

        assert!(tracked_data.init_data().is_none());
        assert!(tracked_data.update_data().is_none());
    }

    #[test]
    fn changed_tracked_data_emits_init_and_update_bytes() {
        let mut tracked_data = TrackedData::default();
        let expected_bytes = [
            FLAGS_INDEX,
            BYTE_TYPE_ID,
            FIRST_FLAGS_VALUE,
            TRACKED_DATA_TERMINATOR_INDEX,
        ];

        tracked_data
            .try_insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();
        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();

        assert_eq!(tracked_data.init_data(), Some(expected_bytes.as_slice()));
        assert_eq!(tracked_data.update_data(), Some(expected_bytes.as_slice()));
    }

    #[test]
    fn same_tick_updates_keep_final_value_per_index() {
        let mut tracked_data = TrackedData::default();
        let expected_bytes = [
            FLAGS_INDEX,
            BYTE_TYPE_ID,
            SECOND_FLAGS_VALUE,
            CUSTOM_NAME_INDEX,
            STRING_TYPE_ID,
            REPLACEMENT_CUSTOM_NAME_VALUE.len() as u8,
            b'r',
            b'e',
            b'p',
            b'l',
            b'a',
            b'c',
            b'e',
            b'm',
            b'e',
            b'n',
            b't',
            TRACKED_DATA_TERMINATOR_INDEX,
        ];

        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();
        tracked_data
            .try_append_update_value(CUSTOM_NAME_INDEX, STRING_TYPE_ID, CUSTOM_NAME_VALUE)
            .unwrap();
        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, SECOND_FLAGS_VALUE)
            .unwrap();
        tracked_data
            .try_append_update_value(
                CUSTOM_NAME_INDEX,
                STRING_TYPE_ID,
                REPLACEMENT_CUSTOM_NAME_VALUE,
            )
            .unwrap();

        assert_eq!(tracked_data.update_data(), Some(expected_bytes.as_slice()));
    }

    #[test]
    fn default_values_are_suppressed_for_spawn_but_update_resets_client() {
        let mut tracked_data = TrackedData::default();
        let expected_update = [
            FLAGS_INDEX,
            BYTE_TYPE_ID,
            DEFAULT_FLAGS_VALUE,
            TRACKED_DATA_TERMINATOR_INDEX,
        ];

        tracked_data
            .try_insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();
        assert!(tracked_data.remove_init_value(FLAGS_INDEX));
        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, DEFAULT_FLAGS_VALUE)
            .unwrap();

        assert!(tracked_data.init_data().is_none());
        assert_eq!(tracked_data.update_data(), Some(expected_update.as_slice()));
    }

    #[test]
    fn invalid_metadata_index_fails_closed() {
        let mut tracked_data = TrackedData::default();

        let init_error = tracked_data
            .try_insert_init_value(
                TRACKED_DATA_TERMINATOR_INDEX,
                BYTE_TYPE_ID,
                FIRST_FLAGS_VALUE,
            )
            .unwrap_err();
        let update_error = tracked_data
            .try_append_update_value(
                TRACKED_DATA_TERMINATOR_INDEX,
                BYTE_TYPE_ID,
                FIRST_FLAGS_VALUE,
            )
            .unwrap_err();

        assert!(matches!(
            init_error,
            TrackedDataError::ReservedTerminatorIndex { .. }
        ));
        assert!(matches!(
            update_error,
            TrackedDataError::ReservedTerminatorIndex { .. }
        ));
        assert!(!tracked_data.remove_init_value(TRACKED_DATA_TERMINATOR_INDEX));
        assert!(tracked_data.init_data().is_none());
        assert!(tracked_data.update_data().is_none());
    }

    #[test]
    fn encoding_failure_does_not_mutate_existing_entries() {
        let mut tracked_data = TrackedData::default();
        tracked_data
            .try_insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();
        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();

        let init_before = tracked_data.init_data().unwrap().to_vec();
        let update_before = tracked_data.update_data().unwrap().to_vec();

        assert!(matches!(
            tracked_data.try_insert_init_value(FLAGS_INDEX, BYTE_TYPE_ID, FailingEncode),
            Err(TrackedDataError::EncodeFailed { .. })
        ));
        assert!(matches!(
            tracked_data.try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, FailingEncode),
            Err(TrackedDataError::EncodeFailed { .. })
        ));

        assert_eq!(tracked_data.init_data(), Some(init_before.as_slice()));
        assert_eq!(tracked_data.update_data(), Some(update_before.as_slice()));
    }

    #[test]
    fn clearing_update_values_removes_despawn_stale_metadata() {
        let mut tracked_data = TrackedData::default();

        tracked_data
            .try_append_update_value(FLAGS_INDEX, BYTE_TYPE_ID, FIRST_FLAGS_VALUE)
            .unwrap();
        tracked_data.clear_update_values();

        assert!(tracked_data.update_data().is_none());
        assert!(tracked_data.update_entries.is_empty());
    }
}
