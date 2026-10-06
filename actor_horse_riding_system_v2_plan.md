1. **Analyze Requirements and Source Material:** The task is to create `actor_horse_riding_system_v2.rs` as a 100% C++ parity equivalent of the horse riding mechanisms found in `CInstanceBase::SHORSE` and `CInstanceBase::MountHorse` / `DismountHorse` / `CanAttackOnMount`. 
   - Found relevant source code in `src/UserInterface/InstanceBase.cpp` & `.h`.
   - `CInstanceBase::SHORSE::GetLevel()` defines levels 1, 2, 3 based on mount VNUM.
   - `CanAttack()` is true if `GetLevel() > 1`.
   - `CanUseSkill()` is true if `GetLevel() > 2` (also handled by existing `horse_combat_skill_engine.rs` but we should probably expose basic parity here for attacks).
   - Motion modes: `MODE_HORSE` (which is enum index 8) up to `MODE_HORSE_BELL` (index 14) in `CRaceMotionData::EMode`.
2. **Create Target File**: `crates/m2-client/src/actor_horse_riding_system_v2.rs` using `run_in_bash_session`.
   - Implement `HorseLevel` logic converting mount vnums to levels (1, 2, 3).
   - Implement `HorseRidingSystem` struct.
   - Add `mount(&mut self, mount_vnum: u32)`, `dismount(&mut self)`.
   - Add `is_mounted(&self) -> bool`.
   - Add `can_attack(&self) -> bool` (level > 1).
   - Add `can_use_skill(&self) -> bool` (level > 2).
   - Add function to map base motion mode to horse motion mode (e.g. `MODE_GENERAL` -> `MODE_HORSE`, `MODE_ONEHAND_SWORD` -> `MODE_HORSE_ONEHAND_SWORD`).
3. **Verify Target File**: Run `cat` on `crates/m2-client/src/actor_horse_riding_system_v2.rs` to ensure it was created correctly.
4. **Create Test File**: `crates/m2-client/tests/test_actor_horse_riding_system_v2.rs` using `run_in_bash_session`.
   - Import the target file directly using `#[path = "../src/actor_horse_riding_system_v2.rs"] mod actor_horse_riding_system_v2;`.
   - Test cases:
     - `test_mount_dismount`
     - `test_horse_level_mapping` (vnums 20101->1, 20110->3, etc.)
     - `test_can_attack` (level 1 -> false, level 2 -> true, level 3 -> true)
     - `test_can_use_skill` (level 1 -> false, level 2 -> false, level 3 -> true)
     - `test_motion_mode_mapping`
5. **Verify Test File**: Run `cat` on the test file to ensure it is created correctly.
6. **Execute Tests**: Run `cargo test --package m2-client` to verify everything works and passes 100%.
7. **Pre-commit Steps**: Call `pre_commit_instructions` tool to complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8. **Submit**: Call `submit` with branch 'pure-rust-main'.
