# Raport z Audytu: AUDIT-GAME-04 — Player State & Stamina

## 1. Podsumowanie architektury zbadanych modułów

Przeanalizowano ścieżkę `src/Client/Player/`, skupiającą się na stanie gracza (ekwipunek, statystyki, zarządzanie eliksirami, pozycjonowanie aktora, itp.). 

Główne założenia architektury:
- Szerokie użycie standardów wprowadzanych w C++23, m.in. `std::expected` (oraz jego customowych wrapperów jak `Client::Core::ExpectedResult` / `Client::Core::Result`).
- Modularna konstrukcja: separacja logiki domenowej (Domain Contracts) od logiki kontrolera.
- Całkowity brak zależności od interfejsów systemowych w analizowanych plikach kontrolerów - duży plus za testowalność.

Jednakże audyt wykazał pewne potencjalne luki w mechanizmach współbieżności, spójności stanów, odporności sieciowej, a także okazje do optymalizacji.

## 2. Zestawienie zidentyfikowanych defektów i niespójności

### Defekt 1: Wyciek / Utrata Spójności Stanu w Pozycjonowaniu (Memory / Logic Determinism)
- **Plik:** `src/Client/Player/PlayerMainActorPositionTracker.cpp`
- **Linie:** 55-66 (`PushNetworkWaypoint`)
- **Opis mechanizmu:** Rozmiar kolejki `m_waypoints` jest ograniczany przez zmienną `m_config.maxQueueSize`, ale przy przekroczeniu zwracany jest po prostu `std::unexpected(TrackerError::QueueOverflow)`. Odrzucanie pakietów pozycyjnych może prowadzić do desynchronizacji klienta i serwera (tzw. "rubberbanding" / ghosting). Dodatkowo użycie `std::deque` wywołuje dynamiczną alokację per dodany punkt trasy. 
- **Wpływ na stabilność:** Wysoki, spadek FPS (mikroprzycięcia przez alokacje), utrata determinizmu pozycji względem serwera.

### Defekt 2: Możliwy Data Race oraz Brak Validacji Typów (Concurrency / Protocol Stream Robustness)
- **Plik:** `src/Client/Player/PlayerAutoPotionController.cpp`
- **Linie:** 32-47, 51-65 (`Update`)
- **Opis mechanizmu:** Metoda pobiera stany z domeny (HP/SP), na ich podstawie generuje pakiety i wysyła sieciowo bez gwarancji synchronizacji ze zmianą stanu ekwipunku. Rzutowania `static_cast<uint32_t>` i operacje wysyłkowe na surowym buforze. W przypadku wywołania tego z wątku głównego (render), a jednoczesnej aktualizacji inwentarza z wątku sieci, dochodzi do wyścigu danych (Data Race).
- **Wpływ na stabilność:** Średni, błędy odczytu statystyk prowadzące do natłoku zapytań wysyłających poty, dławienie kolejki sieciowej. Brak atomowości stanu między zużyciem itemu a odjęciem go z ekwipunku w pamięci.

### Defekt 3: Złożoność czasowa mapowań (Performance)
- **Plik:** `src/Client/Player/PlayerEquipmentController.h/cpp`
- **Linie:** Cała struktura z użyciem `std::unordered_map<EquipmentSlotType, Client::Core::ItemVnum>`
- **Opis mechanizmu:** Użycie `std::unordered_map` dla małego zbioru, ograniczonego, jak zdefiniowany enum `EquipmentSlotType` z kilkoma elementami to strzał w kolano w kontekście Cache Miss'ów i zbytecznej złożoności obcinającej pętlę klatki.
- **Wpływ na stabilność:** Niski, lecz mierzalny wpływ na ścieżkę krytyczną. Każde pobranie statystyk z założonych itemów pożera więcej zasobów niż zwykła tabela.

## 3. Wzorcowa propozycja refaktoryzacji do C++23 / Rust

### C++23: Eliminacja alokacji w Trackerze za pomocą std::array / Ring Buffer
Użycie `std::deque` powoduje fragmentację. Lepszym rozwiązaniem jest bufor kołowy z prealokowaną zawartością, bezpieczny współbieżnie, jeśli obsługiwany lock-free miedzy siecia a renderem (np. MPMC lock-free queue, lub zwykły muteks jeśli rzadko aktualizowany).

```cpp
#include <array>
#include <span>

template <typename T, size_t N>
class RingBuffer {
    std::array<T, N> buffer_;
    size_t head_ = 0;
    size_t tail_ = 0;
    size_t count_ = 0;
public:
    constexpr bool Push(const T& val) noexcept {
        if (count_ >= N) return false;
        buffer_[tail_] = val;
        tail_ = (tail_ + 1) % N;
        count_++;
        return true;
    }
    // O(1) Pop, Peek...
};
```

### C++23: Eliminacja unordered_map w EquipmentController
Użyj płaskiej `std::array`, gdzie indeks odpowiada enumowi. Umożliwi to całkowite pozbycie się dynamicznych alokacji.

```cpp
class PlayerEquipmentController {
private:
    static constexpr size_t SLOT_COUNT = static_cast<size_t>(EquipmentSlotType::Jewelry) + 1;
    std::array<std::optional<Client::Core::ItemVnum>, SLOT_COUNT> equippedItems_;
    
public:
    constexpr std::expected<void, EquipmentError> EquipItem(EquipmentSlotType slot, Client::Core::ItemVnum vnum) noexcept {
        auto& item = equippedItems_[static_cast<size_t>(slot)];
        if (item) return std::unexpected(EquipmentError::SlotOccupied);
        item = vnum;
        return {};
    }
};
```

### Rust: Wzorcowy Controller dla Auto Potions (Thread-Safety and RAII)
W języku Rust możemy wymusić bezpieczeństwo współbieżności i typów, używając `Arc` oraz `RwLock`, oraz bezpiecznych wrapperów sieciowych, co zabezpieczy przed data-race opisywanym w Defekcie 2.

```rust
use std::sync::{Arc, RwLock};

pub struct AutoPotionController {
    manager: Arc<RwLock<AutoPotionManager>>,
    domain_contract: Arc<dyn PlayerDomainContract + Send + Sync>,
    network_port: Arc<dyn NetworkPort + Send + Sync>,
    potion_slots: RwLock<[Option<ItemSlot>; AutoPotionType::Count as usize]>,
}

impl AutoPotionController {
    pub fn update(&self, current_time_ms: u64) -> Result<(), AutoPotionControllerError> {
        let slots = self.potion_slots.read().map_err(|_| AutoPotionControllerError::LockPoisoned)?;
        
        // Zapewnienie dostępu chronionego barierą RwLock + Send/Sync traits
        if let Some(slot) = slots[AutoPotionType::Health as usize] {
            let hp = self.domain_contract.get_current_hp();
            let max_hp = self.domain_contract.get_max_hp();
            
            if let Ok(mut manager) = self.manager.write() {
                if manager.process(AutoPotionType::Health, hp, max_hp, current_time_ms)? > 0 {
                    let packet = encode_use_item(slot)?;
                    self.network_port.send_raw(packet[0], &packet[1..])?;
                }
            }
        }
        Ok(())
    }
}
```

Wykorzystując takie rozwiązania nie tylko uniemożliwiamy powstanie klasycznych problemów C++ (wiszace wskaźniki, uciekanie kontekstu wątku, race condition), ale też optymalizujemy kod eliminując zbędne narzuty w ścieżce krytycznej (Critical Path).
