# Raport z Przeglądu Kodu: AUDIT-SYS-11 — 3D Audio Listener & Stream Buffer

## Podsumowanie Architektury
Moduł `AudioLib` odpowiada za zarządzanie i odtwarzanie dźwięków 2D oraz 3D za pomocą biblioteki `miniaudio`. Architektura oparta jest na wzorcu Singleton (`SoundEngine`), który centralizuje zarządzanie instancjami dźwięków (`MaSoundInstance`), zasobami oraz stanem silnika audio. System dekoduje i przechowuje dane w buforach pamięci (`std::vector<uint8_t>`) ładowanych z wirtualnego systemu plików (PackManager). W aktualnej formie kod wykazuje cechy legacy, łącząc C++17 z manualnym zarządzaniem stanem i brakiem nowoczesnych abstrakcji.

## Zestawienie Zidentyfikowanych Defektów i Niespójności

### 1. Zarządzanie zasobami i cykl życia obiektów (RAII & Memory Lifetime)
- **Plik**: `src/AudioLib/MaSoundInstance.h`, `src/AudioLib/MaSoundInstance.cpp` (linia ~67)
- **Problem**: Brak wykorzystania wzorca RAII dla zasobów `ma_sound` oraz `ma_decoder`. Metoda `Destroy()` musi być wywoływana manualnie z zewnątrz.
- **Wpływ**: Wysokie ryzyko wycieku pamięci i uchwytów systemowych w przypadku rzucenia wyjątku lub wcześniejszego wyjścia ze ścieżki wykonania, zanim `Destroy()` zostanie wywołane.
- **Rekomendacja**: Zamknąć struktury `miniaudio` w inteligentnych wskaźnikach ze specjalizowanym delekcyjnym funktorem (custom deleter) lub w klasie w pełni implementującej RAII (konstruktor inicjalizuje, destruktor zwalnia).

### 2. Poprawność współbieżności i synchronizacja wątków (Thread Safety & Synchronization)
- **Plik**: `src/AudioLib/SoundEngine.cpp` (linie ~130, ~212, ~289)
- **Problem**: Klasa `SoundEngine` posiada współdzielony stan mutowalny (np. `m_Sounds2D`, `m_Sounds3D`, `m_Files`, `m_PlaySoundHistoryMap`), do którego dostęp nie jest chroniony żadnymi mechanizmami synchronizacji (np. `std::mutex`, `std::shared_mutex`).
- **Wpływ**: Krytyczne ryzyko wystąpienia *data races* oraz *undefined behavior*, jeżeli wątek renderujący wywołuje `Update()`, a inny wątek (np. pobierający zasoby z sieci) wywołuje `PlaySound3D()`.
- **Rekomendacja**: Wprowadzenie bezblokujących struktur danych (lock-free) lub odpowiednich prymitywów synchronizacyjnych. W przypadku Rust zalecane użycie `Arc<RwLock<T>>` lub `Arc<Mutex<T>>`.

### 3. Odporność parsera na fragmentację danych (Protocol Stream Robustness)
- **Plik**: `src/AudioLib/Type.cpp` (linia ~72)
- **Problem**: Konwersja czasu za pomocą `(float)atof(...)` i brak ścisłej walidacji danych zejściowych dla rozmiarów i typów (np. buforowanie dźwięku `buffer.resize(soundFile.size()); memcpy(...)`).
- **Wpływ**: Potencjalny błąd rzutowania znaków (*truncation*), podatność na uszkodzone pakiety/pliki konfiguracyjne (brak sanity checks dla długości/zawartości przed odczytem). Brak walidacji po przekroczeniu limitu `size()`.
- **Rekomendacja**: Użyć nowoczesnych bezpiecznych widoków pamięci (`std::span` z C++20), rzutowania bez utraty danych oraz bezbłędnych funkcji parsowania (np. `std::from_chars`).

