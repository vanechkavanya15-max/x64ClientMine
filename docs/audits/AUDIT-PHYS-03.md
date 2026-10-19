# Raport z audytu kodu: AUDIT-PHYS-03 — Dynamic Sphere Raycast & Wall Slide

## 1. Podsumowanie architektury zbadanych modułów

Moduł `SphereLib` wdraża naiwną implementację struktury Bounding Volume Hierarchy (BVH) opartą o "Drzewa Sfer" (Sphere Trees). Kod jest silnie uzależniony od ręcznego zarządzania pamięcią przy pomocy puli (klasa `Pool` w `pool.h`), dwukierunkowych wskaźników listowych (rodzeństwo, dzieci, rodzic), i manualnego cyklu życia wskaźników do surowych danych w `mUserData`. Z racji wykorzystywania podejścia "Array of Structures" (AoS) zamiast "Structure of Arrays" (SoA) i użycia rzutowań z `void*`, architektura nie wykorzystuje potencjału nowoczesnych standardów (C++23) i narażona jest na liczne usterki z obszaru wielowątkowości oraz bezpieczeństwa pamięci.

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

| Plik | Linia | Defekt / Niespójność | Wpływ na stabilność |
|------|-------|----------------------|---------------------|
| `pool.h` | 51-57 | **RAII & Memory Lifetime**: Brak jawnego kasowania całej listy elementów, `Release(void)` zwalnia tylko surowy bufor, lecz cykl życia przechowywanych typów, wskaźników lub ich destruktory nie są adekwatnie wywoływane w przypadku złożonych obiektów, a ręczne zarządzanie listą podwójnie wiązaną przez `SetNext` i `SetPrevious` na surowych wskaźnikach promuje wycieki. | Wysoki: Ryzyko wycieków przy wyjściu z błędem z aplikacji, wiszące wskaźniki przy destrukcji `SpherePack`. |
| `spherepack.h` | 277 | **Business Logic & State Determinism**: `void *mUserData;` ułatwia ominięcie systemu typów. | Średni: Nieokreślone zachowanie (UB) przy rzutowaniu obiektów na zły typ podczas operacji callback. |
| `spherepack.h` | 266-267 | **RAII & Memory Lifetime**: Używanie `SpherePack **mFifo1;` oraz `SpherePack **mFifo2;` jako wskaźniki na elementy Fifo. Kasowanie węzła z drzewa, które jest obecne w Fifo powoduje wiszące wskaźniki przy odczycie w `Process`. | Krytyczny: Odwołania do nieistniejących stref pamięci podczas `SpherePackFactory::Process()` po usunięciu powiązanych węzłów. |
| `spherepack.h` | 382 | **Thread Safety & Synchronization**: Zmienna wskaźnikowa `SpherePackCallback *mCallback;` modyfikowana podczas operacji takich jak `RayTrace` bez mechanizmów ochrony i synchronizacji (`std::mutex`, czy design lock-free). | Krytyczny: Wywoływanie globalnych funkcji (np. `RayTrace`) z kilku wątków spowoduje race condition, re-entrancy i awarię w callbacku. |
| `spherepack.cpp` | 134-142 | **Business Logic & Determinism**: Podatność w integracji. Pętla `Process()` iteruje przez zrekomputowane jednostki uwalniając je bez wcześniejszego zapobiegania nieprawidłowym odwołaniom krzyżowym w `SpherePackFifo`. | Krytyczny: Niestabilność silnika w momentach silnego obciążenia re-integracją elementów w jednej klatce. |
| `spherepack.cpp` | 643-659 | **Performance Optimization**: `VisibilityTest` wykorzystuje głęboką rekursję z testowaniem przeciążonymi flagami i wskaźnikami rodzeństwa. Brak dostępu "cache-friendly". | Średni/Niski: L2/L3 cache miss dla każdego sferycznego nodu na krytycznej ścieżce renderu. |
| `sphere.cpp` | 34-60 | **Performance Optimization / Stream Robustness**: `RayIntersection` posiada bugfix, w którym `V*=-1` odwraca kierunek wektora jeśli origin jest wewnątrz (pętla z Graphics Gem p.388). Nie uwzględnia przypadków z brakiem determinizmu z wartościami NaN czy nieskończonością (INF) podczas pierwiastkowania. | Niski: Spadek poprawności kolizji przy wektorach brzegowych i zniekształcenia rzutu. |

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

