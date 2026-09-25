//! Modul sound_buffer dla skrzynki m2-client-audio
//! Kontener bufora dzwiekowego i zarzadzanie pamiecia LRU
//! SoundBuffer, cache probek dzwiekowych, limit pamieci RAM i zwalnianie najstarszych

use std::collections::{HashMap, VecDeque};

/// Struktura przechowujaca surowe dane dzwiekowe oraz metadane
#[derive(Debug, Clone)]
pub struct SoundData {
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl SoundData {
    /// Oblicza rozmiar danych w pamieci RAM w bajtach
    pub fn size_in_bytes(&self) -> usize {
        self.data.capacity()
    }
}

/// Glowna struktura implementacyjna zarzadzajaca cache (LRU) probek
#[derive(Debug)]
pub struct SoundBuffer {
    /// Mapa przechowujaca dane probek
    cache: HashMap<String, SoundData>,
    /// Kolejka LRU (ostatnio uzyte sa na koncu)
    lru_queue: VecDeque<String>,
    /// Limit pamieci w bajtach
    max_memory_bytes: usize,
    /// Aktualnie zuzyta pamiec w bajtach
    current_memory_bytes: usize,
    /// Czy bufor zostal zainicjalizowany
    pub initialized: bool,
}

impl SoundBuffer {
    /// Tworzy nowy pusty SoundBuffer z domyslnym limitem pamieci (np. 64 MB)
    pub fn new() -> Self {
        Self::with_capacity(64 * 1024 * 1024)
    }

    /// Tworzy nowy pusty SoundBuffer z podanym limitem pamieci w bajtach
    pub fn with_capacity(max_memory_bytes: usize) -> Self {
        Self {
            cache: HashMap::new(),
            lru_queue: VecDeque::new(),
            max_memory_bytes,
            current_memory_bytes: 0,
            initialized: true,
        }
    }

    /// Zwraca true jesli bufor jest gotowy do uzycia
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Zwraca aktualnie zuzyta pamiec w bajtach
    pub fn current_memory_bytes(&self) -> usize {
        self.current_memory_bytes
    }

    /// Dodaje nowa probke do cache. Jesli przekroczono limit, usuwa najstarsze probki.
    pub fn insert(&mut self, name: String, data: SoundData) {
        let size = data.size_in_bytes();
        
        // Jesli jedna probka przekracza calkowity limit, nie mozemy jej dodac, 
        // ewentualnie dodajemy i czyscimy wszystko inne.
        // Tutaj zakladamy, ze jednak dodajemy i od razu evictujemy,
        // ale najpierw zrobimy miejsce, a potem dodamy.
        // Jednak jesli jej rozmiar jest wiekszy niz max, wyczyscimy wszystko i na koncu dodamy,
        // pamietajac ze po dodaniu nadal bedzie wieksza niz max, wiec przy nastepnym insercie
        // zostanie wyevictowana, co jest ok.
        
        if self.cache.contains_key(&name) {
            self.remove(&name);
        }

        self.evict_until_fits(size);

        self.cache.insert(name.clone(), data);
        self.lru_queue.push_back(name);
        self.current_memory_bytes += size;
    }

    /// Zwraca referencje do probki jesli istnieje, uaktualniajac jej pozycje w LRU
    pub fn get(&mut self, name: &str) -> Option<&SoundData> {
        if self.cache.contains_key(name) {
            // Uaktualnienie kolejnosci w LRU
            if let Some(pos) = self.lru_queue.iter().position(|x| x == name) {
                let key = self.lru_queue.remove(pos).unwrap();
                self.lru_queue.push_back(key);
            }
            self.cache.get(name)
        } else {
            None
        }
    }

    /// Usuwa probke z cache, aktualizujac zuzycie pamieci i kolejke
    pub fn remove(&mut self, name: &str) -> bool {
        if let Some(data) = self.cache.remove(name) {
            self.current_memory_bytes -= data.size_in_bytes();
            if let Some(pos) = self.lru_queue.iter().position(|x| x == name) {
                self.lru_queue.remove(pos);
            }
            true
        } else {
            false
        }
    }

    /// Czysci calkowicie cache, zwalniajac pamiec
    pub fn clear(&mut self) {
        self.cache.clear();
        self.lru_queue.clear();
        self.current_memory_bytes = 0;
    }

    /// Usuwa najstarsza probke (eviction), zwraca true jesli cos usunieto
    fn evict_oldest(&mut self) -> bool {
        if let Some(oldest_key) = self.lru_queue.pop_front() {
            if let Some(data) = self.cache.remove(&oldest_key) {
                self.current_memory_bytes -= data.size_in_bytes();
                return true;
            }
        }
        false
    }

    /// Zwalnia pamiec usuwajac najstarsze probki dopoki brakuje miejsca dla nowego elementu
    fn evict_until_fits(&mut self, required_bytes: usize) {
        // Zabezpieczenie przed nieskonczona petla w przypadku elementu wiekszego niz caly cache
        if required_bytes > self.max_memory_bytes {
            self.clear();
            return;
        }

        while self.current_memory_bytes + required_bytes > self.max_memory_bytes {
            if !self.evict_oldest() {
                break;
            }
        }
    }
}
