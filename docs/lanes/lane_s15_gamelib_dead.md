# Raport S1.5 (T056) - Usuniecie martwego kodu GameLib, SpeedTreeLib i UserInterface

## Zakres prac i zrealizowane zadania (W4-06, W5-01, F-NEW-052, F-NEW-055)

W ramach zadania S1.5 (T056) przeprowadzono czyszczenie zdezaktualizowanego i martwego kodu w bibliotekach `GameLib`, `SpeedTreeLib`, `EffectLib`, `EterPythonLib` oraz warstwie `UserInterface`.

---

### 1. GameLib
- **src/GameLib/MapOutdoorWater.cpp** oraz **src/GameLib/MapOutdoor.h**:
  - Usunieto metody i deklaracje: `SubmitWaterPacket(long, float, float)` oraz `SubmitWaterPackets()`.
- **src/GameLib/MapOutdoorRender.cpp** oraz **src/GameLib/MapOutdoor.h**:
  - Usunieto martwa zmienna i bramke warunkowa `m_isDisableSortRendering` (~linie 457-500), pozostawiajac bezposrednie wywolanie zoptymalizowanego sortowania obiektow nieprzezroczystych (`CollectRenderingObject`).
  - Usunieto nieuzywana metode `RenderDungeon()` z `CMapOutdoor` oraz jej deklaracje w `MapOutdoor.h`.
- **src/GameLib/TerrainRenderExtractor.cpp** oraz **src/GameLib/TerrainRenderExtractor.h**:
  - Usunieto nieuzywana metode `Initialize(size_t maxSectors)` oraz pole skladowe `std::vector<uint32_t> m_splatWeightBuffer`.
