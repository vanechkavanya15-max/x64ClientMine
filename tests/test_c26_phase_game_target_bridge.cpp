#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include "Client/Bridge/PhaseGameTargetBridge.h"
#include "Client/Bridge/IPythonRuntime.h"
#include "Client/Network/Protocol/Protocol.h"
#include <vector>
#include <string>
#include <cstring>

namespace Client::Bridge::Test {

class MockPythonRuntime : public IPythonRuntime {
public:
    struct CallLog {
        std::string moduleName;
        std::string methodName;
        std::vector<std::string> args;
    };

    std::vector<CallLog> calls;

    void Initialize() noexcept override {}
    void Shutdown() noexcept override {}

    EterBase::Result<std::string, PythonRuntimeError> ExecuteMethod(
        std::string_view moduleName, 
        std::string_view methodName, 
        std::span<const std::string> args = {}) override
    {
        calls.push_back({std::string(moduleName), std::string(methodName), std::vector<std::string>(args.begin(), args.end())});
        return "";
    }
};

} // namespace Client::Bridge::Test

using namespace Client::Bridge;
using namespace Client::Bridge::Test;
using namespace Client::Network;

TEST_CASE("PhaseGameTargetBridge - TargetCreate") {
    MockPythonRuntime mockRuntime;
    PhaseGameTargetBridge bridge(mockRuntime);

    TPacketGCTargetCreate pack;
    pack.header = GC::TARGET_CREATE_NEW;
    pack.length = sizeof(pack);
    pack.lID = 123;
    std::strncpy(pack.szTargetName, "Enemy", sizeof(pack.szTargetName));

    auto result = bridge.HandleTargetCreate(std::span<const uint8_t>(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack)));
    REQUIRE(result.has_value());

    REQUIRE(mockRuntime.calls.size() == 2);
    
    CHECK(mockRuntime.calls[0].moduleName == "minimap");
    CHECK(mockRuntime.calls[0].methodName == "CreateTarget");
    CHECK(mockRuntime.calls[0].args.size() == 2);
    CHECK(mockRuntime.calls[0].args[0] == "123");
    CHECK(mockRuntime.calls[0].args[1] == "Enemy");

    CHECK(mockRuntime.calls[1].moduleName == "game");
    CHECK(mockRuntime.calls[1].methodName == "BINARY_OpenAtlasWindow");
    CHECK(mockRuntime.calls[1].args.empty());
}

TEST_CASE("PhaseGameTargetBridge - TargetUpdate") {
    MockPythonRuntime mockRuntime;
    PhaseGameTargetBridge bridge(mockRuntime);

    TPacketGCTargetUpdate pack;
    pack.header = GC::TARGET_UPDATE;
    pack.length = sizeof(pack);
    pack.lID = 456;
    pack.lX = 1000;
    pack.lY = 2000;

    auto result = bridge.HandleTargetUpdate(std::span<const uint8_t>(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack)));
    REQUIRE(result.has_value());

    REQUIRE(mockRuntime.calls.size() == 2);
    
    CHECK(mockRuntime.calls[0].moduleName == "minimap");
    CHECK(mockRuntime.calls[0].methodName == "UpdateTarget");
    CHECK(mockRuntime.calls[0].args.size() == 3);
    CHECK(mockRuntime.calls[0].args[0] == "456");
    CHECK(mockRuntime.calls[0].args[1] == "1000");
    CHECK(mockRuntime.calls[0].args[2] == "2000");

    CHECK(mockRuntime.calls[1].moduleName == "background");
    CHECK(mockRuntime.calls[1].methodName == "CreateTargetEffect");
    CHECK(mockRuntime.calls[1].args.size() == 3);
    CHECK(mockRuntime.calls[1].args[0] == "456");
    CHECK(mockRuntime.calls[1].args[1] == "1000");
    CHECK(mockRuntime.calls[1].args[2] == "2000");
}

TEST_CASE("PhaseGameTargetBridge - TargetDelete") {
    MockPythonRuntime mockRuntime;
    PhaseGameTargetBridge bridge(mockRuntime);

    TPacketGCTargetDelete pack;
    pack.header = GC::TARGET_DELETE;
    pack.length = sizeof(pack);
    pack.lID = 789;

    auto result = bridge.HandleTargetDelete(std::span<const uint8_t>(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack)));
    REQUIRE(result.has_value());

    REQUIRE(mockRuntime.calls.size() == 2);
    
    CHECK(mockRuntime.calls[0].moduleName == "minimap");
    CHECK(mockRuntime.calls[0].methodName == "DeleteTarget");
    CHECK(mockRuntime.calls[0].args.size() == 1);
    CHECK(mockRuntime.calls[0].args[0] == "789");

    CHECK(mockRuntime.calls[1].moduleName == "background");
    CHECK(mockRuntime.calls[1].methodName == "DeleteTargetEffect");
    CHECK(mockRuntime.calls[1].args.size() == 1);
    CHECK(mockRuntime.calls[1].args[0] == "789");
}

TEST_CASE("PhaseGameTargetBridge - CreateFly") {
    MockPythonRuntime mockRuntime;
    PhaseGameTargetBridge bridge(mockRuntime);

    TPacketGCCreateFly pack;
    pack.header = GC::CREATE_FLY;
    pack.length = sizeof(pack);
    pack.bType = 1;
    pack.dwStartVID = 10;
    pack.dwEndVID = 20;

    auto result = bridge.HandleCreateFly(std::span<const uint8_t>(reinterpret_cast<const uint8_t*>(&pack), sizeof(pack)));
    REQUIRE(result.has_value());

    REQUIRE(mockRuntime.calls.size() == 1);
    
    CHECK(mockRuntime.calls[0].moduleName == "fly_manager");
    CHECK(mockRuntime.calls[0].methodName == "CreateIndexedFly");
    CHECK(mockRuntime.calls[0].args.size() == 3);
    CHECK(mockRuntime.calls[0].args[0] == "1");
    CHECK(mockRuntime.calls[0].args[1] == "10");
    CHECK(mockRuntime.calls[0].args[2] == "20");
}
