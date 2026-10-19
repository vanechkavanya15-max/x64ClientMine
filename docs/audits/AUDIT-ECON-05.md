# Raport z przeglądu jakości kodu: AUDIT-ECON-05 — Item Limit & Proto Validation

## 1. Podsumowanie architektury zbadanych modułów

Przegląd objął moduły z katalogów `src/Client/Data/` oraz `src/Client/Mimic/`. Architektura opiera się na parsowaniu i ładowaniu danych o przedmiotach (ItemProto) oraz jednostkach (MobProto) z użyciem wielu formatów: CSV, JSON, oraz skompresowanego binarnego ZSTD. Moduł Mimic odpowiada za interpolację ruchu, rate limiting akcji gracza oraz obsługę pakietów heartbeat (PingPong).

Mimo zastosowania pewnych elementów nowoczesnego C++ (takich jak `std::expected`, `std::span`), kod wciąż przejawia problemy typowe dla "legacy C++", m.in. brak jawnej synchronizacji dla zasobów współdzielonych, nieoptymalne alokacje na stercie (użycie `std::unordered_map` na ścieżkach krytycznych) oraz ryzyko wystąpienia UB (Undefined Behavior) z powodu rzutowania buforów (strict aliasing).

## 2. Zestawienie zidentyfikowanych defektów i niespójności

### 2.1. Odporność parsera na fragmentację i złośliwe dane
*   **Plik:** `src/Client/Data/ProtoBinarySerializerZstd.cpp`
*   **Mechanizm:** Metoda `Deserialize`.
*   **Opis:** Alokacja wektora używa niezaufanej wielkości nagłówka: `std::vector<ItemProtoRecord> decompressedRecords(header.recordCount)`. Brak walidacji górnego limitu rekordu powoduje podatność na atak typu OOM (Out Of Memory). Złośliwy nagłówek z dużą wartością `recordCount` spowoduje rzucenie `std::bad_alloc` i crash procesu. Dodatkowo operacja `header.recordCount * sizeof(ItemProtoRecord)` może wywołać integer overflow, co sprawia, że mechanizm ochronny jest nieszczelny.
*   **Wpływ na stabilność:** Krytyczny (zdalne/lokalne zawieszenie procesu).

*   **Plik:** `src/Client/Data/ProtoCSVParserModern.cpp`
*   **Mechanizm:** Metoda `Parse`.
*   **Opis:** Rzutowanie danych wejściowych z użyciem `std::stoul`/`std::stoull` jest opakowane w wolny blok `try/catch`. Podanie zbyt dużego ciągu tekstowego doprowadzi do rzucenia wyjątku `std::out_of_range`. Generuje to również nadmierne obciążenie narzutami wyjątków C++ podczas wczytywania (Performance & Fragmentacja). Do tego dochodzą nieoptymalne operatory na ciągach znaków.

### 2.2. Zarządzanie zasobami i poprawność pamięci (Strict Aliasing & OOB)
*   **Plik:** `src/Client/Data/ItemManagerModernLoader.cpp`
*   **Mechanizm:** Metoda `LoadFromModernBlob`.
*   **Opis:** Surowe rzutowanie wskaźnika typu `uint8_t*` na `CItemData::TItemTable*`: `reinterpret_cast<const CItemData::TItemTable*>(decompressedBuffer.data())`. Taka konstrukcja prowadzi do naruszenia zasady Strict Aliasing oraz potencjalnych błędów unaligned memory access na niektórych architekturach (np. ARM), jeśli wyrównanie bufora decompressedBuffer jest niewystarczające dla typu TItemTable. Zamiast tego powinno użyć się operacji kopiujących lub zapewnić alignas(). Z kolei `ProtoLegacyAdapter.cpp` wymaga walidacji i użycia null terminacji z `std::string_view`.
*   **Wpływ na stabilność:** Średni (Niezdefiniowane zachowanie / UB).

