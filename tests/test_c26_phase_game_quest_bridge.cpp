#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameQuestBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "Client/Network/QuestPacketCodec.h"
#include <cstring>
#include <iostream>

TEST_CASE("PhaseGameQuestBridge - HandleQuestConfirm") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    // Register mock functions to simulate UI responses
    runtime.RegisterMock("game", "BINARY_OnQuestConfirm", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        if (args.size() != 3) {
            return std::unexpected(Client::Bridge::PythonRuntimeError::InvalidArgument);
        }
        return "Success";
    });

    Client::Bridge::PhaseGameQuestBridge bridge(runtime);

    TPacketGCQuestConfirm pack{};
    pack.header = 0x0911;
    pack.length = sizeof(TPacketGCQuestConfirm);
    std::string testMsg = "Do you accept this quest?";
    std::strncpy(pack.msg, testMsg.c_str(), sizeof(pack.msg) - 1);
    pack.timeout = 30;
    pack.requestPID = 12345;

    bool result = bridge.HandleQuestConfirm(pack);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "BINARY_OnQuestConfirm");
    REQUIRE(history[0].args.size() == 3);
    CHECK(history[0].args[0] == testMsg);
    CHECK(history[0].args[1] == "30");
    CHECK(history[0].args[2] == "12345");
}

TEST_CASE("PhaseGameQuestBridge - HandleQuestInfo") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    runtime.RegisterMock("game", "RefreshQuest", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        return "Success";
    });
    
    runtime.RegisterMock("game", "BINARY_ClearQuest", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        return "Success";
    });

    Client::Bridge::PhaseGameQuestBridge bridge(runtime);

    TPacketGCQuestInfo pack{};
    pack.header = 0x0912;
    pack.length = sizeof(TPacketGCQuestInfo);
    pack.index = 99;
    pack.flag = 0; // Simulate END flag (0)

    bool result = bridge.HandleQuestInfo(pack);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 2);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "BINARY_ClearQuest");
    CHECK(history[1].moduleName == "game");
    CHECK(history[1].methodName == "RefreshQuest");
    
    runtime.ClearHistory();

    // Simulate begin
    pack.flag = QUEST_SEND_IS_BEGIN;
    result = bridge.HandleQuestInfo(pack);
    CHECK(result == true);
    history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "RefreshQuest");
}

TEST_CASE("PhaseGameQuestBridge - HandleScript") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    runtime.RegisterMock("event", "RegisterEventSetFromString", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        return "Success";
    });

    Client::Bridge::PhaseGameQuestBridge bridge(runtime);

    std::string script = "say('Hello, brave warrior!')";
    bool result = bridge.HandleScript(script);
    CHECK(result == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "event");
    CHECK(history[0].methodName == "RegisterEventSetFromString");
    REQUIRE(history[0].args.size() == 1);
    CHECK(history[0].args[0] == script);
}
