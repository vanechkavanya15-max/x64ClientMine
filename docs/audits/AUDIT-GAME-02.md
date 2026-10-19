# Clean Code Review: AUDIT-GAME-02 — SkillCastValidator & Cooldowns

## Podsumowanie architektury zbadanych modułów
Zbadano katalog `src/Client/Skills/` odpowiedzialny za mechanikę umiejętności w grze. Architektura bazuje na wzorcach sprawdzania warunków przed rzuceniem umiejętności (np. `SkillRequirementEvaluator`, zbiór klas dziedziczących z `ISkillRequirementRule`), przetwarzaniu formuł, ładowaniu metadanych umiejętności (`SkillTableLoader`, `SkillTableRegistry`) i obsłudze czasu trwania / buforów (`SkillBuffManager`). Moduły używają nowoczesnych elementów (C++23), takich jak `std::expected`, `std::optional`, `std::string_view` i `std::span`. W systemie efektów wizualnych (pipeline) zarządza się aktualizacjami z czasem trwania.

Mimo wykorzystywania nowoczesnych struktur C++23 (np. `std::expected`), pewne moduły cierpią z powodu problemów w zarządzaniu wskaźnikami właścicielskimi (brak wyraźnego RAII z surowymi wskaźnikami gdzie indziej, jeśli występują), bezpieczeństwa wątków, czy potencjalnych problemów parsera (niebezpieczne rzutowania, nieoptymalne działanie pamięci podręcznej i buforowanie).

## Zestawienie zidentyfikowanych defektów kodu i niespójności

1. **Zarządzanie zasobami (RAII & Memory Lifetime) / Alokacje Dynamiczne**
   - **Plik:** `src/Client/Skills/SkillRequirementEvaluator.cpp`, w funkcji `CreateStandardEvaluator` i `AddRule`.
   - **Mechanizm:** Reguły (rules) alokowane są na stercie i trzymane w kontenerze za pomocą `std::unique_ptr<ISkillRequirementRule>`. Prowadzi to do segmentacji pamięci (cache miss) i zbędnych wywołań sterty podczas dodawania podstawowych reguł. Przy częstym używaniu walidacji (co klatkę lub bardzo często dla postaci), dereferencja wirtualnych wskaźników przez wskaźniki na stosie (`unique_ptr`) znacznie degraduje wydajność (brak cache-friendly).
   - **Wpływ na stabilność / wydajność:** Spadki wydajności na ścieżce krytycznej (brak optymalizacji cache).
   - **Poprawa:** Zastąpienie alokacji dynamicznych w `SkillRequirementEvaluator` użyciem `std::variant<RuleA, RuleB...>` lub polimorfizmem na stosie (np. bufor wewnątrz klasy, jeśli mało podklas) oraz iteracją bez opóźnień cache (ang. cache-friendly approach).

2. **Bezpieczeństwo współbieżności i synchronizacja wątków (Thread Safety & Synchronization)**
   - **Plik:** `src/Client/Skills/SkillBuffManager.h` / `SkillBuffManager.cpp`
   - **Mechanizm:** Mapa aktywnych buforów (`std::unordered_map<Client::Core::SkillId, std::chrono::milliseconds> m_activeBuffs`) jest modyfikowana (funkcja `Update`, `AddBuff`, `RemoveBuff`, `ClearAll`) bez mechanizmów synchronizacji.
   - **Wpływ na stabilność:** Jeżeli watek sieciowy wrzuca `AddBuff`, a pętla główna (wątek tick) aktualizuje w tym samym czasie iterację w `Update`, nastąpi Data Race / ubijające proces wywołanie uboczne.
   - **Poprawa:** Użycie mutexów (np. `std::mutex`, lub lock-free structures), by zapewnić bezpieczeństwo podczas dostępu wielowątkowego. Opcjonalnie użycie podejścia "Actor Model" lub "Command Queue", gdzie modyfikacje są kolejkowane do wątku aktualizującego.

