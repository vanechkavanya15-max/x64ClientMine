# Raport z Przeglądu Kodu: AUDIT-ACT-03 — ActorInstance Event & Sound

## 1. Podsumowanie architektury zbadanych modułów
Moduły `ActorInstanceEvent.cpp` oraz `ActorInstanceMotionEvent.cpp` odpowiadają za obsługę zdarzeń powiązanych z instancjami aktorów (postaci/obiektów) w silniku. Architektura opiera się na monolitycznej klasie `CActorInstance`, która deleguje logikę zdarzeń za pomocą interfejsu `IEventHandler` oraz parsuje zrzutowane z zewnątrz struktury danych zdarzeń ruchu (`CRaceMotionData::TMotionEventData`). System ten wykorzystuje globalne singletony (np. `CEffectManager::Instance()`, `SoundEngine::Instance()`) do obsługi efektów i dźwięku. 

Zidentyfikowane wzorce:
- **Polimorfizm i delegacja:** Użycie `IEventHandler` z implementacją `GetEmptyPtr()` jako wzorca Null Object.
- **Data-Driven Events:** Ruch i animacje generują klatki zdarzeń, które są interpretowane w instrukcjach `switch`.
- **Zależności:** Silne powiązanie z DirectX 9 (`D3DXMATRIX`, `D3DXVECTOR3`) oraz globalnymi menedżerami.

## 2. Zestawienie zidentyfikowanych defektów i niespójności

| Plik | Linia | Mechanizm / Opis | Wpływ na stabilność |
|------|-------|------------------|---------------------|
| `ActorInstanceEvent.cpp` | 102 | **RAII & Memory Lifetime:** Użycie surowych wskaźników własnościowych `IEventHandler* m_pkEventHandler`. Dereferencja opiera się na `assert(m_pkEventHandler!=NULL)`. W kompilacji Release może prowadzić do *Null Pointer Dereference* (Segfault) oraz wycieków pamięci. | Wysoki |
| `ActorInstanceMotionEvent.cpp` | 17, 76, 137 | **Protocol Stream Robustness:** Rzutowanie w dół w stylu C (C-style cast), np. `(const CRaceMotionData::TMotionEffectEventData *)c_pData`. Brak weryfikacji granic struktury i typu (poza weryfikacją nagłówka `iType`) – ryzyko wczytania sfragmentaryzowanej / fałszywej struktury i przepełnienia bufora (Buffer Over-read). | Krytyczny |
| `ActorInstanceMotionEvent.cpp` | 171-173 | **Wydajność / Allocations:** Wywołania `clear()` i `resize()` na wektorach w strukturze `m_kSplashArea` (np. `SphereInstanceVector`). Przydzielanie i zwalnianie pamięci sterty (Dynamic Allocation) w pętli obsługi klatki animacji to wąskie gardło optymalizacyjne (ang. *allocation in hot path*). | Średni |
| `ActorInstanceMotionEvent.cpp` | 215, 222 | **Thread Safety & Synchronizacja:** Modyfikacja globalnego stanu przez singletons `CEffectManager::Instance()` i `CFlyingManager::Instance()` na ścieżce animacji. W przypadku rozdzielenia silnika na wątek symulacji i renderera, spowoduje to *Data Race*. | Wysoki |
| `ActorInstanceMotionEvent.cpp` | 227-229 | **RAII & Wiszące referencje:** Tworzenie instancji w locie `CFlyingInstance * pInstance = rfm.CreateFlyingInstanceFlyTarget(...)` i przekazywanie wskaźnika `this` (`pInstance->SetOwner(this)`). Brak śledzenia cyklu życia `CActorInstance` przez lot – ryzyko wiszącego wskaźnika (Dangling Pointer), jeśli aktor zginie / zniknie przed zakończeniem lotu efektu. | Wysoki |

## 3. Propozycja refaktoryzacji (C++23 / Rust)

Obecny kod jest przestarzały i wymaga modernizacji pod kątem bezpieczeństwa pamięci, zrównoleglenia i wydajności.

