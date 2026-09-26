use std::collections::HashMap;

#[derive(Clone)]
pub struct CachedTexture {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u8>,
    pub last_used_tick: u64,
}

pub struct TextureCache {
    max_bytes: usize,
    current_bytes: usize,
    textures: HashMap<String, CachedTexture>,
}

impl TextureCache {
    pub fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            current_bytes: 0,
            textures: HashMap::new(),
        }
    }

    fn normalize_path(path: &str) -> String {
        path.to_lowercase().replace("\\", "/")
    }

    pub fn get_or_load<F>(
        &mut self,
        path: &str,
        cur_tick: u64,
        loader: F,
    ) -> Result<&CachedTexture, &'static str>
    where
        F: FnOnce() -> Result<(u32, u32, Vec<u8>), &'static str>,
    {
        let normalized_path = Self::normalize_path(path);

        if self.textures.contains_key(&normalized_path) {
            let texture = self.textures.get_mut(&normalized_path).unwrap();
            texture.last_used_tick = cur_tick;
            return Ok(self.textures.get(&normalized_path).unwrap());
        }

        let (width, height, rgba_data) = loader()?;
        let texture_size = rgba_data.len();

        let new_texture = CachedTexture {
            width,
            height,
            rgba_data,
            last_used_tick: cur_tick,
        };

        self.current_bytes += texture_size;
        self.textures.insert(normalized_path.clone(), new_texture);

        self.evict_if_needed();

        // The newly inserted texture might have been evicted if it is larger than max_bytes
        // and we just inserted it. But LRU logic should not evict the most recently used one
        // if there's other older textures. 
        // We'll return a reference to it. If it was the only one and got evicted (which means its size > max_bytes),
        // we'll return an error. Let's make sure it's in the cache.
        self.textures
            .get(&normalized_path)
            .ok_or("Texture was loaded but immediately evicted due to memory limits")
    }

    fn evict_if_needed(&mut self) {
        while self.current_bytes > self.max_bytes && !self.textures.is_empty() {
            let mut oldest_path = None;
            let mut oldest_tick = u64::MAX;

            for (path, texture) in &self.textures {
                if texture.last_used_tick < oldest_tick {
                    oldest_tick = texture.last_used_tick;
                    oldest_path = Some(path.clone());
                }
            }

            if let Some(path) = oldest_path {
                if let Some(texture) = self.textures.remove(&path) {
                    self.current_bytes = self.current_bytes.saturating_sub(texture.rgba_data.len());
                }
            } else {
                break; // Should not happen, but prevents infinite loop
            }
        }
    }

    pub fn current_memory_usage(&self) -> usize {
        self.current_bytes
    }

    pub fn clear(&mut self) {
        self.textures.clear();
        self.current_bytes = 0;
    }
}
