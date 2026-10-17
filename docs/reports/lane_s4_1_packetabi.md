# Lane Report: S4.1 Unie i ukryte pola -> nazwane pola (te same offsety) + zero-inicjalizacja [T076]

## 1. Cel zadania
Realizacja zadania S4.1 (T076) zgodnie z planem `RENDERER_CPP_PREP_PLAN.md`:
- Zastapienie unii pakietowych oraz semantyki paddingu (`fPad`) jawnymi polami nazwanymi przy zachowaniu identycznych offsetow i rozmiarow struktur pakietowych.
- Rozdzielenie niebezpiecznego aliasingu w `TRenderFrameUniforms` miedzy `v4SunDirection` a `dwFrameIndex` (F-NEW-018) na osobne pola `vSunLightDir` i `dwFrameIndex`.
- Usuniecie unii `bAlphaBlend` / `fAlpha` w `TDungeonBlockPacket` (F-P1-01), aby flaga przezroczystosci nie nadpisywala wspolczynnika alfy (`fAlpha == 0.99f` po ustawieniu `bAlphaBlend`).
- Usuniecie przestarzalych aliasow `dwBaseTextureID` oraz `dwModelID` w `TDungeonBlockPacket`.
- Zamiana pola paddingu `fPad[2]` w `TParticleInstanceData` na nazwane pola `fHalfWidth`, `fHalfHeight`.
- Zamiana unii w `TActorRenderPacket` na jawne pole `fShadowSize`.
- Wprowadzenie deterministycznej zero-inicjalizacji dla wszystkich 14 typow struktur pakietowych (domyslne inicjalizatory pol `{}` oraz konstruktory domyslne z `std::memset` i domyslnymi wartosciami identycznymi z FFI Rust `packets.rs`).
- Aktualizacja miejsc wywolan w producentach i backendzie (`DungeonBlockRenderExtractor.cpp`, `ParticleSystemInstance.cpp`, `SnowEnvironment.cpp`, `FlyTrace.cpp`, `MapOutdoorSky.cpp`, `SkyBox.cpp`, `ActorInstanceRender.cpp`, `ActorRenderExtractor.cpp`, `D3D9RenderBackend.cpp`).
- Utworzenie i zarejestrowanie dedykowanego testu jednostkowego `tests/test_c26_render_packet_abi.cpp` (`RenderPacketAbiTest`) weryfikujacego kompilacyjne `static_assert` rozmiarow i offsetow pol oraz zachowanie zero-inicjalizacji w runtime.

## 2. Szczegoly wykonanych zmian

1. **`src/EterLib/RenderPacketTypes.h`**:
   - `TRenderFrameUniforms`:
     - Rozdzielono unie `dwFrameIndex` / `v4SunDirection`: usunieto `v4SunDirection`, wprowadzono nazwane pola `D3DXVECTOR3 vSunLightDir{};` (@208) oraz `uint32_t dwFrameIndex{ 0 };` (@220).
     - Usunieto `fFogNear{ 0.0f }` z wnetrza anonimowej struktury w unii mgly (rozwiazujac blad C2926 MSVC).
     - Zdefiniowano konstruktor domyslny `TRenderFrameUniforms()` zerujacy cala strukture za pomoca `std::memset(this, 0, sizeof(*this))`.
     - Rozmiar: dokladnie 288 bajtow, wyrownanie 16 bajtow.
   - `TActorRenderPacket`:
     - Usunieto unie `union { float fPad; float fShadowSize; }`, zastapiono jawnym polem `float fShadowSize{ 0.0f };` (@5004).
     - Dodano domyslne inicjalizatory `{}` dla wszystkich pol oraz konstruktor domyslny z `std::memset`.
     - Rozmiar: dokladnie 5008 bajtow, wyrownanie 16 bajtow.
   - `TParticleInstanceData`:
     - Zastapiono `float fPad[2];` nazwanymi polami `float fHalfWidth{ 0.0f };` (@56) oraz `float fHalfHeight{ 0.0f };` (@60).
     - Dodano domyslne inicjalizatory oraz konstruktor domyslny z `std::memset`.
     - Rozmiar: dokladnie 64 bajty, wyrownanie 16 bajtow.
   - `TDungeonBlockPacket`:
     - Usunieto unie `dwBaseTextureID` (pozostawiono `dwDiffuseTextureID` @64).
     - Usunieto unie `dwModelID` (pozostawiono `dwPrimitiveCount` @136).
     - Rozdzielono unie `fAlpha` i `bAlphaBlend`: `float fAlpha{ 1.0f };` (@148), `uint8_t bAlphaBlend{ 0 };` (@152), `uint8_t bPadDungeon[3]{ 0, 0, 0 };` (@153), `float fPad{ 0.0f };` (@156).
     - Zdefiniowano konstruktor domyslny ustawiajacy macierz jednostkowa `matWorld` oraz `fAlpha = 1.0f`.
     - Rozmiar: dokladnie 160 bajtow, wyrownanie 16 bajtow.
   - `TUIVertex`:
     - Dodano domyslne inicjalizatory `{}` oraz konstruktor domyslny zerujacy pamiec.
     - Rozmiar: dokladnie 24 bajty.
   - Pozostale struktury pakietowe (`TTerrainSectorPacket`, `TWaterPacket`, `TSkyBoxPacket`, `TSpeedTreeInstanceData`, `TSpeedTreePacket`, `TEffectInstancePacket`, `TUIBatchPacket`, `TRenderFramePacket`):
     - Zdefiniowano konstruktory domyslne z gwarantowana zero-inicjalizacja pamieci `std::memset`.
   - Zaktualizowano twarde asercje `static_assert` na koncu pliku pod katem offsetow nowych pol i usunieto asercje dla wycofanych unii.

