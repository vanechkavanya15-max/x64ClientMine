# Lane Report: S2.3 Device Reset Unification [T059]

## 1. Cel zadania
Zunifikowanie trzech rozproszonych implementacji procedury resetowania urzadzenia Direct3D 9 w jedna glowna metode `CGraphicDevice::ResetWithParams(D3DPRESENT_PARAMETERS& d3dpp)`. Eliminacja zbednego zwalniania deklaracji wierzcholkow w `OnLostDevice` backendu D3D9.

## 2. Zmiany w kodzie
1. **`src/EterLib/GrpDevice.h` & `GrpDevice.cpp`**:
   - Dodano metode `static bool CGraphicDevice::ResetWithParams(D3DPRESENT_PARAMETERS& d3dpp)`.
   - `ResetWithParams` zwalnia zasoby `D3DPOOL_DEFAULT` (domyslne listy IB i VB), wykonuje jedyne wywolanie `ms_lpd3dDevice->Reset(&d3dpp)`, aktualizuje `ms_d3dPresentParameter`, odtwarza zasoby `D3DPOOL_DEFAULT` i wywoluje `STATEMANAGER.SetDefaultState()`.
   - `CGraphicDevice::Reset()` deleguje bezposrednio do `ResetWithParams(ms_d3dPresentParameter)`.
2. **`src/EterLib/GrpScreen.cpp`**:
   - `CScreen::RestoreDevice()` zamiast recznego powielania logiki czyszczenia buforow i wywolywania `Reset()` deleguje bezposrednio do `CGraphicDevice::ResetWithParams(rkD3DPP)`.
3. **`src/EterLib/D3D9RenderBackend.cpp`**:
   - Usunieto niepotrzebne zwalnianie i ponowne tworzenie deklaracji wierzcholkow (`ReleaseVertexDeclarations` w `OnLostDevice` oraz `InitVertexDeclarations` w `OnResetDevice`), poniewaz deklaracje wierzcholkow D3D9 przetrwaja reset urzadzenia.
4. **Weryfikacja**:
   - `git grep -n -e "->Reset(" -- src/EterLib` zwraca dokladnie 1 linie dla urzadzenia graficznego (`GrpDevice.cpp:210`).
   - Kompilacja biblioteki `EterLib`: 100% sukces.
