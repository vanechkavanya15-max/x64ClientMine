# Lane Report: S4.2 RenderAbi.h w czystym C bez d3d9/d3dx9 + asercje offsetof [T077]

## 1. Cel zadania
Realizacja zadania S4.2 (T077) zgodnie z planem `RENDERER_CPP_PREP_PLAN.md`:
- Utworzenie niezaleznego naglowka C-ABI `src/EterLib/RenderAbi.h` w czystym C (z obsluga `extern "C"` dla C++), calkowicie pozbawionego zaleznosci od Direct3D (`d3d9.h`, `d3dx9.h`).
- Zdefiniowanie czystych typow bazowych: `TRenderMatrix4x4` (float m[16] z unia `_11.._44`), `TRenderVec2` (float x, y), `TRenderVec3` (float x, y, z), `TRenderVec4` (float x, y, z, w / r, g, b, a), `TRect32` (int32_t left, top, right, bottom), kolory jako `uint32_t` (`TRenderColorARGB`).
- Przeniesienie definicji struktur C-ABI wszystkich 14 pakietow renderera i wierzcholkow (`TUIVertex`, `TPNT2Vertex`, `TRenderFrameUniforms`, `TTerrainSectorPacket`, `TActorRenderPacket`, `TWaterPacket`, `TSkyBoxPacket`, `TSpeedTreeInstanceData`, `TSpeedTreePacket`, `TParticleInstanceData`, `TEffectInstancePacket`, `TUIBatchPacket`, `TDungeonBlockPacket`, `TRenderFramePacket`) do `src/EterLib/RenderAbi.h`.
- Wlaczenie `RenderAbi.h` w `src/EterLib/RenderPacketTypes.h` z zapewnieniem 100% kompatybilnosci wstecznej i funkcji konwersji do/z typow D3DX (`ToD3DX`, `ToAbi`, `ToD3DColor`, `ToRect`).
- Zapewnienie zgodnosci wierzcholka `TPNT2Vertex` w `src/EterLib/GrpBase.h` z definicja z `RenderAbi.h` (w tym unia `texCoord`/`texCoord2` oraz `uv0`/`uv1`).
- Utworzenie i zarejestrowanie dedykowanego testu C `tests/test_c26_render_abi_c.c` (`RenderAbiCTest`) kompilowanego jako czysty C, weryfikujacego kompilacyjne i runtime asercje `offsetof` dla kazdego pola kazdego pakietu i wierzcholka bez uzycia warunkowego `#if defined(_WIN64)`.
- Weryfikacja zerowej obecnosci ciagow `d3d9` i `d3dx9` w `src/EterLib/RenderAbi.h`.

## 2. Szczegoly wykonanych zmian

1. **`src/EterLib/RenderAbi.h`**:
   - Czysty naglowek C kompatybilny z kompilatorami C89, C99, C11 oraz C++.
   - Zdefiniowano uniwersalne makra wyrownania `RENDER_ABI_ALIGN(n)` i asercji kompilacyjnych `RENDER_ABI_STATIC_ASSERT`.
   - Zdefiniowano typy bazowe:
     - `TRenderMatrix4x4`: unia tablicy `float m[16]` oraz nazwanych wspolrzednych `_11`..`_44` (64 bajty).
     - `TRenderVec2`: unia `x, y` oraz `u, v` (8 bajtow).
     - `TRenderVec3`: unia `x, y, z` oraz `v3[3]` (12 bajtow).
     - `TRenderVec4`: unia `x, y, z, w`, `r, g, b, a` oraz `v4[4]` (16 bajtow).
     - `TRect32`: 4 pola `int32_t` (`left`, `top`, `right`, `bottom`) zgodne binarnie z Win32 `RECT` (16 bajtow).
     - `TRenderColorARGB`: alias na `uint32_t`.
     - Enumy: `EEffectBlendMode` i `EUIBlendMode`.
   - Wprowadzono elastyczne makra typow pol (`RENDER_MATRIX_TYPE`, `RENDER_VEC2_TYPE`, `RENDER_VEC3_TYPE`, `RENDER_VEC4_TYPE`, `RENDER_COLOR4_TYPE`, `RENDER_RECT_TYPE`, `RENDER_TREE_WRAPPER_TYPE`), domyslnie mapujace sie w czystym C na typy bazowe ABI.
   - Zdefiniowano wszystkie struktury pakietowe i wierzcholkowe z wyrownaniem 16-bajtowym.
   - W trybie C++ zachowano zero-inicjalizacje konstruktorami domyslnymi (`std::memset`) oraz metode pomocnicza `GetBoneMatrices()`.
   - Umieszczono kompilacyjne asercje rozmiarow dla wszystkich 14 typow.
   - Plik nie zawiera zadnych odniesien ani wlaczen bibliotek Direct3D.

