#![allow(unused_imports)]

use super::*;
use crate::test_support::*;
use crate::*;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;

const TEST_VARINT_SINGLE_BYTE_VALUE: u32 = VARINT_SEGMENT_MASK;
const TEST_VARINT_TWO_BYTE_VALUE: u32 = VARINT_SEGMENT_MASK + STATUS_HANDSHAKE_NEXT_STATE;
const TEST_PACKET_PAYLOAD_FIRST_BYTE: u8 = 0x02;
const TEST_PACKET_PAYLOAD_SECOND_BYTE: u8 = 0x03;
const TEST_PACKET_BODY_LENGTH: u8 = 0x03;
const TEST_MC_STRING: &str = "mc";
const TEST_MC_STRING_LENGTH: u8 = 0x02;
const TEST_STATUS_PORT: u16 = 25565;
const TEST_STATUS_PACKET_ID_BYTE_LENGTH: usize = 1;
const TEST_TOO_LONG_VARINT_BYTES: usize =
    (VARINT_MAX_SHIFT_EXCLUSIVE / VARINT_SEGMENT_BITS) as usize + 1;

#[test]
fn protocol_varint_round_trips_in_memory() {
    let cases = [
        STATUS_PACKET_ID,
        STATUS_HANDSHAKE_NEXT_STATE,
        TEST_VARINT_SINGLE_BYTE_VALUE,
        TEST_VARINT_TWO_BYTE_VALUE,
        DEFAULT_SERVER_PROTOCOL,
    ];

    for value in cases {
        let mut bytes = Vec::new();
        bytes.write_varint(value).expect("write varint");
        let decoded = Cursor::new(bytes).read_varint().expect("read varint");
        assert_eq!(decoded, value);
    }
}

#[test]
fn protocol_string_and_packet_framing_match_expected_bytes() {
    let mut string_bytes = Vec::new();
    string_bytes
        .write_mc_string(TEST_MC_STRING)
        .expect("write string");
    assert_eq!(
        string_bytes,
        vec![TEST_MC_STRING_LENGTH, b'm', b'c'],
        "Minecraft string framing changed"
    );
    assert_eq!(
        Cursor::new(string_bytes)
            .read_mc_string()
            .expect("read string"),
        TEST_MC_STRING
    );

    let payload = [
        TEST_PACKET_PAYLOAD_FIRST_BYTE,
        TEST_PACKET_PAYLOAD_SECOND_BYTE,
    ];
    let mut packet = Vec::new();
    packet
        .write_packet(STATUS_HANDSHAKE_NEXT_STATE, &payload)
        .expect("write packet");
    assert_eq!(
        packet,
        vec![
            TEST_PACKET_BODY_LENGTH,
            STATUS_HANDSHAKE_NEXT_STATE as u8,
            TEST_PACKET_PAYLOAD_FIRST_BYTE,
            TEST_PACKET_PAYLOAD_SECOND_BYTE,
        ]
    );
}

#[test]
fn protocol_status_handshake_fixture_bytes_are_stable() {
    let mut payload = Vec::new();
    payload
        .write_varint(DEFAULT_SERVER_PROTOCOL)
        .expect("protocol varint");
    payload
        .write_mc_string(STATUS_LOCALHOST_ADDRESS)
        .expect("host string");
    payload.extend_from_slice(&TEST_STATUS_PORT.to_be_bytes());
    payload
        .write_varint(STATUS_HANDSHAKE_NEXT_STATE)
        .expect("next state");

    let mut framed = Vec::new();
    framed
        .write_packet(STATUS_PACKET_ID, &payload)
        .expect("status handshake packet");
    let mut cursor = Cursor::new(framed);
    let packet_length = cursor.read_varint().expect("packet length");
    let packet_id = cursor.read_varint().expect("packet id");

    assert_eq!(packet_id, STATUS_PACKET_ID);
    assert_eq!(
        packet_length as usize,
        payload.len() + TEST_STATUS_PACKET_ID_BYTE_LENGTH
    );
}

#[test]
fn protocol_invalid_inputs_fail_closed() {
    let eof = Cursor::new(Vec::new()).read_varint().unwrap_err();
    assert!(eof.contains("failed to fill whole buffer"), "{eof}");

    let too_long = vec![VARINT_CONTINUATION_BIT; TEST_TOO_LONG_VARINT_BYTES];
    let err = Cursor::new(too_long).read_varint().unwrap_err();
    assert_eq!(err, "varint too long");

    let truncated_string = vec![TEST_MC_STRING_LENGTH, b'm'];
    let err = Cursor::new(truncated_string).read_mc_string().unwrap_err();
    assert!(err.contains("failed to fill whole buffer"), "{err}");
}
