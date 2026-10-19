# AUDIT-SYS-06: ModernLogger & Fast String Buffer

## 1. Architektura i Przeznaczenie Modulow

Przeglądane moduły to część infrastruktury bazowej projektu, skoncentrowanej na logowaniu i bezpiecznym przetwarzaniu ciągów znakowych oraz danych binarnych.

- **`LogModern.h` / `ModernLogger.h`**: Nowoczesny system logowania z wykorzystaniem C++23 `std::format`. Eliminuje użycie przestarzałych mechanizmów jak `sprintf` z rodziny C, stawiając na silne typowanie i bezpieczeństwo w czasie kompilacji (dzięki sprawdzaniu formatowania z `std::format_string`).
- **`FastFixedStringBuilder.h`**: Bufor ciągów znakowych na stosie o określonej pojemności, niealokujący pamięci na stercie. Używa `std::format` i nowości z C++23 takich jak `std::expected` (reprezentowane przez wywołania `MakeError` oraz typy z `Result.h`), eliminując wady buforów dynamicznych w wysoce krytycznych częściach kodu.
- **`StringUtils.h`**: Implementacja pomocniczych funkcji na ciągach znakowych pracujących na widokach `std::string_view` i `std::span`, unikając alokacji.
- **`ByteBuffer.h`**: Pamięciowy bufor danych dla sieciowych połączeń TCP. Automatycznie roluje i powiększa zajmowaną pamięć używając `std::memmove` i `std::memcpy`.
- **`RingBuffer.h`**: Pamięć buforowa (circular buffer) alokująca dane raz i zapewniająca operacje bez kopiowania na stos, stworzona z myślą o pakietach sieciowych.

## 2. Zidentyfikowane Defekty i Potencjalne Zagrożenia

### 2.1 Zarzadzanie zasobami i cykl zycia obiektow (RAII & Memory Lifetime)
- **`ByteBuffer.h` (`EnsureCapacity`)**:
  - `EnsureCapacity` wykorzystuje `std::vector<uint8_t> buffer`, którego pojemność może rosnąć bez ograniczeń (brak limitu). Dla nieprzyjaznych danych z zewnątrz (np. wysłanie fragmentów o nieskończonej długości bez nagłówków pakietów, ataki typu Slowloris) może to doprowadzić do wycieku pamięci/wyczerpania RAM (OOM - Out Of Memory). 
  - *Zalecana naprawa*: Ustalenie maksymalnej wielkości bufora i rzucanie wyjątku/błędu (albo `std::expected`), gdy limit zostanie przekroczony.
- **`RingBuffer.h`**: 
  - Używa dynamicznej wektorowej alokacji (`std::vector<uint8_t> buffer_;`) i zarządza manualnie stanem. Klasa jest poprawna dla RAII (wektor zwalnia się sam), ale `std::span` eksponowany w zewnętrznym interfejsie nie może "żyć dłużej" niż cykl obiektu `RingBuffer` i jego wewnętrzny wskaźnik. Należy wyraźnie to zaznaczyć albo używać w bezpiecznym modelu właścicielstwa.

### 2.2 Poprawnosc wspolbieznosci i synchronizacja watkow (Thread Safety & Synchronization)
- **`LogModern.h`**:
  - Metoda `WriteLog` używa operacji wejścia-wyjścia (`std::cerr <<`, `std::cout <<`, `OutputDebugStringA`), jednakże **nie posiada żadnej synchronizacji**. Wynikowo logi z różnych wątków (sieć, main, renderer) będą przeplatać się w konsoli, co uniemożliwi czytanie, lub, co gorsza, doprowadzi do ubocznych efektów dla globalnego stanu I/O.
  - *Zalecana naprawa*: Dodać globalny `std::mutex` lub bezstanową (lock-free) kolejkę logów i dedykowany wątek do przetwarzania wypisywania.
- **`FastFixedStringBuilder.h` i Bufory (ByteBuffer, RingBuffer)**:
  - Z definicji te struktury nie są thread-safe, co jest do zaakceptowania (są często lokalne), natomiast ich wspóldzielenie wymaga osobnej warstwy kontroli przez `std::mutex` bądź implementację kanałów w stylu Rusta.

