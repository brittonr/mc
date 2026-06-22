use std::io::{Seek, Write};

use byteorder::WriteBytesExt;
use valence_nbt::binary::ToModifiedUtf8;

type BigEndian = super::BigEndian;
type File = super::File;
type Location = super::Location;
type Path = std::path::Path;
type SeekFrom = std::io::SeekFrom;
type Storage = super::Storage;
type Error = crate::RegionError;

const COMPRESSION_LEVEL: u32 = 6;
const COMPRESSION_VERSION_BYTES: usize = 1;
const EXTERNAL_CHUNK_SECTORS: usize = 1;
const EXTERNAL_CHUNK_SIZE_FIELD: u32 = 1;
const MAX_INTERNAL_CHUNK_SECTORS: usize = 256;
const STREAM_HEADER_BYTES: usize = 5;
static PADDING_SECTOR: [u8; super::SECTOR_SIZE] = [0; super::SECTOR_SIZE];

pub(crate) struct Input<'a, S> {
    pub(crate) pos_x: i32,
    pub(crate) pos_z: i32,
    pub(crate) chunk: &'a valence_nbt::Compound<S>,
    pub(crate) options: crate::WriteOptions,
    pub(crate) region_root: &'a Path,
}

impl Storage {
    pub(crate) fn set_chunk<S>(&mut self, input: Input<'_, S>) -> Result<(), Error>
    where
        S: ToModifiedUtf8 + std::hash::Hash + Ord,
    {
        let Input {
            pos_x,
            pos_z,
            chunk,
            options,
            region_root,
        } = input;
        let mut compress_buf = Vec::new();
        // Erase the chunk from allocated chunks (not from disk).
        self.delete_chunk(pos_x, pos_z, false, region_root)?;

        // Write the chunk into NBT and compress it according to the compression method.
        let mut compress_cursor = std::io::Cursor::new(&mut compress_buf);
        match options.compression {
            crate::Compression::Gzip => valence_nbt::to_binary(
                chunk,
                flate2::write::GzEncoder::new(&mut compress_cursor, default_compression()),
                "",
            )?,
            crate::Compression::Zlib => valence_nbt::to_binary(
                chunk,
                flate2::write::ZlibEncoder::new(&mut compress_cursor, default_compression()),
                "",
            )?,
            crate::Compression::None => valence_nbt::to_binary(chunk, &mut compress_cursor, "")?,
        }
        let compress_buf = compress_cursor.into_inner();

        let num_sectors_needed = sectors_needed_for_chunk(compress_buf.len())?;
        let (start_sector, num_sectors) = if num_sectors_needed >= MAX_INTERNAL_CHUNK_SECTORS {
            self.write_external_chunk(pos_x, pos_z, compress_buf, options, region_root)?
        } else {
            self.write_internal_chunk(pos_x, pos_z, compress_buf, options, region_root)?
        };

        let location = Location::new()
            .with_offset(sector_offset_field(start_sector)?)
            .with_count(sector_count_field(num_sectors)?);
        let timestamp = current_timestamp_seconds();

        // Write changed header information to file.
        let chunk_idx = Self::chunk_idx(pos_x, pos_z);
        self.file
            .seek(SeekFrom::Start(super::header_location_offset(chunk_idx)?))?;
        self.file.write_u32::<BigEndian>(location.0)?;
        self.file
            .seek(SeekFrom::Start(super::header_timestamp_offset(chunk_idx)?))?;
        self.file.write_u32::<BigEndian>(timestamp)?;

        // Write changed header information to our header.
        self.locations[chunk_idx] = location;
        self.timestamps[chunk_idx] = timestamp;

        // Pad file to multiple of `SECTOR_SIZE`.
        let file_length_bytes = self.file.seek(SeekFrom::End(0))?;
        let padding = file_padding_len(file_length_bytes)?;
        if padding != 0 {
            self.file.write_all(&PADDING_SECTOR[..padding])?;
        }

        Ok(())
    }

    fn write_external_chunk(
        &mut self,
        pos_x: i32,
        pos_z: i32,
        compress_buf: &[u8],
        options: crate::WriteOptions,
        region_root: &Path,
    ) -> Result<(usize, usize), Error> {
        if options.skip_oversized_chunks {
            return Err(Error::OversizedChunk);
        }

        // Write oversized chunk to external file.
        File::create(Self::external_chunk_file(pos_x, pos_z, region_root))?
            .write_all(compress_buf)?;

        let start_sector = super::allocate_sectors(&mut self.used_sectors, EXTERNAL_CHUNK_SECTORS)?;
        self.file
            .seek(SeekFrom::Start(super::sector_byte_offset(start_sector)?))?;

        // Write the exact chunk size, which includes *only* the compression version
        // (the rest of the chunk is external).
        self.file
            .write_u32::<BigEndian>(EXTERNAL_CHUNK_SIZE_FIELD)?;
        // Write the compression, with the marker which says our chunk is oversized.
        self.file
            .write_u8(u8::from(options.compression) | super::EXTERNAL_STREAM_CHUNK_FLAG)?;

        Ok((start_sector, EXTERNAL_CHUNK_SECTORS))
    }

    fn write_internal_chunk(
        &mut self,
        pos_x: i32,
        pos_z: i32,
        compress_buf: &[u8],
        options: crate::WriteOptions,
        region_root: &Path,
    ) -> Result<(usize, usize), Error> {
        // Delete the oversized chunk if it existed before.
        Self::delete_external_chunk_file(pos_x, pos_z, region_root)?;

        let num_sectors_needed = sectors_needed_for_chunk(compress_buf.len())?;
        let start_sector = super::allocate_sectors(&mut self.used_sectors, num_sectors_needed)?;
        self.file
            .seek(SeekFrom::Start(super::sector_byte_offset(start_sector)?))?;

        // Write the exact chunk size, which accounts for the compression version which
        // is not in our compress_buf.
        self.file
            .write_u32::<BigEndian>(chunk_stream_size(compress_buf.len())?)?;
        // Write the compression.
        self.file.write_u8(u8::from(options.compression))?;
        // Write the data.
        self.file.write_all(compress_buf)?;

        Ok((start_sector, num_sectors_needed))
    }
}

fn chunk_stream_size(payload_len: usize) -> Result<u32, Error> {
    let Some(size) = payload_len.checked_add(COMPRESSION_VERSION_BYTES) else {
        return Err(Error::InvalidChunkSize);
    };
    u32::try_from(size).map_err(|_| Error::InvalidChunkSize)
}

fn default_compression() -> flate2::Compression {
    flate2::Compression::new(COMPRESSION_LEVEL)
}

// Anvil region headers store wall-clock seconds by file-format requirement.
#[allow(unknown_lints)]
#[allow(ambient_clock)]
fn current_timestamp_seconds() -> u32 {
    let Ok(duration) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
        return 0;
    };
    match u32::try_from(duration.as_secs()) {
        Ok(seconds) => seconds,
        Err(_) => u32::MAX,
    }
}

fn file_padding_len(file_length: u64) -> Result<usize, Error> {
    let rem = file_length % super::SECTOR_SIZE_U64;
    if rem == 0 {
        return Ok(0);
    }
    let Some(padding) = super::SECTOR_SIZE_U64.checked_sub(rem) else {
        return Err(Error::InvalidChunkSize);
    };
    usize::try_from(padding).map_err(|_| Error::InvalidChunkSize)
}

fn sector_count_field(num_sectors: usize) -> Result<u8, Error> {
    u8::try_from(num_sectors).map_err(|_| Error::InvalidChunkSize)
}

fn sector_offset_field(start_sector: usize) -> Result<u32, Error> {
    u32::try_from(start_sector).map_err(|_| Error::InvalidChunkSectorOffset)
}

fn sectors_needed_for_chunk(payload_len: usize) -> Result<usize, Error> {
    let Some(stream_len) = payload_len.checked_add(STREAM_HEADER_BYTES) else {
        return Err(Error::InvalidChunkSize);
    };
    Ok(stream_len.div_ceil(super::SECTOR_SIZE))
}
