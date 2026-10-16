#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameItemBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include <cstring>
#include <string>

using namespace Client::Bridge;

TEST_CASE("PhaseGameItemBridge - HandleItemSet") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemSet pack{};
    pack.pos.cell = 15;
    pack.vnum = 12345;
    pack.count = 200;

    bool result = bridge.HandleItemSet(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "player");
    CHECK(history[0].methodName == "SetItemData");
    REQUIRE(history[0].args.size() == 3);
    CHECK(history[0].args[0] == "15");
    CHECK(history[0].args[1] == "12345");
    CHECK(history[0].args[2] == "200");
}

TEST_CASE("PhaseGameItemBridge - HandleItemDel") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemDel pack{};
    pack.pos.cell = 42;

    bool result = bridge.HandleItemDel(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "player");
    CHECK(history[0].methodName == "SetItemCount");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "42");
    CHECK(history[0].args[1] == "0");
}

TEST_CASE("PhaseGameItemBridge - HandleItemOwnership") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemOwnership pack{};
    pack.dwVID = 98765;
    std::strncpy(pack.szName, "TestOwner", sizeof(pack.szName) - 1);
    pack.szName[sizeof(pack.szName) - 1] = '\0';

    bool result = bridge.HandleItemOwnership(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "item");
    CHECK(history[0].methodName == "SetOwnership");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "98765");
    CHECK(history[0].args[1] == "TestOwner");
}

TEST_CASE("PhaseGameItemBridge - HandleItemOwnership without null terminator") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemOwnership pack{};
    pack.dwVID = 1111;
    std::memset(pack.szName, 'A', sizeof(pack.szName));

    bool result = bridge.HandleItemOwnership(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "item");
    CHECK(history[0].methodName == "SetOwnership");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "1111");
    std::string expectedName(CHARACTER_NAME_MAX_LEN + 1, 'A');
    CHECK(history[0].args[1] == expectedName);
}

TEST_CASE("PhaseGameItemBridge - HandleItemGroundAdd") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemGroundAdd pack{};
    pack.dwVID = 5555;
    pack.dwVnum = 19;
    pack.lX = 1000;
    pack.lY = 2000;
    pack.lZ = 3000;

    bool result = bridge.HandleItemGroundAdd(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "item");
    CHECK(history[0].methodName == "AddItem");
    REQUIRE(history[0].args.size() == 5);
    CHECK(history[0].args[0] == "5555");
    CHECK(history[0].args[1] == "19");
    CHECK(history[0].args[2] == "1000");
    CHECK(history[0].args[3] == "2000");
    CHECK(history[0].args[4] == "3000");
}

TEST_CASE("PhaseGameItemBridge - HandleItemGroundDel") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameItemBridge bridge(mockRuntime);

    TPacketGCItemGroundDel pack{};
    pack.vid = 5555;

    bool result = bridge.HandleItemGroundDel(pack);
    CHECK(result == true);

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "item");
    CHECK(history[0].methodName == "DelItem");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "5555");
}
