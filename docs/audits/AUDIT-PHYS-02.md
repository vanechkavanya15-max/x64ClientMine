# Clean Code Review: AUDIT-PHYS-02 — SphereLib Hierarchical BVH

## Podsumowanie architektury
Moduł `SphereLib` implementuje hierarchiczną strukturę przestrzenną typu BVH (Bounding Volume Hierarchy) zoptymalizowaną do zadań takich jak sprawdzanie widoczności (frustum culling), testy promieni (raycasting) czy wykrywanie kolizji zasięgu (range test). Klasa bazowa `SpherePack` reprezentuje sferę otaczającą, a relacje miedzy obiektami oparte są na ręcznie zarządzanych drzewach wskaźników (parent, children, next/prev sibling). Występuje także ręczny system zarządzania pamięcią za pomocą struktury `Pool` oraz dwóch FIFO służących do odświeżania (`Integrate` i `Recompute`). Pamięć rezerwowana jest wstępnie (object pooling).

## Zestawienie zidentyfikowanych defektow kodu i niespojnosci

1. **Ręczne zarządzanie cyklem życia i strukturą pamięci (Zasada RAII i pamięć)**
   - *Plik*: `src/SphereLib/spherepack.h` / `pool.h`
   - *Opis*: Użyto przestarzałego kontenera `Pool<Type>` z własną logiką tworzenia dwukierunkowej listy za pomocą gołych wskaźników (raw pointers). `SpherePack` również implementuje dwukierunkową listę wiązaną dla drzewa oraz listy braci (np. `mNextSibling`, `mPrevSibling`).
   - *Wpływ*: Brak RAII, trudne debugowanie wycieków pamięci i ryzyko tzw. "dangling pointers" w sytuacjach współbieżnych oraz przy operacjach odpinania/przypinania (`Unlink`, `AddChild`). Nowoczesne C++ i Rust stosują do tego referencje (ze sprawdzaniem `borrow checker`) lub wektory indeksów zamiast wskaźników, co mocno poprawia CPU cache hit rate.

2. **Bezpieczeństwo operacji na gołych tablicach dynamicznych (Zasada RAII)**
   - *Plik*: `src/SphereLib/spherepack.h` (klasa `SpherePackFifo`)
   - *Opis*: Dynamiczna alokacja wskaźników `new SpherePack *[mFifoSize]` wraz ze zwalnianiem `delete [] mFifo`.
   - *Wpływ*: Silnie łamie RAII; wyjątkowe warunki przed inicjalizacją mogą doprowadzić do wycieków. Zamiast tego należy stosować `std::vector` lub specjalizowane kolejki. Gołe przypisania i odczyty z tablicy są narażone na brak kontroli granic (buffer overrun).

3. **Współbieżność (Thread Safety)**
   - *Plik*: Cały moduł `SphereLib`
   - *Opis*: Żadna operacja wewnątrz drzewa (`Unlink`, modyfikacja list wiązanych, odświeżanie fifo) nie jest zabezpieczona prymitywami synchronizacyjnymi. 
   - *Wpływ*: Jeśli pętla sieciowa albo aktualizacja gier wywołują operacje odświeżające (np. `NewPos` przenosi `SpherePack` do `AddIntegrate`), a wątek renderujący analizuje widoczność, wystąpi race condition mogące uszkodzić strukturę listy (wskaźniki next/prev). Wymagana jest przemyślana architektura (np. lock-free queues dla message-passing lub współdzielenie struktury `std::shared_mutex`).

4. **Wydajność na ścieżce krytycznej i optymalizacja SIMD**
   - *Plik*: `src/SphereLib/spherepack.cpp` (metody testowe jak `SweptSphereTest`, `RangeTest`)
   - *Opis*: Testy odległości i objętości są wyliczane tradycyjnie operacja po operacji bez użycia instrukcji wektorowych i operują na pointer chasing w pamięci sterty (struktura węzłów alokowana niezależnie w Pool). Odświeżanie hierarchii (aktualizacje promienia) generuje spore napotkania gałęzi (branching) i uniemożliwia wektoryzację.
   - *Wpływ*: Słaba wydajność lokalności danych w CPU Cache na ścieżce renderingu i kolizji przy dużej liczbie obiektów w MMO.

## Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### C++23: Przejście z gołych wskaźników i Object Pooli na Data-Oriented Design (DOD) z `std::vector` i Entt

```cpp
#include <vector>
#include <memory>
#include <optional>
#include <span>

struct SphereData {
    Vector3d center;
    float radius;
    uint32_t flags;
};

// Płaska struktura przyjazna dla pamięci podręcznej. Zamiast wskaźników przechowujemy indeksy węzłów.
struct BVHNode {
    SphereData bounds;
    int32_t parent_idx = -1;
    int32_t first_child_idx = -1;
    int32_t next_sibling_idx = -1;
    
    // std::optional używany w nowszym C++ do obsługi wskaźnika userdata (odseparowanie od fizyki)
    void* user_data = nullptr; 
};

class SphereManager {
private:
    std::vector<BVHNode> nodes;
    
    // Użycie nowoczesnych mechanizmów do lock-free kolejek przy synchronizacji z siecią i re-integracji.
    // Zamiast SpherePackFifo, dedykowane kontenery std::deque lub boost::lockfree.
    
public:
    // Praca z std::span dla odporności na bufory wejściowe z sieci (przy integracji sfer)
    void update_positions(std::span<const Vector3d> new_positions, std::span<const uint32_t> node_indices);
};
```

### Rust: Bezpieczeństwo współbieżności i całkowita izolacja pamięci (Lifetimes i Borrow Checker)

```rust
use std::sync::{Arc, RwLock};

struct SphereData {
    center: [f32; 3],
    radius: f32,
    flags: u32,
}

// Hierarchiczna struktura bez dangling pointers, oparta o Indeksy zamiast referencji
// Oszczędza problemy z lifetime przy modyfikacji drzewa
struct BVHNode {
    bounds: SphereData,
    parent: Option<usize>,
    first_child: Option<usize>,
    next_sibling: Option<usize>,
    user_data: Option<usize>, // Indeks do encji ECS
}

pub struct SphereManager {
    nodes: RwLock<Vec<BVHNode>>,
}

impl SphereManager {
    // Thread safe update: Wymagany lock na czas modyfikacji
    pub fn update_position(&self, node_idx: usize, new_pos: [f32; 3]) {
        let mut nodes = self.nodes.write().unwrap(); // RwLock zabezpiecza dostęp miedzy threadami
        if let Some(node) = nodes.get_mut(node_idx) {
            node.bounds.center = new_pos;
            // Dodatkowa logika reintegracji oznaczająca brudny stan
        }
    }
}
```

**Kluczowe modyfikacje:**
1. Usunięcie `SpherePackFifo` na rzecz `std::vector` (C++) / `Vec` (Rust) i unikanie ręcznej obsługi tablic `new`/`delete`.
2. Zamiana referencji/wskaźników wiążących (parent/sibling) na unikalne ID/Indeksy w obrębie managera. Takie rozwiązanie ułatwia Data-Oriented Design (liniowy dostęp w pamięci) i ucieka od problemu wiszących referencji i błędnego zwalniania pamięci.
3. Synchronizacja dostępów sieciowych, fizyki i wyświetlania przez bezpieczne blokady współbieżne (`std::shared_mutex` lub Rust `RwLock`).