### Koncepcja C++23: Smart Pointers, `std::span` i `std::variant`
1. **Eliminacja C-style casts:** Zastąpienie wskaźników do klasy bazowej w unii zdarzeń użyciem `std::variant`, z bezpiecznym dispatchingiem opartym o `std::visit`. Pozwoli to na bezkosztowy i w pełni bezpieczny odczyt właściwego typu.
2. **Memory Lifetime:** Wdrożenie `std::unique_ptr<IEventHandler>` zamiast surowego wskaźnika, lub użycie kompozycji (brak wskaźnika) z polimorfizmem na poziomie interfejsów (wzorzec Strategy oparty o `std::function` lub obiekty wywoływalne). Przekazywanie właściciela (`SetOwner`) z użyciem `std::weak_ptr<CActorInstance>`.
3. **Data-Oriented Design:** Użycie małych wektorów brzegowych, np. `boost::container::small_vector` lub struktury na stosie w przypadku `m_kSplashArea.SphereInstanceVector`, by wyeliminować alokacje na stercie w trakcie trwania pętli klatki. Zamiast `std::string` do nazw kości (np. `strAttachingBoneName`), użycie zahaszowanych wartości (typu `uint32_t` jako ID kości) lub `std::string_view`.

```cpp
// C++23 - Przykład eliminacji C-Style casts i poprawy bezpieczeństwa z std::variant
using TMotionEvent = std::variant<
    std::monostate,
    TMotionEffectEventData,
    TMotionEffectToTargetEventData,
    TMotionAttackingEventData,
    TMotionSoundEventData,
    TMotionFlyEventData
>;

void CActorInstance::ProcessMotionEvent(const TMotionEvent& event) {
    std::visit(std::overloaded {
        [](std::monostate) {},
        [this](const TMotionEffectEventData& data) { this->HandleEffectEvent(data); },
        [this](const TMotionSoundEventData& data) { this->HandleSoundEvent(data); },
        // ...
    }, event);
}
```

### Koncepcja Rust: Ownership & Enums
1. **Bezpieczeństwo współbieżności i pamięci:** Wymuszenie rygorów kompilatora `rustc`. Brak *dangling pointers* przy wywołaniu `SetOwner` (wymagałoby użycia `Rc<RefCell<Actor>>` w architekturze jednowątkowej lub `Arc<Mutex<Actor>>` w wielowątkowej, a docelowo architektury ECS).
2. **Enums z danymi (Tagged Unions):** Definicja zdarzeń w Rust to natywne i bezpieczne mechanizmy typu `Enum`.
3. **Eliminacja globalnych singletonów:** Wstrzykiwanie zależności (Dependency Injection). Silnik wywoływałby funkcje aktualizacji, podając referencje `&mut EffectManager`.

```rust
// Rust - Pattern Matching na bezpiecznym typie Enum
pub enum MotionEvent {
    Effect(MotionEffectEventData),
    EffectToTarget(MotionEffectToTargetEventData),
    Sound(MotionSoundEventData),
    Fly(MotionFlyEventData),
}

impl ActorInstance {
    pub fn process_motion_event(
        &mut self,
        event: &MotionEvent,
        effect_manager: &mut EffectManager,
        sound_engine: &mut SoundEngine
    ) {
        match event {
            MotionEvent::Effect(data) => self.handle_effect(data, effect_manager),
            MotionEvent::Sound(data) => self.handle_sound(data, sound_engine),
            // kompilator upewni się, że wszystkie przypadki są obsłużone
            _ => {} 
        }
    }
}
```

### Podsumowanie działań inżynierskich:
* Refaktoryzacja do DOD (Data-Oriented Design) pozwoli zredukować ilość małych alokacji w `m_kSplashArea`.
* Odejście od globalnego stanu (`Singletonów`) umożliwi wielowątkowy rendering.
* Przejście na `std::variant` i/lub ECS uodporni aplikację na zniekształcone pakiety lub ramki sieciowe i wyeliminuje niebezpieczne rzutowanie z surowych wskaźników typu bajtowego.
