use std::io::Write;

#[cfg(feature = "encryption")]
use aes::cipher::generic_array::GenericArray;
#[cfg(feature = "encryption")]
use aes::cipher::{BlockEncryptMut, BlockSizeUser, KeyIvInit};
use anyhow::ensure;
use bytes::{BufMut, BytesMut};
use tracing::warn;

use crate::var_int::VarInt;
use crate::{CompressionThreshold, Encode, Packet, MAX_PACKET_SIZE};

/// The AES block cipher with a 128 bit key, using the CFB-8 mode of
/// operation.
#[cfg(feature = "encryption")]
type Cipher = cfb8::Encryptor<aes::Aes128>;

#[cfg(feature = "compression")]
const ZLIB_COMPRESSION_LEVEL: u32 = 4;
#[cfg(feature = "compression")]
const UNCOMPRESSED_DATA_LEN_PREFIX_SIZE: usize = 1;

#[derive(Default)]
pub struct PacketEncoder {
    buf: BytesMut,
    #[cfg(feature = "compression")]
    compress_buf: Vec<u8>,
    #[cfg(feature = "compression")]
    threshold: CompressionThreshold,
    #[cfg(feature = "encryption")]
    cipher: Option<Cipher>,
}

impl PacketEncoder {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn append_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes)
    }

    pub fn prepend_packet<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        let start_len = self.buf.len();
        self.append_packet(pkt)?;

        let end_len = self.buf.len();
        let total_packet_len = end_len - start_len;

        // 1) Move everything back by the length of the packet.
        // 2) Move the packet to the new space at the front.
        // 3) Truncate the old packet away.
        self.buf.put_bytes(0, total_packet_len);
        self.buf.copy_within(..end_len, total_packet_len);
        self.buf.copy_within(total_packet_len + start_len.., 0);
        self.buf.truncate(end_len);

        Ok(())
    }

    pub fn append_packet<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        let start_len = self.buf.len();
        let result = self.append_packet_inner(pkt, start_len);

        if result.is_err() {
            self.buf.truncate(start_len);

            #[cfg(feature = "compression")]
            self.compress_buf.clear();
        }

        result
    }

    #[allow(clippy::needless_borrows_for_generic_args)]
    fn append_packet_inner<P>(&mut self, pkt: &P, start_len: usize) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        pkt.encode_with_id((&mut self.buf).writer())?;

        let data_len = self.buf.len() - start_len;

        #[cfg(feature = "compression")]
        if self.threshold.0 >= 0 {
            use std::io::Read;

            use flate2::bufread::ZlibEncoder;
            use flate2::Compression;

            if data_len > self.threshold.0 as usize {
                let mut z = ZlibEncoder::new(
                    &self.buf[start_len..],
                    Compression::new(ZLIB_COMPRESSION_LEVEL),
                );

                self.compress_buf.clear();

                let data_len_size = VarInt(data_len as i32).written_size();

                let packet_len = data_len_size + z.read_to_end(&mut self.compress_buf)?;

                ensure!(
                    packet_len <= MAX_PACKET_SIZE as usize,
                    "packet exceeds maximum length"
                );

                drop(z);

                self.buf.truncate(start_len);

                let mut writer = (&mut self.buf).writer();

                VarInt(packet_len as i32).encode(&mut writer)?;
                VarInt(data_len as i32).encode(&mut writer)?;
                self.buf.extend_from_slice(&self.compress_buf);
            } else {
                let data_len_size = UNCOMPRESSED_DATA_LEN_PREFIX_SIZE;
                let packet_len = data_len_size + data_len;

                ensure!(
                    packet_len <= MAX_PACKET_SIZE as usize,
                    "packet exceeds maximum length"
                );

                let packet_len_size = VarInt(packet_len as i32).written_size();

                let data_prefix_len = packet_len_size + data_len_size;

                self.buf.put_bytes(0, data_prefix_len);
                self.buf
                    .copy_within(start_len..start_len + data_len, start_len + data_prefix_len);

                let mut front = &mut self.buf[start_len..];

                VarInt(packet_len as i32).encode(&mut front)?;
                // Zero for no compression on this packet.
                VarInt(0).encode(front)?;
            }

            return Ok(());
        }

        let packet_len = data_len;

        ensure!(
            packet_len <= MAX_PACKET_SIZE as usize,
            "packet exceeds maximum length"
        );

        let packet_len_size = VarInt(packet_len as i32).written_size();

        self.buf.put_bytes(0, packet_len_size);
        self.buf
            .copy_within(start_len..start_len + data_len, start_len + packet_len_size);

        let front = &mut self.buf[start_len..];
        VarInt(packet_len as i32).encode(front)?;

        Ok(())
    }

    /// Takes all the packets written so far and encrypts them if encryption is
    /// enabled.
    pub fn take(&mut self) -> BytesMut {
        #[cfg(feature = "encryption")]
        if let Some(cipher) = &mut self.cipher {
            for chunk in self.buf.chunks_mut(Cipher::block_size()) {
                let gen_arr = GenericArray::from_mut_slice(chunk);
                cipher.encrypt_block_mut(gen_arr);
            }
        }

        self.buf.split()
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    #[cfg(feature = "compression")]
    pub fn set_compression(&mut self, threshold: CompressionThreshold) {
        self.threshold = threshold;
    }

    /// Initializes the cipher with the given key. All future packets **and any
    /// that have not been [taken] yet** are encrypted.
    ///
    /// [taken]: Self::take
    ///
    /// # Panics
    ///
    /// Panics if encryption is already enabled.
    #[cfg(feature = "encryption")]
    pub fn enable_encryption(&mut self, key: &[u8; 16]) {
        assert!(self.cipher.is_none(), "encryption is already enabled");
        self.cipher = Some(Cipher::new_from_slices(key, key).expect("invalid key"));
    }
}

