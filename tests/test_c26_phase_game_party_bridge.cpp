#include <gtest/gtest.h>
#include <gmock/gmock.h>
#include "Client/Bridge/PhaseGamePartyBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <cstring>
#include <vector>

using namespace testing;
using namespace Client::Bridge;
using namespace EterBase;

class PhaseGamePartyBridgeTest : public Test {
protected:
    MockPythonRuntime mockRuntime;
    PhaseGamePartyBridge bridge{mockRuntime};

    void SetUp() override {
        mockRuntime.Initialize();
        
        mockRuntime.RegisterMock("game", "RecvPartyInviteQuestion", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "AddPartyMember", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "UpdatePartyMemberInfo", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "RemovePartyMember", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "ChangePartyParameter", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    }
    
    void TearDown() override {
        mockRuntime.Shutdown();
    }
};

TEST_F(PhaseGamePartyBridgeTest, HandlePartyInvite_Success) {
    TPacketGCPartyInvite pack{};
    pack.header = 0x01; // GC::PARTY_INVITE
    pack.length = sizeof(TPacketGCPartyInvite);
    pack.leader_pid = 12345;

    std::string_view leaderName = "Alice";

    EXPECT_TRUE(bridge.HandlePartyInvite(pack, leaderName));
    
    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "RecvPartyInviteQuestion");
    EXPECT_THAT(history[0].args, ElementsAre("12345", "Alice"));
}

TEST_F(PhaseGamePartyBridgeTest, HandlePartyAdd_Success) {
    TPacketGCPartyAdd pack{};
    pack.header = 0x02; // GC::PARTY_ADD
    pack.length = sizeof(TPacketGCPartyAdd);
    pack.pid = 67890;
    std::strncpy(pack.name, "Bob", sizeof(pack.name) - 1);
    pack.name[sizeof(pack.name) - 1] = '\0';

    EXPECT_TRUE(bridge.HandlePartyAdd(pack));
    
    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "AddPartyMember");
    EXPECT_THAT(history[0].args, ElementsAre("67890", "Bob"));
}

TEST_F(PhaseGamePartyBridgeTest, HandlePartyUpdate_Success) {
    TPacketGCPartyUpdate pack{};
    pack.header = 0x03; // GC::PARTY_UPDATE
    pack.length = sizeof(TPacketGCPartyUpdate);
    pack.pid = 112233;
    pack.state = 1;
    pack.percent_hp = 100;

    EXPECT_TRUE(bridge.HandlePartyUpdate(pack));
    
    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "UpdatePartyMemberInfo");
    EXPECT_THAT(history[0].args, ElementsAre("112233"));
}

TEST_F(PhaseGamePartyBridgeTest, HandlePartyRemove_Success) {
    TPacketGCPartyRemove pack{};
    pack.header = 0x04; // GC::PARTY_REMOVE
    pack.length = sizeof(TPacketGCPartyRemove);
    pack.pid = 998877;

    EXPECT_TRUE(bridge.HandlePartyRemove(pack));
    
    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "RemovePartyMember");
    EXPECT_THAT(history[0].args, ElementsAre("998877"));
}

TEST_F(PhaseGamePartyBridgeTest, HandlePartyParameter_Success) {
    TPacketGCPartyParameter pack{};
    pack.header = 0x05; // GC::PARTY_PARAMETER
    pack.length = sizeof(TPacketGCPartyParameter);
    pack.bDistributeMode = 2;

    EXPECT_TRUE(bridge.HandlePartyParameter(pack));
    
    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "ChangePartyParameter");
    EXPECT_THAT(history[0].args, ElementsAre("2"));
}
