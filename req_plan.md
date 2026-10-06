Plan:
1. Implement the core mining state logic in `crates/m2-client/src/mining_vein_drill_state_v2.rs` as specified:
   - Target tracking (vein VID)
   - Digging logic (durations, animations checks)
   - Success/Fail math (`150` base chance + skill level bonuses)
   - Pickaxe level limits checks
   - Export FFI functions for C++ to hook into (matching `OnCannotMining` etc.)
2. Add comprehensive unit tests to `crates/m2-client/tests/test_mining_vein_drill_state_v2.rs` verifying 100% of the public Rust module's logic and states.
3. Call `pre_commit_instructions` and follow them to ensure tests pass.
4. Verify tests pass with `RUSTFLAGS="-l sodium" SODIUM_USE_PKG_CONFIG=1 cargo test --package m2-client --test test_mining_vein_drill_state_v2`
5. Submit changes.
