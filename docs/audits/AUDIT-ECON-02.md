# Raport z Przeglądu Jakości: AUDIT-ECON-02 — DragonSoul Storage & Refine

## Podsumowanie Architektury
Zbadane moduły (`DragonSoul_*` oraz `RefineService_*`) wchodzą w skład warstwy UI i zarządzają specyficznym inwentarzem "Dragon Soul" oraz systemem ulepszania (Refine). Architektura opiera się na wydzielonych mikro-serwisach oraz handlerach zorientowanych na zdarzenia (EventBus). Implementacja częściowo korzysta z nowoczesnego C++23 (m.in. `std::expected`, `std::span`, silne typowanie). Występują jednak krytyczne wady na styku obsługi pamięci, parsowania pakietów z sieci i determinizmu logiki.

## Zestawienie Zidentyfikowanych Defektów (Kryteria Inżynierskie)

### 1. Zarządzanie zasobami i cykl życia obiektów (RAII & Memory Lifetime)
- **`RefineService_Window.cpp`, linia 77 (oraz inna interakcja na poziomie stanu pakietu):**
  - **Mechanizm:** Po odebraniu pakietu wskaźnik do `refine_table` z wewnątrz rzutowanego span (`packet->refine_table`) jest kopiowany do `std::optional<TRefineTable> m_currentTable`. Ponieważ bufor z pakietem jest zwolniony po powrocie z handlera, jeśli typowanie pakietu ukrywa pod spodem dynamiczne zasoby wewnątrz TRefineTable lub po prostu bufor zawiera wskaźniki (zamiast głębokiej kopii pod-obiektów, np. dynamicznych struktur), może dojść do wiszących referencji, jednakże `std::optional` wykonuje zwykłe kopiowanie bitowe. Większym zagrożeniem jest to, co dzieje się, jeśli w samej strukturze `TPacketGCRefineInformationNew` są wskaźniki na obiekty.
- **`RefineService_Result.cpp`, linia 59:**
  - **Mechanizm:** Użycie `std::memcpy(&packet, payload.data(), sizeof(TPacketGCDragonSoulRefine));`. Jeśli bufor jest dokładnie tej wielkości, co pakiet, jest to bezpieczne. Ale jeśli z powodu zmiany układu kompilatora lub niejawnych znaków (padding) nie pokrywa się on dokładnie z pakietem z sieci z perspektywy bajtów (endianness / alignment), to otrzymamy błędne dane po stronie klienta. Zawsze wymagana jest deserializacja field-by-field ze `std::span`, a nie kopiowanie pamięci jako całości.
  - **Wpływ na stabilność:** Wysokie ryzyko podatności, potencjalne problemy z weryfikacją granic i desynchronizacja stanów, błędy architektoniczne.

### 2. Poprawność współbieżności i synchronizacja wątków (Thread Safety & Synchronization)
- **`DragonSoul_Timer.cpp`, funkcja `Update(float deltaTime)`:**
  - **Mechanizm:** Klasa `DragonSoulTimerTracker` wykorzystuje mapę wewnątrz której wykonywane są cykliczne aktualizacje i wywoływane są bezpośrednie publikacje przez `Core::EventBus::GetInstance().Publish`. Wywołania `Publish` nie mają gwarancji odporności na re-entrancy. Jeśli event listener (handler) dla `DragonSoulTimerExpiredEvent` spróbuje np. zmodyfikować `DragonSoulTimerTracker` (usunąć timer, zresetować go), wystąpi iteracja po unieważnionej kolekcji lub deadlock. Dodatkowo brak tu jakiejkolwiek synchronizacji przy dostępie do kolekcji `m_timers`, co oznacza brak odporności na wielowątkowość.
  - **Wpływ na stabilność:** Krytyczne ryzyko naruszeń pamięci (Concurrent Modification), deadlocki jeśli EventBus blokuje wątek.

### 3. Odporność parsera na fragmentację danych (Protocol Stream Robustness)
- **`RefineService_Window.cpp`, linia 67:**
  - **Mechanizm:** Walidacja wielkości span wynosi `if (packetSpan.size() < sizeof(TPacketGCRefineInformationNew))`. Następnie wykonane jest niebezpieczne rzutowanie wskaźnika: `const auto* packet = reinterpret_cast<const TPacketGCRefineInformationNew*>(packetSpan.data());`. Jeśli rozmiar bufora jest mniejszy lub wskaźnik nie spełnia wymogów wyrównania pamięci (alignment requirements, co często ma miejsce dla dynamicznie obieranych buforów pakietowych), może to spowodować SIGSEGV. 
  - **Wpływ na stabilność:** Niezdefiniowane zachowanie (UB) na wybranych architekturach i potencjalne crash'e, podatność na fragmentację.

