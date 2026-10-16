#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest.h>
#include "Client/Bridge/PhaseGameDungeonBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include <string>

using namespace Client::Bridge;

TEST_CASE("PhaseGameDungeonBridge - HandleDungeon") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameDungeonBridge bridge(mockRuntime);

    SUBCASE("TIME_ATTACK_START") {
        TPacketGCDungeon pack{};
        pack.subheader = DungeonSub::GC::TIME_ATTACK_START;
        
        bool result = bridge.HandleDungeon(pack);
        CHECK(result == true);
        CHECK(mockRuntime.GetHistory().size() == 0);
    }

    SUBCASE("DESTINATION_POSITION - Success") {
        TPacketGCDungeon pack{};
        pack.subheader = DungeonSub::GC::DESTINATION_POSITION;
        
        uint32_t destX = 12345;
        uint32_t destY = 67890;
        
        mockRuntime.RegisterMock("player", "SetDungeonDestinationPosition", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            CHECK(args.size() == 2);
            CHECK(args[0] == "12345");
            CHECK(args[1] == "67890");
            return "success";
        });
        
        bool result = bridge.HandleDungeon(pack, destX, destY);
        CHECK(result == true);
        CHECK(mockRuntime.GetHistory().size() == 1);
        auto& history = mockRuntime.GetHistory()[0];
        CHECK(history.moduleName == "player");
        CHECK(history.methodName == "SetDungeonDestinationPosition");
        CHECK(history.args.size() == 2);
        CHECK(history.args[0] == "12345");
        CHECK(history.args[1] == "67890");
    }

    SUBCASE("DESTINATION_POSITION - Python Failure") {
        TPacketGCDungeon pack{};
        pack.subheader = DungeonSub::GC::DESTINATION_POSITION;
        
        uint32_t destX = 100;
        uint32_t destY = 200;
        
        bool result = bridge.HandleDungeon(pack, destX, destY);
        CHECK(result == false); // Mock is not registered, should return MethodNotFound -> false
    }

    SUBCASE("Unknown Subheader") {
        TPacketGCDungeon pack{};
        pack.subheader = 255;
        
        bool result = bridge.HandleDungeon(pack);
        CHECK(result == false);
        CHECK(mockRuntime.GetHistory().size() == 0);
    }
}
