1. *Zaimplementuj `rust_bridge/src/state/target.rs`.*
   - Zdefiniuj strukturę przechowującą informacje o celu: VID (identyfikator), pozycję, czy cel żyje.
   - Zdefiniuj Menedżer Celu (`TargetManager`), który pozwala na:
     - Ustawienie aktualnego celu na podstawie VID.
     - Czyszczenie celu (brak celu).
     - Obliczanie odległości od gracza (od podanej pozycji).
     - Walidację czy cel żyje.
     - Bezpieczne przechowywanie stanu np. za pomocą globalnego `Mutex` / `RwLock` lub `thread_local`, zależnie od wymagań FFI, jednak polecenie skupia się na logice `TargetManager` i odporności na błędne VID.
   - Udostępnij w Rust odpowiednie metody dla Menedżera, zwracając ostrożnie typy wskaźnikowe jeśli byłyby użyte, a polegając głównie na bezpiecznych operacjach na obiektach (Option).
2. *Dodaj moduł do `rust_bridge/src/state/mod.rs` i zarejestruj w `lib.rs`.*
   - Zarejestruj `state` jako moduł w `lib.rs`.
   - W `mod.rs` upublicznij moduł `target`.
3. *Zaimplementuj testy w `rust_bridge/tests/test_state_target.rs`.*
   - Przetestuj ustawianie i czyszczenie celu.
   - Przetestuj weryfikację czy cel żyje.
   - Przetestuj obliczanie odległości (distance).
   - Przetestuj reakcje na błędne identyfikatory (VID).
4. *Wykonaj kroki pre commit*
   - Wykonaj polecenia z `pre_commit_instructions` w celu uruchomienia testów w tym `cargo test` i sprawdzenia poprawności.
5. *Zatwierdź zmiany (Submit).*
