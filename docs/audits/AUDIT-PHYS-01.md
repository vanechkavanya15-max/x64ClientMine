# AUDIT-PHYS-01: Terrain Chunk Streaming & Cells
## Raport Przeglądu Jakości Kodu (Clean Code Review)

### 1. Podsumowanie Architektury
Obszar badany obejmuje moduł zarządzania terenem:
- **`TerrainSubsystemFacade`**: Główny punkt wejścia do systemu, spina storage i streaming.
- **`ChunkStreamingCoordinator`**: Rozstrzyga, które sektory załadować w oparciu o pozycję gracza.
- **`TerrainHeightStorage`**: Przechowuje właściwe dane wysokościowe i operuje na cache pod spodem.
- **`LinearSectorQuadtree`**: Oblicza widoczne łatki terenu z Culling'iem na frustum.
- **`TerrainHeightPrefetcher`**: Wcześniej ładuje wysokość w celu uniknięcia lagów w silniku kolizji.

Ogólnie, pomimo nowoczesnych mechanizmów (C++20/C++23 std::expected, std::span), zaobserwowano braki w zarządzaniu zasobami na poziomie rygorystycznego Rust-a, które generują wycieki, wyścigi danych oraz znaczący narzut w pętli gry.

### 2. Zidentyfikowane Defekty (Defects & Inconsistencies)

#### 2.1. Zarządzanie zasobami i cykl życia (RAII)
- **Plik**: `src/GameLib/Terrain/ChunkStreamingCoordinator.cpp` (linia 46)
  - **Opis**: Używany jest surowy wskaźnik `m_heightStorage` w celach modyfikacji stanu (np. usunięcie chunków). Mimo, że koordynator go nie niszczy, brak jest jakiejkolwiek walidacji w czasie życia.
  - **Wpływ**: Wiszące referencje, jeśli Facade zostanie zniszczona inaczej, błędy segmentacji. Wzorzec Rust: `Arc<RwLock<T>>` / `Weak`.
  
- **Plik**: `src/GameLib/Terrain/TerrainHeightStorage.cpp` (BatchSampleHeight)
  - **Opis**: Wywołanie `BatchSampleHeight(const float* x, const float* y, float* outZ, size_t count)` używa surowych wskaźników bez określania granic po stronie C++.
  - **Wpływ**: Out of bounds, buffer overruns. Niebezpieczny widok pamięci.
  
#### 2.2. Poprawność współbieżności i synchronizacja (Thread Safety & Synchronization)
- **Plik**: `src/GameLib/Terrain/TerrainSubsystemFacade.cpp`
  - **Opis**: Funkcja `LoadSectorData` zapisuje obiekty do mapy `m_sectorTrees[coord]` pod wywołaniami zdarzeń, bez osłony thread-safe (lock / mutex).
  - **Wpływ**: Silny data race jeśli wątek sieciowy wywoła `LoadSectorData`, a `CullSectorPatches` z wątku renderera iteruje po mapie. W Rust zostałoby to odrzucone przez Borrow Checker bez `RwLock<HashMap>`.
  
- **Plik**: `src/GameLib/Terrain/ChunkStreamingCoordinator.cpp` (UpdateStreaming)
  - **Opis**: Callbacks typu `m_evictionCallback(coord);` (które kasują wpisy z `m_sectorTrees`!) i publikacja eventów następują poza Mutexem `m_mutex`. To dobrze pod kątem Deadlocków, ale w `TerrainSubsystemFacade` brakuje synchronizacji odbierającego te wywołania!
  - **Wpływ**: Niewidzialne wyścigi pomiędzy wątkiem uaktualniającym (Streamer) a renderującym strukturę `m_sectorTrees`.
  
#### 2.3. Odporność parsera na fragmentację danych (Protocol Stream Robustness)
- Brak wyraźnych usterek w logice streamu protokołu sieciowego w kodzie fizyki, niemniej, brakuje sprawdzania w `ChunkHeightData::SampleHeightTriangular`, czy przesłany bufor ma przynajmniej żądany rozmiar z `rawY * TerrainMetrics::HeightmapSize + rawX`.

#### 2.4. Wydajność na ścieżce krytycznej (Performance Optimization)
- **Plik**: `src/GameLib/Terrain/TerrainHeight_Prefetch.cpp` (linia 46)
  - **Opis**: Zdecydowanie nieoptymalne wykorzystanie `std::vector<float> prefetchedZ(count);`. Alokacja na stercie w pętli obsługi entity.
  - **Wpływ**: Setki alokacji na klatkę. Może wywołać spadki fps z powodu fragmentacji sterty. Należy zastosować std::array z ograniczoną pulą lub stertę tymczasową po stronie silnika uderzając bezpośrednio przez `std::span`.

- **Plik**: `src/GameLib/Terrain/ChunkStreamingCoordinator.cpp` (GetPendingLoadQueue)
  - **Opis**: Używa dwóch alokacji `std::vector` (jeden par `pending`, drugi `result`) dla prostego sortowania i zwrotu.
  - **Wpływ**: Narzut alloc-copy-free.


### 3. Wzorcowa propozycja refaktoryzacji (C++23 / Rust)

1. **Przejście na zwinne widoki (Bounds-checked)**:
   - Zmień `void BatchSampleHeight(const float* x, const float* y, float* outZ, size_t count)` 
     na `void BatchSampleHeight(std::span<const float> x, std::span<const float> y, std::span<float> outZ)`.
   - Zmień w Rust na `fn batch_sample_height(&self, x: &[f32], y: &[f32], out_z: &mut [f32])`.
2. **Korekta wyścigów w `TerrainSubsystemFacade`**:
   - `std::unordered_map<SectorCoord, LinearSectorQuadtree>` wymaga `std::shared_mutex` lub zastosowania biblioteki do concurrent hash map (np. folly/concurrent_hash_map, lub `DashMap` w Rust).
3. **Eliminacja alokacji z Prefetchingu**:
   - Zamiast `std::vector`, użyj tymczasowego wskaźnika wektora buforowanego per-thread, albo bufora przydzielanego na stosie za pomocą makra `alloca` (dla małych pakietów `count < 1024`).
4. **Ownership Rules**:
   - Skoro Facade niszczy się jako pierwsza, koordynator nie powinien pojęcia o surowym wskaźniku `m_heightStorage`. Należy przekazywać komendę (CQRS) w górę, lub dostarczać referencję pod czas update: `void UpdateStreaming(TerrainHeightStorage& storage, float playerX, float playerY)`.
