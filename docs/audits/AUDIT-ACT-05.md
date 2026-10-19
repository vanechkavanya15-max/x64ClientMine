# Raport z Przeglądu Jakości: AUDIT-ACT-05 — ItemData & ItemManager

## 1. Podsumowanie Architektury
Moduły `ItemData` oraz `ItemManager` realizują centralne zarządzanie metadanymi przedmiotów w kliencie gry, wczytywanie konfiguracji z plików (tekstowych, binarnych oraz skompresowanych `.eix/.epk`) i serwowanie tych danych reszcie aplikacji.
Zbudowane w oparciu o archaiczny standard C++ (najprawdopodobniej C++98/03 z elementami C++11), silnie polegają na własnym zarządzaniu pamięcią (Object Pool), "nagich" wskaźnikach własnościowych (raw pointers) oraz manualnym parsowaniu buforów (np. za pomocą `memcpy`).

Brakuje bezpiecznych abstrakcji pamięciowych (np. `std::unique_ptr`, `std::shared_ptr`, `std::span`), ochrony wątków (brak wbudowanych mechanizmów synchronizacji dostępu w `CItemManager`), oraz sprawdzania granic w kluczowych pętlach parsowania binarnych plików `item_proto`.

## 2. Zestawienie Zidentyfikowanych Defektów i Niespójności

### A. Zarządzanie zasobami i cykl życia (RAII & Memory Lifetime)
- **Plik**: `src/GameLib/ItemManager.cpp`, linia ~310 (`LoadItemTable`)
  - **Mechanizm**: `BYTE * pbData = new BYTE[dwDataSize];` z ręcznym zwalnianiem pamięci na ścieżkach sukcesu i błędu.
  - **Defekt**: Klasyczny antywzorzec (memory leak risk w przypadku rzucenia wyjątku). 
  - **Rozwiązanie**: Należy użyć `std::vector<uint8_t>` lub przynajmniej `std::unique_ptr<uint8_t[]>`.
- **Plik**: `src/GameLib/ItemManager.h`, linia ~28 (`m_ItemMap`, `m_vec_ItemRange`)
  - **Mechanizm**: Magazynowanie "nagich" wskaźników własnościowych `CItemData*`.
  - **Defekt**: Obiekty `CItemData` są alokowane w customowym poolu, ale brak kontroli cyklu życia poprzez smart pointery (np. custom deleter w `std::unique_ptr` czy użycie `std::shared_ptr`).

### B. Poprawność parsera i odporność strumienia (Protocol Stream Robustness)
- **Plik**: `src/GameLib/ItemManager.cpp`, linia ~260 (`LoadItemTable`)
  - **Mechanizm**: Obliczanie przesunięcia w buforze `uint8_t* p = file.data(); memcpy(&dwFourCC, p, sizeof(DWORD)); p += sizeof(DWORD);` i sprawdzanie długości `file.size() - (p - file.data()) < dwDataSize`.
  - **Defekt**: Bardzo niebezpieczny pattern (C-style buffer traversal). Brak rygorystycznego bounds checkingu we wczesnych etapach – np. jeśli `file.size() < (sizeof(DWORD) * 5)` plik mógłby wywołać Undefined Behavior przy `memcpy`, choć autor użył wczesnego if-a dla nagłówków.
  - **Wpływ**: Podatność na zniekształcone pliki (buffer over-read, truncating), np. modyfikacja pliku po stronie klienta może spowodować crash.

### C. Poprawność współbieżności i Synchronizacja
- **Plik**: `src/GameLib/ItemManager.cpp`, `CItemManager` class
  - **Mechanizm**: Globalny dostęp poprzez `CSingleton<CItemManager>`.
  - **Defekt**: Mutacje w runtime (np. `MakeItemData` modyfikuje `m_ItemMap`) nie są chronione mutexem (np. brakuje `std::shared_mutex` lub odpowiednika w Rust - `RwLock`). Jeśli wczytywanie / odczytywanie odbywa się na wielu wątkach (np. rendering vs sieć), to `map::insert` i powtórny `find` stworzą data race, powodując memory corruption lub crashe.

### D. Wydajność na ścieżce krytycznej
- **Plik**: `src/GameLib/ItemManager.cpp`, linia ~62 (`GetItemDataPointer`)
  - **Mechanizm**: Użycie `std::upper_bound` na wektorze `m_vec_ItemRange`, by wyszukać wariant przedmiotu po zakresie.
  - **Defekt**: Każde chybienie w `std::map` kosztuje operację `O(log N)` z cache miss, a następnie przeskakuje do przeszukiwania wektora o potencjalnie niskiej lokalności cache, biorąc pod uwagę wskaźniki (pointer chasing - `const CItemData::TItemTable * pTable = (*it)->GetTable();`).

## 3. Wzorcowa Propozycja Refaktoryzacji do Nowoczesnego C++23 / Rust

### Paradygmat w C++23:
```cpp
// 1. Zastąpienie gołych wskaźników przez std::shared_ptr dla bezpieczeństwa wątkowego
#include <memory>
#include <span>
#include <string_view>
#include <shared_mutex>

class ItemManager {
public:
    // Thread-safe fetch
    std::shared_ptr<const ItemData> GetItemData(uint32_t vnum) const noexcept {
        std::shared_lock lock(mutex_);
        if (auto it = itemMap_.find(vnum); it != itemMap_.end()) {
            return it->second; // Zwraca kopię std::shared_ptr zwiększając ref-count pod blokadą
        }
        return SearchRange(vnum);
    }

    // Bezpieczne parsowanie buforów ze std::span (eliminuje out-of-bounds memcpy)
    bool LoadItemTable(std::span<const std::byte> buffer) {
        if (buffer.size() < sizeof(Header)) return false;
        // ...
    }

private:
    mutable std::shared_mutex mutex_;
    std::unordered_map<uint32_t, std::shared_ptr<const ItemData>> itemMap_;
    std::vector<std::shared_ptr<const ItemData>> itemRange_; 
};
```

### Opcjonalna Architektura w Rust:
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct ItemData {
    pub vnum: u32,
    pub name: String, // Bezpieczne stringi, zamiast char[64]
    // ...
}

pub struct ItemManager {
    // Odporny na deadlock-free access i Thread Safe
    item_map: RwLock<HashMap<u32, Arc<ItemData>>>,
    item_range: RwLock<Vec<Arc<ItemData>>>,
}

impl ItemManager {
    pub fn get_item_data(&self, vnum: u32) -> Option<Arc<ItemData>> {
        let map_read = self.item_map.read().unwrap();
        if let Some(item) = map_read.get(&vnum) {
            return Some(item.clone());
        }
        // Fallback dla zakresów
        None
    }

    pub fn load_item_table(&self, data: &[u8]) -> Result<(), &'static str> {
        // Zero-copy parsing np. przy użyciu bilbioteki "nom" lub "zerocopy"
        // Bezpieczny slice zamiast pointer arithmetic
        if data.len() < 16 {
            return Err("Buffer too small");
        }
        // ...
        Ok(())
    }
}
```

Dzięki przejściu na `std::span` (C++) / slajsy (Rust) całkowicie eliminuje się ryzyko odczytu poza pamięcią, a wzorzec ownership/borrowing pozwala kompilatorowi upewnić się, że nie powstaną wycieki pamięci. Dodatkowo zwracanie `std::shared_ptr<const ItemData>`/`Arc<ItemData>` chroni przed wyścigami w wielowątkowym środowisku.
