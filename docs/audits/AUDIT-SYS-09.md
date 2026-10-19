# AUDIT-SYS-09: ScriptLib & Python Interop Core

## 1. Podsumowanie architektury
Analizowane moduły (`src/ScriptLib/`, `src/EterPythonLib/`) pełnią kluczową rolę w integracji logiki gry z Pythonem oraz w systemie graficznego interfejsu użytkownika (UI). `ScriptLib` stanowi warstwę abstrakcji nad C-API Pythona, oferując pomocnicze funkcje rzutowania, bindowania i zarządzania wyjątkami. `EterPythonLib` zawiera implementację menedżera okien (UI) oraz grafik (moduł `ui`), hermetyzując komponenty D3D, logikę aktualizacji i cykl życia widgetów okienkowych (`CWindow`, `CSlotWindow`, itd.).

Oba moduły stosują jednak wzorce projektowe C++ z epoki wczesnych lat 2000, które nie odpowiadają współczesnym standardom C++20/C++23 czy Rust. Powszechnie występują surowe wskaźniki własnościowe, jawne zarządzanie pamięcią (manualne `new` / `delete`), makra zastępujące funkcje oraz niska odporność na błędy wykonania.

## 2. Zestawienie zidentyfikowanych defektów

### A. Zarządzanie zasobami i cykl życia obiektów (Memory Lifetime & RAII)
- **Problem (Wycieki pamięci w drzewach UI):** Zarządzanie oknami opiera się na ręcznym alokowaniu przez `new` i wywołaniach `delete` (np. `src/EterPythonLib/PythonWindowManager.cpp`, `__NewWindow`, `RegisterWindow`). Okna mają cykl życia oparty o ręcznie zarządzane "ReserveDeleteWindowList". W przypadku rzucenia wyjątku przed zarejestrowaniem okna, dojdzie do wycieku pamięci.
- **Problem (Zarządzanie referencjami Pythona):** W module `src/ScriptLib/PythonUtils.cpp` referencje są zarządzane za pomocą jawnego wywoływania `Py_DECREF` / `Py_XDECREF`. Istnieje wiele gałęzi powrotu (wczesne `return`), gdzie poprawne zwolnienie zasobu polega na skrupulatności programisty, co jest bardzo podatne na błędy (np. linie 243, 261).
- **Problem (Surowe wskaźniki przy odczycie buforów):** Bufor na zrzut ekranu w `src/EterPythonLib/PythonGraphic.cpp` alokowany jest przez `new uint8_t[]` (linia 302). Zwracanie go wymaga upewnienia się, że zostanie bezpiecznie usunięty, a błąd w dalszej logice spowoduje wyciek wielomegabajtowego bufora.

### B. Poprawność współbieżności (Thread Safety & Synchronization)
- **Problem (Brak synchronizacji mutowalnego stanu statycznego):** W repozytorium występuje kilka struktur globalnych (np. `g_stTraceBuffer`, zmienne flagowe UI, np. `g_bOutlineBoxEnable` w `PythonWindow.cpp`), które nie są chronione za pomocą prymitywów synchronizacji. Moduł graficzny oraz wywołania z Pythona mogą powodować równoczesny dostęp, jeśli interpreter nie posiada prawidłowo zapiętego GIL (Global Interpreter Lock) bądź jeśli interakcje sieciowe zachodzą poza głównym wątkiem. 
- **Zagrożenie:** O ile Python i renderer operują stricte w głównym wątku, zagrożenie jest odłożone w czasie. W przypadku skalowania architektury na wielowątkowość (np. osobny wątek renderowania i logiki) system UI i parser Pythona będą cierpieć na tzw. race conditions.

