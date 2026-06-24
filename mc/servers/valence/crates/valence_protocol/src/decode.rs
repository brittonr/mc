use std::ops::Range;

#[cfg(feature = "encryption")]
use aes::cipher::{generic_array::GenericArray, BlockDecryptMut, BlockSizeUser, KeyIvInit};
use anyhow::{bail, ensure, Context};
use bytes::{Buf, BytesMut};

use crate::var_int::{VarInt, VarIntDecodeError};
#[cfg(feature = "compression")]
use crate::CompressionThreshold;
use crate::{ByteBackedPacketBody, Decode, Packet, MAX_PACKET_SIZE};

/// The AES block cipher with a 128 bit key, using the CFB-8 mode of
/// operation.
#[cfg(feature = "encryption")]
type Cipher = cfb8::Decryptor<aes::Aes128>;

#[derive(Default)]
pub struct PacketDecoder {
    buf: BytesMut,
    #[cfg(feature = "compression")]
    decompress_buf: BytesMut,
    #[cfg(feature = "compression")]
    threshold: CompressionThreshold,
    #[cfg(feature = "encryption")]
    cipher: Option<Cipher>,
}

impl PacketDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn try_next_packet(&mut self) -> anyhow::Result<Option<PacketFrame>> {
        let Some(layout) = read_frame_layout(&self.buf)? else {
            return Ok(None);
        };

        let mut data;

        #[cfg(feature = "compression")]
        if self.threshold.0 >= 0 {
            data = self.decode_compressed_frame(layout)?;
        } else {
            self.buf.advance(layout.prefix_width);
            data = self.buf.split_to(layout.packet_length_usize());
        }

        #[cfg(not(feature = "compression"))]
        {
            self.buf.advance(layout.prefix_width);
            data = self.buf.split_to(layout.packet_length_usize());
        }

        let packet_id = split_packet_id(&mut data)?;

