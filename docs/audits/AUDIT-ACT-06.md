# Raport z Przeglądu Jakości Kodu: AUDIT-ACT-06 — FlyingInstance Trajectory

## Podsumowanie Architektury

Zbadane moduły (`FlyingInstance`, `FlyingObjectManager`, `FlyTarget`, `FlyingData`, `FlyTrace`) pełnią rolę systemu odpowiedzialnego za zarządzanie obiektami latającymi w grze (pociski, efekty, itd.).
Główne elementy systemu:
- `CFlyingManager`: Globalny Singleton zarządzający cyklem życia instancji (lista `CFlyingInstance` i mapa zdefiniowanych `CFlyingData`).
- `CFlyingInstance`: Klasa reprezentująca konkretny, aktywny obiekt w świecie gry. Posiada logikę poruszania, nakierowywania (`FlyTarget`) i kolizji. 
- `CFlyTarget` i `IFlyTargetableObject`: Komponenty obsługujące śledzenie celu lub pozycji z wykorzystaniem struktury `std::set` zawierającej surowe wskaźniki do targeterów.
- `CFlyTrace`: Komponent odpowiedzialny za renderowanie śladu poruszającego się obiektu, zarządzany poprzez customowy `CDynamicPool`.
- `CFlyingData`: Konfiguracja zachowania i efektów powiązana z typem pocisku.

Obecny kod jest napisany w klasycznym C++ i wykazuje wiele antywzorców z punktu widzenia nowoczesnego C++ (C++23) oraz paradygmatów bezpieczeństwa i współbieżności, takich jak te egzekwowane w języku Rust.

---

## Zestawienie Zidentyfikowanych Defektów (C++23 / Rust)

### 1. Zarządzanie zasobami i cykl życia (RAII & Memory Lifetime)
- **Defekt:** Użycie surowych wskaźników do zarządzania cyklem życia i własnością, manualne usuwanie obiektów. Brak `std::unique_ptr`.
  - **Plik i linia:** `src/GameLib/FlyingObjectManager.cpp`, linie ~25-30:
    ```cpp
    void CFlyingManager::__DestroyFlyingInstanceList()
    {
        TFlyingInstanceList::iterator i;
        for(i = m_kLst_pkFlyInst.begin(); i!=m_kLst_pkFlyInst.end(); ++i)
        {
            CFlyingInstance* pkFlyInst=*i;
            CFlyingInstance::Delete(pkFlyInst);		
        }
    ...
    ```
  - **Opis:** System używa makr lub statycznych metod typu `CFlyingInstance::Delete()` oraz customowych alokatorów połączonych z surowymi wskaźnikami, np. `CFlyTrace::New()` zarządzane przez `ms_kPool.Alloc()`. Taki model wymaga ręcznego kontrolowania pamięci i grozi wyciekami przy każdym wczesnym wyjściu z bloku (np. przez rzucenie wyjątku).
  - **Wpływ:** Memory leaks, Use-After-Free. W środowisku asynchronicznym takie błędy prowadzą bezpośrednio do crashów aplikacji.

- **Defekt:** Nietrwałe, niewspółdzielone zależności (Dangling Pointers)
  - **Plik i linia:** `src/GameLib/FlyingInstance.h`, pola takie jak `IFlyEventHandler * m_pHandler`, `IActorInstance * m_pOwner`, `CFlyingData * m_pData`.
  - **Opis:** Klasa trzyma surowe wskaźniki na obiekty (Właściciel, Handler), co nie daje gwarancji, że podmiot obserwowany/właściciel wciąż istnieje, gdy obiekt pocisku działa.
  - **Wpływ:** Crash programu w przypadku zniszczenia `IActorInstance` przed zakończeniem lotu pocisku. Zamiast tego powinno używać się `std::weak_ptr` (lub powiązań Entity-ID).

### 2. Poprawność współbieżności i synchronizacja wątków (Thread Safety)
- **Defekt:** Mutacja współdzielonego stanu globalnego bez locków
  - **Plik i linia:** `src/GameLib/FlyingObjectManager.h`, klasa jest oznaczona jako `CSingleton<CFlyingManager>`, posiadająca wewnętrzne mapy i listy.
  - **Opis:** Metody takie jak `Update()`, `Render()`, oraz dodawanie nowych pocisków mogą być teoretycznie wywoływane z różnych wątków (np. network thread odbierający pakiet o pocisku vs. main/render thread iterujący po kolekcji). Brak w kodzie jakiejkolwiek sekcji krytycznej, `std::mutex`, `std::shared_mutex` czy rozwiązań lock-free.
  - **Wpływ:** Data race, uszkodzenie kolekcji `m_kLst_pkFlyInst`, undefined behavior (UB).