2. **`src/EterLib/RenderPacketTypes.h`**:
   - Zastapiono powielone definicje struktur wlaczeniem naglowka `RenderAbi.h` ze zdefiniowanymi typami D3DX (`D3DXMATRIX`, `D3DXVECTOR2`, `D3DXVECTOR3`, `D3DXVECTOR4`, `D3DXCOLOR`, `RECT`, `CSpeedTreeWrapper`).
   - Wprowadzono funkcje inline pelnej zgodnosci i bezpiecznej konwersji referencji oraz wskaznikow:
     - `ToD3DX(TRenderMatrix4x4&)` / `ToAbi(D3DXMATRIX&)`
     - `ToD3DX(TRenderVec2&)` / `ToAbi(D3DXVECTOR2&)`
     - `ToD3DX(TRenderVec3&)` / `ToAbi(D3DXVECTOR3&)`
     - `ToD3DX(TRenderVec4&)` / `ToAbi(D3DXVECTOR4&)`
     - `ToD3DColor(TRenderVec4&)`
     - `ToRect(TRect32&)` / `ToAbi(RECT&)`
   - Zachowano pomocnicza strukture `EterLib::Render::TActorBonePalette` oraz pelny zestaw twardych asercji ABI x64.

3. **`src/EterLib/GrpBase.h`**:
   - Zaktualizowano definicje `SPNT2Vertex` / `TPNT2Vertex`, dodajac osłone `_SPNT2VERTEX_DEFINED_` oraz unie `texCoord`/`texCoord2` z `uv0`/`uv1`, w pelni odpowiadajaca definicji z `RenderAbi.h`.

4. **`tests/test_c26_render_abi_c.c` oraz `tests/CMakeLists.txt`**:
   - Utworzono dedykowany test w czystym C kompilowany za pomoca kompilatora C (`PROPERTIES LANGUAGE C`).
   - Pokryto bezwarunkowymi asercjami kompilacyjnymi oraz runtime `assert` kazde pole kazdej struktury pakietowej i wierzcholkowej (lacznie ponad 170 asercji `offsetof` i `sizeof`).
   - Zarejestrowano cel `test_c26_render_abi_c` oraz test CTest `RenderAbiCTest`.

## 3. Weryfikacja

1. **Brak zaleznosci od D3D w RenderAbi.h**:
   - `git grep -n -E "d3d9|d3dx9" -- src/EterLib/RenderAbi.h`: zwrocil kod wyjscia 1 (0 pasujacych linii).

2. **Kompilacja anty-lag (ZAKAZ kompilacji UserInterface)**:
   - Zbudowano wylacznie cele `test_c26_render_abi_c`, `test_c26_render_packet_abi`, `test_c26_render_queue_manager` oraz biblioteki zalezne (`EterLib`, `EterBase`) w trybie jedno-watkowym (`/m:1`).
   - Cel `UserInterface` NIE byl kompilowany.
   - Kompilacja przebiegla bez zadnych bledow.

3. **Wyniki testow jednostkowych**:
   - `ctest --test-dir build -C Release -R RenderAbiCTest --output-on-failure`:
     - 100% PASS (0.00 s).
     - Test programu C `test_c26_render_abi_c.exe` zwrocil kod wyjscia 0 (`[PASS] All RenderAbi C offsets and sizes verified successfully.`).
   - `ctest --test-dir build -C Release -R RenderPacketAbiTest --output-on-failure`:
     - 100% PASS (0.01 s).
     - 5 test cases, 71 asercji doctest: 100% SUCCESS.
   - `ctest --test-dir build -C Release -R RenderQueueManagerTest --output-on-failure`:
     - 100% PASS.
     - 3 test cases, 90 asercji doctest: 100% SUCCESS.
