# Raport z Przeglądu Jakości: AUDIT-GAME-07 (Guild War State Machine & Warp Registry)

## 1. Podsumowanie architektury zbadanych modułów

Przeanalizowano pliki nagłówkowe `GuildContainerModel.h` oraz `WarpRegistryModel.h` znajdujące się w przestrzeni nazw `UserInterface::Domain`. Obie klasy odpowiadają za utrzymanie krytycznego stanu domenowego gry:
- **`GuildContainerModel`**: Zarządza informacjami o gildii, członkach, stopniach, umiejętnościach oraz trwających wojnach gildyjnych na poziomie danego gracza/klienta. Zastąpił on wcześniej używany `CPythonGuild`, oddzielając stan od UI. Publikuje zdarzenia domenowe (np. `GuildInfoUpdatedEvent`) wykorzystując `Core::EventBus::GetInstance()`.
- **`WarpRegistryModel`**: Wzorzec Singleton (lub global registry), który przechowuje dostępne punkty teleportacji (`WarpDestination`) oferowane przez NPC. Dokonuje walidacji żądań skoku (pod kątem poziomu i złota) z użyciem monad (mechanizm `std::expected` / `.and_then`). Również publikuje zdarzenia w `EventBus`.

W kodzie widoczne są próby modernizacji w stronę standardu C++23, w szczególności użycie `std::expected` (z `.and_then` monadycznym chainowaniem) jako mechanizmu propagacji błędów, mocnego typowania z użyciem EterBase (`EterBase::EntityId`, `PlayerLevel`), `std::optional` oraz `std::format` (przez `std::formatter`).

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

Mimo poprawnego kierunku architektonicznego zidentyfikowano braki w zakresie bezpieczeństwa wątkowego, wydajności operacji oraz cyklu życia.

### 2.1. Brak Bezpieczeństwa Wątkowego (Thread Safety & Synchronization)
- **Problem:** Zarówno `GuildContainerModel`, jak i `WarpRegistryModel` przechowują współdzielony stan domenowy w niezabezpieczonych strukturach danych (`std::unordered_map` czy `std::vector`), do których dostęp może potencjalnie następować z różnych wątków (np. główny wątek aktualizacji logicznej, wątek pobierający ramki sieciowe od serwera). Klasa `WarpRegistryModel` dodatkowo implementuje globalny wzorzec Singleton, co czyni ją wysoce podatną na Data Race, jeśli zdarzenia z sieci asynchronicznie modyfikują listę.
- **Plik i linie:** 
  - `GuildContainerModel.h`: mutacje `m_members`, `m_skills`, `m_activeWars` w metodach `UpsertMember`, `RemoveMember`, `StartGuildWar`.
  - `WarpRegistryModel.h`: globalny singleton z mutacjami `destinations_` w `AddDestination`, `ClearDestinations`.
- **Wpływ:** Ryzyko krytycznych segfaultów, naruszeń spójności stanu gry (np. odczyt `destinations_` równocześnie z jego modyfikacją po nadejściu nowej ramki sieciowej). Wywołania callbacków wewnątrz `EventBus` mogą również być podatne na re-entrancy, jeśli nasłuchiwacze synchronicznie zmieniają stany i emitują nowe eventy.

### 2.2. Zbędne Kopie i Alokacje (Performance Optimization)
- **Problem:** Zwrot list `std::vector<WarpDestination>` poprzez wartość w instancjach `WarpResult`.
- **Plik i linia:** `WarpRegistryModel.h`, metoda `GetDestinations(EterBase::EntityId npcId)`:
  ```cpp
  WarpResult<std::vector<WarpDestination>> GetDestinations(EterBase::EntityId npcId) const
  ```
- **Wpływ:** Przekazanie kopii wektora elementów do warstwy walidacji dla każdej operacji lub sprawdzenia GUI prowadzi do licznych, nadmiarowych dynamicznych alokacji na ścieżce krytycznej aktualizacji ekranu (renderowania listy docelowej). Powinien być zwracany `std::span` z odniesieniem (`std::span<const WarpDestination>`).

