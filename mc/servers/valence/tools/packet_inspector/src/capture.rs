use std::fmt;
use std::num::NonZeroUsize;

use serde::Serialize;
use valence_protocol::{PacketSide, PacketState};

use crate::packet_registry::Packet;

pub const DEFAULT_MAX_PACKET_BYTES: usize = 512;
const MINECRAFT_VARINT_MAX_BYTES: usize = 5;
const VARINT_SEGMENT_BITS: u32 = 7;
const VARINT_CONTINUE_BIT: u8 = 0b1000_0000;
const VARINT_VALUE_MASK: i32 = 0b0111_1111;
const HEX_BYTE_WIDTH: usize = 2;
const HEX_SEPARATOR: &str = " ";
const REDACTED_PAYLOAD_LABEL: &str = "<redacted>";
const EMPTY_PAYLOAD_LABEL: &str = "<empty>";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CapturePolicy {
    max_packet_bytes: NonZeroUsize,
    redact_payloads: bool,
}

impl CapturePolicy {
    pub fn new(max_packet_bytes: usize, redact_payloads: bool) -> Result<Self, CapturePolicyError> {
        let max_packet_bytes =
            NonZeroUsize::new(max_packet_bytes).ok_or(CapturePolicyError::MaxPacketBytesZero)?;
        Ok(Self {
            max_packet_bytes,
            redact_payloads,
        })
    }

    pub fn max_packet_bytes(&self) -> usize {
        self.max_packet_bytes.get()
    }

    pub fn redact_payloads(&self) -> bool {
        self.redact_payloads
    }
}

impl Default for CapturePolicy {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_PACKET_BYTES, true).expect("default capture policy is valid")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CapturePolicyError {
    MaxPacketBytesZero,
}

impl fmt::Display for CapturePolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MaxPacketBytesZero => write!(f, "max packet bytes must be greater than zero"),
        }
    }
}

impl std::error::Error for CapturePolicyError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PacketCaptureSummary {
    pub side: PacketSide,
    pub state: PacketState,
    pub id: i32,
    pub name: &'static str,
    pub body_len: usize,
    pub body_preview: String,
    pub truncated: bool,
    pub redacted: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CaptureByteSummary {
    pub declared_packet_len: usize,
    pub available_packet_len: usize,
    pub truncated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CaptureDecodeError {
    EmptyCapture,
    MalformedPacketLengthVarInt,
    PacketTooLarge {
        declared_len: usize,
        max_len: usize,
    },
    IncompletePacket {
        declared_len: usize,
        available_len: usize,
    },
}

impl fmt::Display for CaptureDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCapture => write!(f, "capture is empty at packet length boundary"),
            Self::MalformedPacketLengthVarInt => {
                write!(f, "malformed capture at packet length VarInt boundary")
            }
            Self::PacketTooLarge {
                declared_len,
                max_len,
            } => write!(
                f,
                "capture declares packet length {declared_len}, exceeding configured max {max_len}"
            ),
            Self::IncompletePacket {
                declared_len,
                available_len,
            } => write!(
                f,
                "capture declares packet length {declared_len}, but only {available_len} bytes are available"
            ),
        }
    }
}

impl std::error::Error for CaptureDecodeError {}

pub fn summarize_packet(packet: &Packet, policy: &CapturePolicy) -> PacketCaptureSummary {
    let body_len = packet.data.as_ref().map_or(0, bytes::Bytes::len);
    let (body_preview, truncated) = packet
        .data
        .as_ref()
        .map(|body| payload_preview(body, policy))
        .unwrap_or_else(|| (EMPTY_PAYLOAD_LABEL.to_owned(), false));

    PacketCaptureSummary {
        side: packet.side,
        state: packet.state,
        id: packet.id,
        name: packet.name,
        body_len,
        body_preview,
        truncated,
        redacted: policy.redact_payloads(),
    }
}

pub fn inspect_capture_bytes(
    bytes: &[u8],
    policy: &CapturePolicy,
) -> Result<CaptureByteSummary, CaptureDecodeError> {
    let (declared_packet_len, header_len) = parse_packet_len(bytes)?;
    let available_packet_len = bytes.len().saturating_sub(header_len);

    if declared_packet_len > policy.max_packet_bytes() {
        return Err(CaptureDecodeError::PacketTooLarge {
            declared_len: declared_packet_len,
            max_len: policy.max_packet_bytes(),
        });
    }

    if available_packet_len < declared_packet_len {
        return Err(CaptureDecodeError::IncompletePacket {
            declared_len: declared_packet_len,
            available_len: available_packet_len,
        });
    }

    Ok(CaptureByteSummary {
        declared_packet_len,
        available_packet_len,
        truncated: available_packet_len > policy.max_packet_bytes(),
    })
}