### 2.3. Współbieżność i Data Races (Thread Safety)
*   **Plik:** `src/Client/Mimic/ActionRateLimiter.cpp`
*   **Mechanizm:** Metoda `TryAction`.
*   **Opis:** Klasa korzysta ze struktury `std::unordered_map<ActionType, TokenBucket> m_buckets`. Funkcja `TryAction` (często wywoływana z wątków wejścia/wyjścia sieciowego i pętli gry) modyfikuje zawartość pól (`bucket.tokens--`, `bucket.lastRefillTime += ...`). Brak użycia synchronizacji (np. `std::mutex`, `std::atomic` lub bezblokowych algorytmów) będzie skutkował wyścigami (data races), zgubieniem eventów a ostatecznie segfault.
*   **Wpływ na stabilność:** Wysoki. Podobny problem występuje w `PingPongEngine` podczas asynchronicznego dostępu do statusu serwera z pętli gry.

### 2.4. Wydajność na ścieżce krytycznej (Performance Optimization)
*   **Plik:** `src/Client/Mimic/ActionRateLimiter.cpp` & `ActionRateLimiter.h`
*   **Opis:** Użycie `std::unordered_map` z silnie ograniczoną i zwartą pulą kluczy z enum (5 wartości `ActionType`). Dołożenie narzutu na hash-owanie dla każdej akcji wywołuje deoptymalizację w rejestrach i spadki wydajności w L1 cache CPU. Użycie płaskiej tablicy `std::array` indeksowanej bezpośrednio po enumie usunęłoby ten problem.
*   **Plik:** `src/Client/Data/ProtoJSONParserModern.cpp`
*   **Opis:** Metoda `ExportItemTableToJSON` alokuje stringi i wektory zamiast strumieniowania we/wy, co powoduje zbędne alokacje na stercie i utratę wektoryzacji SIMD.

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### C++23 (Refaktoryzacja ActionRateLimiter dla poprawności współbieżnej i wydajności):

```cpp
#include <array>
#include <atomic>
#include <chrono>
#include <expected>
#include <algorithm>

namespace Client::Mimic {

enum class ActionType : uint8_t {
    Attack = 0, Move, Pickup, UsePotion, CastSkill, Count
};

class ActionRateLimiter {
public:
    void Configure(ActionType type, uint32_t maxTokens, std::chrono::milliseconds refillRate) {
        auto& bucket = m_buckets[static_cast<std::size_t>(type)];
        bucket.maxTokens.store(maxTokens, std::memory_order_relaxed);
        bucket.tokens.store(maxTokens, std::memory_order_relaxed);
        bucket.refillRateMs.store(refillRate.count(), std::memory_order_relaxed);
        bucket.lastRefillTime.store(
            std::chrono::steady_clock::now().time_since_epoch().count(),
            std::memory_order_relaxed
        );
    }

    // Lock-free TryAction z CAS dla bezwzględnej poprawności
    [[nodiscard]] std::expected<void, Core::CommandError> TryAction(
        ActionType type, 
        std::chrono::steady_clock::time_point now = std::chrono::steady_clock::now()) 
    {
        auto& bucket = m_buckets[static_cast<std::size_t>(type)];
        
        auto currentTokens = bucket.tokens.load(std::memory_order_acquire);
        auto maxT = bucket.maxTokens.load(std::memory_order_relaxed);
        auto rateMs = bucket.refillRateMs.load(std::memory_order_relaxed);
        
        if (rateMs > 0) {
            auto lastTime = bucket.lastRefillTime.load(std::memory_order_acquire);
            auto timePassedMs = std::chrono::duration_cast<std::chrono::milliseconds>(
                now.time_since_epoch()).count() - lastTime;
                
            if (timePassedMs >= rateMs) {
                uint32_t tokensToAdd = timePassedMs / rateMs;
                uint32_t newTokens = std::min(maxT, currentTokens + tokensToAdd);
                
                bucket.tokens.store(newTokens, std::memory_order_release);
                bucket.lastRefillTime.store(
                    lastTime + (tokensToAdd * rateMs),
                    std::memory_order_release
                );
                currentTokens = newTokens;
            }
        }

        while (currentTokens > 0) {
            if (bucket.tokens.compare_exchange_weak(currentTokens, currentTokens - 1,
                                                    std::memory_order_release,
                                                    std::memory_order_acquire)) {
                return {};
            }
        }
        
        return std::unexpected(Core::CommandError::RateLimited);
    }

private:
    struct TokenBucket {
        std::atomic<uint32_t> tokens{0};
        std::atomic<uint32_t> maxTokens{0};
        std::atomic<int64_t> refillRateMs{0};
        std::atomic<int64_t> lastRefillTime{0};
    };

    std::array<TokenBucket, static_cast<std::size_t>(ActionType::Count)> m_buckets;
};

} // namespace Client::Mimic
```

