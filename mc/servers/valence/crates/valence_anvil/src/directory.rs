use valence_nbt::binary::{FromModifiedUtf8, ToModifiedUtf8};

type DirEntry = std::fs::DirEntry;
type File = std::fs::File;
type Path = std::path::Path;
type PathBuf = std::path::PathBuf;
type Region = crate::region::Storage;
type RegionEntry = crate::region::CacheEntry;
type RegionError = crate::RegionError;
type RegionPos = crate::region::FilePos;
type WriteOptions = crate::WriteOptions;

const REGION_CACHE_SIZE: usize = 256;
const REGION_FILE_COMPONENTS: usize = 4;
const REGION_FILE_EXTENSION: &str = "mca";
const REGION_FILE_PREFIX: &str = "r";
const REGION_WIDTH_CHUNKS: i32 = 32;

const _: () = {
    assert!(REGION_FILE_COMPONENTS > 0);
};

#[derive(Debug)]
pub struct RegionFolder {
    /// Region files. An LRU cache is used to limit the number of open file
    /// handles.
    regions: lru::LruCache<RegionPos, RegionEntry>,
    /// Path to the directory containing the region files and chunk files.
    region_root: PathBuf,
    /// Options to use for writing the chunk.
    pub write_options: WriteOptions,
}

impl RegionFolder {
    pub fn new<R: Into<PathBuf>>(region_root: R) -> Self {
        Self {
            regions: lru::LruCache::new(region_cache_size()),
            region_root: region_root.into(),
            write_options: WriteOptions::default(),
        }
    }