        Ok(Some(PacketFrame {
            id: packet_id,
            body: data,
        }))
    }

    #[cfg(feature = "compression")]
    pub fn compression(&self) -> CompressionThreshold {
        self.threshold
    }

    #[cfg(feature = "compression")]
    pub fn set_compression(&mut self, threshold: CompressionThreshold) {
        self.threshold = threshold;
    }

    #[cfg(feature = "compression")]
    fn decode_compressed_frame(&mut self, layout: PacketFrameLayout) -> anyhow::Result<BytesMut> {
        use std::io::Write;

        use bytes::BufMut;
        use flate2::write::ZlibDecoder;

        let mut r = &self.buf[layout.packet_data_range()];
        let data_len = VarInt::decode(&mut r)?.0;

        ensure!(
            (0..MAX_PACKET_SIZE).contains(&data_len),
            "decompressed packet length of {data_len} is out of bounds"
        );

        if data_len > 0 {
            ensure!(
                data_len > self.threshold.0,
                "decompressed packet length of {data_len} is <= the compression threshold of {}",
                self.threshold.0
            );

            debug_assert!(self.decompress_buf.is_empty());

            self.decompress_buf.put_bytes(0, data_len as usize);

            let mut z = ZlibDecoder::new(&mut self.decompress_buf[..]);
            z.write_all(r)?;
            ensure!(
                z.finish()?.is_empty(),
                "decompressed packet length is shorter than expected"
            );

            self.buf.advance(layout.total_width);

            Ok(self.decompress_buf.split())
        } else {
            let data_len_len = layout.packet_length_usize() - r.len();
            ensure!(
                r.len() <= self.threshold.0 as usize,
                "uncompressed packet length of {} exceeds compression threshold of {}",
                r.len(),
                self.threshold.0
            );

            let remaining_len = r.len();
            self.buf.advance(layout.prefix_width + data_len_len);

            Ok(self.buf.split_to(remaining_len))
        }
    }

    #[cfg(feature = "encryption")]
    pub fn enable_encryption(&mut self, key: &[u8; 16]) {
        assert!(self.cipher.is_none(), "encryption is already enabled");

        let mut cipher = Cipher::new_from_slices(key, key).expect("invalid key");

        // Don't forget to decrypt the data we already have.
        Self::decrypt_bytes(&mut cipher, &mut self.buf);

        self.cipher = Some(cipher);
    }

    /// Decrypts the provided byte slice in place using the cipher, without
    /// consuming the cipher.
    #[cfg(feature = "encryption")]
    fn decrypt_bytes(cipher: &mut Cipher, bytes: &mut [u8]) {
        for chunk in bytes.chunks_mut(Cipher::block_size()) {
            let gen_arr = GenericArray::from_mut_slice(chunk);
            cipher.decrypt_block_mut(gen_arr);
        }
    }

    pub fn queue_bytes(&mut self, mut bytes: BytesMut) {
        #![allow(unused_mut)]

        #[cfg(feature = "encryption")]
        if let Some(cipher) = &mut self.cipher {
            Self::decrypt_bytes(cipher, &mut bytes);
        }

        self.buf.unsplit(bytes);
    }

    pub fn queue_slice(&mut self, bytes: &[u8]) {
        #[cfg(feature = "encryption")]
        let len = self.buf.len();

        self.buf.extend_from_slice(bytes);

        #[cfg(feature = "encryption")]
        if let Some(cipher) = &mut self.cipher {
            let slice = &mut self.buf[len..];
            Self::decrypt_bytes(cipher, slice);
        }
    }

    pub fn take_capacity(&mut self) -> BytesMut {
        self.buf.split_off(self.buf.len())
    }

    pub fn reserve(&mut self, additional: usize) {
        self.buf.reserve(additional);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct PacketFrameLayout {
    packet_length: i32,
    prefix_width: usize,
    total_width: usize,
}

impl PacketFrameLayout {
    fn packet_length_usize(self) -> usize {
        self.packet_length as usize
    }

    fn packet_data_range(self) -> Range<usize> {
        self.prefix_width..self.total_width
    }
}

fn read_frame_layout(buf: &[u8]) -> anyhow::Result<Option<PacketFrameLayout>> {
    let mut r = buf;

    let packet_len = match VarInt::decode_partial(&mut r) {
        Ok(len) => len,
        Err(VarIntDecodeError::Incomplete) => return Ok(None),
        Err(VarIntDecodeError::TooLarge) => bail!("malformed packet length VarInt"),
    };

    ensure!(packet_len > 0, "zero-length packet is invalid");
    ensure!(
        packet_len <= MAX_PACKET_SIZE,
        "packet length of {packet_len} is out of bounds"
    );

    if r.len() < packet_len as usize {
        return Ok(None);
    }

    let prefix_width = VarInt(packet_len).written_size();
    let total_width = prefix_width + packet_len as usize;

    Ok(Some(PacketFrameLayout {
        packet_length: packet_len,
        prefix_width,
        total_width,
    }))
}

fn split_packet_id(data: &mut BytesMut) -> anyhow::Result<i32> {
    let mut r = &data[..];
    let packet_id = VarInt::decode(&mut r)
        .context("failed to decode packet ID")?
        .0;

    data.advance(data.len() - r.len());

    Ok(packet_id)
}

#[derive(Clone, Debug)]
pub struct PacketFrame {
    /// The ID of the decoded packet.
    pub id: i32,
    /// The contents of the packet after the leading `VarInt` ID.
    pub body: BytesMut,
}

impl PacketFrame {
    /// Converts this frame into a shared byte-backed frame.
    #[must_use]
    pub fn into_byte_backed(self) -> ByteBackedPacketFrame {
        ByteBackedPacketFrame {
            id: self.id,
            body: ByteBackedPacketBody::new(self.body.freeze()),
        }
    }

    /// Attempts to decode this packet as type `P`. An error is returned if the
    /// packet ID does not match, the body of the packet failed to decode, or
    /// some input was missed.
    pub fn decode<'a, P>(&'a self) -> anyhow::Result<P>
    where
        P: Packet + Decode<'a>,
    {
        decode_packet_body(self.id, &self.body, P::ID, P::NAME)
    }
}

/// A packet frame whose body is backed by shared bytes.
#[derive(Clone, Debug)]
pub struct ByteBackedPacketFrame {
    /// The ID of the decoded packet.
    pub id: i32,
    /// The contents of the packet after the leading `VarInt` ID.
    pub body: ByteBackedPacketBody,
}

impl ByteBackedPacketFrame {
    /// Returns the packet body.
    #[must_use]
    pub const fn body(&self) -> &ByteBackedPacketBody {
        &self.body
    }

    /// Consumes this frame into its ID and body.
    #[must_use]
    pub fn into_parts(self) -> (i32, ByteBackedPacketBody) {
        (self.id, self.body)
    }

    /// Attempts to decode this packet as type `P` from shared body bytes.
    pub fn decode<'a, P>(&'a self) -> anyhow::Result<P>
    where
        P: Packet + Decode<'a>,
    {
        decode_packet_body(self.id, self.body.as_bytes(), P::ID, P::NAME)
    }
}