Zgodnie z wymaganiami minimalizacji ryzyka pamięciowego i braku synchronizacji w wielowątkowości, rekomenduje się migrację silnika fizyki z bazującej na wskaźnikach do bazującej na płaskich tablicach identyfikatorów (`Entity Component System` / `SoA`).

### Propozycja C++23 (Modernizacja)

```cpp
// 1. Zastąpienie rzutowań `void*` std::variant i ubezpieczeniem cyklu życia przy pomocy std::unique_ptr lub std::shared_ptr, lub systemów ECS.
// 2. Eliminacja cyklicznych wskaźników dzieci i rodziców na rzecz wektorów indeksowych.
// 3. Dodanie lock-free callback / thread-safe views przez std::span.

#include <vector>
#include <variant>
#include <memory>
#include <span_ext>

struct SphereData {
    alignas(16) std::array<float, 4> center_radius; // xyz + radius dla SIMD
};

class ModernSpherePackFactory {
public:
    void ProcessFrame(std::span<const EntityCommand> commands);
    void RayTrace(const Vector3d& origin, const Vector3d& dir, float max_dist, std::invocable<const Intersection&> auto&& callback) const;

private:
    std::vector<SphereData> node_data;
    std::vector<int> parent_indices;
    std::vector<int> left_child_indices;
    std::vector<int> right_sibling_indices;
    
    // Brak modyfikowalnego mCallback. Callback jest przekazywany w czasie wywołania, unikając race condition.
};
```

### Propozycja Rust (Zero-cost safety & Concurrency)

```rust
// Zapewnienie bezpieczeństwa cyklu życia przy pomocy `Borrow Checker` oraz bezproblemowe ujęcie do bibliotek Rayon.
use nalgebra::Point3;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3<f32>,
    pub radius: f32,
}

pub struct SphereTree {
    nodes: Vec<Sphere>,
    children: Vec<Option<Vec<usize>>>, // Zamiast wskaźników używamy silnie typowanych indeksów
}

impl SphereTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), children: Vec::new() }
    }

    /// Iterator bez alokacji w pętli renderu, bezpieczny w wątkach
    pub fn raycast(&self, origin: Point3<f32>, dir: Point3<f32>) -> impl Iterator<Item = usize> + '_ {
        self.nodes.iter().enumerate().filter(move |(_, s)| {
            Self::intersect(s, origin, dir)
        }).map(|(i, _)| i)
    }

    fn intersect(sphere: &Sphere, origin: Point3<f32>, dir: Point3<f32>) -> bool {
        // Obliczenia bezpieczne wobec fragmentacji z użyciem f32 SIMD
        let oc = origin - sphere.center;
        let b = oc.dot(&dir.coords);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - c;
        discriminant > 0.0
    }
}
```

### Podsumowanie Refaktoryzacji
1. **RAII**: W C++ zastąpiono ręczną listę wiązaną przez wektory o stałym podłożu w pamięci. W Rust Borrow Checker eliminuje wycieki na ścieżce błędów. 
2. **Poprawność współbieżności**: Callbacki wywoływane są bez zapisu do globalnej sterty `mCallback` w klasie. Przesunięcie funkcji do stateless invocation zamyka podatność wielowątkową.
3. **Odporność algorytmiczna i Determinism**: Lepsze operacje na strukturach indeksowych bez ryzyka `mFifo1 == dangling pointer` podczas relokacji obiektów. Brak niebezpiecznego rzutowania `void*`.
4. **Wydajność krytyczna**: Zastosowanie SoA (Structure of Arrays) umożliwi wektoryzację (SIMD) przecięć promienia i znacząco zminimalizuje L1/L2 cache misses przy rekursywnych funkcjach `VisibilityTest`.
