# Protocol 763 broad parser fixture source snapshot

Source: `stevenarella/protocol/src/protocol/versions.rs`
Stevenarella commit: `ba3ce751f04b4fecefe516e06dff3e40363d2e72`
Snapshot purpose: make the external parser-fixture implementation reviewable from the `mc` evidence bundle.

## Fixture coverage table

| Packet family | Valence row | Stevenarella packet | Positive fixture | Negative fixture scope |
| --- | --- | --- | --- | --- |
| `command_tree_raw` | `play/clientbound/0x10 CommandTreeS2CPacket` | `DeclareCommandsRaw` | `protocol_763_high_risk_raw_parser_fixtures_accept_payloads` asserts raw payload bytes are preserved. | Byte-opaque raw payload: malformed semantic command trees are a non-claim until a semantic decoder exists. |
| `chunk_delta_raw` | `play/clientbound/0x43 ChunkDeltaUpdateS2CPacket` | `ChunkDeltaUpdateRaw` | `protocol_763_high_risk_raw_parser_fixtures_accept_payloads` asserts raw payload bytes are preserved. | Byte-opaque raw payload: malformed semantic chunk deltas are a non-claim until a semantic decoder exists. |
| `recipe_book_raw` | `play/clientbound/0x6d SynchronizeRecipesS2CPacket` | `SynchronizeRecipesRaw` | `protocol_763_high_risk_raw_parser_fixtures_accept_payloads` asserts raw payload bytes are preserved. | Byte-opaque raw payload: malformed semantic recipes are a non-claim until a semantic decoder exists. |
| `custom_payload_brand` | `play/serverbound/0x0d CustomPayloadC2SPacket` | `PluginMessageServerbound` | `protocol_763_custom_payload_parser_fixture_accepts_brand_payload` asserts channel `minecraft:brand` and payload bytes. | `protocol_763_custom_payload_parser_fixture_rejects_malformed_channel` rejects invalid UTF-8 and oversized VarInt channel lengths. |

## Source excerpt

```rust
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
            let crate::protocol::packet::Packet::DeclareCommandsRaw(command_packet) = command_packet
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
                0x0f, b'm', b'i', b'n', b'e', b'c', b'r', b'a', b'f', b't', b':', b'b',
                b'r', b'a', b'n', b'd', 0x05, b'P', b'a', b'p', b'e', b'r',
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
```
