#ifndef TEST_MODE_DISABLE_STDAFX
#define TEST_MODE_DISABLE_STDAFX
#endif

#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

#include "Client/Bridge/PhaseGameFishingBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include "Client/Network/WorldPacketCodec.h"
#include <vector>

using namespace Client::Bridge;
using namespace Client::Network;

std::vector<PythonCallRecord> GetMethodCalls(const MockPythonRuntime& runtime, std::string_view moduleName, std::string_view methodName) {
    std::vector<PythonCallRecord> result;
    for (const auto& call : runtime.GetHistory()) {
        if (call.moduleName == moduleName && call.methodName == methodName) {
            result.push_back(call);
        }
    }
    return result;
}

TEST_CASE("PhaseGameFishingBridge SRP Isolation Test") {
    MockPythonRuntime mockRuntime;
    mockRuntime.Initialize();
    
    // Register mocks to avoid MethodNotFound error
    mockRuntime.RegisterMock("character", "StartFishing", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("character", "StopFishing", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("character", "ReactFishing", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("character", "CatchSuccess", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("character", "CatchFail", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("game", "OnFishingFailure", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("game", "OnFishingNotifyUnknown", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });
    mockRuntime.RegisterMock("game", "OnFishingNotifyOrSuccess", [](std::span<const std::string>) -> EterBase::Result<std::string, PythonRuntimeError> { return "OK"; });

    PhaseGameFishingBridge bridge(mockRuntime);

    SUBCASE("Check size properties") {
        CHECK(bridge.GetExpectedSize() == sizeof(TPacketGCFishing));
        CHECK(bridge.IsDynamicSize() == false);
    }

    SUBCASE("Handle START fishing") {
        TPacketGCFishing packet{};
        packet.header = 0; // Don't care
        packet.subheader = FishingSub::GC::START;
        packet.info = 1234;
        packet.dir = 10;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto calls = GetMethodCalls(mockRuntime, "character", "StartFishing");
        REQUIRE(calls.size() == 1);
        CHECK(calls[0].args[0] == "1234");
        CHECK(calls[0].args[1] == "50.000000"); // 10 * 5.0f
    }

    SUBCASE("Handle STOP fishing") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::STOP;
        packet.info = 1234;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto calls = GetMethodCalls(mockRuntime, "character", "StopFishing");
        REQUIRE(calls.size() == 1);
        CHECK(calls[0].args[0] == "1234");
    }
    
    SUBCASE("Handle REACT fishing") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::REACT;
        packet.info = 1234;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto calls = GetMethodCalls(mockRuntime, "character", "ReactFishing");
        REQUIRE(calls.size() == 1);
        CHECK(calls[0].args[0] == "1234");
    }

    SUBCASE("Handle SUCCESS fishing") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::SUCCESS;
        packet.info = 1234;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto calls = GetMethodCalls(mockRuntime, "character", "CatchSuccess");
        REQUIRE(calls.size() == 1);
        CHECK(calls[0].args[0] == "1234");
    }

    SUBCASE("Handle FAIL fishing") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::FAIL;
        packet.info = 1234;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto calls = GetMethodCalls(mockRuntime, "character", "CatchFail");
        REQUIRE(calls.size() == 1);
        CHECK(calls[0].args[0] == "1234");

        auto gameCalls = GetMethodCalls(mockRuntime, "game", "OnFishingFailure");
        REQUIRE(gameCalls.size() == 1);
    }

    SUBCASE("Handle FISH info == 0") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::FISH;
        packet.info = 0;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto gameCalls = GetMethodCalls(mockRuntime, "game", "OnFishingNotifyUnknown");
        REQUIRE(gameCalls.size() == 1);
    }

    SUBCASE("Handle FISH info != 0") {
        TPacketGCFishing packet{};
        packet.subheader = FishingSub::GC::FISH;
        packet.info = 500;
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK(res.has_value());

        auto gameCalls = GetMethodCalls(mockRuntime, "game", "OnFishingNotifyOrSuccess");
        REQUIRE(gameCalls.size() == 1);
        CHECK(gameCalls[0].args[0] == "500");
    }

    SUBCASE("Handle unknown subheader") {
        TPacketGCFishing packet{};
        packet.subheader = 99; // invalid
        
        std::vector<uint8_t> payload(sizeof(packet));
        std::memcpy(payload.data(), &packet, sizeof(packet));

        auto res = bridge.Handle(payload);
        CHECK_FALSE(res.has_value());
        CHECK(res.error() == EterBase::PacketError::InvalidHeader);
    }
}
