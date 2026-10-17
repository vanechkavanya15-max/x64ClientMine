# Raport z realizacji S5.1 (T080): Snapshot klatki (Frame Snapshot)

## 1. Cel zadania
Zadanie S5.1 (T080) z planu `RENDERER_CPP_PREP_PLAN.md`:
"Frame snapshot of camera, projection and lights at start of RenderGame; no GetTransform reads after passes".

Cel techniczny:
Wprowadzenie centralnego snapshotu klatki (`CFrameSnapshot`), rejestrujacego stan kamery (macierze widoku, odwrotnego widoku, pozycje oka), macierze projekcji perspektywicznej, parametry mgly, swiatla oraz indeks klatki i czas globalny na samym poczatku potoku `RenderGame`. Calkowita eliminacja odpytywania sterownika i urzadzenia graficznego (`dev->GetTransform` / `STATEMANAGER.GetTransform`) po wykonaniu przebiegow renderowania w `PythonApplication.cpp` oraz `D3D9RenderBackend.cpp`.

## 2. Zrealizowane modyfikacje

### A. Modul EterLib: CFrameSnapshot (`src/EterLib/CFrameSnapshot.h`, `src/EterLib/CFrameSnapshot.cpp`)
- Zdefiniowano strukture `TFrameSnapshot`:
  * `D3DXMATRIX matView` - macierz widoku aktywnej kamery.
  * `D3DXMATRIX matProj` - macierz perspektywiczna wyznaczona z kata widzenia (FOV), proporcji ekranu (aspect) oraz plaszczyzn obcinania (near/far clip).
  * `D3DXMATRIX matViewProj` - precyzyjny iloczyn macierzy widoku i projekcji.
  * `D3DXMATRIX matInvView` - macierz odwrotna widoku z weryfikacja wyznacznika.
  * `D3DXVECTOR3 v3CameraPos` - pozycja oka kamery (`GetEye()`).
  * `D3DXVECTOR4 v4FogParams` - parametry liniowej mgly (near, far, 1/(far-near), enable).
  * `D3DLIGHT9 lights[3]` - zbuforowane zrodla swiatla (slonce, swiatlo dopelniajace, swiatlo punktowe/lokalne).
  * `float fTime` - czas globalny biezacej klatki.
  * `uint32_t dwFrameIndex` - monotoniczny licznik klatek.
- Zaimplementowano klase `CFrameSnapshot` z interfejsem:
  * `static void Capture(const CCamera& camera, float fFov, float fAspect, float fNear, float fFar, float fTime = 0.0f, uint32_t dwFrameIndex = 0, const D3DXVECTOR4* pFogParams = nullptr, const D3DLIGHT9* pLights = nullptr)`
  * `static void Capture(const CCamera* pCamera, float fFov, float fAspect, float fNear, float fFar, float fTime = 0.0f, uint32_t dwFrameIndex = 0, const D3DXVECTOR4* pFogParams = nullptr, const D3DLIGHT9* pLights = nullptr)` - wariant odporny na brak zainicjalizowanej kamery (nullptr fallback).
  * `static const TFrameSnapshot& Current()` - bezpieczny odczyt biezacego snapshotu.
  * `static void SetCurrent(const TFrameSnapshot& snapshot)` oraz `static void Reset()` - pomocnicze metody do testow jednostkowych oraz higieny stanu.

### B. UserInterface: PythonApplication (`src/UserInterface/PythonApplication.cpp`)
- Na poczatku metody `CPythonApplication::RenderGame()` wywolano `CFrameSnapshot::Capture` z biezaca kamera `CCameraManager::Instance().GetCurrentCamera()`, katem FOV 30.0f, proporcja ekranu, zasiegiem widzenia `fFarClip`, czasem `m_fGlobalTime` oraz licznikiem klatki `s_dwFrameIdx`.
- W sekcji aktualizacji uniformow klatki przed wywolaniem `CRenderQueueManager::Instance().Flush`:
  * Usunieto odczyty urzadzenia `STATEMANAGER.GetTransform(D3DTS_VIEW, &uniforms.matView)` oraz `STATEMANAGER.GetTransform(D3DTS_PROJECTION, &uniforms.matProj)`.
  * Zasilenie `uniforms` bezposrednio z `CFrameSnapshot::Current()`.