2. **Producenci i backend**:
   - `src/GameLib/DungeonBlockRenderExtractor.cpp`:
     - Zaktualizowano przypisanie: `packet.bAlphaBlend = (eMtrlType == CGrannyMaterial::TYPE_BLEND_PNT) ? 1 : 0;` oraz `packet.fPad = 0.0f;`.
     - Flaga `bAlphaBlend` nie niszczy juz wartosci `packet.fAlpha`.
   - `src/EffectLib/ParticleSystemInstance.cpp`:
     - Zamieniono `data.fPad[0]` i `data.fPad[1]` na `data.fHalfWidth = fScaleX;` oraz `data.fHalfHeight = fScaleY;`.
   - `src/GameLib/SnowEnvironment.cpp`:
     - Zamieniono `data.fPad[0]` i `data.fPad[1]` na `data.fHalfWidth = fHalfWidth;` oraz `data.fHalfHeight = fHalfHeight;`.
   - `src/GameLib/FlyTrace.cpp`:
     - Zamieniono `pParticles[i].fPad[0]` i `pParticles[i].fPad[1]` na `pParticles[i].fHalfWidth = m_fSize;` oraz `pParticles[i].fHalfHeight = 0.0f;`.
   - `src/GameLib/MapOutdoorSky.cpp`:
     - Usunieto przypisania do usunietego pola `v4SunDirection` w `UpdateAtmosphere`, `SetSunLight` oraz `SyncUniformsToRenderQueue`. Kierunek slonca jest bezpiecznie zapisywany do `vSunLightDir`.
   - `src/EterLib/SkyBox.cpp`:
     - Zaktualizowano metode `SetSunLight`: przypisanie kierunku do `m_atmosphereUniforms.vSunLightDir` bez nadpisywania `dwFrameIndex`.
   - `src/GameLib/ActorInstanceRender.cpp`:
     - Zastosowano bezpieczna zero-inicjalizacje `TActorRenderPacket packet{};`.
   - `src/GameLib/ActorRenderExtractor.cpp`:
     - Zaktualizowano przypisanie pola `outPacket.fShadowSize = 0.0f;` zamiast `outPacket.fPad`.
   - `src/EterLib/D3D9RenderBackend.cpp`:
     - W renderowaniu czasteczek zamieniono odczyt `p.fPad[0]` i `p.fPad[1]` na `p.fHalfWidth` oraz `p.fHalfHeight`.

3. **Testy (`tests/test_c26_render_packet_abi.cpp` oraz `tests/CMakeLists.txt`)**:
   - Utworzono test jednostkowy doctest zawierajacy:
     - Pelne `static_assert` rozmiarow wszystkich 14 struktur pakietowych (288, 176, 5008, 112, 112, 48, 64, 48, 24, 64, 112, 40, 160, 400).
     - Kompilacyjne `static_assert` offsetow dla wszystkich pol (w tym `vSunLightDir` @208, `dwFrameIndex` @220, `fHalfWidth` @56, `fHalfHeight` @60, `fShadowSize` @5004, `bAlphaBlend` @152, `bPadDungeon` @153, `fPad` @156).
     - `TEST_CASE("RenderPacketABI - Exact Sizes Check")`: weryfikacja runtime rozmiarow struktur.
     - `TEST_CASE("RenderPacketABI - TDungeonBlockPacket Independence and Zero-Init")`: weryfikacja domyslnych wartosci oraz niezaleznosci `fAlpha` od `bAlphaBlend` (`fAlpha == 0.99f` po ustawieniu `bAlphaBlend = 1`).
     - `TEST_CASE("RenderPacketABI - TRenderFrameUniforms Independence (F-NEW-018)")`: weryfikacja niezaleznosci `vSunLightDir` i `dwFrameIndex`.
     - `TEST_CASE("RenderPacketABI - TParticleInstanceData Named Dimensions")`: weryfikacja pol `fHalfWidth` i `fHalfHeight`.
     - `TEST_CASE("RenderPacketABI - TActorRenderPacket Zero-Init and ShadowSize")`: weryfikacja domyslnego stanu pakietu i metody `GetBoneMatrices()`.
   - Zarejestrowano cel `test_c26_render_packet_abi` oraz test CTest `RenderPacketAbiTest` w `tests/CMakeLists.txt`.

## 3. Weryfikacja

1. **Kryterium akceptacji planu**:
   - `git grep -n "fPad\[" -- src/EffectLib src/GameLib`: 0 linii (wynik pusty).
   - `git grep -n "v4SunDirection" -- src`: 0 linii (wynik pusty).
   - `git grep -n "dwBaseTextureID" -- src`: 0 linii (wynik pusty).
   - `git grep -n "dwModelID" -- src`: 0 linii (wynik pusty).

2. **Kompilacja anty-lag (ZAKAZ kompilacji UserInterface)**:
   - Zbudowano wylacznie cele `EterLib`, `EffectLib`, `GameLib` oraz `test_c26_render_packet_abi` / `test_c26_render_queue_manager` (`cmake --build build --config Release --target ... /m:1`).
   - Wszystkie cele skompilowane z kodem wyjscia 0, brak bledow i ostrzezen.
   - Projekt `UserInterface` NIE byl kompilowany.

3. **Wyniki testow jednostkowych**:
   - `ctest --test-dir build -C Release -R RenderPacketAbiTest --output-on-failure`:
     - 100% PASS (0.01 s).
     - 5 test cases, 71 asercji: 100% SUCCESS.
   - `ctest --test-dir build -C Release -R RenderQueueManagerTest --output-on-failure`:
     - 100% PASS (0.01 s).
     - 3 test cases, 90 asercji: 100% SUCCESS.
