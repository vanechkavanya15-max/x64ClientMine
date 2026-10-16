1. Verify the current `PhaseGameTargetBridge` implementation in `src/UserInterface/PythonNetworkStreamPhaseGameTarget.cpp` translates correctly into `Invoke` commands in C++23.
   - `HandleTargetCreate`: Minimap target creation, then calling Python UI window open for target board/atlas.
   - `HandleTargetUpdate`: Minimap and background updates.
   - `HandleTargetDelete`: Minimap and background delete.
   - `HandleCreateFly`: Using flying object manager to trigger particle effect trajectory.

2. I will write a C++23 class in `src/Client/Bridge/PhaseGameTargetBridge.h` and `.cpp` following exactly the same signatures needed for the packets, and using `Client::Bridge::IPythonRuntime& m_pythonRuntime`.
   - `bool HandleTargetCreate(const TPacketGCTargetCreate& pack);` -> Wait! The payload parsing needs to use `std::span<const uint8_t> payload`. So I should take either `span` or use `Decode...` and pass the packet. Let's make it accept `std::span<const uint8_t> payload` and return `EterBase::PacketResult<void>`, similar to how modern packet handlers do it. Or, since this isn't IPacketHandler (because there are 4 packets), I can define 4 separate handlers or just 4 methods:
     `EterBase::PacketResult<void> HandleTargetCreate(std::span<const uint8_t> payload);`
     `EterBase::PacketResult<void> HandleTargetUpdate(std::span<const uint8_t> payload);`
     `EterBase::PacketResult<void> HandleTargetDelete(std::span<const uint8_t> payload);`
     `EterBase::PacketResult<void> HandleCreateFly(std::span<const uint8_t> payload);`
   
   - If I look at the old tests, they pass `TPacketGCTargetCreate` by reference. I'll make it take `std::span<const uint8_t> payload` instead, using `Network::TargetPacketCodec` directly!

3. I will completely rewrite `tests/test_c26_phase_game_target_bridge.cpp`. The rewritten test will:
   - `#include "Client/Bridge/PhaseGameTargetBridge.h"`
   - `#include "Client/Bridge/MockPythonRuntime.h"`
   - Create `MockPythonRuntime mockRuntime;`
   - Instantiate `PhaseGameTargetBridge bridge(mockRuntime);`
   - Construct raw packets using structs, cast to `std::span<const uint8_t>`, and call the bridge methods.
   - Check `mockRuntime.GetCallCount(...)` or verify standard mock behavior.
   - Compile and execute using `g++ -std=c++23 tests/test_c26_phase_game_target_bridge.cpp src/Client/Bridge/PhaseGameTargetBridge.cpp src/Client/Network/TargetPacketCodec.cpp -I src/ -o test_bin && ./test_bin`.

Let's test this assumption before planning.