/// Types that can have packets written to them.
pub trait WritePacket {
    /// Writes a packet to this object. Encoding errors are typically logged and
    /// discarded.
    fn write_packet<P>(&mut self, packet: &P)
    where
        P: Packet + Encode,
    {
        if let Err(e) = self.write_packet_fallible(packet) {
            warn!("failed to write packet '{}': {e:#}", P::NAME);
        }
    }

    /// Writes a packet to this object. The result of encoding the packet is
    /// returned.
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode;

    /// Copies raw packet data directly into this object. Don't use this unless
    /// you know what you're doing.
    fn write_packet_bytes(&mut self, bytes: &[u8]);
}

impl<W: WritePacket> WritePacket for &mut W {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        (*self).write_packet_fallible(packet)
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        (*self).write_packet_bytes(bytes)
    }
}

impl<T: WritePacket> WritePacket for bevy_ecs::world::Mut<'_, T> {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.as_mut().write_packet_fallible(packet)
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.as_mut().write_packet_bytes(bytes)
    }
}

/// An implementor of [`WritePacket`] backed by a `Vec` mutable reference.
///
/// Packets are written by appending to the contained vec. If an error occurs
/// while writing, the written bytes are truncated away.
#[derive(Debug)]
pub struct PacketWriter<'a> {
    pub buf: &'a mut Vec<u8>,
    pub threshold: CompressionThreshold,
}

impl<'a> PacketWriter<'a> {
    pub fn new(buf: &'a mut Vec<u8>, threshold: CompressionThreshold) -> Self {
        Self { buf, threshold }
    }
}

/// Reusable scratch storage for packet encoding helpers.
///
/// Callers that encode many compressed packets can retain this value between
/// writes so compression scratch capacity is reused instead of reallocated.
#[derive(Debug, Default)]
pub struct PacketEncodeScratch {
    #[cfg(feature = "compression")]
    compress_buf: Vec<u8>,
}

impl PacketEncodeScratch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        #[cfg(feature = "compression")]
        self.compress_buf.clear();
    }

    #[cfg(feature = "compression")]
    fn compress_buf(&mut self) -> &mut Vec<u8> {
        &mut self.compress_buf
    }
}

