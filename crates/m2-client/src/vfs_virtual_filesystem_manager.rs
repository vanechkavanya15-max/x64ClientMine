use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex, OnceLock};

use crate::vfs_pck_memory_mapped::{PckArchive, read_file_from_archive, mount_archive};

pub struct VfsManager {
    // 1. Dyskowy cache
    pub base_path: String,
    
    // 2. Mod packi (wyzszy priorytet, ladujemy od konca)
    pub mod_packs: Vec<PckArchive>,
    
    // 3. Zwykle paczki
    pub std_packs: Vec<PckArchive>,
    
    // 4. Cache odpytywanych plikow w RAM (LRU Cache)
    pub file_cache: HashMap<String, Arc<Vec<u8>>>,
    pub cache_queue: VecDeque<String>,
    pub max_cache_size: usize,
    pub current_cache_size: usize,
}

static VFS_MANAGER: OnceLock<Mutex<VfsManager>> = OnceLock::new();

pub fn get_vfs_manager() -> &'static Mutex<VfsManager> {
    VFS_MANAGER.get_or_init(|| {
        Mutex::new(VfsManager::new())
    })
}

impl Default for VfsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VfsManager {
    pub fn new() -> Self {
        Self {
            base_path: String::new(),
            mod_packs: Vec::new(),
            std_packs: Vec::new(),
            file_cache: HashMap::new(),
            cache_queue: VecDeque::new(),
            max_cache_size: 50 * 1024 * 1024, // 50 MB
            current_cache_size: 0,
        }
    }

    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = path.to_string();
    }
    
    pub fn mount_mod_pack(&mut self, path: &str) -> bool {
        if let Ok(archive) = mount_archive(path) {
            self.mod_packs.push(archive);
            return true;
        }
        false
    }
    
    pub fn mount_pack(&mut self, path: &str) -> bool {
        if let Ok(archive) = mount_archive(path) {
            self.std_packs.push(archive);
            return true;
        }
        false
    }
    
    pub fn get_file_bytes(&mut self, virtual_path: &str) -> Option<Arc<Vec<u8>>> {
        let normalized_path = virtual_path.replace('\\', "/").to_lowercase();
        
        // 0. Sprawdz Cache
        if let Some(data) = self.file_cache.get(&normalized_path) {
            // Przesun na koniec kolejki (LRU)
            if let Some(pos) = self.cache_queue.iter().position(|x| x == &normalized_path) {
                self.cache_queue.remove(pos);
                self.cache_queue.push_back(normalized_path.clone());
            }
            return Some(Arc::clone(data));
        }

        let mut file_data = None;

        // 1. Dysk (katalog roboczy /base_path)
        let disk_path = if self.base_path.is_empty() {
            normalized_path.clone()
        } else {
            format!("{}/{}", self.base_path, normalized_path)
        };
        
        if let Ok(mut file) = File::open(&disk_path) {
            let mut buffer = Vec::new();
            if file.read_to_end(&mut buffer).is_ok() {
                file_data = Some(buffer);
            }
        }

        // 2. Mod Packs (od najnowszego/ostatnio dodanego)
        if file_data.is_none() {
            for pack in self.mod_packs.iter().rev() {
                if let Some(data) = read_file_from_archive(pack, &normalized_path) {
                    file_data = Some(data);
                    break;
                }
            }
        }
        
        // 3. Standard Packs (od najnowszego/ostatnio dodanego)
        if file_data.is_none() {
            for pack in self.std_packs.iter().rev() {
                if let Some(data) = read_file_from_archive(pack, &normalized_path) {
                    file_data = Some(data);
                    break;
                }
            }
        }

        if let Some(data) = file_data {
            let arc_data = Arc::new(data);
            self.add_to_cache(normalized_path.clone(), Arc::clone(&arc_data));
            return Some(arc_data);
        }

        None
    }
    
    fn add_to_cache(&mut self, path: String, data: Arc<Vec<u8>>) {
        let size = data.len();
        
        // Oprozniaj cache jesli przekracza limit
        while self.current_cache_size + size > self.max_cache_size && !self.cache_queue.is_empty() {
            if let Some(old_path) = self.cache_queue.pop_front() {
                if let Some(old_data) = self.file_cache.remove(&old_path) {
                    self.current_cache_size -= old_data.len();
                }
            }
        }
        
        self.file_cache.insert(path.clone(), data);
        self.cache_queue.push_back(path);
        self.current_cache_size += size;
    }
    
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
        self.cache_queue.clear();
        self.current_cache_size = 0;
    }
}
