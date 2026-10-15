#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Bridge/PhaseGameCombatBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"

using namespace Client::Bridge;

TEST_CASE("PhaseGameCombatBridge - HandleDamageInfo") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    mockRuntime.RegisterMock("player", "ShowDamage", [](std::span<const std::string>){ return "ok"; });
    PhaseGameCombatBridge bridge(mockRuntime);

    TPacketGCDamageInfo pack{};
    pack.targetId = 12345;
    pack.damage = 999;
    pack.flag = 1;

    bool result = bridge.HandleDamageInfo(pack);

    CHECK(result == true);
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "player");
    CHECK(history[0].methodName == "ShowDamage");
    
    auto args = history[0].args;
    REQUIRE(args.size() == 3);
    CHECK(args[0] == "12345");
    CHECK(args[1] == "999");
    CHECK(args[2] == "1");
}

TEST_CASE("PhaseGameCombatBridge - HandleDead") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    mockRuntime.RegisterMock("player", "ShowDead", [](std::span<const std::string>){ return "ok"; });
    PhaseGameCombatBridge bridge(mockRuntime);

    TPacketGCDead pack{};
    pack.targetId = 67890;

    bool result = bridge.HandleDead(pack);

    CHECK(result == true);
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "player");
    CHECK(history[0].methodName == "ShowDead");
    
    auto args = history[0].args;
    REQUIRE(args.size() == 1);
    CHECK(args[0] == "67890");
}
