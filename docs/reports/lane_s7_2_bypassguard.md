# Raport z realizacji S7.2 (T086): RenderBypassGuard debug detector

## 1. Cel zadania
Realizacja zadania S7.2 (T086) z planu modernizacji silnika renderowania Metin2:
Wdrozenie mechanizmu detekcji i raportowania bezposrednich wywolan rysowania (`DrawPrimitive`, `DrawIndexedPrimitive`, `DrawPrimitiveUP`, itp.) omijajacych aktywny backend renderera per kategoria renderowania (`RenderOwnership::ERenderCategory`).

## 2. Zaimplementowane komponenty

### 2.1. Klasa CRenderBypassGuard (`src/EterLib/RenderBypassGuard.h`, `src/EterLib/RenderBypassGuard.cpp`)
- Implementacja singletonu `CRenderBypassGuard` dziedziczacego po `CSingleton<CRenderBypassGuard>`.
- Przechowywanie licznikow ominiec w tablicy atomowej dla kazdej kategorii z `RenderOwnership::ERenderCategory`.
- Rejestracja miejsc wywolan (`siteName`) w watkowo bezpiecznej strukturze `SBypassSiteRecord`.
- Metody API:
  - `void RecordBypass(RenderOwnership::ERenderCategory category, const char* siteName = nullptr)`: rejestruje nielegalne ominiecie dla wskazanej kategorii.
  - `uint32_t GetBypassCount(RenderOwnership::ERenderCategory category) const`: zwraca licznik ominiec dla wybranej kategorii.
  - `uint32_t GetTotalBypassCount() const`: zwraca sumaryczny licznik wszystkich wykrytych ominiec.
  - `void Reset()`: zeruje liczniki i historie raportow.
  - `void ReportBypasses()`: generuje czytelny raport tekstowy ze wszystkimi kategoriami, w ktorych wykryto bypass (oraz ich aktywnym wlascicielem) i loguje go do silnika.
  - `bool IsDrawAllowed(RenderOwnership::ERenderCategory category) const`: sprawdza czy draw call jest dozwolony (zwraca `true` jesli kategoria nalezy do `Legacy`).
  - `bool ValidateDraw(RenderOwnership::ERenderCategory category, const char* siteName = nullptr)` (oraz aliasy `ValidateDrawCall`, `CheckDrawAllowed`): sprawdza dozwolenie rysowania i w razie wykrycia omijania automatycznie wywoluje `RecordBypass`.
  - `void SetEnabled(bool bEnable)` / `bool IsEnabled() const`: flaga wlaczania/wylaczania detektora.
  - `static const char* CategoryToString(RenderOwnership::ERenderCategory category)`: konwersja identyfikatora kategorii na czytelna nazwe tekstowa.
  - `static bool CheckBypass(RenderOwnership::ERenderCategory category, const char* siteName = nullptr)`: statyczna metoda pomocnicza bezpieczna w przypadku braku zainicjalizowanego singletonu.

### 2.2. RAII Scope Helper: CRenderCategoryScope
- Pomocniczy straznik zakresu `CRenderCategoryScope`, ktory ustawia biezaca kategorie w `CStateManager` na czas trwania bloku i przywraca poprzednia w destruktorze.

### 2.3. Rozszerzenie RenderOwnership (`src/EterLib/RenderOwnership.h`)
- Dodanie `using ERenderCategory = ::ERenderCategory;` oraz `using ERenderOwner = ::ERenderOwner;` wewnatrz klasy `RenderOwnership`, zapewniajac zgodnosc ze specyfikacja `RenderOwnership::ERenderCategory`.

### 2.4. Integracja z CStateManager (`src/EterLib/StateManager.h`, `src/EterLib/StateManager.cpp`)
- Dodanie metod zarzadzania biezaca kategoria renderowania:
  - `void SetCurrentRenderCategory(RenderOwnership::ERenderCategory category)`
  - `RenderOwnership::ERenderCategory GetCurrentRenderCategory() const`
  - `void RecordDrawCall(const char* siteName = nullptr)`
- Pole `m_currentRenderCategory` zainicjalizowane na `ERenderCategory::Count` (brak narzutu przy braku aktywnej kategorii).
- Wstrzykniecie `RecordDrawCall` do metod rysujacych prymitywy:
  - `DrawPrimitive`
  - `DrawPrimitiveUP`
  - `DrawIndexedPrimitive` (oba warianty przeciazone)
  - `DrawIndexedPrimitiveUP`

### 2.5. Test jednostkowy (`tests/test_c26_render_bypass_guard.cpp`, `tests/CMakeLists.txt`)
- Utworzenie celu testowego `test_c26_render_bypass_guard` i rejestracja testu `RenderBypassGuardTest`.
- Pokrycie testowe:
  1. `RenderBypassGuard - Initial State and Reset`: weryfikacja poczatkowego stanu zerowego i resetowania.
  2. `RenderBypassGuard - Direct Bypass Counting`: weryfikacja zliczania per kategoria i bezpieczenstwa out-of-bounds.
  3. `RenderBypassGuard - ValidateDraw with RenderOwnership`: weryfikacja integracji z wlasnoscia `RenderOwnership` (dozwolone dla Legacy, zliczanie bypassow dla D3D9Backend i Wgpu, dynamiczna zmiana wlasciciela).
  4. `RenderBypassGuard - Enable Disable Flag`: weryfikacja flagi deaktywacji straznika.
  5. `RenderBypassGuard - Report Formatting and Content`: weryfikacja formatowania raportu tekstowego.
  6. `RenderBypassGuard - CategoryToString Conversion`: pelne pokrycie konwersji wszystkich kategorii.
  7. `RenderBypassGuard - StateManager Integration and RAII Scope`: weryfikacja integracji z `STATEMANAGER` oraz zachowania `CRenderCategoryScope`.

## 3. Wyniki kompilacji i testow
- Kompilacja celu testowego:
  `cmake --build build --config Release --target test_c26_render_bypass_guard -- /m:1` -> STATUS SUCCESS
- Wynik wykonania testu `test_c26_render_bypass_guard.exe`:
  - 7/7 test cases passed (100%)
  - 89/89 assertions passed
  - 0 failed
- Zgodnosc z instrukcja: nie skompilowano UserInterface, zbudowano wylacznie cel testowy i wymagane zaleznosci statyczne.
