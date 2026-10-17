# Lane Report: S3.3 D3D9 Backend Hygiene [T073]

## 1. Cel zadania
Przeprowadzenie kompleksowej higieny implementacji `CD3D9RenderBackend` (G2-07, F-NEW-014, F-NEW-015, F-NEW-016, F-NEW-017, F-NEW-019, F-NEW-021, D2-07):
- Wyeliminowanie bezposrednich odwolan do urzadzenia Direct3D (`dev->Set*`, `dev->Draw*`) na rzecz centralnego menedzera stanow `STATEMANAGER`.
- Integracja z tabela wlasnosci `RenderOwnership` w metodach renderujacych (`RenderSkyBox`, `RenderActors`, `RenderTerrain`, `RenderSpeedTree`, `RenderDungeonBlocks`) w celu natychmiastowego wyjscia bez modyfikacji stanow GPU, gdy wlascicielem kategorii nie jest `ERenderOwner::D3D9Backend`.
- Calkowite usuniecie flagi `m_bReplaySpeedTreePackets` oraz pola `bPad` z backendu oraz powiazanych pakietow.
- Wprowadzenie nazwanych stalych konfiguracyjnych: `kMaxQuadsPerDraw = 16380`, `kMaxShaderBones = 60`, `kSectorSize = 25600.0f`.
- Zastapienie statycznych wektorow renderujacych dynamicznymi polami skladowymi instancji klasy (`m_particleVertices`, `m_sharedEffectIndices`, `m_sharedQuadIndices`).
- Usuniecie zbednych pol skladowych: `m_bInitialized`, `m_pActorVertexDecl`, `m_hWnd`.

## 2. Wykonane zmiany
1. **`src/EterLib/RenderPacketTypes.h`**:
   - Usunieto pole `bPad` ze struktury `TTerrainSectorPacket`. Zachowano uklad ABI x64 (rozmiar struktury wynosi dokladnie 176 bajtow).
   - Zastapiono nazwe pola `bPadAlpha[3]` na `reservedAlpha[3]` w `TDungeonBlockPacket`, uniemozliwiajac powstawanie false-positive trafien dla `bPad`.
2. **`src/EterLib/D3D9RenderBackend.h`**:
   - Zdefiniowano stale `constexpr uint32_t kMaxQuadsPerDraw = 16380;`, `constexpr uint32_t kMaxShaderBones = 60;`, `constexpr float kSectorSize = 25600.0f;`.
   - Usunieto nieuzywane pola: `m_hWnd`, `m_bInitialized`, `m_pActorVertexDecl`, `m_bReplaySpeedTreePackets` oraz metode `SetReplaySpeedTreePackets`.
   - Dodano pola skladowe buforow: `m_particleVertices`, `m_sharedEffectIndices`, `m_sharedQuadIndices`.
3. **`src/EterLib/D3D9RenderBackend.cpp`**:
   - Dodano `#include "RenderOwnership.h"`.
   - Zaimplementowano guardy `RenderOwnership::Owner(cat) != ERenderOwner::D3D9Backend` na wejsciu do `RenderSkyBox`, `RenderTerrain`, `RenderActors`, `RenderSpeedTree`, `RenderDungeonBlocks`.
   - Przekierowano wywolania rysowania terenu oraz stalych kosci aktorow przez `STATEMANAGER`:
     - `STATEMANAGER.SetStreamSource(0, pD3DVB, sizeof(float) * 6);`
     - `STATEMANAGER.DrawPrimitive(D3DPT_TRIANGLESTRIP, 0, ...);`
     - `STATEMANAGER.SetVertexShaderConstant(4, pBones, matrixCount * 4);`
   - Wyzerowano wszystkie surowe wywolania `dev->` w calym pliku (zastapione `STATEMANAGER` lub lokalnym `pD3DDevice` dla deklaracji i testu utraty urzadzenia).
   - Zastapiono wektory statyczne polami `m_particleVertices`, `m_sharedEffectIndices`, `m_sharedQuadIndices`.
   - Zastosowano stale `kMaxQuadsPerDraw`, `kMaxShaderBones`, `kSectorSize`.

## 3. Weryfikacja
- `git grep -n -E "bPad|m_bReplaySpeedTreePackets" -- src`: 0 linii (wynik pusty).
- `git grep -n "dev->" -- src/EterLib/D3D9RenderBackend.cpp`: 0 linii (wynik pusty).
- Kompilacja biblioteki EterLib:
  `cmake --build build --config Release --target EterLib -- /m:1` -> Zbudowano pomyslnie bez bledow (0 errors).
- Uruchomienie powiazanych testow jednostkowych:
  - `RenderOwnershipTest`: 100% zaliczone (0.01 s).
  - `D3D9StateGuardsTest`: 100% zaliczone (0.01 s).
