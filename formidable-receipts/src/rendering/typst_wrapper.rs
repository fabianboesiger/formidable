use std::collections::HashMap;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use typst::diag::{FileError, FileResult, PackageError, PackageResult, eco_format};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::package::PackageSpec;
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt};
use typst_kit::fonts::{FontSearcher, FontSlot};

/// Main interface that determines the environment for Typst.
pub struct TypstWrapperWorld {
    /// Root path to which files will be resolved.
    root: PathBuf,

    /// The content of a source.
    source: Source,

    /// The standard library.
    library: LazyHash<Library>,

    /// Metadata about all known fonts.
    book: LazyHash<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<FontSlot>,

    /// Map of all known files.
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,

    /// Map of custom data files that can be loaded with Typst's data loading functions.
    /// These are virtual files that exist only in memory.
    data_files: Arc<Mutex<HashMap<String, Vec<u8>>>>,

    /// Cache directory (e.g. where packages are downloaded to).
    cache_directory: PathBuf,

    /// http agent to download packages.
    http: ureq::Agent,

    /// Datetime.
    time: time::OffsetDateTime,
}

impl TypstWrapperWorld {
    pub fn new(root: String, source: String) -> Self {
        let root = PathBuf::from(root);
        let fonts = FontSearcher::new().include_system_fonts(true).search();

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(fonts.book),
            root,
            fonts: fonts.fonts,
            source: Source::detached(source),
            time: time::OffsetDateTime::now_utc(),
            cache_directory: std::env::var_os("CACHE_DIRECTORY")
                .map(|os_path| os_path.into())
                .unwrap_or(std::env::temp_dir()),
            http: ureq::Agent::new_with_defaults(),
            files: Arc::new(Mutex::new(HashMap::new())),
            data_files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add JSON data that can be loaded with `json("filename.json")` in Typst.
    pub fn add_json_data(
        &self,
        filename: &str,
        data: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json_string = serde_json::to_string_pretty(data)?;
        self.add_data_file(filename, json_string.into_bytes())?;
        Ok(())
    }

    /// Internal method to add data files to the virtual file system.
    fn add_data_file(
        &self,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut data_files = self
            .data_files
            .lock()
            .map_err(|_| "Failed to lock data_files")?;
        data_files.insert(filename.to_string(), data);
        Ok(())
    }

    /// Check if a file exists in the virtual data files.
    fn has_data_file(&self, path: &str) -> bool {
        if let Ok(data_files) = self.data_files.lock() {
            data_files.contains_key(path)
        } else {
            false
        }
    }

    /// Get data from virtual data files.
    fn get_data_file(&self, path: &str) -> Option<Vec<u8>> {
        if let Ok(data_files) = self.data_files.lock() {
            data_files.get(path).cloned()
        } else {
            None
        }
    }
}

/// A File that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = std::str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}

impl TypstWrapperWorld {
    /// Helper to handle file requests.
    ///
    /// Requests will be either in packages, a local file, or a virtual data file.
    fn file(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }

        // Check if this is a virtual data file first
        let path_str = id.vpath().as_rooted_path().to_string_lossy();
        if self.has_data_file(&path_str) {
            if let Some(data) = self.get_data_file(&path_str) {
                let entry = FileEntry::new(data, None);
                files.insert(id, entry.clone());
                return Ok(entry);
            }
        }

        let path = if let Some(package) = id.package() {
            // Fetching file from package
            let package_dir = self.download_package(package)?;
            id.vpath().resolve(&package_dir)
        } else {
            // Fetching file from disk
            id.vpath().resolve(&self.root)
        }
        .ok_or(FileError::AccessDenied)?;

        let content = std::fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content, None))
            .clone())
    }

    /// Downloads the package and returns the system path of the unpacked package.
    fn download_package(&self, package: &PackageSpec) -> PackageResult<PathBuf> {
        let package_subdir = format!("{}/{}/{}", package.namespace, package.name, package.version);
        let path = self.cache_directory.join(package_subdir);

        if path.exists() {
            return Ok(path);
        }

        eprintln!("downloading {package}");
        let url = format!(
            "https://packages.typst.org/{}/{}-{}.tar.gz",
            package.namespace, package.name, package.version,
        );

        let response = retry(|| {
            let response = self
                .http
                .get(&url)
                .call()
                .map_err(|error| eco_format!("{error}"))?;

            let status = response.status();
            if !status.is_success() {
                eprintln!("Failed to download package {package} from {url}: {status}");
                return Err(eco_format!(
                    "response returned unsuccessful status code {status}",
                ));
            }

            Ok(response)
        })
        .map_err(|error| PackageError::NetworkFailed(Some(error)))?;
        let compressed_archive = response
            .into_body()
            .read_to_vec()
            .map_err(|error| PackageError::NetworkFailed(Some(eco_format!("{error}"))))?;
        let raw_archive = zune_inflate::DeflateDecoder::new(&compressed_archive)
            .decode_gzip()
            .map_err(|error| PackageError::MalformedArchive(Some(eco_format!("{error}"))))?;
        let mut archive = tar::Archive::new(raw_archive.as_slice());
        archive.unpack(&path).map_err(|error| {
            _ = std::fs::remove_dir_all(&path);
            eprintln!("Failed to unpack package {package} from {url}: {error}");
            PackageError::MalformedArchive(Some(eco_format!("{error}")))
        })?;

        Ok(path)
    }
}

/// This is the interface we have to implement such that `typst` can compile it.
///
/// I have tried to keep it as minimal as possible
impl typst::World for TypstWrapperWorld {
    /// Standard library.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.file(id)?.source(id)
        }
    }

    /// Accessing a specified file (non-file).
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file(id).map(|file| file.bytes.clone())
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        self.fonts[id].get()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = self.time.checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

fn retry<T, E>(mut f: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    if let Ok(ok) = f() { Ok(ok) } else { f() }
}