- **src/GameLib/** (martwe fasady terenu):
  - Usunieto pliki:
    - `src/GameLib/RaymarchTerrainQuery.h`
    - `src/GameLib/TerrainSlopeValidator.h`
    - `src/GameLib/WaterHazardMap.h`
    - `src/GameLib/BoxAreaQuery.h`
    - `src/GameLib/CircleAreaQuery.h`

---

### 2. SpeedTreeLib
- **src/SpeedTreeLib/SpeedTreeForestDirectX.cpp**:
  - Usunieto martwe bramki warunkowe `Forest_RenderToShadow` oraz `Forest_RenderToMiniMap` przy wywolaniach `SubmitTreePackets` i `UpdateCompundMatrix`.
  - Usunieto martwa petle `while (itor != m_pMainTreeMap.end()) (itor++)->second->EndLeafForTreeType();`.
- **src/SpeedTreeLib/Constants.h**:
  - Usunieto zbedne makro `USE_SPEEDGRASS`.
- **Pliki usuniete z SpeedTreeLib**:
  - `src/SpeedTreeLib/BoundaryShapeManager.cpp`
  - `src/SpeedTreeLib/BoundaryShapeManager.h`
  - `src/SpeedTreeLib/SpeedGrassRT.cpp`
  - `src/SpeedTreeLib/SpeedGrassRT.h`
  - `src/SpeedTreeLib/SpeedGrassWrapper.cpp`
  - `src/SpeedTreeLib/SpeedGrassWrapper.h`

---

### 3. UserInterface & Client
- **src/UserInterface/PythonApplication.h** oraz **src/UserInterface/PythonApplicationLogo.cpp**:
  - Usunieto przestarzale pola skladowe DirectShow (`m_pGraphBuilder`, `m_pFilterSG`, `m_pSampleGrabber`, `m_pMediaCtrl`, `m_pMediaEvent`, `m_pVideoWnd`, `m_pBasicVideo`, `m_pCaptureBuffer`, `m_lBufferSize`, `m_pLogoTex`, `m_bLogoError`, `m_bLogoPlay`, `m_nLeft, m_nRight, m_nTop, m_nBottom`).
  - Zastapiono ciala metod `OnLogoOpen`, `OnLogoUpdate`, `OnLogoRender`, `OnLogoClose` bezpiecznymi pustymi no-op stubami, zachowujac zgodnosc z bindingami Pythona.
- **src/UserInterface/PythonBackgroundModule.cpp**:
  - Przeksztalcono `backgroundRenderDungeon` w bezpieczny pusty no-op stub zwracajacy `Py_BuildNone()`.
- **src/UserInterface/PythonTextTail.h** oraz **src/UserInterface/PythonTextTail.cpp**:
  - Usunieto `#include "TextTail/TextTailService.h"` oraz deklaracje i wywolania `TextTailService::Instance().UpdateScreenPositions(...)` i `SetTailPosition(...)`.
  - Usunieto deklaracje `GetService()` i forward declaration `ITextTailService`.
- **src/UserInterface/TextTail/TextTail_ClearAll.cpp**:
  - Usunieto nieuzywany naglowek `#include "ITextTailService.h"`.
- **src/Client/Actor/ActorDespawnCleaner.h** oraz **src/Client/Actor/ActorDespawnCleaner.cpp**:
  - Usunieto zaleznosci od `ITargetHpBarService`, pole `m_targetHpBarService`, metode `CleanTargetHpBar` oraz domene `CleanDomain::TargetHpBar`.
- **Pliki usuniete z UserInterface/TextTail**:
  - `src/UserInterface/TextTail/ITargetHpBarService.h`
  - `src/UserInterface/TextTail/ITextTailService.h`
  - `src/UserInterface/TextTail/TextTailService.h`
  - `src/UserInterface/TextTail/TargetHpBar_Cache.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_Colors.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_Events.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_Hide.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_MiniMob.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_NameAlign.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_ShieldOverlay.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_Show.cpp`
  - `src/UserInterface/TextTail/TargetHpBar_Update.cpp`
  - `src/UserInterface/TextTail/TextTail_AlphaFade.cpp`
  - `src/UserInterface/TextTail/TextTail_BatchDraw.cpp`
  - `src/UserInterface/TextTail/TextTail_ClampScreen.cpp`
  - `src/UserInterface/TextTail/TextTail_FontCache.cpp`
  - `src/UserInterface/TextTail/TextTail_OcclusionCull.cpp`
  - `src/UserInterface/TextTail/TextTail_RegisterActor.cpp`
  - `src/UserInterface/TextTail/TextTail_RegisterItem.cpp`
  - `src/UserInterface/TextTail/TextTail_Remove.cpp`
  - `src/UserInterface/TextTail/TextTail_ScreenProjection.cpp`

---

### 4. EffectLib
- **src/EffectLib/ParticleSystemInstance.h**:
  - Usunieto martwy szablon metody `ForEachParticleRendering`.

---

### 5. EterPythonLib
- **src/EterPythonLib/PythonGraphic.h** oraz **src/EterPythonLib/PythonGraphic.cpp**:
  - Usunieto nieuzywane metody `RenderImage` oraz `RenderAlphaImage`.

---

## Weryfikacja

1. **Weryfikacja wystapien martwych symboli**:
   - Wywolanie `git grep -n -E "SubmitWaterPacket|TextTailService|ITargetHpBarService|SpeedGrass|BoundaryShapeManager" -- "*.cpp" "*.h" "*.c" "*.hpp"` oraz w katalogu `tests/` zwrocilo dokladnie 0 linii.

2. **Kompilacja bibliotek (Zasada Anty-Lag)**:
   - Wykonano: `cmake --build build --config Release --target GameLib SpeedTreeLib EffectLib EterPythonLib -- /m:1`
   - Rezultat: Pomyslna kompilacja wszystkich 4 bibliotek (`GameLib.lib`, `SpeedTreeLib.lib`, `EffectLib.lib`, `EterPythonLib.lib`) bez zadnych bledow.
   - Cel monolitu `UserInterface` nie byl kompilowany w worktree.