### 4. Determinizm logiki biznesowej i atomowość operacji (Business Logic & State Determinism)
- **Plik**: `src/AudioLib/SoundEngine.cpp` (linia ~143), `src/AudioLib/Type.cpp` (linia ~148)
- **Problem**: Operacja `c_rSoundData.fTime / c_fFrameTime` w oparciu o rzutowanie typów zmiennoprzecinkowych zależy od implementacji sprzętowej FP. Dodatkowo timer w `CTimer::Instance().GetCurrentSecond()` bazuje na czasie rzeczywistym.
- **Wpływ**: Brak determinizmu w maszynie stanów klatek (frame state machine). Różne maszyny mogą wygenerować inną ramkę odtworzenia dźwięku (`dwFrame`), co w grze sieciowej/współbieżnej prowadzi do desynchronizacji symulacji.
- **Rekomendacja**: Oddzielenie czasu symulacji gry (fixed tick time) od czasu rzeczywistego (real time/wall clock) oraz stosowanie stałoprzecinkowej arytmetyki do kalkulacji klatek (fixed-point arithmetic lub integer math).

### 5. Wydajność na ścieżce krytycznej (Performance Optimization)
- **Plik**: `src/AudioLib/SoundEngine.cpp` (linie ~114, ~305)
- **Problem**: Odtwarzanie dźwięków alokuje pamięć na ścieżce krytycznej (`buffer.resize()`), co powoduje potencjalne wstrzymania (stall) z powodu blokad alokatora. Obliczanie odległości euklidesowej `sqrtf(dx*dx + dy*dy + dz*dz)` wykonuje się jednowątkowo bez SIMD i instrukcja ta nie jest wykorzystywana.
- **Wpływ**: Spadki liczby klatek na sekundę (frame drops) w momentach ładowania wielu dźwięków, zwiększona fragmentacja sterty.
- **Rekomendacja**: Wprowadzenie pre-alokowanych pul pamięci (memory pools), ładowanie asynchroniczne i wektoryzacja kalkulacji dystansu z użyciem instrukcji SIMD (`std::simd` w C++26 / intrinscics). Zastosowanie wektorów buforów zamiast dynamicznych alokacji.

## Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Koncepcja w C++23
Wykorzystanie `std::span` i inteligentnych wskaźników z deduplikacją (Flyweight/Resource Cache).

```cpp
#include <memory>
#include <span>
#include <string_view>
#include <mutex>
#include <unordered_map>

// RAII Wrapper dla miniaudio resource
struct SoundDecoderDeleter {
    void operator()(ma_decoder* p) const { ma_decoder_uninit(p); delete p; }
};
using UniqueDecoder = std::unique_ptr<ma_decoder, SoundDecoderDeleter>;

class SoundResourceManager {
    std::unordered_map<std::string, std::vector<std::byte>> m_buffers;
    std::shared_mutex m_mutex;

public:
    std::span<const std::byte> GetOrLoadBuffer(std::string_view name) {
        {
            std::shared_lock lock(m_mutex);
            if (auto it = m_buffers.find(std::string(name)); it != m_buffers.end()) {
                return it->second;
            }
        }
        
        // Asynchroniczne ładowanie poza ścieżką krytyczną...
        std::vector<std::byte> newBuffer = LoadFromPackManager(name);
        
        std::unique_lock lock(m_mutex);
        return m_buffers.emplace(name, std::move(newBuffer)).first->second;
    }
};
```

### Koncepcja w Rust
Wykorzystanie systemu pożyczeń (borrow checker) by wyeliminować wycieki z góry i kontrolować współdzielny stan:

```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bytes::Bytes;

pub struct MaSoundInstance {
    // RAII zapewnione przez impl Drop
    decoder: Option<miniaudio::Decoder>,
    sound: miniaudio::Sound,
    identity: String,
}

impl Drop for MaSoundInstance {
    fn drop(&mut self) {
        // Zwalnianie zasobów miniaudio automatycznie po wyjściu ze scope'u
    }
}

pub struct SoundEngine {
    files: Arc<RwLock<HashMap<String, Bytes>>>,
    sounds_3d: [RwLock<MaSoundInstance>; 32],
}

impl SoundEngine {
    pub fn play_sound_3d(&self, name: &str, position: glam::Vec3) -> Option<()> {
        let buffers = self.files.read().unwrap();
        if let Some(buffer) = buffers.get(name) {
            // Bezpieczne, zero-copy pożyczenie przy pomocy Bytes i kontrola wątków
            // Obliczenia wektorowe z użyciem glam (wsparcie SIMD u wierzchołków)
            // ...
            Some(())
        } else {
            None
        }
    }
}
```

Powyższa refaktoryzacja spełnia nowoczesne standardy bezstanowej synchronizacji w silnikach gier i gwarantuje odporność na memory-leaks.
