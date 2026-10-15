#ifndef TEST_MODE_DISABLE_STDAFX
#define TEST_MODE_DISABLE_STDAFX
#endif

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Bridge/PhaseGameWarpBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <cstring>
#include <iostream>

TEST_CASE("PhaseGameWarpBridge - HandleWarp Local") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    // Register mock functions to simulate UI responses
    runtime.RegisterMock("background", "Destroy", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("background", "Create", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("background", "Warp", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("background", "RefreshShadowLevel", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("app", "SetCenterPositionLocal", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("character", "DeleteAllInstances", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("item", "DeleteAllItems", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("texttail", "Clear", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("effect", "DeleteAllInstances", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("player", "StopAllAction", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });

    Client::Bridge::PhaseGameWarpBridge bridge(runtime);

    TPacketGCWarp pack{};
    pack.header = 0x0305;
    pack.length = sizeof(TPacketGCWarp);
    pack.lX = 1000;
    pack.lY = 2000;
    pack.lAddr = 0;
    pack.wPort = 0; // Local warp

    std::span<const uint8_t> payload(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
    auto result = bridge.Handle(payload);
    CHECK(result.has_value() == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 10);
    CHECK(history[0].moduleName == "background");
    CHECK(history[0].methodName == "Destroy");
    CHECK(history[2].moduleName == "background");
    CHECK(history[2].methodName == "Warp");
    CHECK(history[2].args[0] == "1000");
    CHECK(history[2].args[1] == "2000");
}

TEST_CASE("PhaseGameWarpBridge - HandleWarp Remote") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    runtime.RegisterMock("network", "ConnectToWarpServer", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        return "Success";
    });

    Client::Bridge::PhaseGameWarpBridge bridge(runtime);

    TPacketGCWarp pack{};
    pack.header = 0x0305;
    pack.length = sizeof(TPacketGCWarp);
    pack.lX = 1000;
    pack.lY = 2000;
    pack.lAddr = 2130706433; // 127.0.0.1
    pack.wPort = 13000;

    std::span<const uint8_t> payload(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack));
    auto result = bridge.Handle(payload);
    CHECK(result.has_value() == true);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "network");
    CHECK(history[0].methodName == "ConnectToWarpServer");
    CHECK(history[0].args[0] == "2130706433");
    CHECK(history[0].args[1] == "13000");
}

TEST_CASE("PhaseGameWarpBridge - ShowMapName") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    runtime.RegisterMock("game", "ShowMapName", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> {
        return "Success";
    });

    Client::Bridge::PhaseGameWarpBridge bridge(runtime);

    bridge.ShowMapName("metin2_map_a1", 100, 200);

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 1);
    CHECK(history[0].moduleName == "game");
    CHECK(history[0].methodName == "ShowMapName");
    CHECK(history[0].args[0] == "metin2_map_a1");
    CHECK(history[0].args[1] == "100");
    CHECK(history[0].args[2] == "200");
}

TEST_CASE("PhaseGameWarpBridge - Phase Operations") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    runtime.RegisterMock("network", "SetGamePhase", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("character", "Destroy", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("item", "Destroy", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("network", "ClearActorManager", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });
    runtime.RegisterMock("player", "ClearPVPKeySystem", [](std::span<const std::string> args) -> EterBase::Result<std::string, Client::Bridge::PythonRuntimeError> { return "Success"; });

    Client::Bridge::PhaseGameWarpBridge bridge(runtime);

    bridge.SetGamePhase();
    bridge.LeaveGamePhase();

    auto history = runtime.GetHistory();
    REQUIRE(history.size() == 5);
    CHECK(history[0].moduleName == "network");
    CHECK(history[0].methodName == "SetGamePhase");
    CHECK(history[1].moduleName == "character");
    CHECK(history[1].methodName == "Destroy");
}

TEST_CASE("PhaseGameWarpBridge - Handle BufferUnderflow") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    Client::Bridge::PhaseGameWarpBridge bridge(runtime);

    uint8_t tinyPayload[1] = { 0x05 };
    std::span<const uint8_t> payload(tinyPayload, 1);
    
    auto result = bridge.Handle(payload);
    CHECK(result.has_value() == false);
    CHECK(result.error() == EterBase::PacketError::BufferUnderflow);
}

TEST_CASE("PhaseGameWarpBridge - IPacketHandler interface") {
    Client::Bridge::MockPythonRuntime runtime;
    runtime.Initialize();
    
    Client::Bridge::PhaseGameWarpBridge bridge(runtime);
    Client::Network::IPacketHandler& handler = bridge;

    CHECK(handler.GetExpectedSize() == sizeof(TPacketGCWarp));
    CHECK(handler.IsDynamicSize() == false);
}
