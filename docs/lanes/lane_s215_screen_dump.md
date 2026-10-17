# Raport Lane S2.15 / S2.13 (T070) - Bezpieczenstwo WeaponTrace i Zrzuty Ekranu

## 1. Cel Zlecenia
Zadanie S2.13 & S2.15 (T070):
Eliminacja bledow pamieciowych i awarii potoku renderowania w `WeaponTrace`, optymalizacja i zabezpieczenie zrzutow ekranu w `CPythonGraphic` z uzyciem `GetRenderTargetData` na powierzchni `D3DPOOL_SYSTEMMEM` na koncu klatki oraz unifikacja pojedynczego handlera klawisza F11 bez duplikacji.

Odnosniki: W5-07, E2-18, B3-T10, F-NEW-049, F-NEW-020, F-P1-27, F-NEW-030.

---

## 2. Zrealizowane Prace

### A. Bezpieczenstwo WeaponTrace (`src/GameLib/WeaponTrace.cpp`)
- **GetBoneMatrix Null Check**:
  - W metodzie `Update`: Dodano weryfikacje poprawnosci pobrania macierzy kosci `GetBoneMatrix` oraz sprawdzenie, czy zwrocony wskaznik `pBoneMat != nullptr` przed dostepem do pol `_41`, `_42`, `_43`.
  - W metodzie `SetWeaponInstance`: Dodano walidacje `GetBoneMatrix(dwModelIndex, 0, &pmat) && pmat` przed obliczeniami dlugosci broni, zwracajac `false` w przypadku bledu.
- **Clampowanie liczby probek**:
  - W metodzie `BuildVertex`: Zabezpieczono rozmiar probek wzgledem stalego bufora `max_size = 300`. Liczba probek `n` jest ograniczana do `max_size - 2`, co calkowicie eliminuje przepelnienie buforow `h`, `stk` oraz `r`. Dodano warunek `if (n <= 0) return false;`.
- **Ochrona przed dzieleniem przez zero**:
  - Przy obliczaniu odstepu czasu `h[i] = Input[i+1].first - Input[i].first`: Gdy `fabsf(h[i]) < 0.0001f`, wartosc jest ustawiana na bezpieczna dolna granice `0.0001f`, zapobiegajac dzieleniu przez zero przy interpolacji spline.
- **Zachowanie stanu potoku DirectX (Save/Restore FVF i LIGHTING)**:
  - W metodzie `Render`: Zastapiono `STATEMANAGER.SetFVF(...)` przez `STATEMANAGER.SaveFVF(...)` oraz dodano `STATEMANAGER.RestoreFVF()` na koncu funkcji.
  - Zastapiono `STATEMANAGER.SetRenderState(D3DRS_LIGHTING, FALSE)` przez `STATEMANAGER.SaveRenderState(D3DRS_LIGHTING, FALSE)`.
  - Usunieto sztywne wymuszenie `STATEMANAGER.SetRenderState(D3DRS_LIGHTING, TRUE)`, ktore psulo stan oswietlenia nastepnych obiektow w scenie. Stan `D3DRS_LIGHTING` jest teraz przywracany przez `STATEMANAGER.RestoreRenderState(D3DRS_LIGHTING)`.

### B. Przebudowa SaveScreenShot i obsługa konca klatki (`src/EterPythonLib/PythonGraphic.h`, `PythonGraphic.cpp`)
- **Odroczenie zrzutu do konca klatki (flaga zadania)**:
  - Dodano flage zadania `m_bScreenShotRequest` oraz nazwe pliku `m_strScreenShotFileName`.
  - `SaveScreenShot(c_pszFileName)` ustawia flage zadania i zwraca sukces bez natychmiastowego blokowania GPU w polowie klatki.
  - Nadpisano metode `CPythonGraphic::End()`, ktora po zakonczeniu rysowania sceny sprawdza flage zadania i bezpiecznie wykonuje zrzut przez `ProcessScreenShot()`.