fn decode_packet_body<'a, P>(
    actual_id: i32,
    body: &'a [u8],
    expected_id: i32,
    packet_name: &'static str,
) -> anyhow::Result<P>
where
    P: Decode<'a>,
{
    ensure!(
        expected_id == actual_id,
        "packet ID mismatch while decoding '{packet_name}': expected {expected_id}, got {actual_id}"
    );

    let mut r = body;
    let pkt = P::decode(&mut r)?;

    ensure!(
        r.is_empty(),
        "missed {} bytes while decoding '{}'",
        r.len(),
        packet_name
    );

    Ok(pkt)
}

#[cfg(test)]
mod tests {
    use bytes::{Bytes, BytesMut};

    use super::*;
    use crate::{
        ByteBackedBytes, ByteBackedError, ByteBackedStr, CompressionThreshold, Encode,
        PacketEncoder, PacketSide, PacketState,
    };

    const TEST_PACKET_ID: i32 = 90_100;
    const TEST_PACKET_NAME: &str = "byte_backed_decode_test_packet";
    const VALID_TEXT: &str = "hello";
    const VALID_TEXT_MAX_CHARS: usize = 5;
    const VALID_TEXT_FIELD_START: usize = 0;
    const VALID_TEXT_FIELD_END: usize = 5;
    const FRAME_SPLIT_INDEX: usize = 1;
    const ZERO_PACKET_LEN_BYTE: u8 = 0;
    const VARINT_CONTINUE_BYTE: u8 = 0x80;
    const OVERSIZED_PACKET_LEN: i32 = MAX_PACKET_SIZE + 1;
    const COMPRESS_EVERY_PACKET: CompressionThreshold = CompressionThreshold(0);
    const INVALID_COMPRESSED_PACKET_LEN: i32 = 1;
    const INVALID_COMPRESSED_DATA_LEN: i32 = 1;
    const INVALID_COMPRESSION_THRESHOLD: CompressionThreshold = CompressionThreshold(16);
    const INVALID_UTF8: &[u8] = &[0xff];
    const INVALID_UTF8_VALID_PREFIX: usize = 0;
    const BYTE_FIELD_MAX_LEN: usize = 5;
    const TOO_SMALL_BYTE_FIELD_MAX_LEN: usize = 4;

    #[test]
    fn complete_frame_decodes_byte_backed_body() {
        let mut decoder = PacketDecoder::new();
        decoder.queue_bytes(encode_test_packet(
            VALID_TEXT,
            CompressionThreshold::DEFAULT,
        ));

        let frame = decoder.try_next_packet().unwrap().unwrap();
        let byte_backed = frame.into_byte_backed();
        let decoded = byte_backed.decode::<TestPacket>().unwrap();

        let expected_body_len =
            VALID_TEXT.len() + VarInt(i32::try_from(VALID_TEXT.len()).unwrap()).written_size();

        assert_eq!(decoded.text, VALID_TEXT);
        assert_eq!(byte_backed.body().len(), expected_body_len);
    }

    #[test]
    fn split_frame_is_decoded_after_remaining_bytes_arrive() {
        let mut encoded = encode_test_packet(VALID_TEXT, CompressionThreshold::DEFAULT);
        let first = encoded.split_to(FRAME_SPLIT_INDEX);
        let mut decoder = PacketDecoder::new();

        decoder.queue_bytes(first);
        assert!(decoder.try_next_packet().unwrap().is_none());

        decoder.queue_bytes(encoded);
        let frame = decoder.try_next_packet().unwrap().unwrap();
        let decoded = frame.decode::<TestPacket>().unwrap();

        assert_eq!(decoded.text, VALID_TEXT);
        assert!(decoder.try_next_packet().unwrap().is_none());
    }

    #[cfg(feature = "compression")]
    #[test]
    fn compressed_frame_decodes_deterministically() {
        let mut decoder = PacketDecoder::new();
        decoder.set_compression(COMPRESS_EVERY_PACKET);
        decoder.queue_bytes(encode_test_packet(VALID_TEXT, COMPRESS_EVERY_PACKET));

        let frame = decoder.try_next_packet().unwrap().unwrap();
        let decoded = frame.decode::<TestPacket>().unwrap();

        assert_eq!(decoded.text, VALID_TEXT);
        assert!(decoder.try_next_packet().unwrap().is_none());
    }