fn payload_preview(bytes: &bytes::Bytes, policy: &CapturePolicy) -> (String, bool) {
    if policy.redact_payloads() {
        return (REDACTED_PAYLOAD_LABEL.to_owned(), !bytes.is_empty());
    }

    let preview_len = bytes.len().min(policy.max_packet_bytes());
    let truncated = bytes.len() > preview_len;
    let preview = bytes
        .iter()
        .take(preview_len)
        .map(|byte| format!("{byte:0>width$X}", width = HEX_BYTE_WIDTH))
        .collect::<Vec<_>>()
        .join(HEX_SEPARATOR);

    (preview, truncated)
}

fn parse_packet_len(bytes: &[u8]) -> Result<(usize, usize), CaptureDecodeError> {
    if bytes.is_empty() {
        return Err(CaptureDecodeError::EmptyCapture);
    }

    let mut value = 0_i32;
    for (index, byte) in bytes.iter().take(MINECRAFT_VARINT_MAX_BYTES).enumerate() {
        value |= ((*byte as i32) & VARINT_VALUE_MASK) << (VARINT_SEGMENT_BITS * index as u32);
        if byte & VARINT_CONTINUE_BIT == 0 {
            return Ok((value as usize, index + 1));
        }
    }

    Err(CaptureDecodeError::MalformedPacketLengthVarInt)
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    const MAX_PACKET_BYTES: usize = 4;
    const PACKET_ID: i32 = 2;
    const PACKET_NAME: &str = "TestPacket";
    const BODY_BYTES: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
    const VALID_CAPTURE_BYTES: &[u8] = &[0x02, 0x00, 0x01];
    const INCOMPLETE_CAPTURE_BYTES: &[u8] = &[0x03, 0x00];
    const MALFORMED_CAPTURE_BYTES: &[u8] = &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF];

    #[test]
    fn default_policy_redacts_payloads() {
        let packet = test_packet(Bytes::copy_from_slice(BODY_BYTES));

        let summary = summarize_packet(&packet, &CapturePolicy::default());

        assert_eq!(summary.body_len, BODY_BYTES.len());
        assert_eq!(summary.body_preview, REDACTED_PAYLOAD_LABEL);
        assert!(summary.redacted);
        assert!(summary.truncated);
    }

    #[test]
    fn preview_policy_bounds_payload_output() {
        let policy = CapturePolicy::new(MAX_PACKET_BYTES, false).unwrap();
        let packet = test_packet(Bytes::copy_from_slice(BODY_BYTES));

        let summary = summarize_packet(&packet, &policy);

        assert_eq!(summary.body_preview, "01 02 03 04");
        assert!(summary.truncated);
        assert!(!summary.redacted);
    }

    #[test]
    fn valid_capture_bytes_pass() {
        let policy = CapturePolicy::new(MAX_PACKET_BYTES, true).unwrap();

        let summary = inspect_capture_bytes(VALID_CAPTURE_BYTES, &policy).unwrap();

        assert_eq!(summary.declared_packet_len, 2);
        assert_eq!(summary.available_packet_len, 2);
    }

    #[test]
    fn malformed_capture_fails_closed() {
        let policy = CapturePolicy::new(MAX_PACKET_BYTES, true).unwrap();

        let error = inspect_capture_bytes(MALFORMED_CAPTURE_BYTES, &policy).unwrap_err();

        assert_eq!(error, CaptureDecodeError::MalformedPacketLengthVarInt);
    }

    #[test]
    fn incomplete_capture_fails_closed() {
        let policy = CapturePolicy::new(MAX_PACKET_BYTES, true).unwrap();

        let error = inspect_capture_bytes(INCOMPLETE_CAPTURE_BYTES, &policy).unwrap_err();

        assert_eq!(
            error,
            CaptureDecodeError::IncompletePacket {
                declared_len: 3,
                available_len: 1
            }
        );
    }

    #[test]
    fn zero_max_packet_bytes_is_rejected() {
        let error = CapturePolicy::new(0, true).unwrap_err();

        assert_eq!(error, CapturePolicyError::MaxPacketBytesZero);
    }

    fn test_packet(data: Bytes) -> Packet {
        Packet {
            side: PacketSide::Clientbound,
            state: PacketState::Play,
            id: PACKET_ID,
            timestamp: None,
            name: PACKET_NAME,
            data: Some(data),
        }
    }
}
