# Raport z wykonania zadania S1.4 (T055) - Usuniecie martwych elementow modeli bez wolajacych

## Cel zadania
Usuniecie martwych elementow modeli i buforow bez wolajacych w kodzie klienta (B3-T01a, F-NEW-031, F-NEW-024):
1. Usuniecie nieuzywanej prealokowanej puli 140 buforow wierzcholkow deformacji `gs_vbs` (~6.7 MB VRAM), funkcji `GrannyCreateSharedDeformBuffer` oraz `GrannyDestroySharedDeformBuffer` w `src/EterGrnLib/LODController.cpp` oraz uproszczenie alokacji/dealokacji `__AllocDeformVertexBuffer` i `__FreeDeformVertexBuffer`.
2. Usuniecie martwej metody `CGrannyLODController::UpdateSkeleton` z `src/EterGrnLib/LODController.cpp` i `src/EterGrnLib/LODController.h`.
3. Usuniecie zewnetrznych deklaracji i wywolan `GrannyCreateSharedDeformBuffer` i `GrannyDestroySharedDeformBuffer` z `src/UserInterface/PythonApplication.cpp`.
4. Usuniecie martwej metody `CGrannyModelInstance::FillBonePalette` z `src/EterGrnLib/ModelInstance.cpp` oraz jej deklaracji z `src/EterGrnLib/ModelInstance.h`.
5. Usuniecie martwej metody `CGrannyModel::LoadPNTVertices` z `src/EterGrnLib/Model.cpp` oraz jej deklaracji z `src/EterGrnLib/Model.h`.
6. Usuniecie nieuzywanych struktur `TFlyVertex`, `TFlyVertexSet`, `TFlyVertexSetVector` z `src/GameLib/FlyTrace.cpp`.
7. Usuniecie martwych metod `CGraphicThingInstance::RenderWithTwoTexture` oraz `CGraphicThingInstance::BlendRenderWithTwoTexture` z `src/EterGrnLib/ThingInstance.cpp` oraz ich deklaracji z `src/EterGrnLib/ThingInstance.h`, jak rowniez nieuzywanych funktorow `FRenderWithTwoTexture` i `FBlendRenderWithTwoTexture` z `src/EterGrnLib/LODController.h`.

---

## Szczegolowy opis usunietych linii i zmian

### 1. `src/EterGrnLib/LODController.cpp`
- Usunieto `enum { SHARED_VB_500 ... SHARED_VB_NUM = 9 }`.
- Usunieto tablice statyczna `static std::vector<CGraphicVertexBuffer*> gs_vbs[SHARED_VB_NUM]`.
- Usunieto funkcje `__ReserveSharedVertexBuffers`, `GrannyCreateSharedDeformBuffer` oraz `GrannyDestroySharedDeformBuffer`.
- Uproszczono `__AllocDeformVertexBuffer` i `__FreeDeformVertexBuffer` do bezposredniej alokacji/destrukcji pojedynczego bufora dynamicznego bez utrzymywania globalnej puli.
- Usunieto definicje metody `CGrannyLODController::UpdateSkeleton(const D3DXMATRIX * c_pWorldMatrix, float fElapsedTime)`.

### 2. `src/EterGrnLib/LODController.h`
- Usunieto deklaracje nieuzywanych funktorow pomocniczych `FRenderWithTwoTexture` oraz `FBlendRenderWithTwoTexture`.
- Usunieto deklaracje metody `void UpdateSkeleton(const D3DXMATRIX * c_pWorldMatrix, float fElapsedTime)`.

### 3. `src/UserInterface/PythonApplication.cpp`
- Usunieto deklaracje `extern void GrannyCreateSharedDeformBuffer();` oraz `extern void GrannyDestroySharedDeformBuffer();`.
- Usunieto wywolanie `GrannyCreateSharedDeformBuffer();` z metody `CPythonApplication::Create()`.
- Usunieto wywolanie `GrannyDestroySharedDeformBuffer();` z metody `CPythonApplication::Destroy()`.

