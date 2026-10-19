# Raport z przeglądu jakości: AUDIT-GAME-06 — LevelUp & Exp Calculator

## Podsumowanie architektury zbadanych modułów

Przegląd objął moduły odpowiedzialne za obliczanie doświadczenia, różnic poziomów, zarządzanie karami oraz parsowanie danych sieciowych, w szczególności:
- **`src/Client/Formulas/ExpLevelTableMath.h`**: Odpowiada za obliczanie wymaganego i zdobytego doświadczenia z wykorzystaniem arytmetyki stałopozycyjnej w celu ominięcia błędów zaokrągleń w C++.
- **`src/UserInterface/Domain/CharacterStateModel.h`**: Domena stanu postaci zawierająca zmienne takie jak HP, MP, i EXP, hermetyzująca stan poprzez zdarzenia w `EventBus`.
- **`src/GameLib/LevelDifferencePenaltyCalculator.h`**: Oblicza modyfikatory nagród i exp w oparciu o różnice w poziomach bytów w grze.
- **`src/UserInterface/Network/Handlers/SkillLevelHandler.cpp`**: Odpowiada za deserializację i parsowanie ramek sieciowych aktualizujących dane o umiejętnościach.
- **`src/GameLib/ExpTableLookup.h`**: Definiuje limity i sztywne wartości tabel doświadczenia dla poziomów w tablicy (maksymalny poziom: 120).

Architektura opiera się o C++23 (m.in. `std::expected`, `std::optional`, `std::span`), a kod stara się realizować koncepcje Clean Architecture i Zero-Conflict, oddzielając stan domenowy od graficznego UI poprzez magistralę zdarzeń (`EventBus`). W większości obszarów stosowane są typy ścisłe (StrongTypes).

## Zestawienie zidentyfikowanych defektów kodu i niespójności

1. **Plik**: `src/Client/Formulas/ExpLevelTableMath.h`
   - **Linia**: `66` w funkcji `CalculateGainedExp`
   - **Opis mechanizmu**: Precyzja ułamków i operacje stałopozycyjne. Występuje przejście typu float (`double multiplierDouble`) na uint64_t w celu stworzenia promili z bezwzględnym rzutowaniem `static_cast<uint64_t>(multiplierDouble * 1000.0)`. Może to prowadzić do drobnych, ale narastających w czasie odchyleń bez precyzyjnego zaokrąglenia, szczególnie w środowisku rozproszonym (brak determinizmu na skali float w różnych architekturach sprzętowych).
   - **Wpływ na stabilność**: Utrata spójności (desync) na ułamkowych częściach promili pomiędzy Serwerem a Klientem przy dużych wartościach EXP, potencjalne różnice na granicy progów wejścia na nowy poziom. Brak ścisłego determinizmu arytmetycznego (naruszenie kryterium: *Determinizm logiki biznesowej i atomowość operacji*).

2. **Plik**: `src/UserInterface/Network/Handlers/SkillLevelHandler.cpp`
   - **Linie**: `18` (w funkcji `HandleSkillLevel`) oraz `34` (w funkcji `HandleSkillLevelNew`)
   - **Opis mechanizmu**: Rzutowanie surowego wskaźnika bajtów prosto z sieci na strukturę (C-style struct cast via `reinterpret_cast<const Packets::SkillLevelPacket*>(buffer.data())`).
   - **Wpływ na stabilność**: Złamanie reguł aliasingu ścisłego (Strict Aliasing Rule), potencjalne błędy wywoływane złym wyrównaniem pamięci (misaligned reads/alignment fault) na niektórych architekturach np. ARM, i zagrożenie odczytu poza dostępnym buforem, jeśli rozmiar obwiedni bufora jest nierówny rozmiarowi struktury docelowej (brak pełnego bezpieczeństwa - naruszenie kryterium: *Odporność parsera na fragmentację danych*).

3. **Plik**: `src/UserInterface/Domain/CharacterStateModel.h`
   - **Linie**: `79-84` (funkcja `SetHp`)
   - **Opis mechanizmu**: Mimo zwracania `EterBase::MakeError(EterBase::PacketError::MalformedPayload)` gdy `currentHp > maxHp`, błąd jest po prostu raportowany poprzez logi i mechanizm expected. Nie zapobiega to w pełni asynchronicznemu wprowadzaniu danych ze względu na to, jak jest używane w wyższych warstwach, co więcej struktura z eventami może propagować stare informacje bez naprawienia rozbieżności. Dodatkowo brak tu blokad (mutex) pomimo wielowątkowości (w sieci lub renderowaniu).
   - **Wpływ na stabilność**: Ryzyko rozbieżności w systemie HP i wyświetlaniu wartości w GUI - widma niespójnego stanu w pamięci (naruszenie kryterium: *Zarządzanie zasobami i poprawność współbieżności*).

