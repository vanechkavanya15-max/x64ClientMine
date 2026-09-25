1. Zdefiniowanie struktur w `rust_bridge/src/state/player_stats.rs`:
   - `PlayerStats` - struktura przechowująca statystyki (lvl, EXP, max_exp, HP, MP, punkty statusu STR, DEX, INT, CON).
   - `PlayerStatsManager` - menedżer przechowujący stan gry z bezpiecznym dostępem (np. mapa `VID` do `PlayerStats`). Zabezpieczenie poprzez `RwLock` lub `Mutex` i `lazy_static`/`OnceLock` dla instancji globalnej.
   - Wszystkie metody (gettery, settery) muszą zwracać Option/Result i obsługiwać nieistniejące VID lub błędne operacje.

2. Rejestracja modułu w `rust_bridge/src/lib.rs` (lub `rust_bridge/src/state/mod.rs`).

3. Zaimplementowanie testów jednostkowych w `rust_bridge/tests/test_state_player_stats.rs`:
   - Testowanie poprawnego tworzenia statystyk,
   - Testowanie modyfikacji (dodawanie/odejmowanie punktów/HP),
   - Testowanie dostępu poza zakresem i błędnym VID.

4. Uruchomienie testów: `cargo test --manifest-path rust_bridge/Cargo.toml`.
5. Pre-commit check.