3. **Odporność parsera na fragmentację danych (Protocol Stream Robustness)**
   - **Plik:** `src/Client/Skills/SkillTableLoader.cpp`
   - **Mechanizm:** Funkcja `SplitString(tsvData, '\n')` robi wiele kopii tymczasowych dla std::string_view, lub alokuje duże bloki `std::vector<std::string_view>`. W parsowaniu atrybutów wykorzystane rzutowanie z typów standardowych do zdefiniowanych struktur jest oparte o funkcję parsującą bez kontroli przed "out-of-bounds" na `tsvData`. Istnieje zagrożenie niejawnych błędów jeśli wejdą znaki formatujące bez odpowiednich parzystości w kolumnach pliku konfiguracyjnego.
   - **Wpływ na stabilność:** Parser wywala się lub generuje bzdury jeśli strumień z VFS jest okrojony w połowie bajtu, lub znak nowej linii `\n` znajduje się w połowie komórki z cudzysłowami (np. parsowanie string_view z `"`).
   - **Poprawa:** Rekonstrukcja `LoadFromMemory` oparta o bezstanowe/bezpieczne iterowanie za pomocą zaawansowanego parsera (np. zero-copy z walidacją poprawności bajt po bajcie) i sprawdzanie błędów wielkości (zapobieganie truncating przy `ParseInt`).

4. **Wydajność na ścieżce krytycznej (Performance Optimization)**
   - **Plik:** `src/Client/Skills/SkillVisualPipeline.cpp`
   - **Mechanizm:** W `Update()` zastosowano `std::erase_if(activeEffects_, ...)`. Użyto `std::vector`, co jest dobre, jednak podczas każdej klatki wywoływana jest funkcja `StopEffect()`, dla efektów wygasających.
   - **Wpływ na stabilność:** Poprawne logicznie, jednak `std::erase_if` w wektorze powoduje przesunięcia bloków pamięci. Jeśli wiele efektów wygasa w różnym czasie (szczególnie w zatłoczonym miejscu - miasto), mamy masowe wywoływanie kopiowania/przesunięcia dużej ilości elementów struktury `SkillVisualEffect`.
   - **Poprawa:** Jeśli kolejność efektów nie jest istotna (a nie jest), zastosować paradygmat `Swap and Pop` z elementem ostatnim do usuwania efektów i iterację ręczną od końca wektora, zamiast `std::erase_if` aby optymalizować wykorzystanie pamięci wektora.

## Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Rust (Wzorzec Data-Oriented & Traits):
```rust
// Zastąpienie std::unique_ptr polimorficznym enumem (eliminacja pointer-chasing, cache friendly)
pub enum SkillRequirementRule {
    SpHp(SpHpRequirementRule),
    Level(LevelRequirementRule),
    Cooldown(CooldownRequirementRule),
    // ...
}

impl SkillRequirementRule {
    pub fn evaluate(&self, ctx: &SkillEvaluationContext) -> Result<(), SkillEvaluationError> {
        match self {
            Self::SpHp(r) => r.evaluate(ctx),
            Self::Level(r) => r.evaluate(ctx),
            Self::Cooldown(r) => r.evaluate(ctx),
        }
    }
}

pub struct SkillRequirementEvaluator {
    rules: Vec<SkillRequirementRule>, // Zero dynamicznych alokacji per rule (vec tylko jako tablica enumow)
}
```

### C++23 (Wzorzec `std::variant` lub Data-Oriented z SIMD i Lock-free)
```cpp
// Użycie std::variant do trzymania reguł zamiast wirtualnych wskaźników
using RuleVariant = std::variant<
    SpHpRequirementRule,
    LevelRequirementRule,
    CooldownRequirementRule,
    WeaponRequirementRule,
    TargetRequirementRule,
    StateRestrictionRule
>;

class SkillRequirementEvaluator {
    std::vector<RuleVariant> rules_; // Zwykły wektor trzymajacy wszystko na jednej ciagłej płaszczyźnie cache L1
public:
    void AddRule(RuleVariant rule) {
        rules_.push_back(std::move(rule));
    }

    EvaluationResult<void> CanCast(const SkillEvaluationContext& ctx) const {
        for (const auto& rule : rules_) {
            auto result = std::visit([&ctx](const auto& r) { return r.Evaluate(ctx); }, rule);
            if (!result.has_value()) {
                return result;
            }
        }
        return {};
    }
};
```
### Rozwiązanie Thread-Safety: Buff Manager:
Dołożenie Command-Pattern dla thread safety lub standardowego r/w mutexa:
```cpp
class SkillBuffManager {
    mutable std::shared_mutex m_mutex;
    // ... 
    void Update(std::chrono::milliseconds deltaTime) {
        std::unique_lock lock(m_mutex);
        // swap&pop remove
    }
}
```
