# AUDIT-SYS-04: Core Domain Event Structs

## 1. Podsumowanie architektury zbadanych modulow
Zbadane moduly w katalogu `src/Client/Events/` stanowia kregoslup komunikacji opartej na zdarzeniach (Event-Driven Architecture) w kliencie gry. Zdarzenia emituja istotne informacje domenowe dla mechanik takich jak walka (`CombatEvents`), postacie (`CharacterEvents`), przedmioty (`ItemEvents`), UI oraz siec. 
Widoczne jest zastosowanie nowoczesnych mechanizmow C++23 (np. `std::expected` do obslugi bledow, czesciowe wykorzystanie `constexpr`), co jest dobrym punktem wyjscia. Architektura napotyka jednak na powazne problemy ze wspolbieznoscia, bezpieczenstwem w wielowatkowym srodowisku (Network Thread vs Main/UI Thread) oraz wydajnoscia (zbedne alokacje na sciezce krytycznej).

## 2. Zestawienie zidentyfikowanych defektow kodu

### A. Brak synchronizacji i bezpieczenstwa watkowego (Thread Safety)
- **Plik:** `src/Client/Events/EventStateFlagStorage.h` / `.cpp`
  **Opis mechanizmu:** Klasa uzywa `std::unordered_map<std::string, int> m_flags` do zarzadzania stanem flag. Metody `SetFlag`, `GetFlag`, `ClearFlag` nie sa w zaden sposob chronione przed wspolbieznym dostepem (np. brakuje `std::shared_mutex` lub innej formy synchronizacji).
  **Wplyw na stabilnosc:** Otrzymanie pakietu z sieci (Network Thread) i jednoczesny odczyt/modyfikacja w petli renderowania (UI/Main Thread) doprowadzi do uszkodzenia pamieci (Data Race) i krytycznego zrzutu gry (Crash / UB).
- **Plik:** `src/Client/Events/EventAnswerChoiceManager.cpp`
  **Opis mechanizmu:** Mutacje wektora `m_choices` i stanu `m_activeEventIndex` nie sa bezpieczne watkowo. Brak jakiejkolwiek kontroli dostepu przy uzyciu w srodowisku rozproszonym.

### B. Maska ukrywajaca bledy stanu deterministycznego i brak atomowosci
- **Plik:** `src/Client/Events/EventStateFlagStorage.cpp` (linie 5-7)
  **Opis mechanizmu:** Metoda `SetFlag` z pustym kluczem wykonuje `return;` (ciche zignorowanie bledu zamiast rzucenia bledu/expected). 
  **Wplyw na stabilnosc:** Maskuje to potencjalne bledy w logice gry (tworzenie stanow widmo "ghost states"). Brak bledu dla wywolujacego powoduje ciezko wykrywalne usterki niespojnosci maszyny stanow.
- **Plik:** `src/Client/Events/EventAnswerChoiceManager.cpp` (linia 7)
  **Opis mechanizmu:** `SetActiveEvent` bezwarunkowo nadpisuje istniejacy `m_activeEventIndex` i czysci wektor odpowiedzi, co niszczy trwajacy event bez walidacji czy nie powoduje to zawieszenia dialogu klienta (martwy stan UI).

### C. Alokacje dynamiczne na sciezce krytycznej (Wydajnosc i Odpornosc)
- **Plik:** `src/Client/Events/EventScriptParser.cpp` (linia 35 i 38, wnetrze parsera)
  **Opis mechanizmu:** Parser operuje tworzac dynamicznie stringi w petli while, doklejajac znaki litera po literze: `current_token += script[i];`. Brak kontroli maksymalnego rozmiaru oraz wczesnych powrotow.
  **Wplyw na stabilnosc:** Powoduje to ogromny narzut alokatora, fragmentacje pamieci (Low 1% FPS drops) oraz bezposrednia luke bezpieczenstwa (bombe alokacyjna przy dlugich payloadach z sieci bez spacji i nowej linii, np. wyczerpanie pamieci OOM / Out of Memory).
