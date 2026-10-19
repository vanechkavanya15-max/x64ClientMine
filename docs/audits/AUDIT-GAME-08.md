# Raport z Przeglądu Kodu: AUDIT-GAME-08

## 1. Podsumowanie Architektury
Zbadane moduły `PlayerStatsService` oraz `SkillService` (wraz z fasadami i serwisami pomocniczymi) w `src/UserInterface/Services` implementują logikę zarządzania statystykami gracza (punkty HP, SP, EXP, złoto) oraz umiejętnościami w oparciu o architekturę opartą o zdarzenia (EventBus) i fasady przekazujące wywołania do starszych podsystemów (PythonPlayer, StranglerFacade). 
Architektura wykazuje chęć modularności, jednak obecne implementacje zdradzają liczne braki z zakresu bezpieczeństwa typów, bezpieczeństwa wątków (Thread Safety) oraz odporności na nieoczekiwane dane sieciowe.

## 2. Zestawienie Defektów i Niespójności

| Plik | Linia / Funkcja | Kategoria | Opis i Wpływ na Stabilność |
|------|-----------------|-----------|----------------------------|
| `PlayerStatsService.h` | 26: `std::array<int64_t, 255> m_points` | Cykl życia & Pamięć | Stały rozmiar 255. Brak bounds-checkingu przed zapisem w niektórych miejscach może skutkować przepełnieniem bufora (Buffer Overflow). Choć w cpp jest `if (type >= m_points.size())`, użycie std::span lub weryfikowanych widoków pamięci zwiększyłoby bezpieczeństwo. |
| `PlayerStats_SetPoint.cpp` | 13: `SetPoint` | Odporność (Typy) | Rzutowania wartości (np. `m_view.hp = static_cast<uint32_t>(value);` dla wejścia `int64_t`) są niebezpieczne (truncation i signed-to-unsigned conversion). Jeśli serwer przyśle negatywną wartość jako ujemny int64_t, po rzutowaniu na uint32_t otrzymamy kolosalną wartość (underflow/wraparound), co zniszczy statystyki w grze (np. miliony HP / ghost state). |
| `PlayerStats_SetPoint.cpp` / `PlayerStatsService.h` | - | Współbieżność | Brak mechanizmów synchronizacji (mutex, atomic). Dostęp do metody SetPoint z wątku sieci, a GetPoint z wątku rysującego lub pętli logiki doprowadzi do Race Conditions, potencjalnie czytając połowicznie uaktualnione dane, co niszczy determinizm stanu. |
| `SkillService.h` | 51: `UpdateCooltimes` | Wydajność | Funkcja wywoływana co klatkę iteruje po `std::unordered_map<EterBase::SkillId, PlayerSkillView>`. Wskaźniki/Node'y mapy nie są cache-friendly. Taka struktura dla kilkudziesięciu umiejętności to niepotrzebne spowolnienie. Lepsza byłaby płaska tablica (`std::vector` lub `std::flat_map`). |
| `SkillService.h` | - | Współbieżność | Tak samo jak w przypadku statystyk – brak synchronizacji dostępu przy mapie modyfikowanej dynamicznie przez wątki uaktualniające (UpdateCooltimes, SetSkillLevel) a czytanej przez np. rendering (GetSkill) to proszenie się o Data Races i Crash aplikacji. |

## 3. Wzorcowa Propozycja Refaktoryzacji

Aby sprostać wymaganiom nowoczesnych systemów wymagana jest weryfikacja granic przy operacjach, cache-friendly data structures i odpowiednia synchronizacja (lub lock-free).

### Rust (Odporność na typy i bezpieczna współbieżność)
```rust
use std::sync::RwLock;
use std::collections::HashMap; // Lub lepiej flat_map/vec_map
use std::cmp::max;

#[derive(Default, Clone)]
pub struct PlayerPointsView {
    pub hp: u32,
    // ...
}

pub struct PlayerStatsService {
    points: RwLock<[i64; 255]>,
    view: RwLock<PlayerPointsView>,
}

impl PlayerStatsService {
    pub fn set_point(&self, point_type: usize, value: i64) {
        if point_type >= 255 {
            return; // lub Error
        }
        
        let mut pts = self.points.write().unwrap();
        pts[point_type] = value;

        let mut v = self.view.write().unwrap();
        // Bezpieczne clampowanie, zapobiega zjawisku truncation / wrap-around przy ujemnych lub za duzych
        match point_type {
            POINT_HP => v.hp = value.try_into().unwrap_or(0),
            POINT_MAX_HP => v.max_hp = value.try_into().unwrap_or(0),
            // ...
            _ => {}
        }
    }
}
```

### C++23 (Cache friendly design i memory safety)
```cpp
#include <shared_mutex>
#include <vector>
#include <algorithm>
#include <expected>
#include <print>

namespace UserInterface::Services
{
    class SkillService final : public ISkillService {
    public:
        void UpdateCooltimes(float deltaTime) override {
            std::unique_lock lock(m_mutex);
            // Iterujemy po plaskiej tablicy / wektorze zoptymalizowanym pod kątem SIMD / Cache
            for (auto& skill : m_skills) {
                if (skill.cooltimeRemaining > 0.0f) {
                    skill.cooltimeRemaining = std::max(0.0f, skill.cooltimeRemaining - deltaTime);
                }
            }
        }
        
        void SetSkillCooltime(EterBase::SkillId id, float duration) override {
            std::unique_lock lock(m_mutex);
            auto it = std::ranges::find_if(m_skills, [id](const auto& s){ return s.skillId == id; });
            if (it != m_skills.end()) {
                it->cooltimeRemaining = duration;
            }
        }
        
    private:
        mutable std::shared_mutex m_mutex;
        // Płaska struktura (flat_map / std::vector) dla lepszej wydajności w tight loopie
        std::vector<PlayerSkillView> m_skills;
    };
}
```
Zastosowanie powyższych technik likwiduje potencjalne wycieki i dead-locki zapewniając w 100% deterministyczny stan biznesowy w wątkach pobocznych.
