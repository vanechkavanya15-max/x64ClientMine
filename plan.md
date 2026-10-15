1. **Understand Requirements**: Create a new module `UISlotGridController` under `src/Client/UI/`. Ensure 100% test coverage using doctest (`tests/test_c26_ui_slot_grid.cpp`), modern C++23 standards (std::expected, StrongTypes, std::optional, RAII, SRP), Linux compatibility (no DirectX/Windows dependencies), business logic without dummies, 100% isolation (do not touch existing files), headless execution.
2. **Define Data Structures (`UISlotGridController.h`)**:
    - Use `Client::Core::SlotIndex` for slot indexing.
    - Create a struct to represent an item in a slot (e.g., `GridItem` with `ItemVnum`, `count`, maybe `size`).
    - The grid needs dimensions (`width`, `height`).
    - The controller needs methods to add an item to a specific slot, find an empty slot for an item of given dimensions, remove an item, get an item, and move an item.
3. **Define Errors**:
    - Add an enum class `SlotGridError` (e.g., `OutOfBounds`, `SlotOccupied`, `ItemNotFound`, `NotEnoughSpace`).
    - Overload `std::formatter` or provide a `to_string` function.
4. **Implement Business Logic (`UISlotGridController.cpp`)**:
    - `UISlotGridController(uint8_t width, uint8_t height)`
    - `Result<SlotIndex, SlotGridError> PutItem(SlotIndex slot, ItemVnum vnum, uint8_t width, uint8_t height)`
    - `Result<void, SlotGridError> RemoveItem(SlotIndex slot)`
    - `Result<SlotIndex, SlotGridError> FindFreeSlot(uint8_t width, uint8_t height) const`
    - `Result<void, SlotGridError> MoveItem(SlotIndex from, SlotIndex to)`
    - A way to track which grid cells are occupied (e.g., `std::vector<std::optional<SlotIndex>>` where `SlotIndex` points to the top-left cell of the item, or a dedicated `Item` struct).
5. **Write Unit Tests (`tests/test_c26_ui_slot_grid.cpp`)**:
    - Test initialization and dimensions.
    - Test placing 1x1, 1x2, 2x2 items successfully.
    - Test out of bounds placements.
    - Test overlapping placements (should fail).
    - Test finding free slots.
    - Test removing items and freeing up space.
    - Test moving items around.
6. **Compile and Run Tests**: Use CMake or `g++ -std=c++23 ...` to verify tests pass and no memory leaks exist.
7. **Pre-commit**: Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