### C. Odporność parsera i operacje na buforach
- **Problem (Częściowe paczki / przestarzałe API C):** W `src/EterPythonLib/PythonGraphic.cpp` parsowanie nagłówków plików (.sub/.pyc) posługuje się operacjami typu `fread()` bezpośrednio po `malloc()` bez sprawdzania granic i typów na nowoczesnych widokach (np. `std::span`). (Linie 409-415). Błędy I/O, krótkie odczyty lub złośliwe spreparowane pliki mogą łatwo spowodować przepełnienia bufora i luki bezpieczeństwa (Buffer Overflow).
- **Problem (Rzutowania rozmiarów):** Rzutowanie wyników funkcji I/O, takich jak `ftell()`, do `size_t` bywa problematyczne na architekturach 32-bitowych lub w przypadkach plików większych niż dopuszczalny stan bufora. 

### D. Wydajność na ścieżce krytycznej
- **Problem (Fragmentacja sterty):** Moduł UI w locie tworzy drobne obiekty okien (`CImageBox`, `CSlotButton`, `CNumberLine`) i na bieżąco iteruje przez powiązane obiekty używając standardowych map, zestawów i list `std::map`, `std::list`. Przechodzenie po drzewach okien (funkcje `Update()` i `Render()`) na tak rozproszonej strukturze całkowicie niweczy lokalność cache'a procesora (Cache Miss).
- **Problem (Zbyt duża ilość dynamicznych rzutowań rTTI):** Operacje interaktywnego UI, wywołujące dynamicznie funkcje Pythona poprzez nazwy (wyszukiwanie metod po stringach), narzucają nadmiarowy koszt overheadu w każdej klatce (metody z prefiksem `__PyCallClassMemberFunc_ByCString`).

## 3. Propozycja refaktoryzacji - Nowoczesny C++23 / Rust

### Opcja A (C++23 Modernization)
1. **Inteligentne Wskaźniki (Smart Pointers):** Usunięcie wszystkich ręcznych wywołań `new` i `delete` (oraz `malloc`/`free`).
   - Zmiana list i map okien na przechowywanie `std::unique_ptr<CWindow>` (dla okien osieroconych) lub `std::shared_ptr<CWindow>` w drzewie okien. Zależności powinny być modelowane przez `std::weak_ptr<CWindow>`. 
   - Refaktoryzacja `PythonWindowManager` do pracy na unikalnych wskaźnikach własnościowych.
2. **RAII dla Python C-API (np. PyBind11 / std::unique_ptr z custom deleter):** Stworzenie klas opakowujących własnościowe wskaźniki `PyObject*`, np.:
   ```cpp
   struct PyObjectDeleter { void operator()(PyObject* p) const { Py_XDECREF(p); } };
   using PyObjectPtr = std::unique_ptr<PyObject, PyObjectDeleter>;
   ```
3. **Zastąpienie operacji na buforach i `fread` przez `std::span` i `Memory Mapped Files` (MMF):** Ograniczy to alokacje pośrednie. Pliki powinny być ładowane bezpośrednio do ciągłych widoków. 
4. **Data-Oriented Design w UI:** Spłaszczenie hierarchii obiektów renderowalnych do liniowych wektorów (`std::vector<RenderCommand>`) przygotowywanych w czasie aktualizacji logiki okien, poprawiając spójność cache'u.

### Opcja B (Rust Rewrite)
W przypadku pełnej modularyzacji w Rust, cała struktura zyska na gwarancjach borrow-checkera.
1. **PyO3 Integration:** Przerzucenie integracji z Pythonem na bezpieczne ramy PyO3. Wszelkie incydenty typu use-after-free, wycieki referencji Pythona zostaną systemowo uniemożliwione przez odpowiednie wiązania żywotności (Lifetimes `<'py>`).
2. **Architektura ECS dla UI:** Zamiast głębokich drzew dziedziczenia klas okien, wdrożenie modelu Entity Component System (ECS, np. biblioteka `bevy_ecs` lub `hecs`). Każdy widget będzie encją z komponentami (np. `Position`, `Renderable`, `Clickable`), co diametralnie poprawi przetwarzanie i całkowicie wyeliminuje wiszące wskaźniki na obiekty potomne okienek.
3. **Typy bazowe:** Przejście z globalnych mutowalnych wskaźników na wstrzykiwanie zasobów (`Context` / `State`) z wymuszonymi bezpiecznymi referencjami kompilatora.
