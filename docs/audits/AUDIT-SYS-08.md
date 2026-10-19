# AUDIT-SYS-08 — PackLib VFS Mount & Decryption

## Podsumowanie Architektury

Moduł `PackLib` to prosty system wirtualnego systemu plików (VFS) odpowiedzialny za parsowanie indeksów paczek (z uwzględnieniem szyfrowania nagłówków XChaCha20), zarządzanie wirtualnymi ścieżkami oraz dekompresję danych przy użyciu biblioteki Zstd. Posiada mechanizm buforowania w celu odciążenia I/O i przezroczysty system ładowania z dysku jako fallback (assets folder), wspierający też migrację z formatu `.gr2` do `.glb`. Został zaprojektowany w oparciu o C++20/C++23 i używa standardowych mechanizmów synchronizacji (std::shared_mutex) do obsługi wielowątkowego odczytu danych w architekturze headless i klienckiej.

## Identyfikacja defektów kodu i niespójności

### 1. Zarządzanie zasobami i cykl życia obiektów (RAII & Memory Lifetime)
*   **Plik:** `src/PackLib/PackManager.cpp`, linie: `56`, `64`
    *   **Mechanizm / Problem:** Ręczne zarządzanie cyklem życia alokacji `CBufferPool` (przez `new`/`delete`). Mimo, że destruktor klasy je zwalnia, brak wykorzystania smart pointerów (jak `std::unique_ptr`) łamie zasady RAII i idiomatycznego kodu w nowoczesnym C++. W skrajnych sytuacjach wyjątków w konstruktorze mogłoby dojść do wycieku pamięci.
    *   **Wpływ na stabilność:** Obecnie stabilne o ile kod nie zgłasza wyjątków, ale stanowi "code smell" i dług technologiczny.
*   **Plik:** `src/PackLib/PackManager.h`, linia: `25`
    *   **Mechanizm / Problem:** Funkcja `GetAssetBasePath()` zwraca `const std::string&`. Pole to (`m_assetBasePath`) jest oznaczane jako `mutable` i bywa mutowalne pod osłoną unikalnych blokad mutexu (m.in. w funkcji `EnsureAssetIndexBuilt()`). Jeśli jeden z wątków czyta referencję zwróconą przez ten getter, gdy w tym samym momencie inny wątek powoduje zmianę ścieżki i reallokację wektora na zapleczu std::string, prowadzi to do wiszącej referencji i natychmiastowego segmentation fault.
    *   **Wpływ na stabilność:** Krytyczny, potencjalne ryzyko awarii aplikacji i niestabilności w środowisku wielowątkowym (Data Race).

### 2. Poprawność współbieżności i synchronizacja wątków (Thread Safety & Synchronization)
*   **Plik:** `src/PackLib/PackManager.cpp`, linie: `95`, `100`, `105`, `110`, `164`, `169`, `174` (np. w `EnsureAssetIndexBuilt()` i `AddPack()`)
    *   **Mechanizm / Problem:** Operacje I/O w postaci wywołań blokujących `std::filesystem::exists` są wykonywane wewnątrz bloku zabezpieczonego na wyłączność przez `std::unique_lock<std::shared_mutex>`. Te blokady mogą trwać milisekundy, zwłaszcza pod dużym obciążeniem I/O lub na wolniejszych dyskach.
    *   **Wpływ na stabilność:** Potężna degradacja wydajności (starvation) dla czytających wątków (np. renderera czy main loopa) chcących pobrać w tym samym czasie inne wirtualne zasoby (co jest naturalne dla streamingu mapy).

### 3. Odporność parsera na fragmentację danych (Protocol Stream Robustness)
*   **Plik:** `src/PackLib/PackManager.cpp`, linia: `121`
    *   **Mechanizm / Problem:** Do funkcji `NormalizePath(std::string_view in, std::string& out)` podawana jest wartość `entry.file_name` z tablicy znaków o stałej długości zadeklarowanej w strukturze nagłówka pakietu (`char file_name[FILENAME_MAX+1]`). Konstruktor typu z `const char*` przyjmie to wyrażenie do wywołania funkcji `strlen()`, jednak nikt z definicji struktury wejściowej VFS nie gwarantuje bajtu null terminującego jeśli ścieżka dokładnie wypełnia bufor, co prowadzi do potencjalnego overreadu pamięci poza buffer pakietu wejściowego.
    *   **Wpływ na stabilność:** Undefined Behavior.

### 4. Determinizm logiki biznesowej i atomowość operacji (Business Logic & State Determinism)
*   **Plik:** `src/PackLib/PackManager.cpp`, linie: `258`, `391`
    *   **Mechanizm / Problem:** Wykorzystanie globalnego stanu w postaci zmiennych typu `thread_local std::string buf` wewnątrz funkcji takich jak `GetFileWithPool` oraz `IsExist`. W przypadku zagnieżdżonego odpytywania o zasoby (tzw. re-entrancy / rekursywne poszukiwania plików np. ze względu na fallback) lub w wywołaniach zewnętrznych z innych callbacków w obrębie tego samego wątku, wewnętrzna funkcja modyfikująca buffor zniszczy kontekst pierwszej instrukcji.
    *   **Wpływ na stabilność:** Prowadzi to do niefortunnych nadpisań ścieżek "w locie", zwracania błędów braku pliku mimo jego fizycznej obecności i utraty determinizmu VFS (Logic Flaw).