/// A [`WritePacket`] implementation backed by a `Vec` and reusable scratch.
///
/// Like [`PacketWriter`], failed writes truncate the destination back to its
/// pre-write length. Unlike [`PacketWriter`], compressed writes can reuse
/// scratch capacity retained by the caller.
#[derive(Debug)]
pub struct ReusablePacketWriter<'a, 'scratch> {
    pub buf: &'a mut Vec<u8>,
    pub threshold: CompressionThreshold,
    scratch: &'scratch mut PacketEncodeScratch,
}

impl<'a, 'scratch> ReusablePacketWriter<'a, 'scratch> {
    pub fn new(
        buf: &'a mut Vec<u8>,
        threshold: CompressionThreshold,
        scratch: &'scratch mut PacketEncodeScratch,
    ) -> Self {
        Self {
            buf,
            threshold,
            scratch,
        }
    }
}

impl WritePacket for PacketWriter<'_> {
    #[cfg_attr(not(feature = "compression"), track_caller)]
    fn write_packet_fallible<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        let start = self.buf.len();

        let res;

        if self.threshold.0 >= 0 {
            #[cfg(feature = "compression")]
            {
                res = encode_packet_compressed(self.buf, pkt, self.threshold.0 as u32);
            }

            #[cfg(not(feature = "compression"))]
            {
                panic!("\"compression\" feature must be enabled to write compressed packets");
            }
        } else {
            res = encode_packet(self.buf, pkt)
        };

        if res.is_err() {
            self.buf.truncate(start);
        }

        res
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        if let Err(e) = self.buf.write_all(bytes) {
            warn!("failed to write packet bytes: {e:#}");
        }
    }
}

impl WritePacket for ReusablePacketWriter<'_, '_> {
    #[cfg_attr(not(feature = "compression"), track_caller)]
    fn write_packet_fallible<P>(&mut self, pkt: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        let start = self.buf.len();

        let res;

        if self.threshold.0 >= 0 {
            #[cfg(feature = "compression")]
            {
                res = encode_packet_compressed_with_scratch(
                    self.buf,
                    pkt,
                    self.threshold.0 as u32,
                    self.scratch.compress_buf(),
                );
            }

            #[cfg(not(feature = "compression"))]
            {
                panic!("\"compression\" feature must be enabled to write compressed packets");
            }
        } else {
            res = encode_packet(self.buf, pkt)
        };

        if res.is_err() {
            self.buf.truncate(start);
            self.scratch.clear();
        }

        res
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        if let Err(e) = self.buf.write_all(bytes) {
            warn!("failed to write packet bytes: {e:#}");
        }
    }
}

impl WritePacket for PacketEncoder {
    fn write_packet_fallible<P>(&mut self, packet: &P) -> anyhow::Result<()>
    where
        P: Packet + Encode,
    {
        self.append_packet(packet)
    }

    fn write_packet_bytes(&mut self, bytes: &[u8]) {
        self.append_bytes(bytes)
    }
}

fn encode_packet<P>(buf: &mut Vec<u8>, pkt: &P) -> anyhow::Result<()>
where
    P: Packet + Encode,
{
    let start_len = buf.len();

    pkt.encode_with_id(&mut *buf)?;

    let packet_len = buf.len() - start_len;

    ensure!(
        packet_len <= MAX_PACKET_SIZE as usize,
        "packet exceeds maximum length"
    );

    let packet_len_size = VarInt(packet_len as i32).written_size();

    buf.put_bytes(0, packet_len_size);
    buf.copy_within(
        start_len..start_len + packet_len,
        start_len + packet_len_size,
    );

    let front = &mut buf[start_len..];
    VarInt(packet_len as i32).encode(front)?;

    Ok(())
}

#[cfg(feature = "compression")]
fn encode_packet_compressed<P>(buf: &mut Vec<u8>, pkt: &P, threshold: u32) -> anyhow::Result<()>
where
    P: Packet + Encode,
{
    let mut scratch = Vec::new();
    encode_packet_compressed_with_scratch(buf, pkt, threshold, &mut scratch)
}

