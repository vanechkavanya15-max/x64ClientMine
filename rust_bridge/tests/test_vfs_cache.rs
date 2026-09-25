use std::ffi::CString;
use std::ptr;

use m2_client_bridge::vfs::cache::{
    m2_vfs_cache_clear, m2_vfs_cache_create, m2_vfs_cache_destroy, m2_vfs_cache_get, m2_vfs_cache_put,
    m2_vfs_cache_size, VfsLruCache,
};

#[test]
fn test_lru_cache_basic_operations() {
    let mut cache = VfsLruCache::new(3);

    // Wstawienie elementow
    cache.put("plik1.txt", b"dane1");
    cache.put("plik2.txt", b"dane2");
    cache.put("plik3.txt", b"dane3");

    assert_eq!(cache.len(), 3);
    assert_eq!(cache.get("plik1.txt"), Some(&b"dane1"[..]));
    assert_eq!(cache.get("plik2.txt"), Some(&b"dane2"[..]));
    assert_eq!(cache.get("plik3.txt"), Some(&b"dane3"[..]));
}

#[test]
fn test_lru_cache_eviction() {
    let mut cache = VfsLruCache::new(3);

    cache.put("plik1.txt", b"dane1");
    cache.put("plik2.txt", b"dane2");
    cache.put("plik3.txt", b"dane3");
    
    // plik1 znowu podbijany w hierarchii
    cache.get("plik1.txt");

    // Przekroczenie limitu, plik2.txt bedzie wyrzucony, jako najstarszy z nieuzywanych
    cache.put("plik4.txt", b"dane4");

    assert_eq!(cache.len(), 3);
    assert_eq!(cache.get("plik2.txt"), None); // wyrzucony
    assert_eq!(cache.get("plik1.txt"), Some(&b"dane1"[..]));
    assert_eq!(cache.get("plik3.txt"), Some(&b"dane3"[..]));
    assert_eq!(cache.get("plik4.txt"), Some(&b"dane4"[..]));
}

#[test]
fn test_lru_cache_update() {
    let mut cache = VfsLruCache::new(2);

    cache.put("test.txt", b"v1");
    assert_eq!(cache.get("test.txt"), Some(&b"v1"[..]));

    cache.put("test.txt", b"wersja_druga");
    assert_eq!(cache.get("test.txt"), Some(&b"wersja_druga"[..]));
    assert_eq!(cache.len(), 1);
}

#[test]
fn test_lru_cache_clear() {
    let mut cache = VfsLruCache::new(2);

    cache.put("a", b"1");
    cache.put("b", b"2");

    cache.clear();

    assert_eq!(cache.len(), 0);
    assert_eq!(cache.get("a"), None);
    assert_eq!(cache.get("b"), None);

    cache.put("c", b"3");
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get("c"), Some(&b"3"[..]));
}

#[test]
fn test_ffi_basic() {
    let cache_ptr = m2_vfs_cache_create(2);
    assert!(!cache_ptr.is_null());

    let key1 = CString::new("f1").unwrap();
    let data1 = b"abc";
    m2_vfs_cache_put(cache_ptr, key1.as_ptr(), data1.as_ptr(), data1.len());

    assert_eq!(m2_vfs_cache_size(cache_ptr), 1);

    let mut out_data: *const u8 = ptr::null();
    let mut out_size: usize = 0;
    
    let result = m2_vfs_cache_get(cache_ptr, key1.as_ptr(), &mut out_data, &mut out_size);
    assert!(result);
    assert_eq!(out_size, 3);
    
    let slice = unsafe { std::slice::from_raw_parts(out_data, out_size) };
    assert_eq!(slice, b"abc");

    m2_vfs_cache_clear(cache_ptr);
    assert_eq!(m2_vfs_cache_size(cache_ptr), 0);

    m2_vfs_cache_destroy(cache_ptr);
}

#[test]
fn test_ffi_null_pointers() {
    // Te operacje nie powinny powodowac paniki
    m2_vfs_cache_destroy(ptr::null_mut());

    let cache_ptr = m2_vfs_cache_create(1);
    
    m2_vfs_cache_put(ptr::null_mut(), ptr::null(), ptr::null(), 0);
    m2_vfs_cache_put(cache_ptr, ptr::null(), ptr::null(), 0);

    let mut out_data: *const u8 = ptr::null();
    let mut out_size: usize = 0;
    let res = m2_vfs_cache_get(ptr::null_mut(), ptr::null(), &mut out_data, &mut out_size);
    assert!(!res);

    m2_vfs_cache_clear(ptr::null_mut());
    assert_eq!(m2_vfs_cache_size(ptr::null_mut()), 0);

    m2_vfs_cache_destroy(cache_ptr);
}

#[test]
fn test_ffi_create_zero() {
    let cache_ptr = m2_vfs_cache_create(0);
    assert!(cache_ptr.is_null());
}
