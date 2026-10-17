# Lane Report: S4.3 CRenderResourceRegistry texture handles; producers stop writing truncated or raw COM pointers [T078]

## 1. Cel zadania
Realizacja zadania S4.3 (T078) zgodnie z planem `RENDERER_CPP_PREP_PLAN.md`:
- Wprowadzenie centralnego rejestru zasobow renderera `CRenderResourceRegistry` operujacego na bezpiecznych 32-bitowych uchwytach tekstur (`TRenderTextureHandle`), eliminujac niebezpieczne rzutowania i obcinanie 64-bitowych wskaznikow COM Direct3D 9 (`reinterpret_cast<uintptr_t>(pTex) & 0xFFFFFFFF`) w strukturach pakietowych C-ABI.
- Zaimplementowanie podzialu 32-bitowego uchwytu na 20 bitow indeksu slotu oraz 12 bitow licznika generacji zapobiegajacego problemowi ABA przy recyklingu slotow pamieci.
- Zagwarantowanie, ze wartosc 0 oznacza brak tekstury / nieprawidlowy uchwyt (`kInvalidTextureHandle = 0`), a slot 0 jest trwale odrzucany.
- Dziedziczenie `CRenderResourceRegistry` po `CSingleton<CRenderResourceRegistry>` z pelna ochrona wielowatkowa (`std::mutex`).
- Integracja wyrejestrowywania tekstur w `CGraphicTexture` w destruktorze i `DestroyDeviceObjects()` (o ile `m_bOwnsTexture` jest prawdziwe).
- Zastapienie wpisywania obcietych wskaznikow COM w producentach pakietow:
  * `src/EterLib/SkyBox.cpp` (dwSkyTextureID, dwCloudTextureID)
  * `src/GameLib/TerrainRenderExtractor.cpp` (splatTextureId, layerTextureIds)
- Potwierdzenie braku wystapien ciagu `0xFFFFFFFF` w zmodyfikowanych producentach.
- Utworzenie pelnego zestawu testow jednostkowych doctest weryfikujacych poprawnosc alokacji, inkrementacji generacji, odrzucania przestarzalych generacji i slotu 0, odczytu metadanych oraz bezpieczenstwa watkowego.
- Przestrzeganie zasady braku kompilacji `UserInterface` (budowa wylacznie celu testowego w trybie `/m:1`).

## 2. Szczegoly wykonanych zmian

1. **`src/EterLib/RenderResourceRegistry.h` oraz `src/EterLib/RenderResourceRegistry.cpp`**:
   - Zdefiniowano typ `TRenderTextureHandle` jako `uint32_t` ze stala `kInvalidTextureHandle = 0`.
   - Wprowadzono stale bitowe:
     - `kTextureIndexBits = 20` (maska `0x000FFFFF`, obsluga do 1 048 575 unikalnych slotow).
     - `kTextureGenerationBits = 12` (maska `0x00000FFF`, do 4095 generacji).
     - `kTextureGenerationShift = 20`.
   - Zaimplementowano funkcje pomocnicze `RenderResourceGetSlotIndex`, `RenderResourceGetGeneration`, `RenderResourceMakeHandle`, `RenderResourceIsValidHandleFormat`.
   - Zdefiniowano strukture `STextureResourceInfo` przechowujaca wymiary (`width`, `height`), format Direct3D 9 (`D3DFORMAT`), nazwe debugowa (`std::string debugName`) oraz wskaznik `IDirect3DTexture9*`.
   - Zaimplementowano klase `CRenderResourceRegistry` dziedziczaca po `CSingleton<CRenderResourceRegistry>`:
     - `RegisterTexture(IDirect3DTexture9* pTexture, const char* debugName)`:
       - Zwraca `kInvalidTextureHandle` dla `nullptr`.
       - Idempotentnosc: jesli wskaznik jest juz zarejestrowany w mapie `m_textureToHandle`, zwraca istniejacy uchwyt.
       - Przydziela slot z listy wolnych slotow `m_freeSlots` lub dodaje nowy slot na koncu tablicy `m_slots`.
       - Automatycznie odpytuje `pTexture->GetLevelDesc(0, &desc)` w celu zapisania wymiarow i formatu.
     - `RegisterTextureWithInfo(...)`: przeciazenie z jawnym podaniem metadanych.
     - `UnregisterTexture(TRenderTextureHandle handle)` oraz `UnregisterTexture(IDirect3DTexture9* pTexture)`:
       - Zwalnia powiazanie w mapie i strukturze slotu.
       - Inkrementuje licznik generacji slotu (`(generation + 1) & 0x0FFF`, z pominieciem 0).
       - Przekazuje indeks do `m_freeSlots`.
     - `ResolveTexture(TRenderTextureHandle handle)`:
       - Odrzuca `handle == 0` oraz `slot == 0`.
       - Sprawdza czy slot miesci sie w granicach tablicy, jest oznaczony jako zajety oraz czy generacja w uchwycie dokladnie odpowiada biezacej generacji slotu.
       - Zwraca wskaznik `IDirect3DTexture9*` lub `nullptr` w przypadku bledu / przestarzalego uchwytu.
     - Metody dostepowe metadanych: `GetTextureInfo`, `GetWidth`, `GetHeight`, `GetFormat`, `GetDebugName`, `FindTextureHandle`, `IsValidHandle`.
     - Metody statyczne odporne na brak aktywnej instancji singletona: `SafeRegisterTexture`, `SafeUnregisterTexture`, `SafeResolveTexture`.
     - Zapewniono bezpieczenstwo watkowe za pomoca `mutable std::mutex m_mutex`.