#[cfg(feature = "compression")]
#[allow(clippy::needless_borrows_for_generic_args)]
fn encode_packet_compressed_with_scratch<P>(
    buf: &mut Vec<u8>,
    pkt: &P,
    threshold: u32,
    scratch: &mut Vec<u8>,
) -> anyhow::Result<()>
where
    P: Packet + Encode,
{
    use std::io::Read;

    use flate2::bufread::ZlibEncoder;
    use flate2::Compression;

    let start_len = buf.len();

    pkt.encode_with_id(&mut *buf)?;

    let data_len = buf.len() - start_len;

    if data_len > threshold as usize {
        let mut z = ZlibEncoder::new(&buf[start_len..], Compression::new(ZLIB_COMPRESSION_LEVEL));

        scratch.clear();

        let packet_len = VarInt(data_len as i32).written_size() + z.read_to_end(scratch)?;

        ensure!(
            packet_len <= MAX_PACKET_SIZE as usize,
            "packet exceeds maximum length"
        );

        drop(z);

        buf.truncate(start_len);

        VarInt(packet_len as i32).encode(&mut *buf)?;
        VarInt(data_len as i32).encode(&mut *buf)?;
        buf.extend_from_slice(scratch);
    } else {
        let data_len_size = UNCOMPRESSED_DATA_LEN_PREFIX_SIZE;
        let packet_len = data_len_size + data_len;

        ensure!(
            packet_len <= MAX_PACKET_SIZE as usize,
            "packet exceeds maximum length"
        );

        let packet_len_size = VarInt(packet_len as i32).written_size();

        let data_prefix_len = packet_len_size + data_len_size;

        buf.put_bytes(0, data_prefix_len);
        buf.copy_within(start_len..start_len + data_len, start_len + data_prefix_len);

        let mut front = &mut buf[start_len..];

        VarInt(packet_len as i32).encode(&mut front)?;
        // Zero for no compression on this packet.
        VarInt(0).encode(front)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use anyhow::bail;
    use bytes::BytesMut;

    use super::{
        PacketEncodeScratch, PacketEncoder, PacketWriter, ReusablePacketWriter, WritePacket,
    };
    use crate::decode::PacketDecoder;
    use crate::packets::play::DifficultyS2c;
    use crate::{
        CompressionThreshold, Difficulty, Encode, Packet, PacketSide, PacketState, MAX_PACKET_SIZE,
    };

    const COMPRESS_EVERY_PACKET: CompressionThreshold = CompressionThreshold(0);
    const FAILING_PACKET_ID: i32 = 99_001;
    const FAILING_PACKET_NAME: &str = "test_partial_failing_packet";
    const OVERSIZED_PACKET_ID: i32 = 99_002;
    const OVERSIZED_PACKET_NAME: &str = "test_oversized_packet";
    const OVERSIZED_BYTE: u8 = 0xa5;
    const MAX_PACKET_SIZE_USIZE: usize = MAX_PACKET_SIZE as usize;
    const OVERSIZED_BODY_LEN: usize = MAX_PACKET_SIZE_USIZE + 1;
    const STALE_MARKER: &[u8] = b"partial-stale-bytes";
    const TEST_FAILURE_MESSAGE: &str = "intentional encode failure";

    #[test]
    fn packet_encoder_matches_packet_writer_for_default_packet() {
        let packet = valid_packet();
        let mut encoder = PacketEncoder::new();
        let mut writer_bytes = Vec::new();

        encoder.append_packet(&packet).unwrap();
        PacketWriter::new(&mut writer_bytes, CompressionThreshold::DEFAULT)
            .write_packet_fallible(&packet)
            .unwrap();

        assert_eq!(encoder.take().as_ref(), writer_bytes.as_slice());
    }

    #[test]
    fn packet_encoder_clears_partial_bytes_after_uncompressed_error() {
        let mut encoder = PacketEncoder::new();

        encoder.append_packet(&PartialFailingPacket).unwrap_err();
        encoder.append_packet(&valid_packet()).unwrap();

        let bytes = encoder.take();
        assert_no_stale_marker(&bytes);
        assert_valid_difficulty_frame(bytes, CompressionThreshold::DEFAULT);
    }

    #[test]
    fn oversized_packet_does_not_poison_next_packet() {
        let mut encoder = PacketEncoder::new();

        encoder.append_packet(&OversizedPacket).unwrap_err();
        encoder.append_packet(&valid_packet()).unwrap();

        let bytes = encoder.take();
        assert_valid_difficulty_frame(bytes, CompressionThreshold::DEFAULT);
    }

    #[cfg(feature = "compression")]
    #[test]
    fn packet_encoder_clears_partial_bytes_after_compressed_error() {
        let mut encoder = PacketEncoder::new();
        encoder.set_compression(COMPRESS_EVERY_PACKET);

        encoder.append_packet(&PartialFailingPacket).unwrap_err();
        encoder.append_packet(&valid_packet()).unwrap();

        let bytes = encoder.take();
        assert_no_stale_marker(&bytes);
        assert_valid_difficulty_frame(bytes, COMPRESS_EVERY_PACKET);
    }

    #[cfg(feature = "compression")]
    #[test]
    fn reusable_packet_writer_clears_partial_bytes_after_compressed_error() {
        let mut bytes = Vec::new();
        let mut scratch = PacketEncodeScratch::new();

        {
            let mut writer =
                ReusablePacketWriter::new(&mut bytes, COMPRESS_EVERY_PACKET, &mut scratch);
            writer
                .write_packet_fallible(&PartialFailingPacket)
                .unwrap_err();
            writer.write_packet_fallible(&valid_packet()).unwrap()
        };

        assert_no_stale_marker(&bytes);
        assert_valid_difficulty_frame(BytesMut::from(bytes.as_slice()), COMPRESS_EVERY_PACKET);
    }

    fn valid_packet() -> DifficultyS2c {
        DifficultyS2c {
            difficulty: Difficulty::Peaceful,
            locked: true,
        }
    }

    fn assert_valid_difficulty_frame(bytes: BytesMut, threshold: CompressionThreshold) {
        let mut decoder = PacketDecoder::new();

        #[cfg(feature = "compression")]
        decoder.set_compression(threshold);

        #[cfg(not(feature = "compression"))]
        let _ = threshold;

        decoder.queue_bytes(bytes);
        let frame = decoder.try_next_packet().unwrap().unwrap();
        let decoded = frame.decode::<DifficultyS2c>().unwrap();

        assert_eq!(frame.id, DifficultyS2c::ID);
        assert_eq!(decoded, valid_packet());
        assert!(decoder.try_next_packet().unwrap().is_none());
    }

    fn assert_no_stale_marker(bytes: &[u8]) {
        assert!(!bytes
            .windows(STALE_MARKER.len())
            .any(|window| window == STALE_MARKER));
    }

    #[derive(Debug)]
    struct PartialFailingPacket;

    impl Packet for PartialFailingPacket {
        const ID: i32 = FAILING_PACKET_ID;
        const NAME: &'static str = FAILING_PACKET_NAME;
        const SIDE: PacketSide = PacketSide::Clientbound;
        const STATE: PacketState = PacketState::Play;
    }

    impl Encode for PartialFailingPacket {
        fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
            w.write_all(STALE_MARKER)?;
            bail!(TEST_FAILURE_MESSAGE)
        }
    }

    #[derive(Debug)]
    struct OversizedPacket;

    impl Packet for OversizedPacket {
        const ID: i32 = OVERSIZED_PACKET_ID;
        const NAME: &'static str = OVERSIZED_PACKET_NAME;
        const SIDE: PacketSide = PacketSide::Clientbound;
        const STATE: PacketState = PacketState::Play;
    }

    impl Encode for OversizedPacket {
        fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
            let bytes = vec![OVERSIZED_BYTE; OVERSIZED_BODY_LEN];
            w.write_all(&bytes)?;
            Ok(())
        }
    }
}
