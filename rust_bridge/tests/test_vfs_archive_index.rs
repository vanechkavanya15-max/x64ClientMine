use m2_client_bridge::vfs::archive_index::{
    m2_vfs_archive_index_add, m2_vfs_archive_index_free, m2_vfs_archive_index_get,
    m2_vfs_archive_index_new, FileMetadata,
};
use std::ptr;

#[test]
fn test_archive_index_basic_operations() {
    let index_ptr = m2_vfs_archive_index_new();
    assert!(!index_ptr.is_null());

    let test_hash: u32 = 0xDEADBEEF;
    let test_offset: u32 = 1000;
    let test_size: u32 = 500;
    let test_unpacked_size: u32 = 1500;

    // Dodanie elementu
    m2_vfs_archive_index_add(
        index_ptr,
        test_hash,
        test_offset,
        test_size,
        test_unpacked_size,
    );

    let mut metadata_out = FileMetadata {
        hash: 0,
        offset: 0,
        size: 0,
        unpacked_size: 0,
    };

    // Pobranie elementu (sukces)
    let res = m2_vfs_archive_index_get(index_ptr, test_hash, &mut metadata_out);
    assert_eq!(res, 1);
    assert_eq!(metadata_out.hash, test_hash);
    assert_eq!(metadata_out.offset, test_offset);
    assert_eq!(metadata_out.size, test_size);
    assert_eq!(metadata_out.unpacked_size, test_unpacked_size);

    // Pobranie nieistniejacego elementu
    let res_not_found = m2_vfs_archive_index_get(index_ptr, 0xCAFEBABE, &mut metadata_out);
    assert_eq!(res_not_found, 0);

    m2_vfs_archive_index_free(index_ptr);
}

#[test]
fn test_archive_index_null_pointers() {
    // Null ptr dla add
    m2_vfs_archive_index_add(ptr::null_mut(), 123, 0, 0, 0);

    // Null ptr dla get (index)
    let mut metadata_out = FileMetadata {
        hash: 0,
        offset: 0,
        size: 0,
        unpacked_size: 0,
    };
    let res1 = m2_vfs_archive_index_get(ptr::null(), 123, &mut metadata_out);
    assert_eq!(res1, 0);

    // Null ptr dla get (metadata_out)
    let index_ptr = m2_vfs_archive_index_new();
    let res2 = m2_vfs_archive_index_get(index_ptr, 123, ptr::null_mut());
    assert_eq!(res2, 0);

    // Null ptr dla free
    m2_vfs_archive_index_free(ptr::null_mut());

    m2_vfs_archive_index_free(index_ptr);
}
