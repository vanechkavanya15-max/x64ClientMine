#include "doctest/doctest.h"
#include "Client/Bridge/PhaseGameDuelPvpBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "EterBase/Result.h"
#include <vector>

using namespace Client::Bridge;
using namespace EterBase;

TEST_CASE("PhaseGameDuelPvpBridge: HandlePvpPacket") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameDuelPvpBridge bridge(mockRuntime);

    TPacketGCPVP packet{};
    packet.header = 0x0414;
    packet.length = sizeof(TPacketGCPVP);
    packet.dwVIDSrc = 100;
    packet.dwVIDDst = 200;

    SUBCASE("PVP_MODE_AGREE") {
        packet.bMode = PVP_MODE_AGREE;
        auto res = bridge.HandlePvpPacket(packet);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 4);
        CHECK(history[0].methodName == "RemovePVPKey");
        CHECK(history[1].methodName == "RememberChallengeInstance");
        CHECK(history[2].methodName == "RefreshTargetBoardByVID");
        CHECK(history[3].methodName == "RefreshTargetBoardByVID");
    }

    SUBCASE("PVP_MODE_REVENGE") {
        packet.bMode = PVP_MODE_REVENGE;
        auto res = bridge.HandlePvpPacket(packet);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 4);
        CHECK(history[0].methodName == "RemovePVPKey");
        CHECK(history[1].methodName == "RememberRevengeInstance");
    }

    SUBCASE("PVP_MODE_FIGHT") {
        packet.bMode = PVP_MODE_FIGHT;
        auto res = bridge.HandlePvpPacket(packet);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 5);
        CHECK(history[0].methodName == "InsertPVPKey");
        CHECK(history[1].methodName == "ForgetInstance");
        CHECK(history[2].methodName == "ForgetInstance");
    }

    SUBCASE("PVP_MODE_NONE") {
        packet.bMode = PVP_MODE_NONE;
        auto res = bridge.HandlePvpPacket(packet);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 5);
        CHECK(history[0].methodName == "RemovePVPKey");
        CHECK(history[1].methodName == "ForgetInstance");
    }

    SUBCASE("Invalid Mode") {
        packet.bMode = 99;
        auto res = bridge.HandlePvpPacket(packet);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == PacketError::MalformedPayload);
    }
}

TEST_CASE("PhaseGameDuelPvpBridge: HandleDuelStartPacket") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    PhaseGameDuelPvpBridge bridge(mockRuntime);

    SUBCASE("Valid packet with pairs") {
        std::vector<uint32_t> pairs = {200, 300};
        auto res = bridge.HandleDuelStartPacket(pairs, 100);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 5);
        CHECK(history[0].methodName == "InsertDUELKey");
        CHECK(history[0].args[0] == "100");
        CHECK(history[0].args[1] == "200");
        
        CHECK(history[1].methodName == "InsertDUELKey");
        CHECK(history[1].args[0] == "100");
        CHECK(history[1].args[1] == "300");

        CHECK(history[2].methodName == "SetDuelMode");
        CHECK(history[2].args[0] == "1"); // DUEL_START

        CHECK(history[3].methodName == "CloseTargetBoard");
        CHECK(history[4].methodName == "RefreshAllPCTextTail");
    }

    SUBCASE("Valid packet empty pairs (cannot attack)") {
        std::vector<uint32_t> pairs;
        auto res = bridge.HandleDuelStartPacket(pairs, 100);
        CHECK(res.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 3);
        CHECK(history[0].methodName == "SetDuelMode");
        CHECK(history[0].args[0] == "2"); // DUEL_CANNOTATTACK
    }

    SUBCASE("Invalid main character vid") {
        std::vector<uint32_t> pairs = {200};
        auto res = bridge.HandleDuelStartPacket(pairs, 0);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == PacketError::MalformedPayload);
    }
}

TEST_CASE("PhaseGameDuelPvpBridge: Duel lifecycle") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    Client::Bridge::PhaseGameDuelPvpBridge bridge(mockRuntime);
    
    CHECK_FALSE(bridge.IsInDuel());
    bridge.StartDuel(4002);
    CHECK(bridge.IsInDuel());
    CHECK(bridge.GetOpponentVid() == 4002);
    bridge.EndDuel();
    CHECK_FALSE(bridge.IsInDuel());
}
