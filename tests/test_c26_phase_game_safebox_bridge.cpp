#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameSafeboxBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <span>
#include <string_view>

TEST_CASE("PhaseGameSafeboxBridge") {
    Client::Bridge::MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();

    // Reusable utility to register mocks to return success
    auto registerSuccessMock = [&mockRuntime](std::string_view module, std::string_view method) {
        mockRuntime.RegisterMock(module, method, [](std::span<const std::string>) {
            return std::string("");
        });
    };

    Client::Bridge::PhaseGameSafeboxBridge bridge(mockRuntime);

    SUBCASE("HandleSafeboxWrongPassword calls OnSafeBoxError in game module") {
        registerSuccessMock("game", "OnSafeBoxError");
        
        bool result = bridge.HandleSafeboxWrongPassword();
        
        CHECK(result == true);
        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "game");
        CHECK(history[0].methodName == "OnSafeBoxError");
        CHECK(history[0].args.empty());
    }

    SUBCASE("HandleSafeboxSize calls OpenSafeboxWindow in game module with correct size") {
        registerSuccessMock("game", "OpenSafeboxWindow");
        
        bool result = bridge.HandleSafeboxSize(3);
        
        CHECK(result == true);
        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "game");
        CHECK(history[0].methodName == "OpenSafeboxWindow");
        REQUIRE(history[0].args.size() == 1);
        CHECK(history[0].args[0] == "3");
    }

    SUBCASE("HandleSafeboxMoneyChange calls RefreshSafeboxMoney in game module") {
        registerSuccessMock("game", "RefreshSafeboxMoney");
        
        bool result = bridge.HandleSafeboxMoneyChange(15000);
        
        CHECK(result == true);
        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "game");
        CHECK(history[0].methodName == "RefreshSafeboxMoney");
        CHECK(history[0].args.empty());
    }

    SUBCASE("HandleMallOpen calls OpenMallWindow in game module with correct size") {
        registerSuccessMock("game", "OpenMallWindow");
        
        bool result = bridge.HandleMallOpen(5);
        
        CHECK(result == true);
        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "game");
        CHECK(history[0].methodName == "OpenMallWindow");
        REQUIRE(history[0].args.size() == 1);
        CHECK(history[0].args[0] == "5");
    }

    SUBCASE("Returns false if python runtime fails (method not mocked)") {
        // Did not register mock, so runtime will return unexpected(MethodNotFound)
        bool result = bridge.HandleSafeboxWrongPassword();
        
        CHECK(result == false);
        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1); // execution is still recorded in history
        CHECK(history[0].methodName == "OnSafeBoxError");
    }
}
