# Raport Audytu: AUDIT-GAME-01 — Gameplay Action Queue & Handlers

## 1. Podsumowanie architektury zbadanych modułów
Moduł `Client::Gameplay` to domena zarządzająca stanem postaci gracza (Ekwipunek, Paski szybkiego dostępu, Statystyki, Umiejętności) zgodnie ze standardem C++23. Analiza wykazała, że większość modułów stara się realizować założenia *Single Responsibility Principle* oraz bezpiecznego współbieżnego dostępu poprzez `std::shared_mutex` i `std::unique_lock`. Wymuszone jest stosowanie czystych typów standardowych (bez użycia starszych systemów, jak np. Python C API, czy starych makr), choć zaobserwowano kilka znaczących defektów i odchyleń od tych założeń.

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

### Defekt 1: Niebezpieczne parsowanie sieciowe z uzyciem reinterpret_cast
*   **Plik:** `src/Client/Gameplay/QuestCommandHandler.cpp`
*   **Linia:** 31 i 60
*   **Opis mechanizmu:** Użycie `reinterpret_cast` przy tworzeniu `std::span` (`std::span<const uint8_t> payload(reinterpret_cast<const uint8_t*>(&packet), sizeof(packet))`) w celu wysłania surowych danych w pakietach odpowiedzi i potwierdzenia zadania. W nowoczesnym C++ (i w systemach bazujących na Rust) takie podejście rodzi ryzyko problemów z ułożeniem danych w pamięci (padding) oraz potencjalnie nieodpowiednią konwersją znaków.
*   **Wpływ na stabilność:** Może prowadzić do przesyłania uszkodzonych pakietów sieciowych ze względu na nieprawidłowy rozmiar/padding, powodując po stronie serwera odrzucenie ramki lub, co gorsza, podatność na buffer over-read (np. fragmentacja strumienia).

### Defekt 2: Wyciek wlasnosci referencji przy powiadomieniach inwentarza (Callback pod mutexem)
*   **Plik:** `src/Client/Gameplay/InventoryDomain.cpp`
*   **Linia:** 164-182 (`NotifySlotUpdated` wewnątrz `SetItem`, `RemoveItem`)
*   **Opis mechanizmu:** Metoda `NotifySlotUpdated` jest wołana tuż przed zwróceniem wartości w funkcjach modyfikujących ekwipunek, które wywoływane są wewnętrznie (pod unikalną blokadą `std::unique_lock`). Powiadomienia mogą uruchamiać logikę zewnętrznych listenerów, w tym systemu UI (EventBus), podczas gdy domena nadal przetwarza swój stan (w innych metodach na wyższym poziomie stosowane są blokady, co rodzi ryzyko deadlocku/re-entrancy, gdyby callback próbował pobrać stan ekwipunku, podczas gdy mutex jest nadal zablokowany np. na poziomie wywołującego wątku).
*   **Wpływ na stabilność:** Wysokie ryzyko deadlocku lub *re-entrancy* – modyfikacje stanu przez inny wątek wewnątrz callbacka mogą powiesić grę.

### Defekt 3: Złe współużytkowanie statycznego stanu pvpKeys
*   **Plik:** `src/Client/Gameplay/InstanceCombatEngine.cpp`
*   **Linia:** 6-7 i użycie w funkcji
*   **Opis mechanizmu:** Globalny/Statyczny stan (`pvpKeys` chroniony statycznym `pvpKeysMutex`) jest wykorzystywany przez mechanikę walki. Instancjonowany silnik korzysta ze statycznego słownika bez żadnego wyodrębnienia (Scope/Context) dla poszczególnych instancji silnika walki (częsty błąd przy dziedziczonym kodzie).
*   **Wpływ na stabilność:** Brak izolacji instancji silnika – interakcje z wieloma przeciwnikami lub instancjami mogą ulec zjawisku "cross-talk", modyfikując błędne instancje w pamięci co łamie założenia modularyzacji i determinizmu stanu biznesowego.

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Bezpieczne Parsowanie (Rust/C++23)
Aby wyeliminować `reinterpret_cast`, pakiety powinny obsługiwać bezpieczną serializację typu `std::as_bytes` z ograniczeniem dla trywialnie kopiowalnych (Trivially Copyable) struktur:

**C++23 Refactoring:**
```cpp
#include <bit>
// Wymagamy trywialnej kopiowalnosci
static_assert(std::is_trivially_copyable_v<decltype(packet)>);
auto payload = std::as_bytes(std::span{&packet, 1});
auto result = m_networkPort->SendRaw(packet.header, payload);
```

**Rust (Serde / Zerocopy):**
W przypadku implementacji Rusta należy skorzystać z bibliotek takich jak `zerocopy` lub `bytes`, aby w pełni zminimalizować alokację, gwarantując bezpieczne konwersje układu bajtów.

```rust
use zerocopy::{AsBytes, FromBytes};

#[repr(C)]
#[derive(AsBytes, FromBytes, Clone, Copy)]
pub struct QuestAnswerPacket {
    pub header: u8,
    pub answer_idx: u8,
}

// Bezpieczna konwersja ramki
let payload = packet.as_bytes();
let result = network_port.send_raw(packet.header, payload);
```

### Zarządzanie stanem i uniknięcie Deadlocków (Re-entrancy) w Inwentarzu
Rozwiązanie problemu polega na wdrożeniu wzorca *Lock-Free/Deferred Callback*, gdzie zdarzenia domenowe są umieszczane w dedykowanej kolejce wątków i rozgłaszane po zwolnieniu (Drop/Unlock) domeny chronionej przez Mutex. 

**C++23 Refactoring:**
```cpp
void InventoryDomain::FlushEvents() {
    std::vector<InventorySlotUpdatedEvent> pending;
    {
        std::unique_lock lock(m_mutex);
        pending = std::move(m_pendingEvents);
    }
    for (const auto& ev : pending) {
        if (m_slotUpdateCallback) {
            m_slotUpdateCallback(ev);
        }
    }
}
```

**Rust (Actor Model / MPSC Channels):**
```rust
use tokio::sync::mpsc;
use parking_lot::RwLock;

pub struct InventoryDomain {
    state: RwLock<InventoryState>,
    event_tx: mpsc::Sender<InventoryEvent>,
}

impl InventoryDomain {
    pub fn set_item(&self, slot: ItemSlot, item: ItemData) -> Result<(), InventoryError> {
        let mut state = self.state.write();
        state.set_item(slot, item)?;
        
        // Brak ryzyka re-entrancy i blokowania locka, wysyłamy zdarzenie kanałem
        let _ = self.event_tx.try_send(InventoryEvent::SlotUpdated(slot));
        Ok(())
    }
}
```

### Izolacja Instancji Walki (Eliminacja stanu statycznego)
Domeny biznesowe nie mogą operować na stanie statycznym: `std::unordered_set<uint64_t> pvpKeys;` musi być obiektem przypisanym na poziomie określonego bytu np. instancji serwisu bądź menadżera, by nie zakłócać współbieżnego testowania ani wielokrotnych wystąpień (jak instancje lochów). Należy zastosować wstrzykiwanie zależności.