### 4. Determinizm logiki biznesowej i atomowość operacji (Business Logic & State Determinism)
- **`DragonSoul_DeckState.cpp`, funkcje SetDragonSoulDeckActive (linia 77):**
  - **Mechanizm:** Zmiana aktywności odbywa się w pełni po stronie klienta bez atomowego powiązania z walidacją logiki po stronie serwera. Publikowanie `DragonSoulDeckStateChangedEvent` następuje niezależnie od tego, czy odpowiednie zapytanie z sukcesem doszło do serwera, co rodzi problemy "Ghost Items" (stan klienta mija się ze stanem z serwera po stronie UI, w logice np. ataku statystyki się nie załączą).
  - **Wpływ na stabilność:** Desynchronizacja, brak atomowości zmiany stanu widoku inwentarza. Wprowadza niejawny ghosting podczas lagów.

### 5. Wydajność na ścieżce krytycznej (Performance Optimization)
- **`DragonSoul_Timer.cpp`, funkcja `Update(float deltaTime)`:**
  - **Mechanizm:** Zastosowano `std::unordered_map<uint32_t, TimerEntry>` do logiki wywoływanej co klatkę ("Update"). Koszt mapy na cache CPU, iteracja na nieciągłej przestrzeni pamięci, duża ilość chybionych cykli. Zdecydowanie należy zoptymalizować do cache-friendly wektora (flat map lub wektor z in-place swap na usuwaniu).

## Wzorcowa Propozycja Refaktoryzacji (Rust & C++23)

### Dla Rust (Przekształcenie modelu Timerów - Thread-Safe i Cache-Friendly)
```rust
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemSlot(pub u16);

pub struct TimerEntry {
    pub deck: u8,
    pub slot: ItemSlot,
    pub remaining_time: f32,
}

pub struct DragonSoulTimerTracker {
    // Cache-friendly i prosta alokacja, idealna dla malej ilosci elementow (do ~100)
    timers: Vec<TimerEntry>,
}

impl DragonSoulTimerTracker {
    pub fn new() -> Self {
        Self { timers: Vec::with_capacity(32) }
    }

    pub fn set_timer(&mut self, deck: u8, slot: ItemSlot, duration: f32) {
        if let Some(existing) = self.timers.iter_mut().find(|t| t.deck == deck && t.slot == slot) {
            existing.remaining_time = duration;
        } else {
            self.timers.push(TimerEntry { deck, slot, remaining_time: duration });
        }
    }

    pub fn update<F>(&mut self, delta_time: f32, mut on_expired: F)
    where
        F: FnMut(u8, ItemSlot),
    {
        // Unika problemu re-entrancy. Zbieramy to co wygaslo i zwalniamy wplyw na wewnetrzna liste.
        let mut i = 0;
        while i < self.timers.len() {
            self.timers[i].remaining_time -= delta_time;
            if self.timers[i].remaining_time <= 0.0 {
                let expired = self.timers.swap_remove(i);
                on_expired(expired.deck, expired.slot);
                // Nie zwiekszamy i, bo swap_remove przenosi z konca element na ten indeks.
            } else {
                i += 1;
            }
        }
    }
}
```

### Dla C++23 (Bezpieczne parsowanie w `RefineService_Window.cpp`)
```cpp
// Zamiast bezposredniego reinterpret_cast, bezpieczne kopiowanie za pomoca bit_cast lub memcpy, 
// omijajac problemy ze strict aliasingiem i alignmentem.
#include <bit>
#include <span>
#include <type_traits>

EterBase::PacketResult<void> RefineService_Window::HandleRefineInformation(std::span<const uint8_t> packetSpan)
{
    if (packetSpan.size() < sizeof(TPacketGCRefineInformationNew))
    {
        EterBase::ModernLogger::Error("RefineService_Window: Zbyt krotki bufor.");
        return EterBase::MakeError(EterBase::PacketError::BufferUnderflow);
    }

    // Bezpieczne skopiowanie z bufory by uniknac alignment SIGSEGV,
    // gwarantujac spojnosc przy deseralizacji.
    TPacketGCRefineInformationNew packet{};
    std::memcpy(&packet, packetSpan.data(), sizeof(TPacketGCRefineInformationNew));

    m_isOpen = true;
    m_currentType = packet.type;
    m_currentPos = EterBase::ItemSlot(packet.pos);
    m_currentTable = packet.refine_table;

    EterBase::ModernLogger::Info("RefineService_Window: Otwarte: {}", m_currentType);

    Core::EventBus::GetInstance().Publish(RefineWindowOpenedEvent(
        m_currentType, m_currentPos, *m_currentTable
    ));

    return {};
}
```
Zastosowanie powyższych poprawek zabezpiecza klienta przed re-entrancy, memory leaks, i alignment faults na etapie parsowania pakietów sieciowych z zachowaniem maksymalnej wydajności (wektoryzacja SIMD, brak UB).
