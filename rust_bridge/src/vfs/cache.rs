use std::collections::HashMap;
use std::ffi::{c_char, CStr};
use std::slice;

use crate::ffi_guard::safe_ffi_call;

/// Wezel na podwojnie wiazanej liscie utrzymujacej LRU
struct Node {
    key: String,
    data: Vec<u8>,
    prev: usize,
    next: usize,
}

/// Bufor LRU (Least Recently Used) bez uzycia osobnych alokacji wezlow
pub struct VfsLruCache {
    map: HashMap<String, usize>,
    nodes: Vec<Node>,
    head: usize,
    tail: usize,
    free: usize,
    capacity: usize,
    size: usize,
}

impl VfsLruCache {
    /// Tworzy nowy bufor o danej pojemnosci.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Pojemnosc bufora musi byc wieksza niz 0");
        let mut nodes = Vec::with_capacity(capacity);
        for i in 0..capacity {
            nodes.push(Node {
                key: String::new(),
                data: Vec::new(),
                prev: usize::MAX,
                next: if i + 1 < capacity { i + 1 } else { usize::MAX },
            });
        }
        
        Self {
            map: HashMap::with_capacity(capacity),
            nodes,
            head: usize::MAX,
            tail: usize::MAX,
            free: 0,
            capacity,
            size: 0,
        }
    }

    /// Pobiera element z cache i odswieza jego pozycje
    pub fn get(&mut self, key: &str) -> Option<&[u8]> {
        if let Some(&index) = self.map.get(key) {
            self.move_to_head(index);
            Some(&self.nodes[index].data)
        } else {
            None
        }
    }

    /// Wstawia element do cache
    pub fn put(&mut self, key: &str, data: &[u8]) {
        if let Some(&index) = self.map.get(key) {
            self.nodes[index].data.clear();
            self.nodes[index].data.extend_from_slice(data);
            self.move_to_head(index);
        } else {
            let index = if self.size == self.capacity {
                self.pop_tail()
            } else {
                self.alloc_node()
            };
            
            self.nodes[index].key = key.to_string();
            self.nodes[index].data.clear();
            self.nodes[index].data.extend_from_slice(data);
            
            self.map.insert(key.to_string(), index);
            self.insert_head(index);
            self.size += 1;
        }
    }
    
    /// Czysci cache
    pub fn clear(&mut self) {
        self.map.clear();
        for i in 0..self.capacity {
            self.nodes[i].key.clear();
            self.nodes[i].data.clear();
            self.nodes[i].prev = usize::MAX;
            self.nodes[i].next = if i + 1 < self.capacity { i + 1 } else { usize::MAX };
        }
        self.head = usize::MAX;
        self.tail = usize::MAX;
        self.free = 0;
        self.size = 0;
    }
    
    /// Zwraca aktualny rozmiar
    pub fn len(&self) -> usize {
        self.size
    }

    fn move_to_head(&mut self, index: usize) {
        if self.head == index {
            return;
        }
        self.remove_node(index);
        self.insert_head(index);
    }
    
    fn pop_tail(&mut self) -> usize {
        let tail = self.tail;
        assert!(tail != usize::MAX, "Brak ogona na liscie do usuniecia");
        
        let key_to_remove = self.nodes[tail].key.clone();
        self.map.remove(&key_to_remove);
        
        self.remove_node(tail);
        self.size -= 1;
        tail
    }
    
    fn remove_node(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;
        
        if prev != usize::MAX {
            self.nodes[prev].next = next;
        } else {
            self.head = next;
        }
        
        if next != usize::MAX {
            self.nodes[next].prev = prev;
        } else {
            self.tail = prev;
        }
    }
    
    fn insert_head(&mut self, index: usize) {
        self.nodes[index].prev = usize::MAX;
        self.nodes[index].next = self.head;
        
        if self.head != usize::MAX {
            self.nodes[self.head].prev = index;
        }
        
        self.head = index;
        
        if self.tail == usize::MAX {
            self.tail = index;
        }
    }
    
    fn alloc_node(&mut self) -> usize {
        assert!(self.free != usize::MAX, "Brak wolnych wezlow");
        let index = self.free;
        self.free = self.nodes[index].next;
        index
    }
}

// ============================================================================
// C FFI Exports - VFS LRU Cache
// ============================================================================

#[no_mangle]
pub extern "C" fn m2_vfs_cache_create(capacity: usize) -> *mut VfsLruCache {
    safe_ffi_call(std::ptr::null_mut(), || {
        if capacity == 0 {
            return std::ptr::null_mut();
        }
        let cache = Box::new(VfsLruCache::new(capacity));
        Box::into_raw(cache)
    })
}

#[no_mangle]
pub extern "C" fn m2_vfs_cache_destroy(cache_ptr: *mut VfsLruCache) {
    safe_ffi_call((), || {
        if !cache_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(cache_ptr);
            }
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_vfs_cache_put(cache_ptr: *mut VfsLruCache, key: *const c_char, data: *const u8, size: usize) {
    safe_ffi_call((), || {
        if cache_ptr.is_null() || key.is_null() || data.is_null() || size == 0 {
            return;
        }
        let cache = unsafe { &mut *cache_ptr };
        let c_str = unsafe { CStr::from_ptr(key) };
        if let Ok(key_str) = c_str.to_str() {
            let slice = unsafe { slice::from_raw_parts(data, size) };
            cache.put(key_str, slice);
        }
    });
}

#[no_mangle]
pub extern "C" fn m2_vfs_cache_get(
    cache_ptr: *mut VfsLruCache,
    key: *const c_char,
    out_data: *mut *const u8,
    out_size: *mut usize,
) -> bool {
    safe_ffi_call(false, || {
        if cache_ptr.is_null() || key.is_null() || out_data.is_null() || out_size.is_null() {
            return false;
        }
        let cache = unsafe { &mut *cache_ptr };
        let c_str = unsafe { CStr::from_ptr(key) };
        if let Ok(key_str) = c_str.to_str() {
            if let Some(data) = cache.get(key_str) {
                unsafe {
                    *out_data = data.as_ptr();
                    *out_size = data.len();
                }
                return true;
            }
        }
        false
    })
}

#[no_mangle]
pub extern "C" fn m2_vfs_cache_clear(cache_ptr: *mut VfsLruCache) {
    safe_ffi_call((), || {
        if cache_ptr.is_null() {
            return;
        }
        let cache = unsafe { &mut *cache_ptr };
        cache.clear();
    });
}

#[no_mangle]
pub extern "C" fn m2_vfs_cache_size(cache_ptr: *mut VfsLruCache) -> usize {
    safe_ffi_call(0, || {
        if cache_ptr.is_null() {
            return 0;
        }
        let cache = unsafe { &*cache_ptr };
        cache.len()
    })
}
