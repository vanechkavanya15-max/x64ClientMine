use std::collections::HashMap;

/// Reprezentuje zdekodowane dane dzwiekowe z ich parametrami.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedAudio {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<i16>,
}

impl DecodedAudio {
    pub fn new(channels: u16, sample_rate: u32, samples: Vec<i16>) -> Self {
        Self {
            channels,
            sample_rate,
            samples,
        }
    }

    /// Zwraca rozmiar zdekodowanych probek w bajtach.
    pub fn size_in_bytes(&self) -> usize {
        self.samples.len() * std::mem::size_of::<i16>()
    }
}

/// Struktura pomocnicza dla cache, przechowujaca dzwiek i czas (tick) ostatniego uzycia.
#[derive(Debug, Clone)]
struct LruEntry {
    pub audio: DecodedAudio,
    pub last_tick: u64,
}

/// Cache pamieciowy LRU (Least Recently Used) dla zdekodowanych probek audio (np. dzwieki walki, kroki).
#[derive(Debug, Clone)]
pub struct AudioSampleLruCache {
    entries: HashMap<String, LruEntry>,
    max_memory_bytes: usize,
    current_memory_bytes: usize,
    current_tick: u64,
}

impl AudioSampleLruCache {
    /// Tworzy nowy bufor pamieci dzwieku z podanym limitem.
    pub fn new(max_memory_bytes: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_memory_bytes,
            current_memory_bytes: 0,
            current_tick: 0,
        }
    }

    /// Zwraca obecne uzycie pamieci przez cache w bajtach.
    pub fn current_memory_usage(&self) -> usize {
        self.current_memory_bytes
    }

    /// Calkowicie czysci cache, zwalniajac cala zajeta przez niego pamiec.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_memory_bytes = 0;
    }
    
    /// Zwraca true, jezeli dany klucz istnieje w cache.
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }

    /// Pobiera referencje do dzwieku i aktualizuje jego czas (tick) ostatniego uzycia.
    pub fn get_ref(&mut self, key: &str) -> Option<&DecodedAudio> {
        self.current_tick += 1;
        if let Some(entry) = self.entries.get_mut(key) {
            entry.last_tick = self.current_tick;
            Some(&entry.audio)
        } else {
            None
        }
    }

    /// Wstawia zdekodowane audio pod dany klucz. Jesli dodanie tego audio
    /// przekroczy maksymalna ilosc pamieci, starsze wpisy sa usuwane.
    /// Jezeli sama probka jest wieksza od max_memory_bytes, zostanie odrzucona.
    pub fn put(&mut self, key: &str, audio: DecodedAudio) -> Result<(), &'static str> {
        let size = audio.size_in_bytes();

        // Jesli pojedynczy plik jest wiekszy niz pojemnosc calego bufora, odrzuc (bez niszczenia reszty bez powodu)
        if size > self.max_memory_bytes {
            return Err("Probka jest wieksza niz calkowita maksymalna pojemnosc cache");
        }
        
        self.current_tick += 1;
        
        // Zmniejsz rozmiar o juz istniejacy klucz, poniewaz bedzie on nadpisany
        // Uzycie remove() zapobiega bledowi podwojnego odejmowania podczas ewaluacji LRU
        if let Some(existing_entry) = self.entries.remove(key) {
            self.current_memory_bytes -= existing_entry.audio.size_in_bytes();
        }

        self.evict_until_fits(size);

        self.current_memory_bytes += size;
        self.entries.insert(
            key.to_string(),
            LruEntry {
                audio,
                last_tick: self.current_tick,
            },
        );
        
        Ok(())
    }

    /// Metoda wewnetrzna sluzaca do wyrzucania najstarszych elementow (LRU)
    /// zeby zwolnic `needed_size` bajtow.
    fn evict_until_fits(&mut self, needed_size: usize) {
        while self.current_memory_bytes + needed_size > self.max_memory_bytes {
            if self.entries.is_empty() {
                break;
            }

            let mut oldest_key = None;
            let mut oldest_tick = u64::MAX;

            for (key, entry) in self.entries.iter() {
                if entry.last_tick < oldest_tick {
                    oldest_tick = entry.last_tick;
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

impl Default for AudioSampleLruCache {
    fn default() -> Self {
        Self::new(32 * 1024 * 1024)
    }
}
