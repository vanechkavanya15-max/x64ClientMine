#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameShopBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "UserInterface/Contracts/ShopInventoryModel.h"
#include <cstring>
#include <vector>

using namespace Client::Bridge;
using namespace EterBase;

struct ShopBridgeFixture {
    MockPythonRuntime mockRuntime;
    UserInterface::Domain::ShopInventoryModel shopModel;
    PhaseGameShopBridge bridge{mockRuntime, shopModel};

    ShopBridgeFixture() {
        mockRuntime.Initialize();
        mockRuntime.RegisterMock("game", "OnShopError", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "StartShop", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "EndShop", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "RefreshShop", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "SetShopSellingPrice", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "BINARY_PrivateShop_Appear", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
        mockRuntime.RegisterMock("game", "BINARY_PrivateShop_Disappear", [](std::span<const std::string>) -> Result<std::string, PythonRuntimeError> { return "Success"; });
    }

    ~ShopBridgeFixture() {
        mockRuntime.Shutdown();
    }
};

TEST_CASE_FIXTURE(ShopBridgeFixture, "HandleShopEnd_Success") {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::END;

    std::vector<uint8_t> payload;
    CHECK(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "EndShop");
}

TEST_CASE_FIXTURE(ShopBridgeFixture, "HandleShopError_NotEnoughMoney") {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::NOT_ENOUGH_MONEY;

    std::vector<uint8_t> payload;
    CHECK(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "OnShopError");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "NOT_ENOUGH_MONEY");
}

TEST_CASE_FIXTURE(ShopBridgeFixture, "HandleShopError_SoldOut") {
    TPacketGCShop header{};
    header.header = 0x1A;
    header.subheader = ShopSub::GC::SOLDOUT;

    std::vector<uint8_t> payload;
    CHECK(bridge.HandleShop(header, payload));

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "OnShopError");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "SOLDOUT");
}

TEST_CASE_FIXTURE(ShopBridgeFixture, "HandleShopSign_Appear") {
    TPacketGCShopSign pack{};
    pack.header = 0x1B;
    pack.dwVID = 55443;
    std::strncpy(pack.szSign, "SuperSklep", sizeof(pack.szSign) - 1);

    CHECK(bridge.HandleShopSign(pack, false, false));

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "BINARY_PrivateShop_Appear");
    REQUIRE(history[0].args.size() == 2);
    CHECK(history[0].args[0] == "55443");
    CHECK(history[0].args[1] == "SuperSklep");
}

TEST_CASE_FIXTURE(ShopBridgeFixture, "HandleShopSign_Disappear") {
    TPacketGCShopSign pack{};
    pack.header = 0x1B;
    pack.dwVID = 55443;
    pack.szSign[0] = '\0';

    CHECK(bridge.HandleShopSign(pack, false, false));

    auto history = mockRuntime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "BINARY_PrivateShop_Disappear");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == "55443");
}
