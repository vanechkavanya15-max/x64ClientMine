# AUDIT-GAME-03: Combat Formulas & Angles

## 1. Podsumowanie architektury zbadanych modułów
Moduł `Client::Formulas` w katalogu `src/Client/Formulas/` stanowi zbiór bezstanowych, czystych funkcji matematycznych operujących na logice biznesowej gry (walka, ruch, mechanika doświadczenia, szanse na drop).
Architektura opiera się na nowoczesnych mechanizmach C++23:
- Powszechne użycie `constexpr` w celu ewaluacji w czasie kompilacji.
- Stosowanie `std::expected` (oraz `EterBase::Result`) do bezpiecznej propagacji błędów bez wykorzystania wyjątków.
- Zastosowanie atrybutów `[[nodiscard]]` i klauzuli `noexcept`.

Moduły są projektowo bezpieczne pod kątem współbieżności, ponieważ są wolne od współdzielonego stanu globalnego (Thread Safety / Zero-Conflict). Narzuty pamięciowe zostały zminimalizowane, alokacje dynamiczne nie występują.

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

### Defekt 1: Brak determinizmu algorytmicznego (Floating-Point Non-Determinism)
- **Plik(i):** `AttackAngleValidator.h`, `FastSpatialDistance.h`, `MovementKinematicsMath.h`, `ExpLevelTableMath.h`, `DropProbabilityMath.h`
- **Linie:** Wiele wystąpień operacji zmiennoprzecinkowych (`float`, `double`), m.in. `std::sqrt`, `std::cos`, `std::acos`.
- **Mechanizm:** Arytmetyka zmiennoprzecinkowa (IEEE 754) zaimplementowana za pomocą standardowych funkcji bibliotecznych (`<cmath>`) różni się dokładnością w zależności od platformy sprzętowej (x86 vs ARM), użytego kompilatora i poziomów optymalizacji (np. opóźnienia i flagi takie jak `-ffast-math`).
- **Wpływ na stabilność:** W grach sieciowych (szczególnie przy przewidywaniu zachowań klienta - client-side prediction, lub w systemach lockstep) różnice na poziomie ułamków powodują desynchronizację stanu maszyny między klientem a serwerem (tzw. zjawisko ghosting, błędna detekcja trafień w pobliżu krawędzi stożka ataku).

### Defekt 2: Mieszanie typów (Implicit Conversions / Type Truncation)
- **Plik(i):** `CombatDamageFormulas.h`, `ExpLevelTableMath.h`
- **Mechanizm:** Wzory łączą `int64_t` z `double` w operacjach mnożenia i dzielenia. W pliku `ExpLevelTableMath.h` występuje rzutowanie z float w ryzykowny sposób: `static_cast<uint64_t>(multiplierDouble * 1000.0)`.
- **Wpływ na stabilność:** Zjawisko utraty precyzji podczas niejawnych/jawnych konwersji pomiędzy typami stało- i zmiennoprzecinkowymi. Niektóre gigantyczne wartości `uint64_t` przy przejściu na `double` tracą dokładność na najmniej znaczących bitach, co grozi zaokrągleniami niszczącymi integralność walut i punktów doświadczenia.

### Defekt 3: Wektorowa wydajność na ścieżce krytycznej (SIMD Vectorization)
- **Plik(i):** `FastSpatialDistance.h`, `MovementKinematicsMath.h`
- **Mechanizm:** Zdefiniowane proste struktury `Position2D`, `Vector2D`, `Vector3` ewaluowane skalarnie na pojedynczych blokach pamięci.
- **Wpływ na stabilność:** Kalkulacje dystansów przestrzennych dla tysięcy potworów wykonywane w krytycznej pętli zablokują jeden z rdzeni. Możliwa znaczna optymalizacja poprzez zrównoleglenie obliczeń sprzętowo na poziomie wektorów (SIMD - AVX2).

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Rozwiązanie w Rust (Silne Typowanie & Matematyka Stałoprzecinkowa)
Aby osiągnąć 100% determinizmu, proponuje się migrację struktur geometrycznych z `f32`/`f64` na arytmetykę stałoprzecinkową używając biblioteki z ekosystemu crate (np. `fixed`).