    fn region<'a>(
        regions: &'a mut lru::LruCache<RegionPos, RegionEntry>,
        region_root: &Path,
        region_x: i32,
        region_z: i32,
    ) -> Result<Option<&'a mut Region>, RegionError> {
        // Need to double get the entry from the cache to make the borrow checker happy.
        // Polonius will fix this eventually.
        if regions.get_mut(&(region_x, region_z)).is_some() {
            match regions.get_mut(&(region_x, region_z)) {
                Some(RegionEntry::Occupied(region)) => return Ok(Some(region)),
                Some(RegionEntry::Vacant) => return Ok(None),
                None => unreachable!(),
            }
        }

        let path = region_root.join(format!("r.{region_x}.{region_z}.mca"));

        let file = match File::options().read(true).write(true).open(path) {
            Ok(file) => file,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                regions.put((region_x, region_z), RegionEntry::Vacant);
                return Ok(None);
            }
            Err(err) => return Err(err.into()),
        };

        // TODO: this is ugly.
        // TODO: try_get_or_insert_mut
        regions.try_get_or_insert((region_x, region_z), || {
            Region::open(file).map(|region| RegionEntry::Occupied(Box::new(region)))
        })?;
        let Some(RegionEntry::Occupied(res)) = regions.get_mut(&(region_x, region_z)) else {
            unreachable!()
        };
        Ok(Some(res))
    }

    /// Gets the raw chunk at the given chunk position.
    ///
    /// Returns `Ok(Some(chunk))` if the chunk exists and no errors occurred
    /// loading it. Returns `Ok(None)` if the chunk does not exist and no
    /// errors occurred attempting to load it. Returns `Err(_)` if an error
    /// occurred attempting to load the chunk.
    pub fn get_chunk<S>(
        &mut self,
        pos_x: i32,
        pos_z: i32,
    ) -> Result<Option<crate::RawChunk<S>>, RegionError>
    where
        S: for<'a> FromModifiedUtf8<'a> + std::hash::Hash + Ord,
    {
        let region_x = pos_x.div_euclid(REGION_WIDTH_CHUNKS);
        let region_z = pos_z.div_euclid(REGION_WIDTH_CHUNKS);

        let Some(region) = Self::region(&mut self.regions, &self.region_root, region_x, region_z)?
        else {
            return Ok(None);
        };

        region.get_chunk(pos_x, pos_z, &self.region_root)
    }

    /// Deletes the chunk at the given chunk position, returning whether the
    /// chunk existed before it was deleted.
    ///
    /// Note that this only marks the chunk as deleted so that it cannot be
    /// retrieved, and can be overwritten by other chunks later. It does not
    /// decrease the size of the region file.
    pub fn delete_chunk(&mut self, pos_x: i32, pos_z: i32) -> Result<bool, RegionError> {
        let region_x = pos_x.div_euclid(REGION_WIDTH_CHUNKS);
        let region_z = pos_z.div_euclid(REGION_WIDTH_CHUNKS);

        let Some(region) = Self::region(&mut self.regions, &self.region_root, region_x, region_z)?
        else {
            return Ok(false);
        };

        region.delete_chunk(pos_x, pos_z, true, &self.region_root)
    }

    /// Sets the raw chunk at the given position, overwriting the old chunk if
    /// it exists.
    pub fn set_chunk<S>(
        &mut self,
        pos_x: i32,
        pos_z: i32,
        chunk: &valence_nbt::Compound<S>,
    ) -> Result<(), RegionError>
    where
        S: ToModifiedUtf8 + std::hash::Hash + Ord,
    {
        let region_x = pos_x.div_euclid(REGION_WIDTH_CHUNKS);
        let region_z = pos_z.div_euclid(REGION_WIDTH_CHUNKS);

        let region = match Self::region(&mut self.regions, &self.region_root, region_x, region_z)? {
            Some(region) => region,
            None => {
                let path = self
                    .region_root
                    .join(format!("r.{region_x}.{region_z}.mca"));

                let file = File::options()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(false)
                    .open(path)?;

                // TODO: try_get_or_insert_mut
                self.regions.put(
                    (region_x, region_z),
                    RegionEntry::Occupied(Box::new(Region::create(file)?)),
                );
                let Some(RegionEntry::Occupied(region)) =
                    self.regions.get_mut(&(region_x, region_z))
                else {
                    unreachable!()
                };
                region
            }
        };

        region.set_chunk(crate::region::write::Input {
            pos_x,
            pos_z,
            chunk,
            options: self.write_options,
            region_root: &self.region_root,
        })
    }

    /// Returns an iterator over all existing chunks in all regions.
    pub fn all_chunk_positions(
        &mut self,
    ) -> Result<impl Iterator<Item = Result<(i32, i32), RegionError>> + '_, RegionError> {
        fn extract_file_coordinates(
            file: std::io::Result<DirEntry>,
        ) -> Result<Option<(i32, i32)>, RegionError> {
            let file = file?;

            if !file.file_type()?.is_file() {
                return Ok(None);
            }

            let file_name = file
                .file_name()
                .into_string()
                .map_err(|_| RegionError::OsStringConv)?;

            // read the file name as r.x.z.mca
            let mut split = file_name.splitn(REGION_FILE_COMPONENTS, '.');
            if split.next() != Some(REGION_FILE_PREFIX) {
                return Ok(None);
            }
            let Some(Ok(x)) = split.next().map(str::parse) else {
                return Ok(None);
            };
            let Some(Ok(z)) = split.next().map(str::parse) else {
                return Ok(None);
            };
            if split.next() != Some(REGION_FILE_EXTENSION) {
                return Ok(None);
            }

            Ok(Some((x, z)))
        }

        fn chunks_for_file(
            this: &mut RegionFolder,
            pos: Result<(i32, i32), RegionError>,
        ) -> impl Iterator<Item = Result<(i32, i32), RegionError>> {
            let positions = match pos {
                Ok((region_x, region_z)) => {
                    match RegionFolder::region(
                        &mut this.regions,
                        &this.region_root,
                        region_x,
                        region_z,
                    ) {
                        Ok(Some(region)) => region.chunk_positions(region_x, region_z),
                        Ok(None) => Vec::new(),
                        Err(err) => vec![Err(err)],
                    }
                }
                Err(err) => vec![Err(err)],
            };
            positions.into_iter()
        }

        Ok(std::fs::read_dir(&self.region_root)?
            .filter_map(|file| extract_file_coordinates(file).transpose())
            .flat_map(|pos| chunks_for_file(self, pos)))
    }
}

fn region_cache_size() -> std::num::NonZeroUsize {
    let Some(cache_size) = std::num::NonZeroUsize::new(REGION_CACHE_SIZE) else {
        unreachable!();
    };
    cache_size
}
