# Audit Report: AUDIT-GAME-05 (CombatState & Potion System)

## 1. Architektura i cel modułów (Architecture Summary)
Badane moduły stanowią krytyczne komponenty logiki biznesowej:
- **`CombatState`** (plik: `src/UserInterface/Domain/CombatState.h`): Zarządza stanem walki aktorów, m.in. śledzi okna nietykalności, aktualny stan animacji i historię trafień dla zapobiegania wielokrotnym obrażeniom z tego samego ataku w danej klatce. Zawiera logikę parsowania sieciowych struktur zdarzeń trafienia (`AttackHitPacket`).
- **`PotionInventoryModel`** (plik: `src/UserInterface/Domain/PotionInventoryModel.h`): Model reprezentujący część ekwipunku przeznaczoną na mikstury/eliksiry. Wykorzystuje nowożytne paradygmaty C++ (m m.in. `std::expected`, `std::optional`), emitując jednocześnie zdarzenia przez magistralę (EventBus) celem poinformowania UI o aktualizacjach bez silnego sprzęgania.

## 2. Zestawienie zidentyfikowanych defektów i niespójności

### 2.1 CombatState.h

| Problem | Ścieżka i linia | Kategoria | Mechanizm i wpływ |
|---------|-----------------|-----------|------------------|
| **Zbyt silne pakowanie struktur sieciowych (`#pragma pack(push, 1)`)** | `CombatState.h:17-23` | Protokół/Wydajność | Struktura `AttackHitPacket` używa `pack(1)`. Chociaż poprawne z punktu widzenia zgodności, odczyt poszczególnych pól (np. `targetId`, `hitType`, `skillId`) może powodować unaligned memory access (niezrównane odczyty pamięci), co na pewnych architekturach (ARM) potrafi skutkować przerwaniami sprzętowymi, a na x86 obniżeniem wydajności. |
| **Brak walidacji przy castingu z bufora sieciowego (`reinterpret_cast`)** | `CombatState.h:145-146` | Odporność parsera / Protokół | Parsowanie rzutuje po prostu wskaźnik na surowy bufor (`reinterpret_cast<const AttackHitPacket*>(payload.data())`). Jeśli struktura sieciowa nie jest rygorystycznie zdefiniowana pod kątem kolejności bitów (endianness), a aplikacja obsługuje cross-platform, rzutowanie spowoduje ciche błędy logiczne i wygeneruje niespójny stan logiki walki. Również problem związany z Strict Aliasing Rule w C++. |
| **Koszty alokacji przy śledzeniu nietykalności (Performance Penalty)** | `CombatState.h:159` | Wydajność / Allocations | Użycie zagnieżdżonego `std::unordered_map<uint32_t, std::unordered_map<uint32_t, TimePoint>>` dla `hitRecords_`. Przy każdym nowym ataku lub celu następuje alokacja na stercie wewnątrz ramy czasowej update'u logiki. Może to prowadzić do sporych skoków w użyciu procesora i dużej fragmentacji przy bitwach z wieloma celami. |
| **Pruning historii (O(N) iteration in prune)** | `CombatState.h:87-106` | Wydajność | Funkcja `PruneExpiredHits` iteruje liniowo po mapach na stercie, wykonując częste deallokacje, co w `std::unordered_map` jest relatywnie wolne (brak cache-friendly wektoryzacji, wiele skoków w pamięci). |

### 2.2 PotionInventoryModel.h

| Problem | Ścieżka i linia | Kategoria | Mechanizm i wpływ |
|---------|-----------------|-----------|------------------|
| **Kopiowanie wartości przed sprawdzeniem `Find`** | `PotionInventoryModel.h:122-133` | Wydajność (Drobne) | Użycie `.and_then` transformuje `data` na stercie (wartości typów opcjonalnych), co choć elastyczne, w przypadku dużych wektorów ekwipunku może stanowić narzut w pętli renderowania klatki. Można to zrealizować prościej (wektoryzacja SIMD po typach). |
| **Ryzyko wyścigu na EventBus** | `PotionInventoryModel.h:74` | Thread Safety / Re-entrancy | Emisja na `EventBus::GetInstance().Publish(...)` ze zmienionego już modelu (tzw. "dirty state"). Jeżeli `EventBus` w odpowiedzi wywoła z powrotem odczyt modelu (`GetPotion`) lub wręcz modyfikację, wystąpi re-entrancy, co przy braku blokad może prowadzić do modyfikacji współdzielonej struktury podczas iteracji i rzucenia wyjątku. Model powinien uważać na side-effects publishowania. |
| **Użycie wektora std::optional** | `PotionInventoryModel.h:138` | Cache Locality | `std::vector<std::optional<PotionSlotData>> slots_`. Lepszym rozwiązaniem pod kątem "cache locality" byłaby osobna struktura bitsetu obecności + surowy wektor wartości (tzw. Structure of Arrays). `std::optional` powiększa każdy element (zazwyczaj o padding w strukturze, np. z 8 bajtów do 12-16 bajtów), psując gęstość danych. |