### 2.3. O(N) wyszukiwanie stringów na gorącej ścieżce
- **Problem:** Wyszukiwanie członka gildii po nazwie znakowej, w obrębie wszystkich iteracji hash-mapy.
- **Plik i linia:** `GuildContainerModel.h`, metoda `GetMemberByName`:
  ```cpp
  for (const auto& [id, member] : m_members) { if (member.name == name) return member; }
  ```
- **Wpływ:** Degradacja cache'u, długi czas iteracji w przypadku dużych gildii. Należy wziąć pod uwagę użycie zoptymalizowanego dwukierunkowego mapowania (np. dodatkowy lookup-map po nazwie z wykorzystaniem pointerów lub transparent hash).

### 2.4. Brak Weryfikacji Rozmiaru Ramki / Granic (Protocol Stream Robustness / Safety)
- **Zauważone:** W strukturach brak jawnego użycia widoków chronionych przed bufor overread. Ponadto `std::string gradeName`, `std::string mapName` może sugerować brak alokacji "in-place", a zła deserializacja surowych bajtów stringów bez limitu do tego modelu grozi problemami. Należy zwrócić uwagę na `std::string` w modelach pamięci, preferując małe, zafiksowane w buforze `std::array<char, MAX_LEN>`.

## 3. Wzorcowa propozycja refaktoryzacji do C++23 / Rust

### Dla Rust (Idealna Architektura)

W języku Rust problem Thread-Safety oraz Lifecycle Management znika przy poprawnym użyciu RwLock z ARC. Globalne wzorce Mutability (jak w WarpRegistry) należy wyeliminować na rzecz jawnego przekazywania referencji do stanu świata (`&mut Context`).

```rust
// Moduł warp_registry.rs
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct WarpRegistry {
    // Współdzielony, ale bezpieczny wątkowo (Read-Write Lock) dostęp
    destinations: Arc<RwLock<HashMap<EntityId, Vec<WarpDestination>>>>,
}

impl WarpRegistry {
    pub fn new() -> Self {
        Self { destinations: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn get_destinations(&self, npc_id: EntityId) -> Option<Vec<WarpDestination>> {
        // Clone is cheap for Arc or we return references/slices based on lifetime
        let lock = self.destinations.read().unwrap();
        lock.get(&npc_id).cloned() // LUB zwracanie slice'ów przy założeniu cyklu życia
    }
}
```

### Dla C++23 (Propozycja wdrożenia natychmiastowego)

Wprowadzenie rygorystycznego zarządzania `std::span` w celu optymalizacji wydajności, ochrona dostępu przy pomocy `std::shared_mutex` lub całkowite oddzielenie event-loopa w bezwątkowy model typu Actor (kolejkowanie mutacji z asynchronicznych wątków IO poprzez RingBuffer).

```cpp
#include <shared_mutex>
#include <span>

class WarpRegistryModel {
public:
    // Zastąpienie kopiującego typu zwrotnego niealokującym, bezpiecznym spanem.
    WarpResult<std::span<const WarpDestination>> GetDestinations(EterBase::EntityId npcId) const {
        std::shared_lock lock(rw_mutex_);
        if (auto it = destinations_.find(npcId); it != destinations_.end()) {
            return std::span<const WarpDestination>{it->second.begin(), it->second.end()};
        }
        return std::unexpected(WarpRegistryError::NpcNotFound);
    }

    WarpResult<void> AddDestination(EterBase::EntityId npcId, WarpDestination destination) {
        std::unique_lock lock(rw_mutex_);
        destinations_[npcId].push_back(std::move(destination));
        // Publikacja eventu PO zwolnieniu zamka zapobiega dead-lockom / re-entrancy
        lock.unlock(); 
        Core::EventBus::GetInstance().Publish(WarpRegistryUpdatedEvent{npcId});
        return {};
    }

private:
    mutable std::shared_mutex rw_mutex_;
    std::unordered_map<EterBase::EntityId, std::vector<WarpDestination>> destinations_;
};
```
Zastosowanie powyższych modyfikacji rozwiązuje krytyczne problemy ze spójnością pamięci i wyciekami na ścieżce błędów oraz wielokrotnie optymalizuje zużycie pamięci (eliminacja L1/L2 Cache miss przy odpytywaniu o destynacje).
