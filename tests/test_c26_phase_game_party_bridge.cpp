#include "doctest.h"
#include "Client/Bridge/PhaseGamePartyBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <cstring>
#include <vector>

using namespace Client::Bridge;
using namespace EterBase;

TEST_CASE("PhaseGamePartyBridge - HandlePartyInvite") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    mockRuntime.RegisterMock("game", "RecvPartyInviteQuestion", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    
    PhaseGamePartyBridge bridge{mockRuntime};
    
    TPacketGCPartyInvite pack{};
    pack.header = 0x01; // GC::PARTY_INVITE
    pack.length = sizeof(TPacketGCPartyInvite);
    pack.leader_pid = 12345;

    std::string_view leaderName = "Alice";

    CHECK(bridge.HandlePartyInvite(pack, leaderName));
    
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "RecvPartyInviteQuestion");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "12345");
    CHECK(history[0].args[1] == "Alice");
}

TEST_CASE("PhaseGamePartyBridge - HandlePartyAdd") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    mockRuntime.RegisterMock("game", "AddPartyMember", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    
    PhaseGamePartyBridge bridge{mockRuntime};

    TPacketGCPartyAdd pack{};
    pack.header = 0x02; // GC::PARTY_ADD
    pack.length = sizeof(TPacketGCPartyAdd);
    pack.pid = 67890;
    std::strncpy(pack.name, "Bob", sizeof(pack.name) - 1);
    pack.name[sizeof(pack.name) - 1] = '\0';

    CHECK(bridge.HandlePartyAdd(pack));
    
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "AddPartyMember");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "67890");
    CHECK(history[0].args[1] == "Bob");
}

TEST_CASE("PhaseGamePartyBridge - HandlePartyUpdate") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    mockRuntime.RegisterMock("game", "UpdatePartyMemberInfo", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    
    PhaseGamePartyBridge bridge{mockRuntime};

    TPacketGCPartyUpdate pack{};
    pack.header = 0x03; // GC::PARTY_UPDATE
    pack.length = sizeof(TPacketGCPartyUpdate);
    pack.pid = 112233;
    pack.state = 1;
    pack.percent_hp = 100;

    CHECK(bridge.HandlePartyUpdate(pack));
    
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "UpdatePartyMemberInfo");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "112233");
}

TEST_CASE("PhaseGamePartyBridge - HandlePartyRemove") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    mockRuntime.RegisterMock("game", "RemovePartyMember", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    
    PhaseGamePartyBridge bridge{mockRuntime};

    TPacketGCPartyRemove pack{};
    pack.header = 0x04; // GC::PARTY_REMOVE
    pack.length = sizeof(TPacketGCPartyRemove);
    pack.pid = 998877;

    CHECK(bridge.HandlePartyRemove(pack));
    
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "RemovePartyMember");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "998877");
}

TEST_CASE("PhaseGamePartyBridge - HandlePartyParameter") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    mockRuntime.RegisterMock("game", "ChangePartyParameter", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    
    PhaseGamePartyBridge bridge{mockRuntime};

    TPacketGCPartyParameter pack{};
    pack.header = 0x05; // GC::PARTY_PARAMETER
    pack.length = sizeof(TPacketGCPartyParameter);
    pack.bDistributeMode = 2;

    CHECK(bridge.HandlePartyParameter(pack));
    
    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "ChangePartyParameter");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "2");
}
