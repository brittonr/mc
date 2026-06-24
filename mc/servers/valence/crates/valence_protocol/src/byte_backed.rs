use std::ops::Range;

use bytes::Bytes;
use thiserror::Error;

/// A shared, byte-backed packet body.
///
/// The body owns a [`Bytes`] handle, so cloned values keep the packet storage
/// alive without tying callers to a borrow of the decoder buffer. The bytes are
/// raw packet-body bytes and do not include the leading packet ID.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ByteBackedPacketBody {
    bytes: Bytes,
}

/// A validated byte field backed by shared packet bytes.
///
/// Constructors check the configured byte bound before this type is exposed.
/// The type owns a [`Bytes`] slice, so it can safely outlive the packet body it
/// was sliced from.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ByteBackedBytes {
    bytes: Bytes,
}

/// A UTF-8 string field backed by shared packet bytes.
///
/// Constructors validate UTF-8 and the configured UTF-16 character bound used
/// by the Minecraft protocol. The type owns a [`Bytes`] slice, so it can safely
/// outlive the packet body it was sliced from.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ByteBackedStr {
    bytes: Bytes,
    utf16_char_count: usize,
}

/// Errors returned while constructing byte-backed packet values.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Error)]
pub enum ByteBackedError {
    /// A requested slice range is outside the owner bytes.
    #[error("byte-backed range {start}..{end} is invalid for owner length {owner_len}")]
    RangeOutOfBounds {
        /// Requested range start.
        start: usize,
        /// Requested range end.
        end: usize,
        /// Length of the owning byte buffer.
        owner_len: usize,
    },
    /// A byte field exceeded its configured byte length.
    #[error("byte-backed field length {len} exceeds maximum {max_len}")]
    ByteLengthExceeded {
        /// Actual byte length.
        len: usize,
        /// Configured maximum byte length.
        max_len: usize,
    },
    /// A string field was not valid UTF-8.
    #[error("byte-backed string is invalid UTF-8 at byte {valid_up_to}")]
    InvalidUtf8 {
        /// The prefix length that was valid UTF-8.
        valid_up_to: usize,
        /// The invalid sequence length when known.
        error_len: Option<usize>,
    },
    /// A UTF-8 string exceeded its configured UTF-16 character length.
    #[error("byte-backed string UTF-16 char count {char_count} exceeds maximum {max_chars}")]
    Utf16LengthExceeded {
        /// Actual UTF-16 character count.
        char_count: usize,
        /// Configured maximum UTF-16 character count.
        max_chars: usize,
    },
}

impl ByteBackedPacketBody {
    /// Creates a packet body from already-owned shared bytes.
    #[must_use]
    pub const fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    /// Copies a byte slice into a shared packet body.
    #[must_use]
    pub fn copy_from_slice(bytes: &[u8]) -> Self {
        Self {
            bytes: Bytes::copy_from_slice(bytes),
        }
    }

    /// Returns the packet body as bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the byte length of the packet body.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the packet body is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Clones the shared byte handle.
    #[must_use]
    pub fn clone_bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    /// Consumes this body and returns the shared byte handle.
    #[must_use]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }

    /// Returns a checked shared slice of this packet body.
    pub fn slice(&self, range: Range<usize>) -> Result<Self, ByteBackedError> {
        Ok(Self::new(slice_checked(&self.bytes, range)?))
    }

    /// Returns a checked byte field from this packet body.
    pub fn byte_field(
        &self,
        range: Range<usize>,
        max_len: usize,
    ) -> Result<ByteBackedBytes, ByteBackedError> {
        ByteBackedBytes::new(slice_checked(&self.bytes, range)?, max_len)
    }

    /// Returns a checked UTF-8 field from this packet body.
    pub fn utf8_field(
        &self,
        range: Range<usize>,
        max_chars: usize,
    ) -> Result<ByteBackedStr, ByteBackedError> {
        ByteBackedStr::new(slice_checked(&self.bytes, range)?, max_chars)
    }
}

