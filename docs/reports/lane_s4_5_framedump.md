# Lane Report: S4.5 Frame dump v2: chunked, SkyBox i Dungeon, 64-bit identyfikatory, deterministyczne bajty [T079]

## 1. Cel zadania
Realizacja zadania S4.5 (T079) zgodnie z planem `RENDERER_CPP_PREP_PLAN.md`:
- Modernizacja formatu binarnego zrzutu klatki Golden Master do wersji v2 (`Frame dump v2`).
- Zdefiniowanie struktury pliku opartej na chunkach:
  - Magiczny naglowek pliku: `'M', '2', 'F', 'R'` (`0x5246324D`).
  - Wersja formatu: `uint32_t version = 2`.
  - Kazda sekcja zapisywana jako chunk o naglowku: `{ uint32_t dwTag, uint32_t dwCount, uint32_t dwElementSize, uint32_t dwDataSize }`.
  - Tagi sekcji:
    - `CHUNK_UNIFORMS` (`'UNIF'`, `0x46494E55`)
    - `CHUNK_SKYBOX` (`'SKYB'`, `0x42594B53`)
    - `CHUNK_TERRAIN` (`'TERR'`, `0x52524554`)
    - `CHUNK_DUNGEON` (`'DUNG'`, `0x474E5544`)
    - `CHUNK_WATER` (`'WATR'`, `0x52544157`)
    - `CHUNK_SPEEDTREE` (`'TREE'`, `0x45455254`)
    - `CHUNK_ACTORS` (`'ACTR'`, `0x52544341`)
    - `CHUNK_EFFECTS` (`'EFCT'`, `0x54434645`)
    - `CHUNK_UI` (`'UIBD'`, `0x44424955`)
- Wlaczenie brakujacych wczesniej sekcji: `SkyBox` (`CHUNK_SKYBOX`) oraz `Dungeon` (`CHUNK_DUNGEON`).
- Zapis pelnych 64-bitowych identyfikatorow zasobow i obiektow (`uint64_t`: texture id, model id, tree type id, mesh id) zamiast dotychczasowego obcinania do 32-bit (`& 0xFFFFFFFF`).
- Gwarancja pelnego determinizmu binarnego:
  - Jawne zerowanie paddingow i wskaznikow runtime (`nullptr` / `0`).
  - Identyczna sekwencja bajtow przy wielokrotnym eksporcie tej samej klatki (identyczny CRC32).
- Utworzenie i zarejestrowanie testu jednostkowego `tests/test_c26_frame_dump.cpp` (`FrameDumpTest`).

## 2. Szczegoly wykonanych zmian

1. **`src/EterLib/RenderQueueManager.h`**:
   - Wprowadzono przestrzen nazw `FrameDumpV2` ze stalymi `M2FR_MAGIC`, `M2FR_VERSION`, tagami sekcji oraz strukturami naglowkowymi spakowanymi `#pragma pack(push, 1)`:
     - `THeader`: naglowek pliku (16 bajtow: magic, version, chunkCount=9, reserved=0).
     - `TChunkHeader`: naglowek chunka (16 bajtow: dwTag, dwCount, dwElementSize, dwDataSize).
     - `TSpeedTreeBatchHeader`: naglowek partii drzew (40 bajtow: dwTreeCRC, dwFlags, 64-bit uTreeTypeID, 64-bit uTrunkTextureID, 64-bit uLeafTextureID, dwInstanceCount, dwPad).
     - `TEffectBatchHeader`: naglowek partii efektow (24 bajty: dwEffectID, dwBlendMode, 64-bit uTextureID, dwParticleCount, dwPad).
     - `TUIBatchHeader`: naglowek partii interfejsu (40 bajtow: 64-bit uTextureID, bUseScissor, scissorRect, dwBlendMode, dwVertexCount, dwIndexCount).
     - `TDumpParsedFrame`: struktura zdeserializowanej klatki umozliwiajaca pelna inspekcje zrzutu.
   - Dodano kompilacyjne asercje `static_assert` weryfikujace rozmiary naglowkow.
   - Zadeklarowano metody:
     - `bool ExportFrameToBinary(const char* c_pszFileName) const;`
     - `bool ExportFrameToBinaryV2(const char* c_pszFileName) const;`
     - `bool ParseFrameDumpV2(const char* c_pszFileName, TDumpParsedFrame& outFrame);`

