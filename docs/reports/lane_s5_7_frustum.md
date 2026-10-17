# Raport z realizacji S5.7 (W4-08): Jeden modul frustum

## 1. Cel zadania
Zadanie S5.7 (W4-08) z planu `RENDERER_CPP_PREP_PLAN.md`:
Ujednolicenie modulu Frustum (`SphereLib/frustum.*`) jako jedynego zrodla ekstrakcji plaszczyzn bryly widzenia (View Frustum) ze zunifikowanej macierzy widoku i projekcji (`viewProj`), obsluga konwencji negacji osi Y dla terenu Metin2 oraz eliminacja duplikatow ekstrakcji i wywolan `D3DXPlaneNormalize` z `GameLib` oraz `SpeedTreeLib`.

## 2. Zrealizowane modyfikacje

### A. Modul SphereLib (`src/SphereLib/frustum.h`, `src/SphereLib/frustum.cpp`)
- Wprowadzono wyliczenie `EFrustumPlane` dla jednoznacznej identyfikacji plaszczyzn:
  - `FRUSTUM_PLANE_NEAR` (0)
  - `FRUSTUM_PLANE_FAR` (1)
  - `FRUSTUM_PLANE_LEFT` (2)
  - `FRUSTUM_PLANE_RIGHT` (3)
  - `FRUSTUM_PLANE_BOTTOM` (4)
  - `FRUSTUM_PLANE_TOP` (5)
- Dodano statyczna metode `Frustum::ExtractPlanes(const D3DXMATRIX & mat, D3DXPLANE * pPlanesOut, bool bNegatePlaneY = false)`:
  - Ekstrahuje 6 plaszczyzn bryly widzenia zgodnie z konwencja wierszowa DirectX dla macierzy widoku i projekcji.
  - Dokonuje normalizacji plaszczyzn z zabezpieczeniem przed dzieleniem przez zero (`NormalizePlaneUnified` uzywajace `D3DXPlaneNormalize`).
  - Umozliwia opcjonalna negacje skladowej Y plaszczyzny (`bNegatePlaneY`).
- Zaktualizowano `BuildViewFrustum` oraz `BuildViewFrustum2` w celu wykorzystania wspolnej metody `ExtractPlanes`.
- Dodano obsluge konwencji terenu Metin2 (negacja osi Y punktu testowego):
  - Rozszerzono metody `ViewVolumeTest`, `IsBoxVisible`, `ViewVolumeTestAABB`, `IsSphereVisible` o flage `bool bNegateY = false`.
  - Dodano dedykowana metode pomocnicza `ViewVolumeTestTerrain(const Vector3d &c_v3Center, const float c_fRadius, const float c_fMargin = 0.0f)`.
- Zdefiniowano alias typu `using CFrustum = Frustum;`.

### B. MapOutdoor (`src/GameLib/MapOutdoor.h`, `src/GameLib/MapOutdoor.cpp`, `src/GameLib/MapOutdoorRender.cpp`)
- W `MapOutdoor.h`: dodano instancje `Frustum m_Frustum` oraz zaktualizowano sygnature `BuildViewFrustum(const D3DXMATRIX & mat)`.
- W `MapOutdoor.cpp`: usunieto reczna ekstrakcje 6 plaszczyzn oraz petle z wywolaniami `D3DXPlaneNormalize`. Zastapiono je wywolaniem `m_Frustum.BuildViewFrustum(mat)` i synchronizacja z `m_plane`.
- W `MapOutdoorRender.cpp`: w metodzie `__RenderTerrain_RecurseRenderQuadTree_CheckBoundingCircle` usunieto reczna petle iloczynow skalarnych `D3DXPlaneDotCoord` i reczna negacje `center.y = -center.y`. Zastapiono je bezposrednim wywolaniem `m_Frustum.ViewVolumeTestTerrain(c_v3Center, c_fRadius, 0.0f)`.

### C. SpeedTreeLib (`src/SpeedTreeLib/SpeedTreeWrapper.cpp`)
- Dolaczono naglowek `SphereLib/frustum.h`.
- W metodzie `CSpeedTreeWrapper::TestFrustumCulling()` usunieto cala zduplikowana reczna ekstrakcje plaszczyzn ze snapshotu `matViewProj` oraz reczne wywolania `DirectX::XMVectorDivide(..., len)`.
- Zastapiono powyzsze operacje pojedynczym wywolaniem `CFrustum::ExtractPlanes(matViewProj, planes)`.

### D. Test jednostkowy (`tests/test_c26_frustum.cpp`, `tests/CMakeLists.txt`)
- Utworzono test jednostkowy `test_c26_frustum.cpp` oparty na frameworku `doctest`:
  1. `Frustum - ExtractPlanes from known perspective projection matrix`: sprawdzenie poprawnosci ekstrakcji 6 plaszczyzn (Near, Far, Left, Right, Bottom, Top) ze znanej macierzy rzutowania perspektywicznego, weryfikacja dlugosci normalnych, orientacji i dystansow dla punktow referencyjnych.
  2. `Frustum - BuildViewFrustum and ViewVolumeTest for spheres`: weryfikacja klasyfikacji sfer (INSIDE, PARTIAL, OUTSIDE) wzgledem granic bryly widzenia.
  3. `Frustum - IsBoxVisible and ViewVolumeTestAABB`: weryfikacja przeciecia i zawierania bryl AABB.
  4. `Frustum - Metin2 Terrain Convention (Y Negation)`: weryfikacja zgodnosci parametru negacji osi Y (`bNegateY` oraz `bNegatePlaneY`) z konwencja ujemnych wspolrzednych terenu Metin2.
  5. `Frustum - Zero Matrix Robustness`: weryfikacja odpornosci na macierze zdegenerowane i brak dzielenia przez zero (brak wartosci NaN).
- Zarejestrowano cel testowy `test_c26_frustum` oraz test CTest `FrustumTest` w `tests/CMakeLists.txt`.

## 3. Weryfikacja i akceptacja

1. **Weryfikacja braku duplikatow `D3DXPlaneNormalize`:**
   - Wykonanie `git grep "D3DXPlaneNormalize" src/GameLib src/SpeedTreeLib`:
     Wynik: 0 linii.
   - Wykonanie `git grep -n "D3DXPlaneNormalize" -- src/`:
     Wynik: dokladnie 1 linia w calym drzewie `src/`: `src/SphereLib/frustum.cpp:34`.

2. **Kompilacja celu testowego (zgodnie z regula ZAKAZ KOMPILACJI UserInterface):**
   - Komenda: `cmake --build build --config Release --target test_c26_frustum -- /m:1`
   - Wynik: kompilacja zakonczona sukcesem (kod wyjscia 0).

3. **Wykonanie testu jednostkowego:**
   - Komenda: `.\build\tests\Release\test_c26_frustum.exe`
   - Wynik:
     ```
     [doctest] test cases:   5 |   5 passed | 0 failed | 0 skipped
     [doctest] assertions: 110 | 110 passed | 0 failed |
     [doctest] Status: SUCCESS!
     ```
   - CTest: `ctest --test-dir build -C Release -R FrustumTest` -> 100% tests passed.
