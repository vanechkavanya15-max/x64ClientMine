# Raport z audytu jakości kodu: AUDIT-ECON-06

## 1. Podsumowanie architektury zbadanych modułów
Moduł `DumpProto` to narzędzie do przetwarzania i konwersji tekstowych danych konfiguracyjnych gry (pliki CSV, TSV) do postaci skompresowanych i zaszyfrowanych plików binarnych (np. `item_proto`, `mob_proto`). 

Architektura narzędzia bazuje na przestarzałych konstrukcjach języka C++ (C++98/03), silnie wykorzystując ręczne zarządzanie pamięcią, globalny stan, nagie wskaźniki (raw pointers) do wyrażania własności oraz niebezpieczne operacje na ciągach znaków. Logika ładowania CSV jest zrealizowana poprzez w pełni niestrzeżone struktury i polimorfizm ręcznie usuwanych typów (`cCsvTable`, `cCsvFile`). Moduł korzysta z kompresji LZO i szyfrowania TEA, wchodząc w bezpośrednie operacje na zrzutach struktur pamięci. Zidentyfikowano rażące luki w cyklu życia obiektów i architekturze odpornej na wyjątki.

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

| Plik | Linia | Mechanizm | Opis defektu | Wpływ na stabilność / Bezpieczeństwo |
|---|---|---|---|---|
| `dump_proto.cpp` | ~302, 706 | Wyciek Pamięci (Brak RAII) | Alokacja `test_mob_table = new TMobTable[...]` (oraz odpowiednik dla przedmiotów) nie posiada `delete[]`. Nagie wskaźniki są gubione, a pamięć wycieka w ścieżce sukcesu i błędu. | Zbędne zużycie pamięci przy uruchamianiu. Powtarzalny leak na pętli restartu. |
| `dump_proto.cpp` | 125, 186 | Globalny Stan / Wiszące wskaźniki | Zmienne globalne `m_pMobTable` i `m_pItemTable` zarządzane ręcznie (`new`, `free`, `delete`). Brak determinizmu, gdy użyto `free` w `BuildItemTable` zamiast `delete[]`. | Otwiera drogę na double-free, undefined behavior lub crash aplikacji. |
| `ItemCSVReader.cpp`| 16 | Zwracanie tablicy wskaźnikiem | Funkcja `StringSplit` alokuje `new string[30]` i zwraca nagi wskaźnik. Wywołujący odpowiada za wywołanie `delete[]`. | Krytyczne naruszenie RAII. Jeśli wywołujący zgłosi wyjątek, wystąpi wyciek pamięci. |
| `CsvFile.h` | 269 | Ręczny cykl życia obiektów | Kontener `ROWS m_Rows` przechowujący nagie wskaźniki `cCsvRow*`. Destrukcja następuje w `Destroy()`. | Ryzyko utraty wywołania niszczenia lub wycieki w przypadku wystąpienia wyjątków parsera. |
| `dump_proto.cpp` | 203 | Przepełnienie bufora / Brak `\0` | Wywołania `strncpy` (np. na `szName`) bez dodawania terminatora null na końcu bufora, jeśli ciąg wejściowy jest równy długości `CHARACTER_NAME_MAX_LEN`. | Odczyty z nieznanych obszarów pamięci (OOB Read) podczas wyświetlania nazw, memory corruption. |
| `lzo.cpp` | 135 | Ręczna alokacja zasobów | `BYTE* pbDecryptBuffer = new BYTE[...]` zwraca wskaźnik bez ownershipu. Wywołujący z `Decompress` zwalnia go ręcznie. | Utrudnione śledzenie alokacji w architekturze kryptograficznej, ryzyko wycieku w kluczowych ścieżkach błędu. |
| `CsvFile.cpp` | 67 | Buffering o stałym rozmiarze | Wczytywanie z limitowanego bufora tablicowego: `file.getline(buf, 2048)`. Brak uwzględnienia zbyt długich wierszy. | Przekłamanie danych (Truncation) lub uszkodzenie ramek protokołu. Parser nie jest odporny na złośliwą konfigurację. |
| `lzo.cpp` | 155 | Data Race (Thread Safety) | Klasa Singleton `CLZO` posiada współdzielony pre-alokowany bufor roboczy `m_pWorkMem`, wykorzystywany przy kompresji. | Wykonanie zrzutu protokołu z wielu wątków równolegle zniszczy pamięć LZO poprzez konflikt na wspólnym buforze. |

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Propozycja refaktoryzacji: Nowoczesny C++23

