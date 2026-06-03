#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RegionError {
    #[error("an I/O error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to convert OsString")]
    OsStringConv,
    #[error("chunk is allocated, but stream is missing")]
    MissingChunkStream,
    #[error("invalid chunk sector offset")]
    InvalidChunkSectorOffset,
    #[error("invalid chunk size")]
    InvalidChunkSize,
    #[error("invalid chunk position")]
    InvalidChunkPosition,
    #[error("invalid compression scheme number of {0}")]
    InvalidCompressionScheme(u8),
    #[error("failed to parse NBT: {0}")]
    Nbt(#[from] valence_nbt::Error),
    #[error("not all chunk NBT data was read")]
    TrailingNbtData,
    #[error("oversized chunk")]
    OversizedChunk,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
#[non_exhaustive]
pub enum Compression {
    Gzip = 1,
    #[default]
    Zlib = 2,
    None = 3,
}

impl Compression {
    pub(crate) fn from_u8(compression: u8) -> Option<Compression> {
        match compression {
            value if value == u8::from(Compression::Gzip) => Some(Compression::Gzip),
            value if value == u8::from(Compression::Zlib) => Some(Compression::Zlib),
            value if value == u8::from(Compression::None) => Some(Compression::None),
            _ => None,
        }
    }
}

impl From<Compression> for u8 {
    fn from(compression: Compression) -> Self {
        match compression {
            Compression::Gzip => 1,
            Compression::Zlib => 2,
            Compression::None => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[non_exhaustive]
pub struct WriteOptions {
    /// Set the compression method used to write chunks. This can be useful to
    /// change in order to write anvil files compatible with older Minecraft
    /// versions.
    pub compression: Compression,

    /// Set whether to skip writing oversized chunks (>1MiB after compression).
    /// Versions older than 1.15 (19w36a) cannot read oversized chunks, so this
    /// may be useful for writing region files compatible with those
    /// versions.
    pub skip_oversized_chunks: bool,
}

/// A chunk represented by the raw compound data.
pub struct RawChunk<S = String> {
    pub data: valence_nbt::Compound<S>,
    pub timestamp: u32,
}