2. **`src/EterLib/GrpTexture.h` oraz `src/EterLib/GrpTexture.cpp`**:
   - Dodano metode `uint32_t GetTextureHandle() const` do `CGraphicTexture`.
   - W destruktorze `~CGraphicTexture()` oraz w `DestroyDeviceObjects()`:
     - Sprawdzenie `if (m_bOwnsTexture && m_lpd3dTexture)`.
     - Wywolanie `CRenderResourceRegistry::Instance().UnregisterTexture(m_lpd3dTexture)` z zabezpieczeniem `InstancePtr()`.

3. **`src/EterLib/SkyBox.cpp`**:
   - W procedurze `CSkyBox::Render()`:
     - Zastapiono `pkt.dwSkyTextureID = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(pTex) & 0xFFFFFFFF);`
       bezpiecznym uchwytem `CRenderResourceRegistry::Instance().RegisterTexture(pTex, "SkyBox_Face")`.
     - Zastapiono `pkt.dwCloudTextureID = static_cast<uint32_t>(reinterpret_cast<uintptr_t>(pTexObj->GetD3DTexture()) & 0xFFFFFFFF);`
       bezpiecznym uchwytem `CRenderResourceRegistry::Instance().RegisterTexture(pTexObj->GetD3DTexture(), "SkyBox_Cloud")`.
     - Domyslna wartosc zainicjalizowana do `kInvalidTextureHandle`.

4. **`src/GameLib/TerrainRenderExtractor.cpp`**:
   - W metodzie `CTerrainRenderExtractor::ExtractPatch`:
     - Zastapiono obcinanie wskaznika splattingu:
       `outPacket.splatTextureId = CRenderResourceRegistry::Instance().RegisterTexture(static_cast<IDirect3DTexture9*>(outPacket.pSplatTexture), "Terrain_Splat");`
     - Zastapiono obcinanie wskaznika w warstwie aktywnej splata:
       `outPacket.splatTextureId = CRenderResourceRegistry::Instance().RegisterTexture(rSplat.pd3dTexture, "Terrain_SplatPatch");`
     - Wprowadzono rejestracje warstw dominujacych oraz warstw aktywnych do `outPacket.layerTextureIds[l]` jako prawidlowe uchwyty tekstur.

5. **`tests/test_c26_render_resource_registry.cpp` oraz `tests/CMakeLists.txt`**:
   - Utworzono mock `MockDirect3DTexture9` implementujacy metody `IDirect3DTexture9` i `GetLevelDesc`.
   - Napisano 8 przypadkow testowych pokrywajacych:
     - Format bitowy uchwytu (20 bitow indeksu slotu, 12 bitow licznika generacji, maski, stale).
     - Podstawowa rejestracje, idempotentnosc i rozwiazywanie uchwytow.
     - Ponowne uzycie slotu po zwolnieniu z prawidlowa inkrementacja generacji.
     - Odrzucenie nieprawidlowego uchwytu (stara generacja po ponownym uzyciu, uchwyt 0, slot 0, sloty spoza zakresu).
     - Zapis i odczyt metadanych zasobu (`STextureResourceInfo`, szerokosc, wysokosc, format, nazwa debugowa).
     - Wyrejestrowanie bezposrednio przez wskaznik Direct3D 9 i wyszukiwanie uchwytu.
     - Przekroczenie granicy 12 bitow generacji (wrap-around bez przyjmowania wartosci 0).
     - Wspolbieznosc i bezpieczenstwo watkowe (wielowatkowe rejestrowanie i zwalnianie).
   - Zarejestrowano cel `test_c26_render_resource_registry` oraz test CTest `RenderResourceRegistryTest`.

## 3. Weryfikacja

1. **Weryfikacja usuniecia obcinania wskaznikow COM (0xFFFFFFFF)**:
   - Polecenie: `git grep -n "0xFFFFFFFF" -- src/EterLib/SkyBox.cpp src/GameLib/TerrainRenderExtractor.cpp`
   - Wynik: 0 linii (kod wyjscia 1). Wszystkie wystapienia zastapione prawidlowymi uchwytami `TRenderTextureHandle`.

2. **Kompilacja anty-lag (ZAKAZ kompilacji UserInterface)**:
   - Zbudowano wylacznie cel testowy:
     `cmake --build build --config Release --target test_c26_render_resource_registry -- /m:1`
   - Cel `UserInterface` NIE byl kompilowany.
   - Kompilacja zakonczona sukcesem: kod wyjscia 0.

3. **Wyniki testow jednostkowych**:
   - Uruchomienie bezposrednie:
     `.\build\tests\Release\test_c26_render_resource_registry.exe`
     - 8 test cases, 1319 asercji doctest: 100% SUCCESS, 0 failed.
   - Uruchomienie przez CTest:
     `ctest --test-dir build -C Release -R RenderResourceRegistryTest --output-on-failure`
     - 100% tests passed (0.01 sec).