## 3. Wzorcowa propozycja refaktoryzacji (C++23)

### Ograniczenie zagnieżdżonych hashmap w CombatState
Zamiast `std::unordered_map<K, std::unordered_map<K, V>>` należy użyć pojedynczej, płaskiej kolekcji z połączonym kluczem (np. `std::pair<uint32_t, uint32_t>` jako klucz dla np. FlatMap lub vectora uporządkowanego/przydzielonego z góry z puli (Memory Pool) pod dany update frame).
Alternatywnie, wektor płaski + `std::remove_if` i wektoryzacja SIMD do usuwania wygasłych hitów byłaby wielokrotnie szybsza niż wielokrotne usuwanie węzłów z map.

```cpp
// C++23 - Zamiast podwójnego unordered_map (propozycja z Flat-Array / FlatMap cache)
struct HitRecord {
    uint32_t attackId;
    uint32_t targetId;
    TimePoint expiration;
};

// Zastępuje std::unordered_map<...>. 
// Znacznie lepsze użycie cache'u procesora. Iteracja w PruneExpiredHits staje się banalna.
std::vector<HitRecord> hitRecords_;

// Refaktoryzacja Prune:
void PruneExpiredHits(TimePoint currentTime) {
    std::erase_if(hitRecords_, [currentTime](const HitRecord& rec) {
        return currentTime >= rec.expiration;
    });
}
```

### Eliminacja Strict Aliasing i Unaligned Access
Parsowanie pamięci z wykorzystaniem `std::memcpy` lub nowożytnego rzutowania bezpiecznego z uwzględnieniem endianness:
```cpp
// Bezpieczne parsowanie w C++20/C++23
bool ProcessCombatPayload(std::span<const uint8_t> payload) {
    if (payload.size() < sizeof(AttackHitPacket)) return false;

    AttackHitPacket packet;
    // Skopiowanie eliminuje problemy z unaligned memory access:
    std::memcpy(&packet, payload.data(), sizeof(AttackHitPacket));
    
    // Potem ew. konwersja do natywnego endianness, np. std::byteswap dla sieci.
    packet.targetId = std::byteswap(packet.targetId); // jeśli sieć była BigEndian, a używamy np. LE.
    // ...
}
```

### Optymalizacja PotionInventoryModel
Zmiana układu danych na SoA (Structure of Arrays) albo chociaż eliminacja re-entrancy w `EventBus`.

```cpp
// Opóźnione publikowanie zdarzeń w przypadku blokady (defer), by uniknąć wyścigu w pętli eventowej.
std::expected<void, PotionError> SetPotion(EterBase::ItemSlot slot, EterBase::ItemVnum vnum, uint32_t count) {
    // ... walidacja
    slots_[slot.get()] = PotionSlotData{vnum, count};

    // Zabezpieczenie przed Re-entrancy, ew. dodanie flagi `is_updating_`.
    UserInterface::Core::EventBus::GetInstance().Enqueue( // zamiast .Publish (natychmiastowe synchroniczne)
        PotionSlotUpdateEvent(slot, vnum, count)
    );
}
```

## Podsumowanie
Przegląd ujawnił, że kod logicznie jest poprawny i hermetyczny, korzysta z nowoczesnego standardu i ładnie unika śmieci i surowych wskaźników. Główne obszary ulepszeń dotyczą **Memory Lifetime/Performance** w ścieżce trafień (`hitRecords_` zagnieżdżone w unordered_map generujące nadmierne alokacje) oraz **Protocol Stream Robustness** przy obsłudze buforów pakietu przez `reinterpret_cast`. Zastosowanie płaskich kontenerów i standardowego memcpy/bit_cast znacznie obniży ryzyko usterek i podniesie wydajność.

