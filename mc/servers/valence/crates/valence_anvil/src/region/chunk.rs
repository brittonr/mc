use std::io::{Read, Seek};

use byteorder::{ReadBytesExt, WriteBytesExt};
use valence_nbt::binary::FromModifiedUtf8;

type BigEndian = super::BigEndian;
type File = super::File;
type Location = super::Location;
type Path = std::path::Path;
type PathBuf = std::path::PathBuf;
type Error = crate::RegionError;
type SeekFrom = std::io::SeekFrom;
type Storage = super::Storage;

const COMPRESSION_VERSION_BYTES: usize = 1;
const MAX_DECOMPRESSED_CHUNK_BYTES: u64 = 268_435_456;

impl Storage {
    pub(crate) fn get_chunk<S>(
        &mut self,
        pos_x: i32,
        pos_z: i32,
        region_root: &Path,
    ) -> Result<Option<crate::RawChunk<S>>, Error>
    where
        S: for<'a> FromModifiedUtf8<'a> + std::hash::Hash + Ord,
    {
        let chunk_idx = Self::chunk_idx(pos_x, pos_z);

        let location = self.locations[chunk_idx];
        let timestamp = self.timestamps[chunk_idx];

        if location.is_none() {
            // No chunk exists at this position.
            return Ok(None);
        }

        let (sector_offset, sector_count) = location.offset_and_count()?;

        // If the sector offset was inside the header, then the chunk data would be
        // inside the region header. That doesn't make any sense.
        if sector_offset < super::HEADER_SECTORS {
            return Err(Error::InvalidChunkSectorOffset);
        }

        // Seek to the beginning of the chunk's data.
        self.file
            .seek(SeekFrom::Start(super::sector_byte_offset(sector_offset)?))?;

        let exact_chunk_size_bytes = usize::try_from(self.file.read_u32::<BigEndian>()?)
            .map_err(|_| Error::InvalidChunkSize)?;
        if exact_chunk_size_bytes == 0 {
            return Err(Error::MissingChunkStream);
        }

        // Size of this chunk in sectors must always be >= the exact size.
        let Some(sector_bytes) = sector_count.checked_mul(super::SECTOR_SIZE) else {
            return Err(Error::InvalidChunkSize);
        };
        if sector_bytes < exact_chunk_size_bytes {
            return Err(Error::InvalidChunkSize);
        }

        let mut compression = self.file.read_u8()?;

        let data_buf = if Self::is_external_stream_chunk(compression) {
            compression = Self::external_chunk_version(compression);
            let external_file = File::open(Self::external_chunk_file(pos_x, pos_z, region_root))?;
            read_external_chunk_file(external_file)?
        } else {
            // The size includes the version of the stream, but we have already read that.
            let Some(payload_len) = exact_chunk_size_bytes.checked_sub(COMPRESSION_VERSION_BYTES)
            else {
                return Err(Error::MissingChunkStream);
            };
            let mut data_buf = vec![0; payload_len].into_boxed_slice();
            self.file.read_exact(&mut data_buf)?;
            data_buf
        };

        let r = data_buf.as_ref();

        // What compression does the chunk use?
        let payload = match crate::Compression::from_u8(compression) {
            Some(crate::Compression::Gzip) => {
                let z = flate2::bufread::GzDecoder::new(r);
                Payload::Owned(read_decompressed_to_end(z)?)
            }
            Some(crate::Compression::Zlib) => {
                let z = flate2::bufread::ZlibDecoder::new(r);
                Payload::Owned(read_decompressed_to_end(z)?)
            }
            // Uncompressed
            Some(crate::Compression::None) => Payload::Borrowed(r),
            // Unknown
            None => return Err(Error::InvalidCompressionScheme(compression)),
        };
        let mut nbt_slice = payload.as_slice();

        let (data, _) = valence_nbt::from_binary(&mut nbt_slice)?;

        if !nbt_slice.is_empty() {
            return Err(Error::TrailingNbtData);
        }

        Ok(Some(crate::RawChunk { data, timestamp }))
    }