### C. Backend D3D9: D3D9RenderBackend (`src/EterLib/D3D9RenderBackend.cpp`)
- W metodzie `CD3D9RenderBackend::BeginFrame`:
  * Usunieto wywolania `STATEMANAGER.GetTransform` dla `D3DTS_VIEW` oraz `D3DTS_PROJECTION`.
  * Brakujace macierze w uniformach sa uzupelniane bezposrednio ze snapshotu klatki `CFrameSnapshot::Current()`.
- W metodzie `CD3D9RenderBackend::RenderActors`:
  * Usunieto zbedne odczyty i zapisy macierzy swiata (`STATEMANAGER.GetTransform(D3DTS_WORLD, &matSavedWorld)` / `SetTransform`), ktore nie byly modyfikowane w petli shaderow kosci.
- W metodzie `CD3D9RenderBackend::RenderEffects`:
  * Zastapiono `STATEMANAGER.GetTransform(D3DTS_VIEW, &matView)` pobraniem macierzy ze snapshotu klatki lub z uniformow.
- W metodzie `CD3D9RenderBackend::RenderDungeonBlocks`:
  * Zastapiono odczyt `STATEMANAGER.GetTransform(D3DTS_WORLD, &matSavedWorld)` standardowym mechanizmem stosu stanow `STATEMANAGER.SaveTransform(D3DTS_WORLD, &matIdentity)` oraz `STATEMANAGER.RestoreTransform(D3DTS_WORLD)`.

### D. Testy jednostkowe (`tests/test_c26_frame_snapshot.cpp`, `tests/CMakeLists.txt`)
- Utworzono zestaw testow oparty o doctest:
  1. `TFrameSnapshot - Default Initialization`: weryfikacja domyslnych macierzy jednostkowych, zerowania pozycji, czasu i swiatel.
  2. `CFrameSnapshot::Capture - Camera view, inverse, and perspective projection`: weryfikacja zgodnosci ze stanem CCamera, poprawnosci mnozenia macierzy widoku i odwrotnosci (wynik to macierz jednostkowa) oraz projekcji perspektywicznej.
  3. `CFrameSnapshot::Capture - Custom Fog and Lights Injection`: weryfikacja wstrzykiwania niestandardowych swiatel i parametrow mgly.
  4. `CFrameSnapshot::Capture - Robustness with Null Camera pointer`: odpornosc na przekazanie wskaznika nullptr na kamere.
  5. `CFrameSnapshot - SetCurrent and Reset`: sprawdzanie pelnego resetowania i podmiany snapshotu.
- Zarejestrowano cel `test_c26_frame_snapshot` w `tests/CMakeLists.txt` oraz dodano test do CTest (`FrameSnapshotTest`).

## 3. Weryfikacja i akceptacja

1. **Weryfikacja braku GetTransform w zmodyfikowanych plikach:**
   ```powershell
   git grep -n "GetTransform" -- src/UserInterface/PythonApplication.cpp src/EterLib/D3D9RenderBackend.cpp
   ```
   Wynik: **0 linii** (kod zakonczenia 1, brak jakichkolwiek dopasowan).

2. **Kompilacja wylacznie celu testowego (ZAKAZ kompilacji UserInterface):**
   ```powershell
   cmake --build build --config Release --target test_c26_frame_snapshot -- /m:1
   ```
   Wynik: Kompilacja i konsolidacja zakonczona sukcesem (kod wyjscia 0).

3. **Uruchomienie testow jednostkowych:**
   ```powershell
   .\build\tests\Release\test_c26_frame_snapshot.exe
   ```
   Wynik:
   ```
   [doctest] test cases:  5 |  5 passed | 0 failed | 0 skipped
   [doctest] assertions: 58 | 58 passed | 0 failed |
   [doctest] Status: SUCCESS!
   ```

4. **Weryfikacja CTest:**
   ```powershell
   ctest --test-dir build -C Release -R FrameSnapshotTest
   ```
   Wynik: 100% tests passed, 0 tests failed out of 1.
