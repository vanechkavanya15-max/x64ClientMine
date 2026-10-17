# Lane Report: S3.1 Render Ownership Table & Cached Config [T071]

## 1. Cel zadania
Wprowadzenie scentralizowanej tabeli wlasnosci renderowania `RenderOwnership` przydzielajacej wlasciciela wykonawczego (`Legacy`, `D3D9Backend`, `Wgpu`) dla kazdej kategorii tresci renderera (`ERenderCategory`), eliminacja wielokrotnego odczytu `config.json` z dysku co klatke oraz zastepstwo rozproszonego parsowania.

## 2. Zmiany w kodzie
1. **`src/EterLib/RenderOwnership.h` / `RenderOwnership.cpp`**:
   - Definicja `enum class ERenderCategory` (18 kategorii + `Count`).
   - Definicja `enum class ERenderOwner` (`Legacy`, `D3D9Backend`, `Wgpu`).
   - Klasa `RenderOwnership` z atomowa tablica stanow `s_owners`.
   - Jednorazowy odczyt `M2_GRAPHICS_ENGINE` oraz `config.json` z pamiecia podreczna (`IsWgpuConfigured()`).
   - Domyslna konfiguracja: `Effect` i `UI` -> `D3D9Backend`, pozostale -> `Legacy`.
2. **`src/EterLib/WgpuRenderBackend.cpp`**:
   - `CWgpuRenderBackend::IsConfiguredInConfigJson()` deleguje bezposrednio do `RenderOwnership::IsWgpuConfigured()`.
3. **`src/EterLib/TextureUploadBridge.cpp`**:
   - Uzycie `RenderOwnership::IsWgpuConfigured()`.
4. **`src/UserInterface/PythonApplication.cpp`**:
   - Zastapienie bezposrednich sprawdzen `config.json` wywolaniem `RenderOwnership::IsWgpuConfigured()`.
5. **`tests/test_c26_render_ownership.cpp` & `tests/CMakeLists.txt`**:
   - 4 test cases weryfikujace domyslne wlasnosci, mutacje `SetOwner`, kontrakt `LegacyDraws` oraz out-of-bounds safety.

## 3. Weryfikacja
- Kompilacja celu `test_c26_render_ownership` z biblioteka `EterLib`: 100% PASS (0 bledow).
- `ctest -R RenderOwnershipTest`: 100% PASS (0.01 s).