### 5. Wydajność na ścieżce krytycznej (Performance Optimization)
*   **Plik:** `src/PackLib/Pack.cpp`, linia: `98`
    *   **Mechanizm / Problem:** `result.resize(entry.file_size)` inicjalizuje zadeklarowany rozmiar struktury zerami (default initialization) tuż po lokacji. Chwilę potem linia nr 106 wykorzystuje natywny interfejs `ZSTD_decompressDCtx` nadpisujący kompletnie całą zainicjowaną przestrzeń. Niekorzystne dla wektorów o kilkunastu/kilkudziesięciu gigabajtach pakietów rozpakowywanych symultanicznie. Ponadto wartość z pliku wejściowego (`entry.file_size`) nie ma progu bezpieczeństwa – po sfałszowanym indeksie VFS to bezpośrednio spowoduje wyjątek `std::bad_alloc`.
    *   **Wpływ na stabilność:** Niska efektywność (nadmiarowy bandwidth pamięci dla L2/L3) i wektor na atak OOM przez spakowany "bombowy" plik w VFS.

## Propozycja Refaktoryzacji do Nowoczesnego C++23 / Rust

### C++23:
1. **RAII**: Zamiana `CBufferPool* m_pBufferPool` na `std::unique_ptr<CBufferPool> m_pBufferPool`.
2. **Bezpieczny ciąg znaków (Span)**: Użycie span dla bezpiecznego przekazania nazwy pliku:
   `NormalizePath(std::string_view(entry.file_name, strnlen(entry.file_name, FILENAME_MAX)), normKey);`
3. **Zarządzanie wektorem do dekompresji (resize_and_overwrite / OOM Check)**: 
   Dodanie limitu sanitarnego dla `entry.file_size` (np. MAX_FILE_SIZE = 512MB) oraz użycie metody `resize_and_overwrite` z biblioteki standardowej dla uniknięcia zero-initalizacji:
   ```cpp
   if (entry.file_size > MAX_ALLOWED_FILE_SIZE) { return false; }
   result.resize_and_overwrite(entry.file_size, [&](char* buf, size_t n) {
        return entry.file_size; // bez wypelniania zerami
   });
   ```
4. **Usunięcie thread_local state**: Deklaracja standardowych wektorów lokalnych: `std::string buf;`. Eliminacja re-entrancy flaws i unikanie współdzielenia buforów, gdzie koszt alokacji lokalnego stringa może być zlikwidowany przez SSO (Small String Optimization).
5. **Poprawki Współbieżności**: Skopiowanie wartości mutowalnej zmiennej zwracanej przed wyjściem ze strefy krytycznej w getterze (zamiast powrotu referencji). Zminimalizowanie blokad w I/O - sprawdzanie ścieżki i kanonizacji poza lockiem, blokując jedynie wpis do `m_packNames`.

### Rust (Docelowy model dla modularności - VFS):
1. Struktura `PackManager` operowałaby na zwięzłych i bezpiecznych typach współdzielonych. 
   ```rust
   pub struct PackManager {
       load_from_pack: AtomicBool,
       entries: Arc<RwLock<HashMap<String, PackFileMapEntry>>>,
       asset_base_path: Arc<RwLock<Option<PathBuf>>>,
   }
   ```
2. Zamiast `resize()` bez weryfikacji zastosowanoby `Vec::with_capacity(size)` w połączeniu z `Read::read_exact` / bezpiecznym buforowaniem do granicy `Take<R>`, zwracającym błędy poprzez naturalne `Result<Vec<u8>, VfsError>`. Brak zero-initialize odbywa się przez ustrukturyzowane, nadzorowane i weryfikowane operacje `MaybeUninit<T>` i `Read` traits, całkowicie bezpiecznie dla stanu pamięci.
3. Stringi w obrębie paczki (ograniczone strukturalnie `[u8; FILENAME_MAX]`) mogłyby być odczytane w sposób kontrolowany dzięki CStr z pełnym memory bounds check (lub konwersją `lossy`).
4. Użycie globalnego `thread_local!` dla bufforów zostałoby wykluczone na poziomie logiki ze względu na model pożyczeń `borrow-checker` (który sam domyślnie zabezpieczyłby re-entrancy flaw, o ile nie stosowanoby obejścia typu `RefCell`).
5. Blokady (I/O) można rozwiązać dzięki asynchronicznym I/O opartym na Tokio (`tokio::fs::metadata`) z pełnym oddzieleniem executorów od CPU-bound tasks (dekompresji w Rayon), likwidując problem "starvation".