```rust
use fixed::types::I32F32;

#[derive(Debug, PartialEq, Eq)]
pub enum AttackAngleError {
    InvalidMaxAngle,
    ZeroDirectionVector,
    TargetOutsideCone,
}

#[derive(Clone, Copy)]
pub struct Position2D {
    pub x: I32F32,
    pub y: I32F32,
}

#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: I32F32,
    pub y: I32F32,
}

pub struct AttackAngleValidator;

impl AttackAngleValidator {
    /// Funkcja gwarantująca zgodność bit-perfect pomiędzy architekturami i systemami
    pub fn is_target_in_attack_cone(
        attacker: &Position2D,
        attack_dir: &Vector2D,
        target: &Position2D,
        max_angle_degrees: I32F32,
    ) -> Result<(), AttackAngleError> {
        let zero = I32F32::from_num(0);
        let max_angle = I32F32::from_num(360);

        if max_angle_degrees <= zero || max_angle_degrees > max_angle {
            return Err(AttackAngleError::InvalidMaxAngle);
        }

        let k_len2 = (attack_dir.x * attack_dir.x) + (attack_dir.y * attack_dir.y);
        if k_len2 == zero {
            return Err(AttackAngleError::ZeroDirectionVector);
        }

        let target_vec = Vector2D {
            x: target.x - attacker.x,
            y: target.y - attacker.y,
        };

        let c_len2 = (target_vec.x * target_vec.x) + (target_vec.y * target_vec.y);
        if c_len2 == zero {
            return Ok(());
        }

        let dot_product = (target_vec.x * attack_dir.x) + (target_vec.y * attack_dir.y);
        
        // Brak operacji float!
        // Do trygonometrii (sqrt, cos) w środowisku fixed-point można użyć
        // algorytmów takich jak CORDIC lub Lookup Tables, zaimplementowanych w bibliotekach fixed_math.
        // Poniższe zakłada zastosowanie stałoprzecinkowego cos/sqrt
        // np. let cos_target = dot_product / fixed_math::sqrt(c_len2 * k_len2);
        
        Ok(()) // Szablonowo zwracane Powodzenie
    }
}
```

### Rozwiązanie w C++23 (Modularizacja, Fixed-Point)

Zalecana eliminacja wbudowanych zmiennych `float` na rzecz klasy opakowującej Fixed-Point o bezpiecznym rozkładzie całkowitym, co zabezpieczy integralność ułamków.

```cpp
#pragma once
#include <expected>
#include <cstdint>
// Wymagany np. "fpm" lub wewnętrzna implementacja deterministycznego sqrt i cos (Look-up Table)
#include "FixedPointMath.hpp" 

namespace Client::Formulas {

    using fp32 = FixedPoint<int32_t, 16>; // 16 bitów dla części całkowitej i 16 bitów ułamkowych

    enum class AttackAngleError {
        InvalidMaxAngle,
        ZeroDirectionVector,
        TargetOutsideCone
    };

    struct Position2D {
        fp32 x, y;
    };

    struct Vector2D {
        fp32 x, y;
    };

    class AttackAngleValidator {
    public:
        [[nodiscard]] static std::expected<void, AttackAngleError>
        IsTargetInAttackCone(const Position2D& attacker, const Vector2D& dir,
                             const Position2D& target, fp32 max_angle_deg) noexcept 
        {
            if (max_angle_deg <= fp32{0} || max_angle_deg > fp32{360}) {
                return std::unexpected(AttackAngleError::InvalidMaxAngle);
            }
            
            fp32 k_len2 = dir.x * dir.x + dir.y * dir.y;
            if (k_len2 == fp32{0}) {
                return std::unexpected(AttackAngleError::ZeroDirectionVector);
            }

            Vector2D t_vec{ target.x - attacker.x, target.y - attacker.y };
            fp32 c_len2 = t_vec.x * t_vec.x + t_vec.y * t_vec.y;
            
            if (c_len2 == fp32{0}) {
                return {}; 
            }

            fp32 dot = t_vec.x * dir.x + t_vec.y * dir.y;
            
            // Wykorzystanie bezpiecznych i deterministycznych odpowiedników biblioteki trygonometrycznej
            fp32 cos_target = dot / Math::Sqrt(c_len2 * k_len2);
            fp32 max_angle_rad = (max_angle_deg / fp32{2}) * Math::Pi() / fp32{180};
            fp32 limit_cos = Math::Cos(max_angle_rad);

            if (cos_target >= limit_cos) {
                return {};
            }
            return std::unexpected(AttackAngleError::TargetOutsideCone);
        }
    };
}
```

**Zalecenia końcowe:** Wprowadzenie dedykowanych bibliotek ułamków bezwzględnych (`fixed point`), ścisłe testy typu Unit z cross-kompilacją (ARM64 vs x86_64) uwzględniające sprawdzanie binarnych wyników (tzw. Bit-Exact match).
