#[path = "granny_gr2_binary_parser.rs"]
pub mod granny_gr2_binary_parser;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use granny_gr2_binary_parser::{GrannyMesh, GrannySkeleton};

#[derive(Debug, Clone)]
pub struct GrannyModelData {
    pub meshes: Vec<GrannyMesh>,
    pub skeletons: Vec<GrannySkeleton>,
}

pub struct GrannyModelLruCache {
    capacity: usize,
    cache: HashMap<String, Arc<GrannyModelData>>,
    order: VecDeque<String>,
}

impl GrannyModelLruCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Arc<GrannyModelData>> {
        if self.cache.contains_key(key) {
            // Move to the front of the MRU queue (which is the back of VecDeque)
            if let Some(pos) = self.order.iter().position(|k| k == key) {
                self.order.remove(pos);
            }
            self.order.push_back(key.to_string());
            self.cache.get(key).cloned()
        } else {
            None
        }
    }

    pub fn put(&mut self, key: String, data: Arc<GrannyModelData>) {
        if self.capacity == 0 {
            return;
        }

        if self.cache.contains_key(&key) {
            // Update the existing entry and make it MRU
            self.cache.insert(key.clone(), data);
            if let Some(pos) = self.order.iter().position(|k| k == &key) {
                self.order.remove(pos);
            }
            self.order.push_back(key);
        } else {
            if self.cache.len() >= self.capacity {
                // Evict the least recently used element (front of VecDeque)
                if let Some(lru_key) = self.order.pop_front() {
                    self.cache.remove(&lru_key);
                }
            }
            // Insert the new element
            self.cache.insert(key.clone(), data);
            self.order.push_back(key);
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }
}
