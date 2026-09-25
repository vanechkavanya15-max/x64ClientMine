use crate::ffi_guard::safe_ffi_call;
use std::collections::HashMap;

/// Struktura przechowujaca metadane pojedynczego pliku w archiwum VFS.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FileMetadata {
    pub hash: u32,
    pub offset: u32,
    pub size: u32,
    pub unpacked_size: u32,
}

/// Indeks plikow VFS mapujacy hash (CRC32 nazwy pliku) na metadane.
pub struct ArchiveIndex {
    files: HashMap<u32, FileMetadata>,
}

impl ArchiveIndex {
    /// Tworzy nowy, pusty indeks.
    pub fn new() -> Self {
        ArchiveIndex {
            files: HashMap::new(),
        }
    }

    /// Dodaje metadane pliku do indeksu.
    pub fn add(&mut self, metadata: FileMetadata) {
        self.files.insert(metadata.hash, metadata);
    }

    /// Zwraca metadane pliku o zadanym hashu, jesli istnieje.
    pub fn get(&self, hash: u32) -> Option<FileMetadata> {
        self.files.get(&hash).copied()
    }
}

// ============================================================================
// C FFI Exports - VFS Archive Index
// ============================================================================

/// Tworzy nowy indeks VFS i zwraca wskaznik (handle).
#[no_mangle]
pub extern "C" fn m2_vfs_archive_index_new() -> *mut ArchiveIndex {
    safe_ffi_call(std::ptr::null_mut(), || {
        let index = Box::new(ArchiveIndex::new());
        Box::into_raw(index)
    })
}

/// Zwalnia pamiec zajmowana przez indeks VFS.
#[no_mangle]
pub extern "C" fn m2_vfs_archive_index_free(index_ptr: *mut ArchiveIndex) {
    safe_ffi_call((), || {
        if !index_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(index_ptr); // Zostanie zniszczony po wyjsciu z bloku
            }
        }
    });
}

/// Dodaje plik do indeksu VFS.
#[no_mangle]
pub extern "C" fn m2_vfs_archive_index_add(
    index_ptr: *mut ArchiveIndex,
    hash: u32,
    offset: u32,
    size: u32,
    unpacked_size: u32,
) {
    safe_ffi_call((), || {
        if index_ptr.is_null() {
            return;
        }
        let index = unsafe { &mut *index_ptr };
        let metadata = FileMetadata {
            hash,
            offset,
            size,
            unpacked_size,
        };
        index.add(metadata);
    });
}

/// Pobiera metadane pliku z indeksu na podstawie hasha.
/// Zwraca 1 w przypadku sukcesu (i kopiuje dane pod metadata_out),
/// lub 0, jesli plik nie istnieje lub parametry sa bledne.
#[no_mangle]
pub extern "C" fn m2_vfs_archive_index_get(
    index_ptr: *const ArchiveIndex,
    hash: u32,
    metadata_out: *mut FileMetadata,
) -> u32 {
    safe_ffi_call(0, || {
        if index_ptr.is_null() || metadata_out.is_null() {
            return 0;
        }
        let index = unsafe { &*index_ptr };
        if let Some(metadata) = index.get(hash) {
            unsafe {
                *metadata_out = metadata;
            }
            1
        } else {
            0
        }
    })
}
