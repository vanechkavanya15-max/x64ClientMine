# Lane Report: S3.5 Sprzatanie CRenderQueueManager, Odpiecie od D3D9 i Wersja ABI [T075]

## 1. Cel zadania
Sprzatanie i refaktoryzacja `CRenderQueueManager` w celu calkowitego odpiecia od bezposrednich zaleznosci od Direct3D9 (`D3D9RenderBackend.h`, `StateManager.h`) oraz `SpeedTreeLib` (`SpeedTreeWrapper.h`), usuniecie martwych i duplikowanych metod, wprowadzenie licznikow porzuconych pakietow przy przepelnieniu buforow instancji czasteczek i drzew, a takze zdefiniowanie scislej wersji ABI renderera (`M2_RENDER_ABI_VERSION = 1`) wraz z funkcja mapujaca rozmiary struktur pakietowych.

## 2. Zmiany w kodzie
1. **`src/EterLib/RenderQueueManager.h` oraz `RenderQueueManager.cpp`**:
   - Usunieto martwe metody:
     - `Clear()` (pelna kopia `BeginFrame()`, zastapiono domyslnym destruktorem `= default`).
     - `GetDungeonBlockPackets()` (zbedny duplikat istniejacej metody `GetDungeonPackets()`).
     - `FlushSpeedTree()` (0 wywolan w kodzie).
     - `ClearParticleData()` oraz `ClearSpeedTreeInstanceData()` (martwe, nieuzywane poza klasa).
   - Usunieto naglowki `#include "SpeedTreeLib/SpeedTreeWrapper.h"`, `#include "D3D9RenderBackend.h"` oraz `#include "StateManager.h"`.
   - Zrealizowano `CaptureReferenceScreenshot` wylacznie poprzez delegacje do wirtualnej metody aktywnego backendu `IRenderBackend::CaptureFrame`.
   - Zaimplementowano liczniki porzuconych pakietow/instancji `m_droppedParticles` oraz `m_droppedSpeedTree` inkrementowane przy probie przepelnienia puli instancji (`m_particleDataPool`, `m_speedTreeInstancePool`) wraz z logowaniem diagnostycznym przez `TraceError`.
   - Dodano metody pomocnicze: `GetDroppedParticles()`, `GetDroppedSpeedTree()`, `ResetDroppedCounters()` oraz reset licznikow w `BeginFrame()`.
   - Zdefiniowano stala `constexpr uint32_t M2_RENDER_ABI_VERSION = 1;` oraz metody `CRenderQueueManager::GetPacketSizes()` / `CRenderQueueManager::GetPacketSizeMap()` (a takze wolne funkcje pomocnicze `GetRenderPacketSizes()` / `GetRenderPacketSizeMap()`) zwracajace tablice i mape `sizeof` dla wszystkich 13 typow pakietow renderera.
2. **`src/EterLib/IRenderBackend.h`**:
   - Zadeklarowano czysto wirtualna metode `virtual bool CaptureFrame(const char* szFilename) = 0;`.
3. **`src/EterLib/D3D9RenderBackend.h` oraz `D3D9RenderBackend.cpp`**:
   - Przeniesiono implementacje zrzutu ekranu D3D9 do `CD3D9RenderBackend::CaptureFrame(const char* szFilename)`.
4. **`src/EterLib/WgpuRenderBackend.h` oraz `WgpuRenderBackend.cpp`**:
   - Zaimplementowano `CWgpuRenderBackend::CaptureFrame(const char* szFilename)` delegujace do `CD3D9RenderBackend::Instance().CaptureFrame(szFilename)`.
5. **`tests/test_c26_render_queue_manager.cpp` oraz `tests/CMakeLists.txt`**:
   - Utworzono zestaw testow jednostkowych w doctest:
     - `RenderQueueManager - Submit and Flush on Mock IRenderBackend`: weryfikacja submitowania wszystkich kategorii pakietow i ich prawidlowego przekazania do atrapy backendu podczas `Flush`, oraz weryfikacja delegacji `CaptureReferenceScreenshot` do aktywnego backendu.
     - `RenderQueueManager - Dropped Packet Counting on Pool Overflow`: weryfikacja zliczania porzuconych instancji przy przepelnieniu buforow czasteczek i drzew oraz resetowania licznikow.
     - `RenderQueueManager - ABI Version and Packet Sizes`: weryfikacja `M2_RENDER_ABI_VERSION == 1` oraz rozmiarow 13 struktur pakietowych (zgodnych z `packets.rs`).
   - Zarejestrowano cel `test_c26_render_queue_manager` powiazany z `EterLib`, `EterBase`, `doctest` oraz test ctest `RenderQueueManagerTest`.

## 3. Weryfikacja
1. **Brak zaleznosci D3D9 w RenderQueueManager.cpp**:
   - `git grep -n -E "D3D9RenderBackend.h|StateManager.h" -- src/EterLib/RenderQueueManager.cpp` zwrocilo dokladnie 0 linii.
2. **Kompilacja anty-lag**:
   - Zbudowano wylacznie cele `EterLib` oraz `test_c26_render_queue_manager` (bez kompilacji UserInterface).
3. **Wyniki testow**:
   - `ctest --test-dir build -C Release -R RenderQueueManagerTest --output-on-failure`: 100% PASS (0.01 s).
   - Wszystkie 3 test cases oraz 90 asercji zakonczone statusem SUCCESS.
