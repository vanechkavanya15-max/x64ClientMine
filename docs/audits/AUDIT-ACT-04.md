# Raport z Przeglądu Kodu (Clean Code Review) - AUDIT-ACT-04

## Podsumowanie architektury zbadanych modułów
Moduły `RaceData` oraz `RaceManager` odpowiadają za zarządzanie danymi o rasach, modelach i animacjach w grze (prawdopodobnie Metin2). `RaceManager` działa jako Singleton zarządzający zasobami (mapa `CRaceData*`), implementujący podstawową wielowątkowość dla ładowania danych w tle. `RaceData` przechowuje informacje o siatkach 3D, punktach montażu (attaching bones), systemach cząsteczkowych (smoke) oraz kombosach i animacjach.

Architektura opiera się na klasycznym podejściu z surowymi wskaźnikami, własnych alokatorach pamięci (`CDynamicPool`) oraz ręcznej synchronizacji `std::mutex`. Zidentyfikowano szereg problemów architektonicznych, które znacząco obniżają bezpieczeństwo i stabilność (w szczególności błędy współbieżności i potencjalne naruszenia pamięci).

## Zestawienie zidentyfikowanych defektów kodu i niespójności

1. **Współbieżność: Wyścig danych na zmiennej statycznej (Data Race)**
   - **Plik:** `src/GameLib/RaceManager.cpp`
   - **Metoda:** `CRaceManager::GetFullPathFileName`
   - **Opis:** Funkcja używa niestrzeżonej zmiennej `static std::string s_stFileName`, która jest modyfikowana przy każdym wywołaniu. Przy dostępie z wielu wątków (np. wątek sieciowy, renderera i ładowania w tle) dochodzi do uszkodzenia struktury std::string i undefined behavior.
   - **Wpływ:** Krytyczny (Crash klienta, nadpisywanie wskaźników pamięci).

2. **Współbieżność: Brak synchronizacji dla `m_pSelectedRaceData`**
   - **Plik:** `src/GameLib/RaceManager.cpp`, `src/GameLib/RaceManager.h`
   - **Opis:** Surowy wskaźnik `m_pSelectedRaceData` jest modyfikowany w `SelectRace` i odczytywany w `LoadRaceData` / `LoadLocalRaceData` bez żadnego zabezpieczenia muteksem ani w postaci wskaźnika atomowego.
   - **Wpływ:** Średni (Użycie usuniętego lub nadpisanego wskaźnika, niespójny stan wybranej rasy).

3. **Zarządzanie zasobami: Thread-Safety niestandardowych alokatorów (`CDynamicPool`)**
   - **Plik:** `src/GameLib/RaceData.cpp`
   - **Metoda:** `CRaceData::New()` i `CRaceData::Delete()`
   - **Opis:** Obiekty `CRaceData` są alokowane z globalnego poola `ms_kPool`. Funkcja `__LoadRaceData` z `RaceManager` jest wołana współbieżnie (bez trzymania blokady mapy), co oznacza, że różne wątki będą naraz wywoływać `CRaceData::New()`. Jeśli `CDynamicPool` nie używa `std::mutex` wewnątrz (a standardowo w tym silniku nie używa), dojdzie do uszkodzenia sterty poola.
   - **Wpływ:** Wysoki (Uszkodzenie pamięci poola alokatora).

4. **Odporność i fragmentacja: Ryzyko przepełnienia bufora i obcięcia ścieżek**
   - **Plik:** `src/GameLib/RaceManager.cpp`
   - **Metoda:** `CRaceManager::__LoadRaceData`
   - **Opis:** Użycie buforów na stosie `char shapeFileName[256];` wraz z `_snprintf`. Zbyt długa ścieżka `pathName` lub zmienna złośliwie zmodyfikowana z serwera może zostać bezszelestnie obcięta, skutkując ładowaniem złych plików lub ukrytymi błędami odczytu.
   - **Wpływ:** Średni (Możliwe problemy z długimi ścieżkami w wirtualnym systemie plików).

## Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

Zamiast ręcznie zarządzać cyklem życia za pomocą `New()`, `Delete()` oraz muteksów chroniących surowe wskaźniki, sugerowane jest przejście na paradygmat RAII.

### Podejście C++23:
```cpp
// RaceManager.h
#include <shared_mutex>
#include <memory>
#include <string_view>
#include <filesystem>

class CRaceManager {
public:
    std::shared_ptr<CRaceData> GetRaceData(uint32_t raceIndex);
    void SelectRace(uint32_t raceIndex);
    std::shared_ptr<CRaceData> GetSelectedRaceData();
    
    // Zastąpienie statycznego std::string
    std::filesystem::path GetFullPath(std::string_view fileName) const;

private:
    std::shared_ptr<CRaceData> LoadRaceDataInternal(uint32_t raceIndex);

    mutable std::shared_mutex m_raceDataMutex;
    std::unordered_map<uint32_t, std::shared_ptr<CRaceData>> m_raceDataMap;
    
    // Atomowy wskaźnik chroni przed data race'em na ścieżce wyboru
    std::atomic<std::shared_ptr<CRaceData>> m_selectedRaceData;
};

// RaceManager.cpp
std::filesystem::path CRaceManager::GetFullPath(std::string_view fileName) const {
    if (fileName.empty()) return {};
    if (fileName.front() == '.') return fileName;
    return std::filesystem::path(m_strPathName) / fileName;
}
```

### Podejście idiologiczne Rust (Architecture Concept):
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::{Path, PathBuf};

pub struct RaceData {
    pub race_index: u32,
    // Model i dane ...
}

pub struct RaceManager {
    base_path: PathBuf,
    race_data_map: RwLock<HashMap<u32, Arc<RaceData>>>,
    selected_race: RwLock<Option<Arc<RaceData>>>,
    loading_races: RwLock<HashSet<u32>>,
}

impl RaceManager {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            race_data_map: RwLock::new(HashMap::new()),
            selected_race: RwLock::new(None),
            loading_races: RwLock::new(HashSet::new()),
        }
    }

    pub fn get_race_data(&self, race_index: u32) -> Option<Arc<RaceData>> {
        {
            let map = self.race_data_map.read().unwrap();
            if let Some(data) = map.get(&race_index) {
                return Some(Arc::clone(data));
            }
        }
        
        // Ładowanie z pliku bez blokowania całej mapy na czas I/O
        let new_data = self.load_race_data_internal(race_index)?;
        let arc_data = Arc::new(new_data);
        
        let mut map = self.race_data_map.write().unwrap();
        // Sprawdzenie double-checked locking
        let data = map.entry(race_index).or_insert(arc_data);
        Some(Arc::clone(data))
    }
    
    pub fn get_full_path(&self, file_name: &str) -> PathBuf {
        if file_name.starts_with('.') {
            PathBuf::from(file_name)
        } else {
            self.base_path.join(file_name)
        }
    }
}
```

### Podsumowanie refaktoryzacji:
1. Usunięcie `CDynamicPool` i zastąpienie standardowymi smart pointerami - eliminuje wycieki pamięci i konieczność ręcznego `Delete`.
2. Zastosowanie `std::shared_mutex` i `std::atomic<std::shared_ptr<T>>` - bezpieczny dostęp do zasobów z wielu wątków z zachowaniem optymalnego odczytu (lock-free odczyt w atomics).
3. Zamiana `_snprintf` na `std::filesystem::path` (C++23) i `PathBuf` (Rust) - eliminuje całkowicie podatności na przekroczenia granic buforów (Buffer Overflows) oraz ucinki tekstowe na łączonych plikach systemowych.