Zamiast modyfikowania niestrzeżonych tablic i gołych wskaźników, system należy przenieść na w pełni zautomatyzowane RAII z zapewnieniem Bounds Checking'u.

**1. RAII i Automatyczne zarządzanie pamięcią:**
Zastąpienie globalnych nagich wskaźników przez dynamiczne wektory i unikalne wskaźniki ograniczające scope wycieków pamieci.
```cpp
// Zamiast TMobTable* test_mob_table = new TMobTable[size];
std::vector<TMobTable> test_mob_table(test_MobTableSize);

// Zamiast ROWS (std::vector<cCsvRow*>) w cCsvFile, co naprawia brak niszczenia obiektów przy błędach IO:
using ROWS = std::vector<std::unique_ptr<cCsvRow>>;
```

**2. Bezpieczne parsowanie znaków - `std::string_view` (Zero-allocation copies):**
Redukcja nadmiernych alokacji w funkcjach narzędziowych do cięcia ciągów tekstowych.
```cpp
// Zoptymalizowana wersja trim zamiast tworzenia kilku std::string w procesie:
constexpr std::string_view trim(std::string_view str) {
    auto start = str.find_first_not_of(" \t\v\n\r");
    if (start == std::string_view::npos) return {};
    auto end = str.find_last_not_of(" \t\v\n\r");
    return str.substr(start, end - start + 1);
}
```

**3. Odporność parsera z wykorzystaniem widoków `std::span`:**
Zabezpieczenie szyfrujących bloków z kryptografią.
```cpp
// Przesyłanie referencji do rozmiarów używając span uniemożliwia buffer-overflow
bool CLZO::Decompress(std::span<const std::byte> input_buffer, std::span<std::byte> output_buffer, uint32_t key);
```

### Propozycja refaktoryzacji: Rust (Safety by design)

Alternatywnie zastosowanie języka Rust niweluje z marszu klasę błędów RAII, alokacyjnych oraz współbieżnych z racji Borrow Checkera.

**1. Ograniczenie złożoności z Serde:**
Ręczne parsowanie wierszy, liczenie "quote state" zostanie zastąpione sprawnym silnikiem.
```rust
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MobTable {
    pub vnum: u32,
    pub name: String,
    pub locale_name: String,
    pub rank: u8,
    pub battle_type: u8,
    pub level: u8,
    // ...
}

fn load_mob_table(path: &str) -> Result<Vec<MobTable>, Box<dyn std::error::Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(path)?;
        
    let table: Result<Vec<MobTable>, _> = reader.deserialize().collect();
    Ok(table?)
}
```

**2. Eliminacja Thread-Safety Issues oraz Memory Leaks przy dekompresji:**
Pamięć i wskaźniki podążają za regułami Lifetime. Nie ma możliwości zapomnienia o zwolnieniu wyjściowego bufora (jak w przypadku `pbDecryptBuffer`), gdyż jest on instancją trzymaną w zasięgu leksykalnym.

```rust
pub fn compress(data: &[u8]) -> Result<Vec<u8>, LzoError> {
    // Alokuje bezpieczny wektor; 
    // Po wyjściu ze scope'u z błędem pamięć automatycznie się zwalnia.
    let capacity = data.len() + data.len() / 64 + 16 + 3;
    let mut compressed = Vec::with_capacity(capacity);
    
    // użycie bezpiecznych wrapperów (miniz_oxide / lzo_rust) ...
    Ok(compressed)
}
```

Dzięki rygorystycznym zasadom Borrow Checker'a, dzielenie singletona z wbudowanym współdzielonym buforem (`GetWorkMemory()`) nie zostałoby dopuszczone przy kompilacji pod architekturę multi-thread (brak `Send`/`Sync`), zapobiegając usterkom opisanym w punkcie 2.