impl ByteBackedBytes {
    /// Creates a byte field from shared bytes after checking `max_len`.
    pub fn new(bytes: Bytes, max_len: usize) -> Result<Self, ByteBackedError> {
        validate_byte_len(bytes.len(), max_len)?;
        Ok(Self { bytes })
    }

    /// Copies a byte slice into a byte field after checking `max_len`.
    pub fn copy_from_slice(bytes: &[u8], max_len: usize) -> Result<Self, ByteBackedError> {
        Self::new(Bytes::copy_from_slice(bytes), max_len)
    }

    /// Creates a byte field by slicing a shared packet body.
    pub fn from_body_range(
        body: &ByteBackedPacketBody,
        range: Range<usize>,
        max_len: usize,
    ) -> Result<Self, ByteBackedError> {
        body.byte_field(range, max_len)
    }

    /// Returns the validated bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the byte length of the field.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the field is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Clones the shared byte handle.
    #[must_use]
    pub fn clone_bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    /// Consumes this field and returns the shared byte handle.
    #[must_use]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }
}

impl ByteBackedStr {
    /// Creates a UTF-8 field from shared bytes after checking `max_chars`.
    pub fn new(bytes: Bytes, max_chars: usize) -> Result<Self, ByteBackedError> {
        let text = validate_utf8(&bytes)?;
        let utf16_char_count = text.encode_utf16().count();
        validate_utf16_len(utf16_char_count, max_chars)?;
        Ok(Self {
            bytes,
            utf16_char_count,
        })
    }

    /// Copies a string slice into a UTF-8 field after checking `max_chars`.
    pub fn copy_from_str(text: &str, max_chars: usize) -> Result<Self, ByteBackedError> {
        Self::new(Bytes::copy_from_slice(text.as_bytes()), max_chars)
    }

    /// Creates a UTF-8 field by slicing a shared packet body.
    pub fn from_body_range(
        body: &ByteBackedPacketBody,
        range: Range<usize>,
        max_chars: usize,
    ) -> Result<Self, ByteBackedError> {
        body.utf8_field(range, max_chars)
    }

    /// Returns the validated string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).expect("ByteBackedStr stores valid UTF-8")
    }

    /// Returns the validated string bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the byte length of the field.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns whether the field is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns the UTF-16 character count validated for this field.
    #[must_use]
    pub const fn utf16_char_count(&self) -> usize {
        self.utf16_char_count
    }

    /// Clones the shared byte handle.
    #[must_use]
    pub fn clone_bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    /// Consumes this field and returns the shared byte handle.
    #[must_use]
    pub fn into_bytes(self) -> Bytes {
        self.bytes
    }
}

impl From<Bytes> for ByteBackedPacketBody {
    fn from(bytes: Bytes) -> Self {
        Self::new(bytes)
    }
}

impl From<ByteBackedPacketBody> for Bytes {
    fn from(body: ByteBackedPacketBody) -> Self {
        body.into_bytes()
    }
}

fn slice_checked(bytes: &Bytes, range: Range<usize>) -> Result<Bytes, ByteBackedError> {
    let len = bytes.len();
    if range.start > range.end || range.end > len {
        return Err(ByteBackedError::RangeOutOfBounds {
            start: range.start,
            end: range.end,
            owner_len: len,
        });
    }

    Ok(bytes.slice(range))
}

fn validate_byte_len(len: usize, max_len: usize) -> Result<(), ByteBackedError> {
    if len > max_len {
        return Err(ByteBackedError::ByteLengthExceeded { len, max_len });
    }

    Ok(())
}

fn validate_utf8(bytes: &[u8]) -> Result<&str, ByteBackedError> {
    std::str::from_utf8(bytes).map_err(|err| ByteBackedError::InvalidUtf8 {
        valid_up_to: err.valid_up_to(),
        error_len: err.error_len(),
    })
}