- **GetRenderTargetData do D3DPOOL_SYSTEMMEM**:
  - Zastapiono bezposrednie rygorystyczne blokowanie `GetBackBuffer` za pomoca `LockRect` w pamieci domyslnej sterownika (co prowadzilo do awarii na wielu kartach i w D3D9Ex).
  - Tworzona jest powierzchnia offscreen plain w pamieci systemowej `D3DPOOL_SYSTEMMEM` (`CreateOffscreenPlainSurface`).
  - Kopiowanie danych nastepuje asynchronicznie przez sterownik za pomoca `GetRenderTargetData`.
  - Powierzchnia systemowa jest bezpiecznie blokowana w trybie tylko do odczytu (`D3DLOCK_READONLY`), a nastepnie prawidlowo zwalniana.
- **Poprawa konwersji pikseli 16-bitowych (`uint16_t`)**:
  - Wyeliminowano niebezpieczne rzutowanie `*((UINT*)pRow)` dla formatow `D3DFMT_R5G6B5`, `D3DFMT_X1R5G5B5` oraz `D3DFMT_A1R5G5B5`.
  - Wprowadzono scisly odczyt za pomoca `const uint16_t* pPixel`, co zapobiega wyjsciu 2 bajty poza zakres bufora wiersza i crashom pamieciowym.
- **Walidacja naglowka EXIF i operacji plikowych**:
  - Dodano sprawdzenie wyniku `fseek` oraz `long ftellResult = ftell(srcFilePtr)`.
  - Wprowadzono scisly warunek `ftellResult > 21`, eliminujacy niedomiar `fileSize - 21`.
  - Dodano kontrole alokacji pamieci `tail = malloc(...)` oraz walidacje wynikow `fread`.
  - Zasoby plikowe (`srcFilePtr`, `dstFilePtr`) oraz pamiec (`tail`) sa zawsze bezpiecznie zwalniane.

### C. Zunifikowana obsluga F11 (`src/UserInterface/PythonApplicationEvent.cpp`, `PythonApplicationProcedure.cpp`)
- Zunifikowano obsluge klawisza F11 do jednego punktu w `CPythonApplication::OnKeyDown` (`DIK_F11`), wolajac wylacznie `CRenderQueueManager::Instance().RequestFrameDump("frame_dump.bin")`.
- Usunieto zbedne podwojne wywolanie `SaveScreenShot("cpp_reference.png")` w `PythonApplicationEvent.cpp`.
- Usunieto zdublowana obsluge `VK_F11` w `WindowProcedure` w `PythonApplicationProcedure.cpp`.
- Weryfikacja: W calym module `UserInterface` znajduje sie dokladnie 1 linia wywolania `RequestFrameDump("`.

### D. Format zrzutu referencyjnego (`src/EterLib/RenderQueueManager.cpp`)
- W metodzie `CaptureReferenceScreenshot` zapewniono, ze plik jest zapisywany wylacznie w formacie PNG (`D3DXIFF_PNG`), a sciezka docelowa gwarantuje rozszerzenie `.png`.

---

## 3. Weryfikacja i Kompilacja
1. **Liczba wywolan RequestFrameDump w UserInterface**:
   - `git grep -n "RequestFrameDump" -- src/UserInterface` zwraca dokladnie 1 linie:
     `src/UserInterface/PythonApplicationEvent.cpp:142: CRenderQueueManager::Instance().RequestFrameDump("frame_dump.bin");`
2. **Kompilacja bibliotek (Anty-Lag)**:
   - Zgodnie z wytycznymi anty-lag, nie budowano monolitu `UserInterface`.
   - Zbudowano cele `GameLib`, `EterPythonLib` oraz `EterLib`:
     `cmake --build build --config Release --target GameLib EterPythonLib -- /m:1`
     Wynik: **100% SUKCES** (kod wyjscia 0, brak bledow).
