1. **Utworzenie naglowka `UIWindowHierarchyTree.h` w `src/Client/UI/`**
   - Definicja silnego typu identyfikatora okna bazujaca na `Client::Core::StrongType` (np. `WindowId`).
   - Stworzenie struktury bledu domenowego `HierarchyError` za pomoca `enum class`.
   - Klasa `UIWindowHierarchyTree` hermetyzujaca relacje okien (AddWindow, RemoveWindow, SetParent, BringToFront, SendToBack, IsAncestor) i zwracajaca wyniki poprzez `std::expected<void, HierarchyError>`.
   - Zastosowanie struktury przechowujacej dane, bez wyciekow RAII - tylko identyfikatory.
   - Zachowanie w pelni niezaleznej architektury, nie korzystajac z plikow API systemu Windows czy DirectX.

2. **Implementacja struktury `UIWindowHierarchyTree.cpp` w `src/Client/UI/`**
   - Dodawanie i usuwanie wezlow z dbaloscia o cykle zaleznosci oraz odpowiednie zarzadzanie dziecmi podczas usuwania kaskadowego.
   - Sortowanie (zmiana Z-order) poprzez przestawianie w wektorze `children`.

3. **Utworzenie pliku testowego `tests/test_c26_ui_hierarchy.cpp`**
   - Testowanie happy path oraz edge cases, takich jak proby utworzenia cyklu w drzewie, niepoprawnego usuwania lub przypisywania do nieistniejacych rodzicow.
   - Uzycie dyrektywy `#define TEST_MODE_DISABLE_STDAFX` do zachowania trybu headless na srodowisku Linuksowym.
   - Bezpieczna ewaluacja enuma błędu w testach poprzez `EXPECT_TRUE(a == b)` zamiast `EXPECT_EQ`.

4. **Kompilacja testow i weryfikacja**
   - Kompilacja testu za pomoca g++ w kontenerze z C++23: `g++ -std=c++23 -Isrc -Isrc/Client -Itests tests/test_c26_ui_hierarchy.cpp src/Client/UI/UIWindowHierarchyTree.cpp -o tests/test_c26_ui_hierarchy`
   - Wykonanie binarnego pliku z testami by potwierdzic `100% PASS`.
   - Posprzatanie utworzonych binarek (`rm`).

5. **Wywolanie Pre-commit i zakonczenie zadania**
   - Realizacja procedur `pre_commit_instructions`, nastepnie push (commit z opisem dzialania wg. SRP) za pomoca narzedzia `submit`.