    #[test]
    fn malformed_packet_length_varint_fails_closed() {
        let mut decoder = PacketDecoder::new();
        decoder.queue_bytes(BytesMut::from(
            &[VARINT_CONTINUE_BYTE; VarInt::MAX_SIZE][..],
        ));

        let err = decoder.try_next_packet().unwrap_err();

        assert!(err.to_string().contains("malformed packet length VarInt"));
    }

    #[test]
    fn zero_length_packet_fails_closed() {
        let mut decoder = PacketDecoder::new();
        decoder.queue_bytes(BytesMut::from(&[ZERO_PACKET_LEN_BYTE][..]));

        let err = decoder.try_next_packet().unwrap_err();

        assert!(err.to_string().contains("zero-length packet is invalid"));
    }

    #[test]
    fn oversized_packet_length_fails_closed() {
        let mut decoder = PacketDecoder::new();
        decoder.queue_bytes(encode_packet_len(OVERSIZED_PACKET_LEN));

        let err = decoder.try_next_packet().unwrap_err();

        assert!(err.to_string().contains("packet length"));
        assert!(err.to_string().contains("out of bounds"));
    }

    #[cfg(feature = "compression")]
    #[test]
    fn invalid_compressed_length_fails_closed() {
        let mut decoder = PacketDecoder::new();
        decoder.set_compression(INVALID_COMPRESSION_THRESHOLD);
        decoder.queue_bytes(encode_invalid_compressed_frame());

        let err = decoder.try_next_packet().unwrap_err();

        assert!(err.to_string().contains("compression threshold"));
    }

    #[test]
    fn invalid_utf8_field_fails_before_public_exposure() {
        let err = ByteBackedStr::new(Bytes::copy_from_slice(INVALID_UTF8), VALID_TEXT_MAX_CHARS)
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
    fn byte_backed_field_rejects_oversized_public_field() {
        let body = ByteBackedPacketBody::copy_from_slice(VALID_TEXT.as_bytes());
        let err = ByteBackedBytes::from_body_range(
            &body,
            VALID_TEXT_FIELD_START..VALID_TEXT_FIELD_END,
            TOO_SMALL_BYTE_FIELD_MAX_LEN,
        )
        .unwrap_err();

        assert_eq!(
            err,
            ByteBackedError::ByteLengthExceeded {
                len: BYTE_FIELD_MAX_LEN,
                max_len: TOO_SMALL_BYTE_FIELD_MAX_LEN,
            }
        );
    }

    #[test]
    fn byte_backed_text_field_can_outlive_packet_body() {
        let field = {
            let body = ByteBackedPacketBody::copy_from_slice(VALID_TEXT.as_bytes());
            ByteBackedStr::from_body_range(
                &body,
                VALID_TEXT_FIELD_START..VALID_TEXT_FIELD_END,
                VALID_TEXT_MAX_CHARS,
            )
            .unwrap()
        };

        assert_eq!(field.as_str(), VALID_TEXT);
    }

    fn encode_test_packet(text: &str, threshold: CompressionThreshold) -> BytesMut {
        let mut encoder = PacketEncoder::new();

        #[cfg(feature = "compression")]
        encoder.set_compression(threshold);

        #[cfg(not(feature = "compression"))]
        let _ = threshold;

        encoder.append_packet(&TestPacket { text }).unwrap();
        encoder.take()
    }

    fn encode_packet_len(packet_len: i32) -> BytesMut {
        let mut bytes = Vec::new();
        VarInt(packet_len).encode(&mut bytes).unwrap();
        BytesMut::from(bytes.as_slice())
    }

    #[cfg(feature = "compression")]
    fn encode_invalid_compressed_frame() -> BytesMut {
        let mut bytes = Vec::new();
        VarInt(INVALID_COMPRESSED_PACKET_LEN)
            .encode(&mut bytes)
            .unwrap();
        VarInt(INVALID_COMPRESSED_DATA_LEN)
            .encode(&mut bytes)
            .unwrap();
        BytesMut::from(bytes.as_slice())
    }

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    struct TestPacket<'a> {
        text: &'a str,
    }

    impl Packet for TestPacket<'_> {
        const ID: i32 = TEST_PACKET_ID;
        const NAME: &'static str = TEST_PACKET_NAME;
        const SIDE: PacketSide = PacketSide::Serverbound;
        const STATE: PacketState = PacketState::Play;
    }

    impl<'a> Decode<'a> for TestPacket<'a> {
        fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
            Ok(Self {
                text: <&str>::decode(r)?,
            })
        }
    }

    impl Encode for TestPacket<'_> {
        fn encode(&self, w: impl std::io::Write) -> anyhow::Result<()> {
            self.text.encode(w)
        }
    }
}
