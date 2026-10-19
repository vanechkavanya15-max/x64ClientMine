# Raport AUDIT-PHYS-04

**Obszar przeglądu:** `src/PRTerrainLib/`

## 1. Podsumowanie architektury zbadanych modułów
Biblioteka `PRTerrainLib` odpowiada za zarządzanie przestrzennymi danymi terenu (heightmap, tilemap, watermap, attrmap, shadowmap) oraz zestawami tekstur środowiskowych (`TextureSet`). Architektura opiera się głownie na monolitycznej klasie `CTerrainImpl`, zarządzającej cyklem życia kafelków terenu, oraz `CTextureSet`. Obecna struktura opiera się na niskopoziomowych mechanizmach z języka C (np. `memcpy`, `memset`, wskaźniki surowe) opakowanych w klasy C++. Brakuje jednolitego zarządzania zasobami (RAII) oraz bezpiecznej synchronizacji wątkowej, co w środowisku wielowątkowym może prowadzić do nieprzewidywalnych błędów. Z punktu widzenia projektowania zorientowanego na dane (Data-Oriented Design), struktury nie są odpowiednio ułożone dla optymalnego przetwarzania z wykorzystaniem instrukcji wektorowych (SIMD) i pamięci podręcznej (Cache).

## 2. Zestawienie zidentyfikowanych defektów kodu i niespójności

1. **Zarządzanie zasobami i cykl życia (RAII & Memory Lifetime)**
   - *Plik*: `src/PRTerrainLib/Terrain.cpp`, `CTerrainImpl::Initialize()`
   - *Mechanizm*: Inicjalizacja instancji przy użyciu `memset(this, 0, ...)` (lub poszczególnych zmiennych np. `memset(&m_TerrainSplatPatch, 0, sizeof(m_TerrainSplatPatch))`) wywodzi się z C. W C++23 użycie tego mechanizmu w strukturach mających nietrywialne pola (w przyszłości np. string, vector) trwale korumpuje pamięć, wycieka zasoby i ukrywa błędy niezainicjowanych zmiennych przy błędach I/O.
   - *Wpływ na stabilność*: Otwiera wektor na memory corruption oraz niezdefiniowane zachowania (UB). Brak stosowania `std::unique_ptr` czy COM smart pointers (np. `ComPtr<IDirect3DTexture9>`) grozi wyciekami pamięci przy ścieżce błędów (early return w `LoadAttrMap` lub `SetTexture`).

2. **Współbieżność i bezpieczeństwo wątków (Thread Safety & Synchronization)**
   - *Plik*: `src/PRTerrainLib/Terrain.cpp` (`CTerrainImpl::SetTextureSet`, `GetTextureSet`)
   - *Mechanizm*: Zmienna statyczna `ms_pTextureSet` reprezentująca globalny stan puli tekstur środowiska jest aktualizowana i modyfikowana bez użycia zabezpieczeń wątkowych (brak `std::mutex` lub `std::atomic`).
   - *Wpływ na stabilność*: Wyścig danych (Data Race) przy dostępie do terenu pomiędzy wątkiem sieci (pobierającym stan świata), pętlą główną, a wątkiem renderera. Może prowadzić do zawieszeń typu use-after-free lub crashy w silniku.

3. **Odporność parsera na fragmentację danych (Protocol Stream Robustness)**
   - *Plik*: `src/PRTerrainLib/TextureSet.cpp` (`CTextureSet::Load`)
   - *Mechanizm*: Używanie funkcji z rodziny C (`atoi`, `atof`) do parsowania danych wejściowych bez mechanizmu fail-safe i sprawdzania przekroczenia zakresu. Dodatkowo brak hermetyzacji parsera binarnego; pliki terenu parsują pakiety po wskaźnikach z surowego bufora, bazując na rzutowaniu struktur zależnych od wyrównania kompilatora (`#pragma pack(1)` jest przestarzałe i nieprzenośne m.in. dla ARM).
   - *Wpływ na stabilność*: Silent Failures — niepoprawne formatowanie wejścia prowadzi do mapowania zer lub bzdurnych wartości (nan, infinity), rozstrajając stan fizyczny obiektów (ghosting/przenikanie).