### 2.3 Odpornosc parsera na fragmentacje danych (Protocol Stream Robustness)
- **`ByteBuffer.h`**: 
  - Wykorzystuje typowanie `uint32_t` dla offsetów. Trzeba zadbać o to, żeby przesyłane długości lub operacje np. matematyczne dla nowej pojemności `new_capacity = static_cast<uint32_t>(buffer.size()) * 2;` nie powodowały *integer overflow*. O ile w grze pakiety nie są tak gigantyczne, warto używać w takich metodach jakichś limitów.
  - Kod dla bufora cyklicznego w `RingBuffer.h` wywłaszcza pamięć pre-alokowaną, ale brakuje sprawdzenia, czy `count` w `AdvanceWrite(size_t count)` ma logiczny sens dla zewnętrznych wywołań w kontekście parserów.

### 2.4 Wydajnosc na sciezce krytycznej (Performance Optimization)
- **`LogModern.h` (`ModernLogger::Log`)**:
  - `std::string message = std::format(fmt, std::forward<Args>(args)...);` wykonuje dynamiczną alokację pamięci (std::string) za każdym razem, gdy cokolwiek jest logowane, w tym dla poziomu *Trace/Debug*, niezależnie od tego, czy poziom logowania pozwala na jego wydrukowanie. 
  - *Zalecana naprawa*: Sprawdzaj `level` i omiń wykonywanie drogich obliczeń `std::format` i alokacji `std::string`, gdy dany poziom logowania jest globalnie wyłączony lub poniżej progu.
- **`FastFixedStringBuilder.h`**:
  - Używa alokacji na stosie i `std::format_to_n`, co jest świetne dla wydajności, i bezpiecznie omija stertę.

## 3. Wzorcowa Propozycja Refaktoryzacji

### Propozycja do C++23

Poprawa z użyciem C++23 - synchronizacja logów i lazy-evaluation (opóźnienie formatowania), by uniknąć narzutu wydajnościowego:

```cpp
// LogModern.h - Fixes

class ModernLogger {
    static inline std::mutex s_logMutex;
    static inline LogLevel s_minLevel = LogLevel::Info;

public:
    template <typename... Args>
    static void Log(LogLevel level, std::format_string<Args...> fmt, Args&&... args) {
        if (level < s_minLevel) {
            return; // Eliminacja zbędnej alokacji
        }
        try {
            // Mozna uzyc FastFixedStringBuilder zeby calkowicie ominac alokacje std::string, ale dla logow to i tak OK
            std::string message = std::format(fmt, std::forward<Args>(args)...);
            WriteLog(level, message);
        } catch (...) {
            WriteLog(LogLevel::Error, "Log formatting exception occurred");
        }
    }
private:
    static void WriteLog(LogLevel level, std::string_view message) {
        std::lock_guard<std::mutex> lock(s_logMutex);
        // ... (kod z output string i cout)
    }
};
```

### Propozycja dla Rust
Rozwiazanie uzywa makr ktore leniwie formatuja kod (eliminujac overhead) i calkowite wlascicielstwo (ownership).

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::{Level, Log, Metadata, Record};

lazy_static! {
    static ref LOG_MUTEX: Mutex<()> = Mutex::new(());
}

struct ModernLogger;

impl Log for ModernLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _lock = LOG_MUTEX.lock().unwrap();
            let prefix = match record.level() {
                Level::Error => "[ERROR]",
                Level::Warn => "[WARN]",
                Level::Info => "[INFO]",
                Level::Debug => "[DEBUG]",
                Level::Trace => "[TRACE]",
            };
            println!("{} {}", prefix, record.args());
        }
    }

    fn flush(&self) {}
}
```

```rust
// Przyklad dla bezpiecznego bufora bajtow - zastapienie ByteBuffer
pub struct ByteBuffer {
    buffer: Vec<u8>,
    read_position: usize,
    write_position: usize,
    max_capacity: usize, // Wprowadzenie maksymalnego limitu by zapobiec OOM
}
```