fn validate_utf16_len(char_count: usize, max_chars: usize) -> Result<(), ByteBackedError> {
    if char_count > max_chars {
        return Err(ByteBackedError::Utf16LengthExceeded {
            char_count,
            max_chars,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PACKET_BYTES: &[u8] = b"prefix-field-suffix";
    const FIELD_START: usize = 7;
    const FIELD_END: usize = 12;
    const OUT_OF_BOUNDS_START: usize = 12;
    const OUT_OF_BOUNDS_END: usize = 32;
    const MAX_FIELD_BYTES: usize = 5;
    const TOO_SMALL_FIELD_BYTES: usize = 4;
    const VALID_TEXT: &str = "field";
    const WIDE_TEXT: &str = "\u{1f680}";
    const MAX_VALID_TEXT_CHARS: usize = 5;
    const MAX_ONE_UTF16_CHAR: usize = 1;
    const WIDE_TEXT_UTF16_CHARS: usize = 2;
    const INVALID_UTF8: &[u8] = &[0xff];
    const INVALID_UTF8_VALID_PREFIX: usize = 0;
    const EXPECTED_FIELD: &[u8] = b"field";

    #[test]
    fn byte_field_slices_shared_packet_body() {
        let field = {
            let body = ByteBackedPacketBody::copy_from_slice(PACKET_BYTES);
            ByteBackedBytes::from_body_range(&body, FIELD_START..FIELD_END, MAX_FIELD_BYTES)
                .unwrap()
        };

        assert_eq!(field.as_bytes(), EXPECTED_FIELD);
        assert_eq!(field.len(), MAX_FIELD_BYTES);
    }

    #[test]
    fn byte_field_rejects_oversized_field() {
        let body = ByteBackedPacketBody::copy_from_slice(PACKET_BYTES);
        let err =
            ByteBackedBytes::from_body_range(&body, FIELD_START..FIELD_END, TOO_SMALL_FIELD_BYTES)
                .unwrap_err();

        assert_eq!(
            err,
            ByteBackedError::ByteLengthExceeded {
                len: MAX_FIELD_BYTES,
                max_len: TOO_SMALL_FIELD_BYTES,
            }
        );
    }

    #[test]
    fn byte_field_rejects_stale_or_invalid_owner_range() {
        let body = ByteBackedPacketBody::copy_from_slice(PACKET_BYTES);
        let err = ByteBackedBytes::from_body_range(
            &body,
            OUT_OF_BOUNDS_START..OUT_OF_BOUNDS_END,
            MAX_FIELD_BYTES,
        )
        .unwrap_err();

        assert_eq!(
            err,
            ByteBackedError::RangeOutOfBounds {
                start: OUT_OF_BOUNDS_START,
                end: OUT_OF_BOUNDS_END,
                owner_len: PACKET_BYTES.len(),
            }
        );
    }

    #[test]
    fn utf8_field_validates_text_and_ownership() {
        let field = {
            let body = ByteBackedPacketBody::copy_from_slice(PACKET_BYTES);
            ByteBackedStr::from_body_range(&body, FIELD_START..FIELD_END, MAX_VALID_TEXT_CHARS)
                .unwrap()
        };

        assert_eq!(field.as_str(), VALID_TEXT);
        assert_eq!(field.as_bytes(), EXPECTED_FIELD);
        assert_eq!(field.utf16_char_count(), MAX_VALID_TEXT_CHARS);
    }

    #[test]
    fn utf8_field_rejects_invalid_utf8() {
        let err = ByteBackedStr::new(Bytes::copy_from_slice(INVALID_UTF8), MAX_VALID_TEXT_CHARS)
            .unwrap_err();

        assert_eq!(
            err,
            ByteBackedError::InvalidUtf8 {
                valid_up_to: INVALID_UTF8_VALID_PREFIX,
                error_len: Some(INVALID_UTF8.len()),
            }
        );
    }

    #[test]
    fn utf8_field_rejects_utf16_char_overflow() {
        let err = ByteBackedStr::copy_from_str(WIDE_TEXT, MAX_ONE_UTF16_CHAR).unwrap_err();

        assert_eq!(
            err,
            ByteBackedError::Utf16LengthExceeded {
                char_count: WIDE_TEXT_UTF16_CHARS,
                max_chars: MAX_ONE_UTF16_CHAR,
            }
        );
    }
}