- **Plik:** `src/Client/Events/EventAnswerChoiceManager.cpp` (linia 46)
  **Opis mechanizmu:** Metoda `GetChoices()` zwraca `std::vector<AnswerChoice>` przez wartosc, wykonujac kosztowna gleboka kopie stosunkowo dynamicznych kolekcji.
  **Wplyw na stabilnosc:** Powinno zwracac `std::span<const AnswerChoice>`, aby uniknac alokacji podczas ciaglego sprawdzania stanow interfejsu (render thread / game loop).

### D. Semantyka Domenowa (Magic Numbers & Lifetime)
- **Plik:** `src/Client/Events/ActorEvents.h`
  **Opis mechanizmu:** Pole `targetId` w `ActorEmotionStartedEvent` wykorzystuje wartosc `0` jako flage braku celu (z komentarzem `// 0 if no target`).
  **Wplyw na stabilnosc:** To przestarzaly antywzorzec. C++23 / Rust definiuje do tego semantyke optionala: uzycie std::optional<Core::EntityId>, zapobiegajac niekontrolowanemu, niejawemu kastowaniu ID zera do instancji nieistniejacego aktora.

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### C++23: Refaktoryzacja EventScriptParser (Zero-Copy & Safety bounds)
Zamiast alokowac dynamiczne obiekty `std::string` na kazdym kroku parsujacym, wykorzystujemy lekki widok pamieci `std::string_view` (Zero-Copy) dla tokenow oraz `std::span` dla buforow, co podnosi wydajnosc silnika oraz gwarantuje bezpieczenstwo.

```cpp
#include <string_view>
#include <vector>
#include <expected>

namespace Client::Events {

struct ScriptArgumentView {
    std::string_view value;
};

struct ScriptCommandView {
    std::string_view name;
    std::vector<ScriptArgumentView> arguments;
};

class EventScriptParser {
public:
    // Zwracamy widoki z oryginalnego bufora wiadomosci - brak alokacji stringow
    static std::expected<std::vector<ScriptCommandView>, ParserError> Parse(std::string_view script) noexcept;
};
}
```

### Rust: Bezpieczna domena EventStateFlagStorage (Thread-Safe Synchronization)
Wzorzec dla jezyka Rust zapobiegajacy Data Race z wykorzystaniem gwarantowanych sprawdzianow Borrow Checkera, chronionej mutowalnosci (RwLock) i bezblednej obslugi bledow za pomoca dedykowanego wyniku `Result`.

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EventStateError {
    #[error("Flag not found in the storage state")]
    NotFound,
    #[error("Invalid, empty flag key provided")]
    InvalidKey,
}

pub struct EventStateFlagStorage {
    // Odporny na wyscigi, wielowatkowy Shared Data Access (Readers-Writer Lock)
    flags: Arc<RwLock<HashMap<String, i32>>>,
}

impl EventStateFlagStorage {
    pub fn new() -> Self {
        Self {
            flags: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Atomowa modyfikacja flag ze zwracaniem wlasciwego bledu
    pub fn set_flag(&self, key: &str, value: i32) -> Result<(), EventStateError> {
        if key.is_empty() {
            return Err(EventStateError::InvalidKey);
        }
        let mut map = self.flags.write().expect("RwLock poisoned!");
        map.insert(key.to_string(), value);
        Ok(())
    }

    // W bezpieczny sposob czyta kooperacyjnie w trybie Read-Only
    pub fn get_flag(&self, key: &str) -> Result<i32, EventStateError> {
        if key.is_empty() {
            return Err(EventStateError::InvalidKey);
        }
        let map = self.flags.read().expect("RwLock poisoned!");
        map.get(key).copied().ok_or(EventStateError::NotFound)
    }
}
```
Zastosowanie tych wzorcow w calym module `Client::Events` wprowadza poprawnosc w obszarze cyklu zycia, wielowatkowosci i braku wyciekow zasobow.
