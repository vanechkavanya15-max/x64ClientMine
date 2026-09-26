use std::collections::HashMap;

/// Reprezentuje pojedyncza probke dzwieku zaweirajaca jej dane
pub struct WavAudio {
    pub name: String,
    pub data: Vec<u8>,
}

impl WavAudio {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self { name, data }
    }

    /// Zwraca rozmiar danych dzwieku w bajtach
    pub fn size_in_bytes(&self) -> usize {
        self.data.len()
    }
}

/// Struktura pomocnicza dla cache, przechowujaca dzwiek i czas ostatniego uzycia (do algorytmu LRU)
struct CacheEntry {
    pub audio: WavAudio,
    pub last_used_time: u32,
}

/// Modul implementujacy bufor pamieci dla probek audio z algorytmem LRU Cache
pub struct SoundBufferCache {
    entries: HashMap<String, CacheEntry>,
    max_memory_bytes: usize,
    current_memory_bytes: usize,
}

impl SoundBufferCache {
    /// Tworzy nowy bufor pamieci dzwieku z podanym limitem
    pub fn new(max_memory_bytes: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_memory_bytes,
            current_memory_bytes: 0,
        }
    }

    /// Tworzy nowy bufor pamieci z domyslnym limitem 32 MB
    pub fn default() -> Self {
        Self::new(33_554_432)
    }

    /// Zwraca obecne uzycie pamieci przez cache w bajtach
    pub fn current_memory_usage(&self) -> usize {
        self.current_memory_bytes
    }

    /// Calkowicie czysci cache, zwalniajac cala zajeta przez niego pamiec
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_memory_bytes = 0;
    }

    /// Zwraca dzwiek z cache lub laduje go za pomoca podanej funkcji loader.
    /// Przy kazdym dostepie odswieza czas uzycia `cur_time_ms`.
    /// Utrzymuje zuzycie pamieci w ryzach limitu za pomoca algorytmu LRU.
    pub fn get_or_load<F>(
        &mut self,
        name: &str,
        cur_time_ms: u32,
        loader: F,
    ) -> Result<&WavAudio, &'static str>
    where
        F: FnOnce() -> Result<WavAudio, &'static str>,
    {
        if let Some(entry) = self.entries.get_mut(name) {
            entry.last_used_time = cur_time_ms;
        } else {
            let new_audio = loader()?;
            let size = new_audio.size_in_bytes();

            self.evict_until_fits(size);

            self.current_memory_bytes += size;
            self.entries.insert(
                name.to_string(),
                CacheEntry {
                    audio: new_audio,
                    last_used_time: cur_time_ms,
                },
            );
        }

        Ok(&self.entries.get(name).unwrap().audio)
    }

    /// Metoda wewnetrzna sluzaca do ewaluacji i wyrzucania najstarszych elementow (LRU)
    /// zeby zwolnic `needed_size` bajtow.
    fn evict_until_fits(&mut self, needed_size: usize) {
        if needed_size > self.max_memory_bytes {
            self.clear();
            return;
        }

        while self.current_memory_bytes + needed_size > self.max_memory_bytes {
            if self.entries.is_empty() {
                break;
            }

            let mut oldest_key = None;
            let mut oldest_time = u32::MAX;

            for (key, entry) in self.entries.iter() {
                if entry.last_used_time < oldest_time {
                    oldest_time = entry.last_used_time;
                    oldest_key = Some(key.clone());
                }
            }

            if let Some(key) = oldest_key {
                if let Some(entry) = self.entries.remove(&key) {
                    self.current_memory_bytes -= entry.audio.size_in_bytes();
                }
            } else {
                break;
            }
        }
    }
}

impl Default for SoundBufferCache {
    fn default() -> Self {
        Self::default()
    }
}
