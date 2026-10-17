# Lane Report: S3.2 LegacyDraws Gates u Wszystkich Zywych Producentow Renderera [T072]

## 1. Cel zadania
Wdrozenie bramek `RenderOwnership::LegacyDraws(ERenderCategory::<Kategoria>)` u wszystkich 12 zywych producentow bezposrednich wywolan rysowania (draw calls) w celu uniemozliwienia podwojnego renderowania obiektow i umozliwienia plynnego przelaczania wlasnosci renderera zgodnie ze wzorcem Strangler Fig (N0.2 pkt 4, W4-10, W5-16, F-P0-01..F-P0-06).

Dodatkowo weryfikacja i zabezpieczenie backendu `CD3D9RenderBackend`, aby wykonywal renderowanie z pakietow kolejki wylacznie wtedy, gdy wlascicielem danej kategorii jest `ERenderOwner::D3D9Backend`.

## 2. Zmiany w kodzie producentow (bramki LegacyDraws)
1. **SkyBox (`ERenderCategory::SkyBox`)**:
   - `src/EterLib/SkyBox.cpp`:
     - Oplecenie wywolania `STATEMANAGER.DrawPrimitive` w `CSkyObjectQuad::Render()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::SkyBox)`.
     - Oplecenie bezposredniego bloku rysowania w Direct3D 9 w `CSkyBox::Render()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::SkyBox)`.
     - Dodanie wczesnego wyjscia w `CSkyBox::RenderCloud()` jesli `!RenderOwnership::LegacyDraws(ERenderCategory::SkyBox)`.
2. **Teren (`ERenderCategory::Terrain`)**:
   - `src/GameLib/MapOutdoorRenderHTP.cpp`:
     - Bramka `!RenderOwnership::LegacyDraws(ERenderCategory::Terrain)` w `CMapOutdoor::__RenderTerrain_RenderHardwareTransformPatch()`.
     - Bramka wokolo `STATEMANAGER.DrawIndexedPrimitive` w `CMapOutdoor::__HardwareTransformPatch_RenderPatchNone()`.
   - `src/GameLib/MapOutdoorRenderSTP.cpp`:
     - Bramka `!RenderOwnership::LegacyDraws(ERenderCategory::Terrain)` w `CMapOutdoor::__RenderTerrain_RenderSoftwareTransformPatch()`.
     - Bramka wokolo `STATEMANAGER.DrawIndexedPrimitive` w `CMapOutdoor::__SoftwareTransformPatch_RenderPatchNone()`.
3. **Woda (`ERenderCategory::Water`)**:
   - `src/GameLib/MapOutdoorWater.cpp`:
     - Oplecenie `STATEMANAGER.SetStreamSource` i `STATEMANAGER.DrawPrimitive` w `CMapOutdoor::DrawWater()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::Water)`.
4. **SpeedTree (`ERenderCategory::SpeedTree`)**:
   - `src/SpeedTreeLib/SpeedTreeForestDirectX.cpp`:
     - Oplecenie bloku bezposredniego renderowania drzew w `CSpeedTreeForestDirectX::Render()` po zrzuceniu pakietow do `CRenderQueueManager`.
   - `src/SpeedTreeLib/SpeedTreeWrapper.cpp`:
     - Bramka wokolo `DrawIndexedPrimitive` w `RenderBranches()` oraz `RenderFronds()`.
     - Bramka wokolo `DrawPrimitive` w `RenderLeaves()`.
     - Bramka wokolo `DrawPrimitiveUP` w `RenderBillboards()`.
5. **Dungeon (`ERenderCategory::Dungeon`)**:
   - `src/GameLib/DungeonBlock.cpp`:
     - Oplecenie bezposredniego rysowania siatek w `CDungeonModelInstance::RenderDungeonBlock()` po ekstrakcji pakietow.
     - Oplecenie bezposredniego rysowania cienia w `CDungeonModelInstance::RenderDungeonBlockShadow()`.
6. **Modele (`ERenderCategory::Model`)**:
   - `src/EterGrnLib/LODController.cpp`:
     - Oplecenie wywolan `RenderWithOneTexture()`, `BlendRenderWithOneTexture()`, `RenderWithTwoTexture()`, `BlendRenderWithTwoTexture()`, `RenderToShadowMap()`, `RenderShadow()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::Model)`.