- **Defekt:** Wzajemne usuwanie wskaźników w listenerze (Re-entrancy hazard)
  - **Plik i linia:** `src/GameLib/FlyTarget.cpp` oraz `src/GameLib/FlyTarget.h`
    ```cpp
    inline void IFlyTargetableObject::ClearFlyTargeter() {
        std::set<CFlyTarget*>::iterator it;
        for(it = m_FlyTargeterSet.begin();it!=m_FlyTargeterSet.end();++it) {
            (*it)->NotifyTargetClear();
        }
        m_FlyTargeterSet.clear();
    }
    ```
  - **Opis:** Jeśli wywołanie `NotifyTargetClear()` spowodowałoby, że wewnątrz metody usunięty lub zmodyfikowany zostanie wskaźnik do `IFlyTargetableObject`, nastąpi błędny odczyt iteratorów kolekcji, prowadząc do crasha lub deadlocka (gdyby istniały locki).
  - **Wpływ:** Silne powiązanie (Tight Coupling) uniemożliwiające bezpieczną współbieżność.

### 3. Wydajność na ścieżce krytycznej (Performance)
- **Defekt:** Użycie `std::list` oraz `std::map` w systemie aktualizowanym co klatkę.
  - **Plik i linia:** `src/GameLib/FlyingObjectManager.h` (`TFlyingInstanceList`, `TFlyingDataMap`).
  - **Opis:** Kolekcje oparte na listach wskaźnikowych mocno obniżają cache-locality i uniemożliwiają (bądź spowalniają) wektoryzację (SIMD) dla logiki iteracji lotu. W kodzie pojawia się jedynie częściowe użycie np. `MathSIMD::Store3`, ale przez złą reprezentację (SoA zamiast AoS lub chociaż ciągłego w pamięci wektora), zysk jest znikomy.
  - **Wpływ:** Słaba wydajność, wysoki koszt alokacji / dealokacji na stercie oraz ciągły cache miss w `CFlyingManager::Update()`.

---

## Wzorcowa Propozycja Refaktoryzacji do C++23 / Rust

### Paradygmat: Data-Oriented Design (DOD) i Inteligentne Wskaźniki
Aby uzyskać 100% bezpieczeństwo wielowątkowe (odpowiednik gwarancji Rust'a), należy zrezygnować z OOP i `Singletonu` wskaźnikowego na rzecz modelu podobnego do **Entity Component System (ECS)**.

**1. Transformacja w C++23:**
1. Zastąpienie starych kontenerów ciągłą pamięcią (`std::vector`).
2. Reprezentacja celu używając jednoznacznych uchwytów (Entity ID) zamiast surowych wskaźników (`m_pOwner` itp.).
3. Separacja Update'u (logic) od Render'u.

```cpp
#include <vector>
#include <memory>
#include <optional>
#include <span>

// C++23: Użycie indeksów zamiast wskaźników dla uniknięcia dangling pointers
using EntityID = uint32_t;

struct FlyingInstanceData {
    DirectX::XMFLOAT3 position;
    DirectX::XMFLOAT3 velocity;
    DirectX::XMFLOAT3 acceleration;
    EntityID owner;
    EntityID target;
    uint32_t flyingDataIndex; 
    bool active;
};

class FlyingSystem {
private:
    std::vector<FlyingInstanceData> instances;
    // ...
public:
    // Update z użyciem std::span dla bezpiecznego dostępu do bufora
    void Update(std::span<FlyingInstanceData> activeInstances, float deltaTime) {
        // Kod podatny na auto-wektoryzację i SIMD
        for (auto& inst : activeInstances) {
            if (!inst.active) continue;
            // Obliczenia używając MathSIMD...
        }
    }
};
```

**2. Transformacja w Rust:**
W Ruście problem współdzielonego, mutowalnego stanu (`std::set<CFlyTarget*>`) zostaje odrzucony na etapie kompilacji przez Borrow Checker.

```rust
use std::sync::{Arc, RwLock};
use glam::Vec3;

// Stan pocisku bez zależności OOP
#[derive(Clone, Copy)]
pub struct FlyingInstance {
    pub position: Vec3,
    pub velocity: Vec3,
    pub owner_id: Option<u32>,
    pub target_id: Option<u32>,
    pub is_alive: bool,
}

// Globalny stan chroniony muteksem (np. parking_lot::RwLock) lub lepiej przekazywany 
// jako parametr przez system (jak w Bevy ECS)
pub struct FlyingManager {
    instances: Vec<FlyingInstance>,
}

impl FlyingManager {
    // Brak referencji własnościowych pomiędzy Ownerem a Instancją.
    // Iteracja tylko po jednym wektorze zachowuje bezpieczeństwo cache i nie blokuje programu.
    pub fn update(&mut self, dt: f32) {
        for instance in &mut self.instances {
            if !instance.is_alive { continue; }
            instance.position += instance.velocity * dt;
            // ...
        }
    }
}
```

### 3. Zmiana Polityki Synchronizacji Sieciowej (Protocol Stream Robustness)
Przeniesienie pakietów `CreateIndexedFly` z wątku sieciowego powinno odbywać się za pomocą kolejek *Lock-Free* (np. `std::atomic` + Circular Buffer lub `crossbeam-channel` w Rust), by eliminować wyścig pomiędzy modyfikowaniem kolekcji (spawnem po stronie serwera/klienta), a Render Loop'em na głównym wątku.
