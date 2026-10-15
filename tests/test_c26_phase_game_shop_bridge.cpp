#include <gtest/gtest.h>
#include <gmock/gmock.h>
#include "Client/Bridge/PhaseGameShopBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "UserInterface/Contracts/ShopInventoryModel.h"
#include <cstring>
#include <vector>

using namespace testing;
using namespace Client::Bridge;
using namespace EterBase;

class PhaseGameShopBridgeTest : public Test {
protected:
    MockPythonRuntime mockRuntime;
    UserInterface::Domain::ShopInventoryModel shopModel;
    PhaseGameShopBridge bridge{mockRuntime, shopModel};

    void SetUp() override {
        mockRuntime.Initialize();
        mockRuntime.RegisterMock("game", "OnShopError", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "StartShop", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "EndShop", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "RefreshShop", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "SetShopSellingPrice", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "BINARY_PrivateShop_Appear", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "BINARY_PrivateShop_Disappear", [](std::span<const std::string> args) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    }

    void TearDown() override {
        mockRuntime.Shutdown();
    }
};

TEST_F(PhaseGameShopBridgeTest, HandleShopEnd_Success) {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::END;

    std::vector<uint8_t> payload;
    EXPECT_TRUE(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "EndShop");
}

TEST_F(PhaseGameShopBridgeTest, HandleShopError_NotEnoughMoney) {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::NOT_ENOUGH_MONEY;

    std::vector<uint8_t> payload;
    EXPECT_TRUE(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "OnShopError");
    EXPECT_THAT(history[0].args, ElementsAre("NOT_ENOUGH_MONEY"));
}

TEST_F(PhaseGameShopBridgeTest, HandleShopError_SoldOut) {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::SOLDOUT;

    std::vector<uint8_t> payload;
    EXPECT_TRUE(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "OnShopError");
    EXPECT_THAT(history[0].args, ElementsAre("SOLDOUT"));
}

TEST_F(PhaseGameShopBridgeTest, HandleShopSign_Appear) {
    TPacketGCShopSign pack{};
    pack.header = 0x1B;
    pack.dwVID = 55443;
    std::strncpy(pack.szSign, "SuperSklep", sizeof(pack.szSign) - 1);

    EXPECT_TRUE(bridge.HandleShopSign(pack, false, false));

    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "BINARY_PrivateShop_Appear");
    EXPECT_THAT(history[0].args, ElementsAre("55443", "SuperSklep"));
}

TEST_F(PhaseGameShopBridgeTest, HandleShopSign_Disappear) {
    TPacketGCShopSign pack{};
    pack.header = 0x1B;
    pack.dwVID = 55443;
    pack.szSign[0] = '\0';

    EXPECT_TRUE(bridge.HandleShopSign(pack, false, false));

    auto history = mockRuntime.GetHistory();
    ASSERT_EQ(history.size(), 1);
    EXPECT_EQ(history[0].moduleName, "game");
    EXPECT_EQ(history[0].methodName, "BINARY_PrivateShop_Disappear");
    EXPECT_THAT(history[0].args, ElementsAre("55443"));
}