7. **WeaponTrace (`ERenderCategory::WeaponTrace`)**:
   - `src/GameLib/WeaponTrace.cpp`:
     - Oplecenie wywolania `STATEMANAGER.DrawPrimitiveUP` w `CWeaponTrace::Render()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::WeaponTrace)`.
8. **EffectMesh (`ERenderCategory::EffectMesh`)**:
   - `src/EffectLib/EffectMeshInstance.cpp`:
     - Oplecenie wywolania `STATEMANAGER.DrawPrimitiveUP` w `CEffectMeshInstance::OnRender()` bramka `RenderOwnership::LegacyDraws(ERenderCategory::EffectMesh)`.
9. **Tekst (`ERenderCategory::Text`)**:
   - `src/EterLib/GrpTextInstance.cpp`:
     - Bramka w lambdzie `DrawBatch` dla glifow tekstu i obrysow.
     - Bramka wokolo rysowania zaznaczenia tekstu (`DEFAULT_IB_FILL_RECT`).
     - Bramki wokolo rysowania kursora tekstu oraz podkreslenia kompozycji IME.
10. **Minimapa (`ERenderCategory::MiniMap`)**:
    - `src/UserInterface/PythonMiniMap.cpp`:
      - Oplecenie petli rysowania kafelkow terenu minimapy bramka `RenderOwnership::LegacyDraws(ERenderCategory::MiniMap)`.
11. **Cien (`ERenderCategory::Shadow`)**:
    - `src/GameLib/MapOutdoorCharacterShadow.cpp`:
      - Bramka `!RenderOwnership::LegacyDraws(ERenderCategory::Shadow)` w `CMapOutdoor::BeginRenderCharacterShadowToTexture()` oraz `EndRenderCharacterShadowToTexture()`.
12. **Movie (`ERenderCategory::Movie`)**:
    - `src/UserInterface/MovieMan.cpp`:
      - Bramka `!RenderOwnership::LegacyDraws(ERenderCategory::Movie)` w `CMovieMan::PlayMovie()`.
      - Bramka wokolo `Blt` / `GDIBlt` w `CMovieMan::RenderStreamToSurface()`.

## 3. Zmiany w CD3D9RenderBackend
W pliku `src/EterLib/D3D9RenderBackend.cpp` dodano weryfikacje wlasnosci kategorii dla wszystkich metod renderujacych pakiety:
- `RenderSkyBox`: sprawdzenie `Owner(ERenderCategory::SkyBox) == ERenderOwner::D3D9Backend`
- `RenderTerrain`: sprawdzenie `Owner(ERenderCategory::Terrain) == ERenderOwner::D3D9Backend`
- `RenderActors`: sprawdzenie `Owner(ERenderCategory::Model) == ERenderOwner::D3D9Backend`
- `RenderWater`: sprawdzenie `Owner(ERenderCategory::Water) == ERenderOwner::D3D9Backend`
- `RenderSpeedTree`: sprawdzenie `Owner(ERenderCategory::SpeedTree) == ERenderOwner::D3D9Backend`
- `RenderEffects`: sprawdzenie `Owner(ERenderCategory::Effect) == ERenderOwner::D3D9Backend`
- `RenderUI`: sprawdzenie `Owner(ERenderCategory::UI) == ERenderOwner::D3D9Backend`
- `RenderDungeonBlocks`: sprawdzenie `Owner(ERenderCategory::Dungeon) == ERenderOwner::D3D9Backend`

## 4. Weryfikacja
1. **Liczba bramek LegacyDraws w kodzie zrodlowym**:
   - Polecenie `git grep -c "LegacyDraws" -- src` zwrocilo lacznie 36 wystapien w 16 plikach (wymog >= 10 spelniony z duzym zapasem).
2. **Kompilacja bibliotek docelowych**:
   - `cmake --build build --config Release --target EterLib GameLib SpeedTreeLib EterGrnLib EffectLib -- /m:1`
   - Wynik: 100% powodzenia, kod wyjscia 0, 0 bledow.
   - Zachowano zasade anty-lag (UserInterface nie byl kompilowany).
3. **Testy jednostkowe**:
   - `ctest -C Release -R RenderOwnershipTest`: 100% PASS (1/1 passed, 0.01 s).