2. **`src/EterLib/RenderQueueManager.cpp`**:
   - Zaimplementowano `ExportFrameToBinaryV2`:
     - Zapis naglowka pliku v2 z liczba chunkow rowna 9.
     - Sekwencyjny zapis 9 chunkow w deterministycznej kolejnosci: UNIF, SKYB, TERR, DUNG, WATR, TREE, ACTR, EFCT, UIBD.
     - Dla stalych struktur pakietowych (`UNIF`, `SKYB`, `TERR`, `DUNG`, `WATR`, `ACTR`) ustawiono `dwElementSize = sizeof(...)` oraz `dwDataSize = dwCount * dwElementSize`.
     - Dla partii o zmiennej liczbie instancji/wierzcholkow (`TREE`, `EFCT`, `UIBD`) ustawiono `dwElementSize = 0` oraz wyliczono dokladna sume bajtow danych w `dwDataSize`.
     - Wyzerowano wszystkie wskazniki i paddingi przed zapisem binarnym.
     - Wszystkie identyfikatory tekstur i modeli zapisywane sa jako pelne `uint64_t`.
   - Zaimplementowano `ParseFrameDumpV2`:
     - Weryfikacja magicznego naglowka i wersji formatu.
     - Odczyt naglowkow chunkow, parsowanie danych wg tagu z bezpiecznym pomijaniem nieznanych tagow za pomoca `fseek`.

3. **`tests/test_c26_frame_dump.cpp` oraz `tests/CMakeLists.txt`**:
   - Utworzono test jednostkowy doctest:
     - `FrameDumpV2 - All Packet Types Export and Chunk Verification`: zrzut klatki ze wszystkimi 8 typami pakietow + uniformy, odczyt i weryfikacja wszystkich 9 naglowkow chunkow, sum danych, licznikow, 64-bitowych ID oraz wyzerowania wskaznikow.
     - `FrameDumpV2 - Byte Determinism and CRC32 Equality`: dwa niezalezne zrzuty tej samej klatki do osobnych plikow binarnych, weryfikacja tozsamosci bajtowej (`memcmp`) oraz zgodnosci sum kontrolnych CRC32 (`GetFileCRC32`).
     - `FrameDumpV2 - Empty Frame Chunked Structure`: weryfikacja poprawnej struktury pliku dla pustej klatki (dokladnie 448 bajtow: 16 B naglowek + 9 * 16 B chunk headers + 288 B uniforms).
     - `FrameDumpV2 - 64-bit ID Integrity`: weryfikacja braku obcinania gornych 32 bitow w identyfikatorach tekstur i modeli (wartosci powyzej `0xFFFFFFFF`).
   - Zarejestrowano cel `test_c26_frame_dump` i test ctest `FrameDumpTest` w `tests/CMakeLists.txt`.

## 3. Weryfikacja

1. **Kompilacja anty-lag (ZAKAZ kompilacji UserInterface)**:
   - Zbudowano wylacznie cel testowy: `cmake --build build --config Release --target test_c26_frame_dump -- /m:1`.
   - Wszystkie biblioteki zalezne (`EterLib`, `EterBase`) skompilowane bez bledow.
   - Projekt `UserInterface` NIE byl kompilowany.

2. **Wyniki testow jednostkowych**:
   - `ctest --test-dir build -C Release -R FrameDumpTest --output-on-failure`:
     - 100% tests passed (0.01 s).
   - Bezposrednie uruchomienie `test_c26_frame_dump.exe`:
     - 4 test cases: 4 passed, 0 failed.
     - 138 assertions: 138 passed, 0 failed.
     - Status: SUCCESS!