    pub(crate) fn delete_chunk(
        &mut self,
        pos_x: i32,
        pos_z: i32,
        delete_on_disk: bool,
        region_root: &Path,
    ) -> Result<bool, Error> {
        let chunk_idx = Self::chunk_idx(pos_x, pos_z);

        let location = self.locations[chunk_idx];
        if location.is_none() {
            // Chunk already missing, nothing to delete.
            return Ok(false);
        }

        if delete_on_disk {
            self.file
                .seek(SeekFrom::Start(super::header_location_offset(chunk_idx)?))?;
            self.file.write_u32::<BigEndian>(0)?;

            Self::delete_external_chunk_file(pos_x, pos_z, region_root)?;
        }

        let (sector_offset, sector_count) = location.offset_and_count()?;
        if sector_offset >= super::HEADER_SECTORS {
            let Some(end_index) = sector_offset.checked_add(sector_count) else {
                return Err(Error::InvalidChunkSectorOffset);
            };
            let len = self.used_sectors.len();
            self.used_sectors[sector_offset.min(len)..end_index.min(len)].fill(false);
        }

        self.locations[chunk_idx] = Location::new();

        Ok(true)
    }

    pub(crate) fn chunk_positions(
        &self,
        region_x: i32,
        region_z: i32,
    ) -> Vec<Result<(i32, i32), Error>> {
        self.locations
            .iter()
            .enumerate()
            .filter_map(move |(index, location)| {
                if location.is_none() {
                    None
                } else {
                    Some(global_position_for_index(PositionInput {
                        region_x,
                        region_z,
                        index,
                    }))
                }
            })
            .collect()
    }

    pub(super) fn external_chunk_file(pos_x: i32, pos_z: i32, region_root: &Path) -> PathBuf {
        region_root
            .to_path_buf()
            .join(format!("c.{pos_x}.{pos_z}.mcc"))
    }

    pub(super) fn delete_external_chunk_file(
        pos_x: i32,
        pos_z: i32,
        region_root: &Path,
    ) -> Result<(), Error> {
        match std::fs::remove_file(Self::external_chunk_file(pos_x, pos_z, region_root)) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn is_external_stream_chunk(stream_version: u8) -> bool {
        (stream_version & super::EXTERNAL_STREAM_CHUNK_FLAG) != 0
    }

    fn external_chunk_version(stream_version: u8) -> u8 {
        stream_version & !super::EXTERNAL_STREAM_CHUNK_FLAG
    }
}

enum Payload<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl Payload<'_> {
    fn as_slice(&self) -> &[u8] {
        match self {
            Payload::Borrowed(slice) => slice,
            Payload::Owned(bytes) => bytes,
        }
    }
}

fn read_decompressed_to_end<R: Read>(reader: R) -> Result<Vec<u8>, Error> {
    let mut bounded_reader = reader.take(MAX_DECOMPRESSED_CHUNK_BYTES);
    let mut decompressed_bytes = Vec::new();
    bounded_reader.read_to_end(&mut decompressed_bytes)?;
    Ok(decompressed_bytes)
}

fn read_external_chunk_file(file: File) -> Result<Box<[u8]>, Error> {
    let file_len_bytes = file.metadata()?.len();
    let capacity_bytes = usize::try_from(file_len_bytes).map_err(|_| Error::InvalidChunkSize)?;
    let mut buf = Vec::with_capacity(capacity_bytes);
    let mut bounded_file = file.take(file_len_bytes);
    bounded_file.read_to_end(&mut buf)?;
    Ok(buf.into_boxed_slice())
}

struct PositionInput {
    region_x: i32,
    region_z: i32,
    index: usize,
}

fn global_position_for_index(input: PositionInput) -> Result<(i32, i32), Error> {
    let (local_x, local_z) = Storage::chunk_coords_from_index(input.index)?;
    let x = Storage::global_chunk_coord(input.region_x, local_x)?;
    let z = Storage::global_chunk_coord(input.region_z, local_z)?;
    Ok((x, z))
}