### Propozycja idiomatyczna Rust (Dla pełnego bezpieczeństwa pamięci, synchronizacji i determinizmu):

```rust
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
pub enum ActionType {
    Attack = 0,
    Move = 1,
    Pickup = 2,
    UsePotion = 3,
    CastSkill = 4,
}

const ACTION_COUNT: usize = 5;

struct TokenBucket {
    tokens: AtomicU32,
    max_tokens: AtomicU32,
    refill_rate_ms: AtomicI64,
    last_refill_time: AtomicI64, // użycie offsetów czasowych względem startu aplikacji
}

pub struct ActionRateLimiter {
    buckets: [TokenBucket; ACTION_COUNT],
    start_time: Instant,
}

impl ActionRateLimiter {
    pub fn new() -> Self {
        Self {
            // Bez-alokacyjna struktura na stosie lub bezpieczna struktura sterty
            buckets: std::array::from_fn(|_| TokenBucket {
                tokens: AtomicU32::new(0),
                max_tokens: AtomicU32::new(0),
                refill_rate_ms: AtomicI64::new(0),
                last_refill_time: AtomicI64::new(0),
            }),
            start_time: Instant::now(),
        }
    }

    pub fn configure(&self, action: ActionType, max_tokens: u32, refill_rate: Duration) {
        let bucket = &self.buckets[action as usize];
        bucket.max_tokens.store(max_tokens, Ordering::Relaxed);
        bucket.tokens.store(max_tokens, Ordering::Relaxed);
        bucket.refill_rate_ms.store(refill_rate.as_millis() as i64, Ordering::Relaxed);
        bucket.last_refill_time.store(self.start_time.elapsed().as_millis() as i64, Ordering::Relaxed);
    }

    pub fn try_action(&self, action: ActionType) -> Result<(), &'static str> {
        let bucket = &self.buckets[action as usize];
        
        let mut current_tokens = bucket.tokens.load(Ordering::Acquire);
        let max_t = bucket.max_tokens.load(Ordering::Relaxed);
        let rate_ms = bucket.refill_rate_ms.load(Ordering::Relaxed);
        
        if rate_ms > 0 {
            let last_time = bucket.last_refill_time.load(Ordering::Acquire);
            let now_ms = self.start_time.elapsed().as_millis() as i64;
            let time_passed = now_ms - last_time;
            
            if time_passed >= rate_ms {
                let tokens_to_add = (time_passed / rate_ms) as u32;
                let new_tokens = std::cmp::min(max_t, current_tokens + tokens_to_add);
                
                bucket.tokens.store(new_tokens, Ordering::Release);
                bucket.last_refill_time.store(last_time + ((tokens_to_add as i64) * rate_ms), Ordering::Release);
                current_tokens = new_tokens;
            }
        }
        
        loop {
            if current_tokens == 0 {
                return Err("RateLimited");
            }
            
            match bucket.tokens.compare_exchange_weak(
                current_tokens,
                current_tokens - 1,
                Ordering::Release,
                Ordering::Acquire
            ) {
                Ok(_) => return Ok(()),
                Err(val) => current_tokens = val,
            }
        }
    }
}
```

### Konkluzje Refaktoryzacyjne:
1. Należy niezwłocznie wyeliminować użycie wyjątków C++ dla konwersji ciągów znaków na liczby na rzecz `std::from_chars`, który wykonuje się z deterministycznym narzutem czasowym i nie angażuje mechanizmów unwindingu stosu.
2. Usunięcie `std::unordered_map` eliminuje ryzyko fragmentacji sterty (Performance Optimization) i nieprzewidywalnego narzutu podczas re-hashowania, co zapewnia w 100% płaskie struktury (Data-Oriented Design).
3. Podmienienie tradycyjnych inkrementacji na `compare_exchange_weak` (CAS) zapewnia, że środowisko będzie deterministyczne, bezpieczne dla synchronizacji wątków (Thread Safety) bez korzystania z blokujących muteksów (lock-free programming).
4. `std::span` i operacje na blokach Pamięci Ograniczonej muszą posiadać twarde assercje przed zrzutowaniem ich pod obiekty strukturalne.