4. **Wydajność na ścieżce krytycznej (Performance Optimization & DOD)**
   - *Plik*: `src/PRTerrainLib/TextureSet.cpp` (`CTextureSet::Load`), `CTerrainImpl::GetShadowMapColor`
   - *Mechanizm*: Złożoność przetwarzania na stosie z wieloma niepotrzebnymi alokacjami w pętlach (`_snprintf` w mapie, tworzenie obiektów std::string per tick parsera). Rozbicie na obszerne matryce dwuwymiarowe (`m_awRawHeightMap`, `m_abyTileMap`, itp.) niezapewniające wyrównania wielokrotności cache-line. 
   - *Wpływ na stabilność*: Ogromne ilości Cache Misses i obniżenie frametime'u/tickrate'u maszyny stanów, uniemożliwiające automatyczną wektoryzację pętli roboczych (Brak `alignas(64)` i SoA - Struct of Arrays).

## 3. Wzorcowa propozycja refaktoryzacji do nowoczesnego C++23 / Rust

### Migracja na C++23 (Faza Przejściowa):
- **Widoki Pamięci i Parsing**: Zamienić surowe wskaźniki bajtowe `BYTE*` na `std::span<const std::byte>`, zapewniając bezpieczeństwo w dostępie i weryfikację wymiarów (bounds-checking) poprzez `std::span::subspan`.
- **Inicjalizacja RAII**: Zastąpić `memset` na standardową inicjalizację składowych klasy w miejscu deklaracji (np. `std::array<WORD, HEIGHTMAP_RAW_YSIZE * HEIGHTMAP_RAW_XSIZE> m_awRawHeightMap{};`). Zarządzać teksturami z DirectX przez bezpieczny `Microsoft::WRL::ComPtr`.
- **Nowoczesne API Konwersji**: Zastąpić wrażliwe `atoi/atof` wydajnym i bezpiecznym `std::from_chars`, wykorzystując `std::expected` do reprezentacji sukcesu lub błędu parsującej ścieżki logiki.
- **Cache Locality i SIMD**: Zaprojektować wyrównanie dużych tabel terenu, używając specyfikatora `alignas(64)`. Przerobić struktury pod kątem optymalizacji Data-Oriented Design (SoA) w obszarach, które tego wymagają, aby prefetcher CPU i wektoryzacja pracowały optymalnie.

### Migracja na środowisko Rust (Headless/Backend):
- **Brak Danych Współdzielonych bez Kontroli**: Zrezygnowanie z globalnych stanów statycznych (jak `ms_pTextureSet`). Całość stanu terenu pożądanego współbieżnie powinna zostać zamodelowana w ramach Wzorca ECS (Entity-Component-System - np. za pośrednictwem crate'u `bevy_ecs` lub `legion`) bądź za pomocą systemowej instancji zabezpieczonej `Arc<RwLock<TextureSet>>`.
- **Zero-Copy Parsing**: Przeniesienie parsowania plików binarnych (`LoadAttrMap`, `LoadWaterMap`) na nowoczesne, w 100% bezpieczne systemy takie jak `zerocopy` lub `bytemuck`, które bez użycia ani jednego `unsafe` reinterpretują ciągi bajtów (`&[u8]`) z zachowaniem kontroli i walidacji `repr(C)`. Rozwiąże to problem endiannessu i paddingu struktur.
- **Odporność na Błędy**: Całkowite odrzucenie powrotów typu `bool` i cichego raportowania `TraceError`. Użycie `Result<Self, TerrainError>` wymuszającego obsłużenie każdego negatywnego scenariusza (np. `TerrainError::BufferTruncation`, `TerrainError::InvalidMagicNumber`). Zapewni to absolutny determinizm maszyny stanów po stronie serwera headless i odporność infrastruktury sieciowej na pakiety fuzzingu. 
- **Wydajność Algorytmiczna (SIMD)**: Skorzystanie z crate `std::simd` w modułach intensywnie przeliczających współrzędne i atrybuty mapy w celu wyciskania maksymalnej wydajności procesorów współczesnej architektury w operacjach na wielu danych w jednym takcie.