### 4. `src/EterGrnLib/ModelInstance.cpp` i `src/EterGrnLib/ModelInstance.h`
- Usunieto definicje metody `CGrannyModelInstance::FillBonePalette(EterLib::Render::TActorBonePalette& outPalette) const` z `ModelInstance.cpp`.
- Usunieto deklaracje `bool FillBonePalette(EterLib::Render::TActorBonePalette& outPalette) const;` z `ModelInstance.h`.

### 5. `src/EterGrnLib/Model.cpp` i `src/EterGrnLib/Model.h`
- Usunieto definicje metody `CGrannyModel::LoadPNTVertices()` z `Model.cpp`.
- Usunieto deklaracje `bool LoadPNTVertices();` z `Model.h`.

### 6. `src/GameLib/FlyTrace.cpp`
- Usunieto definicje struktur `TFlyVertex`, `TFlyVertexSet` oraz typ `TFlyVertexSetVector`.

### 7. `src/EterGrnLib/ThingInstance.cpp` i `src/EterGrnLib/ThingInstance.h`
- Usunieto implementacje metod `CGraphicThingInstance::RenderWithTwoTexture()` oraz `CGraphicThingInstance::BlendRenderWithTwoTexture()` z `ThingInstance.cpp`.
- Usunieto deklaracje `void RenderWithTwoTexture();` oraz `void BlendRenderWithTwoTexture();` z `ThingInstance.h`.

---

## Wyniki weryfikacji

### Weryfikacja grep
Polecenie weryfikacyjne dla kodu zrodlowego:
```powershell
git grep -n -E "GrannyCreateSharedDeformBuffer|FillBonePalette|TFlyVertex" -- "src/*.cpp" "src/*.h"
# Wynik: 0 linii (kod powrotu 1)

git grep -n -E "GrannyDestroySharedDeformBuffer" -- "src/*.cpp" "src/*.h"
# Wynik: 0 linii (kod powrotu 1)

git grep -n -E "RenderWithTwoTexture|BlendRenderWithTwoTexture" -- "src/EterGrnLib/ThingInstance*"
# Wynik: 0 linii (kod powrotu 1)
```

Weryfikacja na calym drzewie `src` z wylaczeniem zewnetrznego indeksu analizy `src/graphify-out`:
```powershell
git grep -n -E "GrannyCreateSharedDeformBuffer|FillBonePalette|TFlyVertex" -- src ':!src/graphify-out'
# Wynik: 0 linii (kod powrotu 1)
```

### Weryfikacja kompilacji
Zgodnie z zelazna zasada ANTY-LAG (brak kompilacji celu UserInterface):
```powershell
cmake --build build --config Release --target EterGrnLib GameLib -- /m:1
```
Wynik: **Sukces** (kod wyjscia 0). Wszystkie zmienione biblioteki skompilowaly sie bez zadnych bledow.

Dodatkowo zbudowano i uruchomiono test powiazany `test_c26_actor_weapon_trace`:
```powershell
ctest --test-dir build -C Release -R "test_c26_actor_weapon_trace" --output-on-failure
# Wynik: 100% tests passed, 0 tests failed out of 1
```

---

## Podsumowanie zmian w plikach
```
 src/EterGrnLib/LODController.cpp        | 133 +-------------------------------
 src/EterGrnLib/LODController.h          |  19 -----
 src/EterGrnLib/Model.cpp                |  23 ------
 src/EterGrnLib/Model.h                  |   1 -
 src/EterGrnLib/ModelInstance.cpp        |   6 --
 src/EterGrnLib/ModelInstance.h          |   1 -
 src/EterGrnLib/ThingInstance.cpp        |  19 -----
 src/EterGrnLib/ThingInstance.h          |   2 -
 src/GameLib/FlyTrace.cpp                |  32 --------
 src/UserInterface/PythonApplication.cpp |   5 --
 10 files changed, 4 insertions(+), 237 deletions(-)
```
