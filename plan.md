1. Add `state` module to `rust_bridge/src/lib.rs`.
2. Implement `rust_bridge/src/state/mod.rs` to export `actor_manager`.
3. Implement `rust_bridge/src/state/actor_manager.rs`.
   - Create `Actor` struct containing at least `vid` (Virtual ID), `x` and `y` coordinates.
   - Create `ActorManager` struct holding a collection of actors (e.g., `HashMap<u32, Actor>` where the key is the VID).
   - Implement methods for adding/updating actors, removing actors, getting an actor by VID, and finding all actors within a radius R from a given coordinate (x,y).
   - Ensure these methods are safe, returning `Option` or handling out-of-bounds errors gracefully.
   - Polish comments must NOT use diacritical marks (np. uzywamy "z", "l", "s", "c", "a", "e", "n", "o" zamiast "ż/ź", "ł", "ś", "ć", "ą", "ę", "ń", "ó").
4. Write unit tests in `rust_bridge/tests/test_state_actor_manager.rs`.
   - Test adding, removing, updating.
   - Test retrieving by valid/invalid VID.
   - Test querying within radius R.
   - Verify `cargo test` passes.
5. Do pre-commit steps.
