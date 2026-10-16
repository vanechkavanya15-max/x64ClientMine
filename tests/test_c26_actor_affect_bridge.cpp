#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include "Client/Actors/ActorAffectBridge.h"
#include "Client/Bridge/MockPythonRuntime.h"
#include <string>

using namespace Client::Actors;
using namespace Client::Bridge;

TEST_SUITE("ActorAffectBridge") {
    TEST_CASE("SetAffect - Success when PythonRuntime is available and method succeeds") {
        MockPythonRuntime mockRuntime;
        mockRuntime.Initialize();

        // Register a mock callback that returns success
        mockRuntime.RegisterMock("chrmgr", "SetAffect", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 3);
            CHECK(args[0] == "1234"); // vid
            CHECK(args[1] == "56");   // affectType
            CHECK(args[2] == "1");    // isVisible
            return "OK";
        });

        ActorAffectBridge bridge(&mockRuntime);
        auto result = bridge.SetAffect(1234, 56, true);

        CHECK(result.has_value());

        auto history = mockRuntime.GetHistory();
        REQUIRE(history.size() == 1);
        CHECK(history[0].moduleName == "chrmgr");
        CHECK(history[0].methodName == "SetAffect");
        REQUIRE(history[0].args.size() == 3);
        CHECK(history[0].args[0] == "1234");
        CHECK(history[0].args[1] == "56");
        CHECK(history[0].args[2] == "1");
    }

    TEST_CASE("SetAffect - Success with isVisible false") {
        MockPythonRuntime mockRuntime;
        mockRuntime.Initialize();

        mockRuntime.RegisterMock("chrmgr", "SetAffect", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            REQUIRE(args.size() == 3);
            CHECK(args[0] == "999");
            CHECK(args[1] == "42");
            CHECK(args[2] == "0");
            return "OK";
        });

        ActorAffectBridge bridge(&mockRuntime);
        auto result = bridge.SetAffect(999, 42, false);

        CHECK(result.has_value());
    }

    TEST_CASE("SetAffect - Fails when PythonRuntime is null") {
        ActorAffectBridge bridge(nullptr);
        auto result = bridge.SetAffect(1234, 56, true);

        REQUIRE(!result.has_value());
        CHECK(result.error() == ActorAffectError::PythonRuntimeNotAvailable);
    }

    TEST_CASE("SetAffect - Fails when Python execution fails") {
        MockPythonRuntime mockRuntime;
        mockRuntime.Initialize();

        // Register a mock callback that returns an error
        mockRuntime.RegisterMock("chrmgr", "SetAffect", [](std::span<const std::string> args) -> EterBase::Result<std::string, PythonRuntimeError> {
            return std::unexpected(PythonRuntimeError::ExecutionFailed);
        });

        ActorAffectBridge bridge(&mockRuntime);
        auto result = bridge.SetAffect(1234, 56, true);

        REQUIRE(!result.has_value());
        CHECK(result.error() == ActorAffectError::SetAffectFailed);
    }
}