4. **Plik**: `src/GameLib/ExpTableLookup.h`
   - **Linia**: `16` i inicjalizacja `EXP_TABLE`
   - **Opis mechanizmu**: Sztywny limit poziomu `MAX_LEVEL = 120`. Odczyty dla rzutów po tej tablicy chroni przed *out of bounds* za pomocą sprawdzenia indeksów, ale logika w `GetTotalExpForLevel` w przypadku rozszerzenia tabeli w przyszłości używa pętli do iteracyjnego sumowania za każdym wywołaniem, zamiast wstępnie sparametryzowanej kalkulacji czy mapowania.
   - **Wpływ na stabilność**: Brak błędów katastrofalnych, ale iteracyjne wyliczanie może wpłynąć na wydajność. Brak wektoryzacji czy cache'owania (naruszenie kryterium: *Wydajność na ścieżce krytycznej*).

## Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### 1. Refaktoryzacja Parserów Sieciowych (Eliminacja surowych rzutowań)

Należy całkowicie zaniechać bezpośredniego castowania `reinterpret_cast` na obiekty na rzecz bezpiecznego parsowania z użyciem mechanizmów np. w C++23 typu `std::memcpy` lub bibliotek obsługujących bezpieczne widoki:

**C++23:**
```cpp
// Użycie std::memcpy chroni przed błędami strict-aliasingu i niewyrównania:
Packets::SkillLevelPacket packet;
if (buffer.size() < sizeof(packet)) {
    return { false, false, false };
}
std::memcpy(&packet, buffer.data(), sizeof(packet));
```

**Rust:**
```rust
// Wbudowany system serde lub bytemuck do bezpiecznej transformacji bajtów:
use bytemuck::{Pod, Zeroable};

#[repr(C, packed)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct SkillLevelPacket {
    header: u16,
    length: u16,
    skill_levels: [u8; 255],
}

pub fn handle_skill_level(buffer: &[u8]) -> Result<SkillUpdateResult, NetworkError> {
    // Zero cost, bounds checked & strict aliasing safe
    let packet: &SkillLevelPacket = bytemuck::try_from_bytes(&buffer[..size_of::<SkillLevelPacket>()])
        .map_err(|_| NetworkError::BufferTooSmall)?;
    // ...
}
```

### 2. Refaktoryzacja Czystej Arytmetyki (Odporność i wydajność)

Aby uniknąć problemów numerycznych z float/double, powinniśmy użyć czystej stałoprzecinkowej arytmetyki z kontrolą przepełnienia dla operacji doświadczenia.

**Rust:**
```rust
// Typy ścisłe w Rust bez operacji float chroniące przed utratą precyzji:
pub fn calculate_gained_exp(base_exp: u64, player_level: u32, mob_level: u32) -> Result<u64, ExpMathError> {
    let multiplier_permille: u64 = calculate_level_difference_permille(player_level, mob_level);
    
    if multiplier_permille == 0 {
        return Ok(0);
    }
    
    // Zapobiegamy overflow używając checked_mul i sprawdzając ułamki reszt
    let q = base_exp / 1000;
    let r = base_exp % 1000;
    
    let part1 = q.checked_mul(multiplier_permille).ok_or(ExpMathError::CalculationOverflow)?;
    let part2 = (r * multiplier_permille) / 1000;
    
    part1.checked_add(part2).ok_or(ExpMathError::CalculationOverflow)
}
```

### 3. Zabezpieczenie domeny stanu za pomocą Mutex/RwLock lub aktorów

Jeżeli instancja domenowa jak `CharacterStateModel` jest współdzielona, jej aktualizacje powinny być atomowe za pomocą `std::atomic` (C++23) lub chronione rygorystycznym pożyczaniem z ryglem.

**Rust (Actor/Mutex):**
```rust
use std::sync::RwLock;

pub struct CharacterStateModel {
    id: EntityId,
    state: RwLock<CharacterState>,
}

impl CharacterStateModel {
    pub fn set_hp(&self, current_hp: u32, max_hp: u32) -> Result<(), EntityError> {
        if current_hp > max_hp {
            return Err(EntityError::MalformedPayload);
        }
        let mut state = self.state.write().unwrap();
        state.hp = current_hp;
        state.max_hp = max_hp;
        state.combat_flags.is_dead = current_hp == 0;
        
        // Emisja eventu
        EventBus::publish(HpChangeEvent { id: self.id, hp: current_hp, max_hp });
        Ok(())
    }
}
```
