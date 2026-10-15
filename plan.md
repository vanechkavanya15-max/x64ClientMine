1. **Utworz pliki mostu dla PhaseGame (PhaseGameCombatBridge)**:
   - Uzyj narzedzia `write_file` aby utworzyc plik `src/Client/Bridge/PhaseGameCombatBridge.h` definiujacy interfejs i klase `PhaseGameCombatBridge` (SRP, izolacja dla GC::DAMAGE_INFO, GC::DEAD).
   - Uzyj narzedzia `write_file` aby utworzyc plik `src/Client/Bridge/PhaseGameCombatBridge.cpp` z implementacja metod `HandleDamageInfo(const TPacketGCDamageInfo&)` oraz `HandleDead(const TPacketGCDead&)`. Zostana one uzyte do przekazywania argumentow do UI przez `IPythonRuntime`.

2. **Zweryfikuj utworzone pliki mostu**:
   - Uzyj narzedzia `run_in_bash_session` z poleceniem `cat` i grep aby zweryfikowac ze `PhaseGameCombatBridge.h` i `PhaseGameCombatBridge.cpp` zostaly poprawnie zapisane na dysku.

3. **Dodaj test jednostkowy dla `PhaseGameCombatBridge`**:
   - Uzyj narzedzia `write_file` aby utworzyc plik testu `tests/test_c26_phase_game_combat_bridge.cpp`. Testy uzyja biblioteki `doctest.h` i zmockuja PythonRuntime uzywajac `MockPythonRuntime`. Sprawdza one, czy pakiety sa poprawnie przetwarzane.

4. **Zweryfikuj utworzony plik testowy**:
   - Uzyj narzedzia `run_in_bash_session` z poleceniem `cat` aby sprawdzic czy plik `tests/test_c26_phase_game_combat_bridge.cpp` jest wlasciwie sformatowany i zapisany.

5. **Dodaj nowy cel testu do CMakeLists.txt**:
   - Uzyj narzedzia `replace_with_git_merge_diff` zeby zaktualizowac `tests/CMakeLists.txt`. Nalezy dodac nowa sekcje z `add_executable(test_c26_phase_game_combat_bridge test_c26_phase_game_combat_bridge.cpp)` i innymi wpisami, aby dolaczyc kod.

6. **Zweryfikuj poprawki w CMakeLists.txt**:
   - Uzyj narzedzia `run_in_bash_session` z poleceniem `git diff tests/CMakeLists.txt` zeby sprawdzic czy wpisy sie dodaly poprawnie.

7. **Kompilacja i weryfikacja**:
   - Z racji, ze build przez `cmake` psuje sie czesto na windowsowych zaleznosciach (EterBase - windows.h) w linux kontenerze, uruchomie bezposrednio `run_in_bash_session` do kompilacji testu przez `g++ -std=c++23 -I./src -I./tests tests/test_c26_phase_game_combat_bridge.cpp src/Client/Bridge/PhaseGameCombatBridge.cpp -o test_bin && ./test_bin && rm test_bin`. 

8. **Kroki Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Submit**:
   - Przeprowadz git commit uzywajac narzedzia `submit`.
